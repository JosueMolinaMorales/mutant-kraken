use std::time::Instant;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    io::BufRead,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
};

use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;

use cli_table::{Table, WithTitle};

use crate::cli::MutationCommandConfig;
use crate::config::MutantKrakenConfig;
use crate::error::{self, MutantKrakenError, Result};
use crate::mutation_tool::{
    mutation::{FileMutations, Mutation, MutationResult},
    MutationOperators,
};
use crate::{gradle, html_gen};

use super::MutationToolBuilder;

const MAX_BUILD_THREADS: f32 = 5f32;

pub struct MutationTool {
    parser: Arc<Mutex<tree_sitter::Parser>>,
    pub mutate_config: MutationCommandConfig,
    pub mutation_operators: Arc<Vec<MutationOperators>>,
    pub output_directory: String,
    pub mutation_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub enable_mutation_comment: bool,
    thread_pool: rayon::ThreadPool,
    pub mutantkraken_config: MutantKrakenConfig,
}

impl Default for MutationTool {
    fn default() -> Self {
        MutationToolBuilder::new().build()
    }
}

impl MutationTool {
    /// Creates a new instance of the mutation tool with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `mutate_config` - The mutation command configuration.
    /// * `mutantkraken_config` - The MutantKraken configuration.
    /// * `output_directory` - The output directory for the mutated files.
    /// * `mutation_operators` - The mutation operators to use.
    /// * `enable_mutation_comment` - Whether to enable mutation comments in the mutated files.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new instance of the mutation tool, or an error if one occurred.
    pub fn new(
        mutate_config: MutationCommandConfig,
        mutantkraken_config: MutantKrakenConfig,
        output_directory: String,
        mutation_operators: Vec<MutationOperators>,
        enable_mutation_comment: bool,
    ) -> Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_kotlin::language())
            .map_err(|e| {
                MutantKrakenError::Error(format!("Error while setting language: {}", e))
            })?;

        // Validate config path
        // Check if it exists
        let config_path = Path::new(&mutate_config.path);
        if !config_path.exists() {
            return Err(MutantKrakenError::Error("Path does not exist".into()));
        }
        if !config_path.is_dir() {
            return Err(MutantKrakenError::Error("Path is not a directory".into()));
        }

        // Create directories
        // If the Backups directory exists, delete it
        let backup_dir = Path::new(&output_directory).join("backups");
        if backup_dir.exists() {
            std::fs::remove_dir_all(&backup_dir).expect("Could not delete backup directory");
        }
        // If the mutations directory exists, delete it
        let mutation_dir = Path::new(&output_directory).join("mutations");
        if mutation_dir.exists() {
            std::fs::remove_dir_all(&mutation_dir).expect("Could not delete mutations directory");
        }
        if !mutation_dir.exists() {
            fs::create_dir_all(&mutation_dir)?;
        }
        if !backup_dir.exists() {
            fs::create_dir(&backup_dir)?;
        }

        // Create thread pool
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(mutantkraken_config.threading.max_threads)
            .build()
            .map_err(|e| {
                MutantKrakenError::Error(format!("Error while creating thread pool: {}", e))
            })?;

        Ok(Self {
            mutate_config,
            parser: Arc::new(Mutex::new(parser)),
            mutation_operators: Arc::new(mutation_operators),
            output_directory,
            mutation_dir,
            backup_dir,
            enable_mutation_comment,
            thread_pool,
            mutantkraken_config,
        })
    }

    /// Creates a mutated file name by appending the mutation ID to the original file name.
    ///
    /// # Arguments
    ///
    /// * `file_name` - A string slice that holds the name of the original file.
    /// * `mutation` - A reference to a `Mutation` struct that holds the mutation ID.
    ///
    /// # Returns
    ///
    /// A `Result` that contains a string with the mutated file name if successful, or a `MutantKrakenError` if an error occurs.
    fn create_mutated_file_name(&self, file_name: &str, mutation: &Mutation) -> Result<String> {
        Ok(format!(
            "{}_{}",
            mutation.id,
            Path::new(&file_name)
                .file_name()
                .ok_or(MutantKrakenError::Error(
                    "Error Creating Mutated File Name".into()
                ))?
                .to_str()
                .ok_or(MutantKrakenError::Error(
                    "Eror Creating Mutated File Name".into()
                ))?
        ))
    }

    /// Mutates the project by gathering files, gathering mutations per file, generating mutations per file,
    /// building and testing, reporting results, saving results in csv, and generating an HTML report.
    pub fn mutate(&mut self) -> Result<()> {
        tracing::info!("Mutation tool started...");
        // Phase 1: Get files from project
        println!("[1/6] 📂 Gathering files...");
        let mut existing_files = self.get_files_from_project()?;
        // Phase 2: Gather mutations per file
        println!("[2/6] 🔎 Gathering mutations...");
        let file_mutations = self.gather_mutations_per_file(&mut existing_files)?;
        // Store all mutations in a json file
        self.store_mutations(&file_mutations)?;
        // Phase 3: Generate mutations per file
        println!("[3/6] 🔨 Generating mutations...");
        self.generate_mutations_per_file(&file_mutations)?;
        // Phase 4: Build and test
        println!("[4/6] 🏗 Building and testing...");
        let mutations = self.build_and_test(&file_mutations)?;
        // Phase 5: Report results
        println!("[5/6] 📊 Reporting results...");
        self.report_results(&mutations)?;
        // Phase 6: Save Results in csv
        println!("[6/6] 💾 Saving results...");
        self.save_results(&mutations)?;
        // Phase 7: Generate HTML Report
        println!("[7/7] 📊 Generating HTML report...");
        html_gen::build_html_page(&mutations, &Path::new(self.output_directory.as_str()));
        Ok(())
    }

    /// Store All Mutations into a json filed name mutations.json within mutant-kraken-dist/mutations directory
    fn store_mutations(&self, file_mutations: &HashMap<String, FileMutations>) -> Result<()> {
        std::fs::write(
            Path::new(self.output_directory.as_str())
                .join("mutations")
                .join("mutations.json"),
            serde_json::to_string_pretty(file_mutations)
                .map_err(|_| error::MutantKrakenError::ConversionError)?,
        )
        .map_err(|e| error::MutantKrakenError::Error(e.to_string()))?;

        Ok(())
    }

    /// Saves the given mutations to a CSV file located in the `OUT_DIRECTORY`.
    ///
    /// # Arguments
    ///
    /// * `mutations` - A reference to a vector of `Mutation` structs to be saved.
    ///
    /// # Errors
    ///
    /// Returns a `MutantKrakenError` if there was an error creating the CSV writer, serializing a mutation,
    /// flushing the CSV writer, or writing to the output file.
    ///
    fn save_results(&self, mutations: &Vec<Mutation>) -> Result<()> {
        let mut writer = csv::WriterBuilder::new()
            .from_path(Path::new(self.output_directory.as_str()).join("output.csv"))
            .map_err(|e| {
                MutantKrakenError::Error(format!("Error while creating csv writer: {}", e))
            })?;
        for mutation in mutations {
            writer.serialize(mutation).map_err(|e| {
                MutantKrakenError::Error(format!("Error while serializing mutation: {}", e))
            })?;
        }
        writer.flush().map_err(|e| {
            MutantKrakenError::Error(format!("Error while flushing csv writer: {}", e))
        })?;
        Ok(())
    }

    /// This function retrieves all files from the project directory specified in the `mutate_config` field of the `Tool` struct.
    /// It returns a `Result` containing a `Vec` of `String`s representing the file paths.
    /// If no files are found in the directory, it returns an `Err` containing a `MutantKrakenError`.
    fn get_files_from_project(&self) -> Result<Vec<String>> {
        let start = Instant::now();
        let mut existing_files: Vec<String> = vec![];
        self.get_files_from_directory(self.mutate_config.path.clone(), &mut existing_files)?;
        let end = Instant::now();
        tracing::info!(
            "Gathering files took {} seconds",
            end.duration_since(start).as_secs()
        );
        tracing::debug!(
            "Found a total of {} files. Files found from path: {:#?}",
            existing_files.len(),
            existing_files
        );
        tracing::info!("Gathering all mutations for files...");
        if existing_files.is_empty() {
            tracing::error!("No files found in path");
            return Err(MutantKrakenError::Error("No files found in path".into()));
        }
        Ok(existing_files)
    }

    fn report_results(&self, mutations: &Vec<Mutation>) -> Result<()> {
        let mut total_mutations = 0;
        let mut total_killed_mutants = 0;
        let mut total_survived_mutants = 0;
        let mut total_timeouts_or_build_fails = 0;
        total_mutations += mutations.len();
        mutations.iter().for_each(|m| match m.result {
            MutationResult::Killed => total_killed_mutants += 1,
            MutationResult::Survived => total_survived_mutants += 1,
            _ => total_timeouts_or_build_fails += 1,
        });
        if self.mutantkraken_config.output.display_end_table {
            cli_table::print_stdout(mutations.with_title()).map_err(|e| {
                MutantKrakenError::Error(format!("Error while printing table: {}", e))
            })?;
        }
        let table = vec![
            vec!["Total mutations".to_string(), total_mutations.to_string()],
            vec![
                "Total killed mutants".to_string(),
                total_killed_mutants.to_string(),
            ],
            vec![
                "Total survived mutants".to_string(),
                total_survived_mutants.to_string(),
            ],
            vec![
                "Total timeouts or build fails".to_string(),
                total_timeouts_or_build_fails.to_string(),
            ],
            vec![
                "Mutation score".to_string(),
                format!(
                    "{}%",
                    (total_killed_mutants as f32
                        / (total_killed_mutants + total_survived_mutants) as f32)
                        * 100.0
                ),
            ],
        ]
        .table();
        cli_table::print_stdout(table)?;
        Ok(())
    }

    fn copy_files(&self, file_mutations: &HashMap<String, FileMutations>) -> Result<()> {
        // Make Copies of all files
        for (file_name, _) in file_mutations.iter() {
            let original_file_path = Path::new(file_name).to_path_buf();
            let original_file_name = original_file_path
                .file_name()
                .ok_or(MutantKrakenError::MutationBuildTestError)?
                .to_str()
                .ok_or(MutantKrakenError::MutationBuildTestError)?;
            let backup_path = self.backup_dir.join(original_file_name);
            // Save a copy of the original file
            fs::copy(&original_file_path, &backup_path)?;
        }
        Ok(())
    }

    /// Builds and tests mutated files in parallel using a thread pool.
    ///
    /// # Arguments
    ///
    /// * `file_mutations` - A hashmap containing the mutations for each file.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Mutation` structs.
    ///
    fn build_and_test(
        &mut self,
        file_mutations: &HashMap<String, FileMutations>,
    ) -> Result<Vec<Mutation>> {
        self.gradle_checks()?;

        // Get total number of mutations
        let num_mutations = file_mutations
            .iter()
            .fold(0, |acc, (_, fm)| acc + fm.mutations.len());
        // Set up progress bar
        let progress_bar = create_progress_bar(num_mutations)?;

        // Make Copies of all files
        self.copy_files(file_mutations)?;

        // Merge all mutants into one vector
        let mut chunks = create_mutation_chucks(file_mutations);

        // Set up threading
        let path = Arc::new(self.mutate_config.path.clone());
        let mutation_dir = Arc::new(self.mutation_dir.clone());
        let backup_dir = Arc::new(self.backup_dir.clone());
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(chunks.len())
            .build()
            .map_err(|e| MutantKrakenError::Error(e.to_string()))?;
        thread_pool.scope(|s| {
            for chunck in chunks.iter_mut() {
                // Create unique temp directory
                let uuid = uuid::Uuid::new_v4();
                let mut td =
                    Path::new(self.output_directory.as_str()).join(format!("temp/{}", uuid));
                fs::create_dir_all(&td).expect("Failed to create temp directory");
                // Create directory structure inside temp directory that matches the original project
                let dir = PathBuf::from(&self.mutate_config.path);
                let mut config_prefix = PathBuf::new();
                for c in dir.components() {
                    if let Component::Normal(dir) = c {
                        td = td.join(dir);
                        config_prefix = config_prefix.join(dir);
                    }
                }
                fs::create_dir_all(&td).expect("Failed to create temp directory");
                self.create_temp_directory(dir, &td)
                    .expect("Failed to create temp directory");
                // Run gradle build and tests in parallel
                let path = path.clone();
                let mutation_dir = mutation_dir.clone();
                let backup_dir = backup_dir.clone();
                let progress_bar = progress_bar.clone();
                s.spawn(move |_| {
                    chunck.iter_mut().for_each(|mutation| {
                        let original_file_name = mutation.file_name.clone();
                        let file_name = Path::new(&original_file_name)
                            .strip_prefix(path.as_ref())
                            .expect("Failed to strip prefix");
                        let original_file_path =
                            PathBuf::from(format!("{}/{}", td.display(), file_name.display()));

                        progress_bar.inc(1);
                        let mutated_file_path = mutation_dir.join(format!(
                            "{}_{}",
                            mutation.id,
                            Path::new(&file_name)
                                .file_name()
                                .expect("Failed to get the filename")
                                .to_str()
                                .expect("Failed to convert file name to string")
                        ));

                        if let Err(err) = gradle::run(
                            &PathBuf::from(&td),
                            &mutated_file_path,
                            &original_file_path,
                            mutation,
                        ) {
                            tracing::error!("An error occurred building and testing: {}", err);
                            mutation.result = MutationResult::BuildFailed;
                        }
                        let backup_path = backup_dir.join(
                            Path::new(&file_name)
                                .file_name()
                                .expect("Failed to convert file name to string")
                                .to_str()
                                .expect("Failed to convert file name to string"),
                        );
                        // Restore original file
                        fs::copy(backup_path, &original_file_path)
                            .expect("Failed to restore original file");
                    });
                });
            }
        });
        progress_bar.finish();
        // Delete temp directory
        if let Err(err) = fs::remove_dir_all(Path::new(self.output_directory.as_str()).join("temp"))
        {
            println!("[ERROR] Failed to delete mutant-kraken-dist/temp directory. Please view logs for more information.");
            tracing::error!(
                "Failed to delete mutant-kraken-dist/temp directory: {}",
                err
            );
        }
        Ok(chunks.into_iter().flatten().collect())
    }

    fn gradle_checks(&mut self) -> Result<()> {
        let path = PathBuf::from(&self.mutate_config.path);
        if !gradle::build_project_success(&path)? {
            return Err(MutantKrakenError::Error(
                "Project does not build successfully. Please fix the errors and try again.".into(),
            ));
        }

        Ok(if !gradle::project_tests_pass(&path)? {
            return Err(MutantKrakenError::Error(
                "Project tests do not pass. Please fix the errors and try again.".into(),
            ));
        })
    }

    /// Recursively creates a temporary directory and copies all files from the given directory into it,
    /// excluding the "mutant-kraken-dist" folder. If a file is named "gradlew" or "gradlew.bat", it is copied
    /// to the temporary directory with the same permissions. All other files are written to the temporary
    /// directory with their original contents.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to copy files from.
    /// * `temp_dir` - The temporary directory to create and copy files into.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation is successful, or an error if any file operations fail.
    fn create_temp_directory(&self, dir: PathBuf, temp_dir: &Path) -> Result<()> {
        for entry in dir.read_dir()? {
            let path = entry?.path();
            let file_name = path
                .file_name()
                .ok_or(MutantKrakenError::ConversionError)?
                .to_str()
                .ok_or(MutantKrakenError::ConversionError)?;
            // Ignore the mutant-kraken-dist folder
            // We can add to this to ignore more things.
            if file_name == "mutant-kraken-dist" {
                continue;
            }
            if path.is_dir() {
                let temp_dir = temp_dir.join(file_name);
                fs::create_dir(&temp_dir)?;
                self.create_temp_directory(path, &temp_dir)?;
            } else {
                let file_contents = fs::read(&path)?;
                if file_name == "gradlew" || file_name == "gradlew.bat" {
                    // We copy here so that we keep the same permissions
                    fs::copy(&path, temp_dir.join(file_name))?;
                } else {
                    fs::write(temp_dir.join(file_name), file_contents)?;
                }
            }
        }
        Ok(())
    }

    /// Generates mutations for each file in the given `file_mutations` HashMap.
    /// Each mutation is applied to the corresponding file, and the resulting mutated file is written to
    /// the output directory specified in the `MutationTool` configuration.
    fn generate_mutations_per_file(
        &self,
        file_mutations: &HashMap<String, FileMutations>,
    ) -> Result<()> {
        tracing::info!("Generating mutations per file");
        let start = Instant::now();
        self.thread_pool.scope(|s| {
            // Iterate over each file and generate mutations
            file_mutations.iter().for_each(|(file_name, fm)| {
                let file_str = fs::read_to_string(file_name).expect("Failed to read file");
                s.spawn(move |_| {
                    // Iterate over each mutation and apply it to the file
                    fm.mutations.iter().for_each(|m| {
                        let new_op_bytes = m.new_op.as_bytes();
                        let mut file = file_str.as_bytes().to_vec();

                        // Add the mutation to the vector of bytes
                        file.splice(m.start_byte..m.end_byte, new_op_bytes.iter().cloned());
                        // Add comment above mutation about the mutation
                        let file = file
                            .lines()
                            .enumerate()
                            .map(|(i, line)| {
                                let mut line = line.expect("Failed to convert line to string");
                                if i == m.line_number - 1 && self.enable_mutation_comment {
                                    line = format!("{}\n{}", m, line);
                                }
                                line
                            })
                            .collect::<Vec<String>>()
                            .join("\n");

                        // Create a file name for the mutated file
                        let mutated_file_name = self.mutation_dir.join(
                            self.create_mutated_file_name(file_name, m)
                                .expect("Failed to create mutated file name"),
                        );
                        // Write the mutated file to the output directory
                        fs::write(mutated_file_name, file).expect("Failed to write mutated file");
                    });
                });
            });
        });
        let end = Instant::now();
        tracing::info!(
            "Generating mutations per file took: {}",
            end.duration_since(start).as_secs_f64()
        );
        Ok(())
    }

    /// Gathers mutations for each file in the given list of existing files.
    ///
    /// # Arguments
    ///
    /// * `existing_files` - A mutable slice of strings representing the paths to the existing files.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `HashMap` of `String` keys and `FileMutations` values, or an error if the operation fails.
    ///
    fn gather_mutations_per_file(
        &mut self,
        existing_files: &mut [String],
    ) -> Result<HashMap<String, FileMutations>> {
        // Record the start time for measuring performance
        let start = Instant::now();

        // Create shared mutable state for collecting mutations and mutation count
        let file_mutations: Arc<Mutex<HashMap<String, FileMutations>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let mutation_count = Arc::new(Mutex::new(0));
        // Use thread pool to parallelize mutation gathering for each file
        self.thread_pool.scope(|s| {
            for file in existing_files {
                // Clone shared state for each thread
                let mutation_count = mutation_count.clone();
                let file_mutations = file_mutations.clone();
                let parser = self.parser.clone();
                let mutation_operators = self.mutation_operators.clone();

                // Spawn a thread for each file
                s.spawn(move |_| {
                    // Parse the file content using the parser
                    let ast = parser
                        .lock()
                        .expect("Failed to lock parser")
                        .parse(fs::read_to_string(&file).expect("File Not Found!"), None)
                        .expect("Parsing file failed");

                    // Iterate through mutation operators to find mutations
                    for mut_op in mutation_operators.iter() {
                        // Get a list of mutations that can be made
                        let mutations = mut_op.find_mutation(&ast, file);

                        // Update mutation count and file mutations
                        *mutation_count
                            .lock()
                            .expect("Failed to lock mutation_count var") += mutations.len();
                        file_mutations
                            .lock()
                            .expect("Failed to lock file_mutations var")
                            .entry(file.clone())
                            .and_modify(|fm| fm.mutations.extend(mutations.clone()))
                            .or_insert(FileMutations {
                                mutations: mutations.clone(),
                            });
                    }
                });
            }
        });

        // Record the end time and log the time taken to gather mutations
        let end = Instant::now();
        tracing::info!(
            "Time to gather mutations: {}",
            end.duration_since(start).as_secs()
        );

        // Unwrap and log the total mutation count
        let mutation_count = Arc::try_unwrap(mutation_count)
            .map_err(|_| MutantKrakenError::Error("Failed to unwrap mutation_count".to_string()))?
            .into_inner()
            .map_err(|_| MutantKrakenError::Error("Failed to unwrap mutation_count".to_string()))?;
        tracing::info!("Mutations made to all files");
        tracing::info!("Total mutations made: {}", mutation_count);

        // Return an error if no mutations were found, otherwise return the collected file mutations
        if mutation_count == 0 {
            return Err(MutantKrakenError::Error(
                "No mutations were found in the project".into(),
            ));
        }
        Ok(Arc::try_unwrap(file_mutations)
            .map_err(|_| MutantKrakenError::Error("Failed to unwrap file_mutations".to_string()))?
            .into_inner()
            .map_err(|_| MutantKrakenError::Error("Failed to unwrap file_mutations".to_string()))?)
    }

    /// Gets all files from the given directory and adds them to the given vector.
    /// This function is recursive, so it will also get files from subdirectories.
    /// It will ignore files and directories that match the ignore patterns in the `MutantKrakenConfig`.
    /// It will also ignore the `mutant-kraken-dist` directory.
    /// If the given path is not a directory, it will return an error.
    fn get_files_from_directory(
        &self,
        path: String,
        existing_files: &mut Vec<String>,
    ) -> Result<()> {
        // Open the directory at the given path
        let directory = Path::new(path.as_str())
            .read_dir()
            .map_err(|_| MutantKrakenError::MutationGatheringError)?;

        // Iterate over entries in the directory
        for entry in directory {
            // Handle potential errors when reading directory entries
            let entry = entry.map_err(|_| MutantKrakenError::MutationGatheringError)?;
            let path = entry.path();

            // Skip processing if the entry corresponds to a specific directory
            if path
                .file_name()
                .ok_or(MutantKrakenError::MutationGatheringError)?
                .to_str()
                .ok_or(MutantKrakenError::MutationGatheringError)?
                == "mutant-kraken-dist"
            {
                continue;
            }

            // Recursively process subdirectories
            if path.is_dir() {
                self.get_files_from_directory(
                    path.to_str()
                        .ok_or(MutantKrakenError::MutationGatheringError)?
                        .to_string(),
                    existing_files,
                )?;
                continue;
            }

            // Skip files with extensions other than "kt"
            if path.extension() != Some("kt".as_ref()) {
                continue;
            }

            // Skip files in ignored directories
            if path.components().any(|p| {
                self.mutantkraken_config
                    .ignore
                    .ignore_directories
                    .iter()
                    .map(|ignore_dirs| Component::Normal(OsStr::new(ignore_dirs)))
                    .any(|ignore_dir| p == ignore_dir)
            }) {
                continue;
            }

            let file_name = entry.file_name();

            // Skip files matching the specified regex patterns
            if self
                .mutantkraken_config
                .ignore
                .ignore_files
                .iter()
                .any(|ignore_file| {
                    Regex::new(ignore_file)
                        .expect("Failed to convert given regex")
                        .is_match(
                            file_name
                                .to_str()
                                .expect("Failed to convert file name to string"),
                        )
                })
            {
                continue;
            }

            // Add the path to the list of existing files
            existing_files.push(
                path.to_str()
                    .ok_or(MutantKrakenError::Error(
                        "Failed to convert os str to string".into(),
                    ))?
                    .to_string(),
            );
        }

        Ok(())
    }
}

fn create_mutation_chucks(file_mutations: &HashMap<String, FileMutations>) -> Vec<Vec<Mutation>> {
    // Merge all mutants into one vector
    let all_mutations: Vec<Mutation> = file_mutations
        .values()
        .flat_map(|fm| fm.mutations.clone())
        .collect();

    // Partition the mutations into chunks
    let chunk_size = ((all_mutations.len() as f32) / MAX_BUILD_THREADS).ceil() as usize;
    let chunks: Vec<Vec<Mutation>> = all_mutations
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect();
    chunks
}

fn create_progress_bar(num_mutations: usize) -> Result<Arc<ProgressBar>> {
    let progress_bar = Arc::new(ProgressBar::new(num_mutations as u64));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} - Running tests...",
            )
            .map_err(|e| MutantKrakenError::Error(e.to_string()))?
            .progress_chars("=> "),
    );
    Ok(progress_bar)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use uuid::Uuid;

    use crate::mutation_tool::test_util::*;

    use super::*;

    fn create_temp_directory(file_contents: &str) -> (Uuid, String) {
        let mutation_test_id = Uuid::new_v4();
        let file_id = format!("./{}/{}.kt", mutation_test_id, mutation_test_id);
        let output_directory = format!("./{}/mutations/", mutation_test_id);
        // Create temp output directory
        fs::create_dir_all(&output_directory).unwrap();
        // Add test files to directory
        fs::write(file_id, file_contents).unwrap();

        (mutation_test_id, output_directory)
    }

    fn remove_directory(mutation_test_id: Uuid) {
        fs::remove_dir_all(format!("./{}", mutation_test_id)).unwrap()
    }

    fn create_mutator_with_specific_operators(
        mutation_test_id: Uuid,
        output_directory: String,
        operators: Vec<MutationOperators>,
    ) -> MutationTool {
        MutationTool::new(
            MutationCommandConfig {
                path: format!("./{}", mutation_test_id),
            },
            MutantKrakenConfig::default(),
            output_directory,
            operators,
            false,
        )
        .unwrap()
    }

    fn get_mutated_file_name(file_name: &str, m: &Mutation, output_directory: String) -> PathBuf {
        Path::new(&output_directory).join("mutations").join(format!(
            "{}_{}",
            m.id,
            Path::new(&file_name).file_name().unwrap().to_str().unwrap()
        ))
    }

    fn assert_all_mutation_files_were_created(
        mutator: &mut MutationTool,
        mutation_test_id: Uuid,
        output_directory: String,
    ) {
        let fm = mutator
            .gather_mutations_per_file(&mut mutator.get_files_from_project().unwrap())
            .unwrap();
        mutator.generate_mutations_per_file(&fm).unwrap();
        // Check that the mutated files were created
        for (file_name, fm) in fm {
            for m in fm.mutations.clone() {
                let mutated_file_name =
                    get_mutated_file_name(&file_name, &m, output_directory.clone());
                assert!(Path::new(mutated_file_name.to_str().unwrap()).exists());
            }
        }
        // Remove directory
        remove_directory(mutation_test_id);
    }

    fn assert_all_mutations_are_correct(
        mutator: &mut MutationTool,
        mutation_test_id: Uuid,
        output_directory: String,
    ) {
        let fm = mutator
            .gather_mutations_per_file(&mut mutator.get_files_from_project().unwrap())
            .unwrap();
        mutator.generate_mutations_per_file(&fm).unwrap();
        // Check that the mutated files were created
        for (file_name, fm) in fm {
            for m in fm.mutations {
                let mutated_file_name =
                    get_mutated_file_name(&file_name, &m, output_directory.clone());
                let mut_file = fs::read_to_string(mutated_file_name)
                    .unwrap()
                    .as_bytes()
                    .to_vec();
                let new_op_len = m.new_op.as_bytes().len();
                let mut_range = m.start_byte..(m.start_byte + new_op_len);
                // Checks that the mutated file does not have the same contents as the original file
                // Print out strings
                assert_eq!(m.new_op.as_bytes().to_vec(), mut_file[mut_range].to_vec());
            }
        }
        // Remove contents in temp directory
        remove_directory(mutation_test_id);
    }

    // #[test]
    // fn test_tool_exists_if_no_mutable_files_found() {
    //     todo!()
    // }

    // #[test]
    // fn test_tool_exists_if_no_mutations_are_made() {
    //     todo!()
    // }

    #[test]
    fn test_mutate_arithmetic_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ArithmeticReplacementOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_arithmetic_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ArithmeticReplacementOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_assignment_mutated_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_ASSIGNMENT_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::AssignmentReplacementOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_assignment_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_ASSIGNMENT_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::AssignmentReplacementOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_logical_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LOGICAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::LogicalReplacementOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_logical_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LOGICAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::LogicalReplacementOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_relational_mutated_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_RELATIONAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::RelationalReplacementOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_relational_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_RELATIONAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::RelationalReplacementOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_unary_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_UNARY_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::UnaryReplacementOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_unary_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_UNARY_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::UnaryReplacementOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_unary_removal_mutated_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::UnaryRemovalOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_unary_removal_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::UnaryRemovalOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_null_assertion_mutated_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_TEST_NULL_ASSERTION_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::NotNullAssertionOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_no_null_assertion_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_TEST_NULL_ASSERTION_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::NotNullAssertionOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_elvis_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_ELVIS_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ElvisRemoveOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_elvis_remove_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_ELVIS_LITERAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ElvisRemoveOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_elvis_literal_change_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_ELVIS_LITERAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ElvisLiteralChangeOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_elvis_literal_change_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_ELVIS_LITERAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ElvisLiteralChangeOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_literal_change_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LITERAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::LiteralChangeOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_literal_remove_mutations_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LITERAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::LiteralChangeOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_exception_change_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_EXCEPTION_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ExceptionChangeOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_exception_change_mutations_file_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_EXCEPTION_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ExceptionChangeOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_when_branch_remove_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_WHEN_EXPRESSION_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::WhenRemoveBranchOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_when_branch_remove_mutations_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_WHEN_EXPRESSION_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::WhenRemoveBranchOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_label_remove_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_LABEL_REMOVING_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::RemoveLabelOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_label_remove_mutations_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_LABEL_REMOVING_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::RemoveLabelOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_functional_binary_replacement_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_FUNCTIONAL_BINARY_REPLACEMENT_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::FunctionalBinaryReplacementOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_functional_binary_replacement_mutations_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_FUNCTIONAL_BINARY_REPLACEMENT_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::FunctionalBinaryReplacementOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_functional_replacement_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_FUNCTIONAL_REPLACEMENT_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::FunctionalReplacementOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_functional_replacement_mutations_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_FUNCTIONAL_REPLACEMENT_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::FunctionalReplacementOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }
}
