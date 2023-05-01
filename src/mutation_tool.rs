use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    io::BufRead,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    error::{KodeKrakenError, Result},
    gradle,
    mutation::{FileMutations, Mutation, MutationResult},
    mutation_operators::{AllMutationOperators, MutationOperators},
    MutationCommandConfig,
};

use cli_table::{Table, WithTitle};

const OUT_DIRECTORY: &str = "./kode-kraken-dist";
const MAX_BUILD_THREADS: f32 = 5f32;

pub struct MutationToolBuilder {
    verbose: bool,
    config: Option<MutationCommandConfig>,
    mutation_operators: Option<Vec<MutationOperators>>,
    enable_mutation_comment: bool,
    thread_count: Option<usize>,
}

impl Default for MutationToolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MutationToolBuilder {
    pub fn new() -> Self {
        Self {
            verbose: false,
            config: None,
            mutation_operators: None,
            enable_mutation_comment: false,
            thread_count: None,
        }
    }
    pub fn set_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    pub fn set_config(mut self, config: MutationCommandConfig) -> Self {
        self.config = Some(config);
        self
    }
    pub fn set_mutation_operators(mut self, mutation_operators: Vec<MutationOperators>) -> Self {
        self.mutation_operators = Some(mutation_operators);
        self
    }
    pub fn set_mutation_comment(mut self, enable_mutation_comment: bool) -> Self {
        self.enable_mutation_comment = enable_mutation_comment;
        self
    }
    pub fn set_thread_count(mut self, thread_count: usize) -> Self {
        self.thread_count = Some(thread_count);
        self
    }

    pub fn build(self) -> MutationTool {
        let config = self.config.unwrap_or_default();
        let mutation_operators = self
            .mutation_operators
            .unwrap_or(AllMutationOperators::new().get_mutation_operators());
        let thread_count = self.thread_count.unwrap_or(30);
        MutationTool::new(
            self.verbose,
            config,
            OUT_DIRECTORY.into(),
            mutation_operators,
            self.enable_mutation_comment,
            thread_count,
        )
        .unwrap()
    }
}

pub struct MutationTool {
    parser: Arc<Mutex<tree_sitter::Parser>>,
    verbose: bool,
    config: MutationCommandConfig,
    mutation_operators: Arc<Vec<MutationOperators>>,
    mutation_dir: PathBuf,
    backup_dir: PathBuf,
    enable_mutation_comment: bool,
    thread_pool: rayon::ThreadPool,
}

impl Default for MutationTool {
    fn default() -> Self {
        Self::new(
            false,
            MutationCommandConfig::default(),
            OUT_DIRECTORY.into(),
            AllMutationOperators::new().get_mutation_operators(),
            false,
            30,
        )
        .unwrap()
    }
}

impl MutationTool {
    fn new(
        verbose: bool,
        config: MutationCommandConfig,
        output_directory: String,
        mutation_operators: Vec<MutationOperators>,
        enable_mutation_comment: bool,
        thread_count: usize,
    ) -> Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_kotlin::language())
            .map_err(|e| KodeKrakenError::Error(format!("Error while setting language: {}", e)))?;

        // Validate config path
        // Check if it exists
        let config_path = Path::new(&config.path);
        if !config_path.exists() {
            return Err(KodeKrakenError::Error("Path does not exist".into()));
        }
        if !config_path.is_dir() {
            return Err(KodeKrakenError::Error("Path is not a directory".into()));
        }

        // If output directory exists, clear it
        if Path::new(&output_directory).exists() {
            fs::remove_dir_all(&output_directory)?; // TODO: Remove unwrap
        }
        // Create directories
        let mutation_dir = Path::new(&output_directory).join("mutations");
        let backup_dir = Path::new(&output_directory).join("backups");
        if !mutation_dir.exists() {
            fs::create_dir_all(&mutation_dir)?; // TODO: Remove unwrap
        }
        if !backup_dir.exists() {
            fs::create_dir(&backup_dir)?; // TODO: Remove unwrap
        }

        // Create thread pool
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build()
            .unwrap();

        Ok(Self {
            verbose,
            config,
            parser: Arc::new(Mutex::new(parser)),
            mutation_operators: Arc::new(mutation_operators),
            mutation_dir,
            backup_dir,
            enable_mutation_comment,
            thread_pool,
        })
    }

    fn create_mutated_file_name(&self, file_name: &str, mutation: &Mutation) -> Result<String> {
        Ok(format!(
            "{}_{}",
            mutation.id,
            Path::new(&file_name)
                .file_name()
                .ok_or(KodeKrakenError::Error(
                    "Error Creating Mutated File Name".into()
                ))?
                .to_str()
                .ok_or(KodeKrakenError::Error(
                    "Eror Creating Mutated File Name".into()
                ))? // TODO: Remove unwrap
        ))
    }

    pub fn mutate(&mut self) -> Result<()> {
        tracing::info!("Mutation tool started...");
        // Phase 1: Get files from project
        println!("[1/6] 📂 Gathering files...");
        let mut existing_files = self.get_files_from_project();
        // Phase 2: Gather mutations per file
        println!("[2/6] 🔎 Gathering mutations...");
        let file_mutations = self.gather_mutations_per_file(&mut existing_files).unwrap();
        // Phase 3: Generate mutations per file
        println!("[3/6] 🔨 Generating mutations...");
        self.generate_mutations_per_file(&file_mutations).unwrap();
        // Phase 4: Build and test
        println!("[4/6] 🏗 Building and testing...");
        let mutations = self.build_and_test(&file_mutations).unwrap();
        // Phase 5: Report results
        println!("[5/6] 📊 Reporting results...");
        self.report_results(&mutations).unwrap();
        // Phase 6: Save Results in csv
        println!("[6/6] 💾 Saving results...");
        self.save_results(&mutations);

        Ok(())
    }

    fn save_results(&self, mutations: &Vec<Mutation>) {
        let mut writer = csv::WriterBuilder::new()
            .from_path(Path::new(OUT_DIRECTORY).join("output.csv"))
            .unwrap();
        for mutation in mutations {
            writer.serialize(mutation).unwrap();
        }
        writer.flush().unwrap();
    }

    fn get_files_from_project(&self) -> Vec<String> {
        let mut existing_files: Vec<String> = vec![];
        Self::get_files_from_directory(self.config.path.clone(), &mut existing_files).unwrap();
        if self.verbose {
            tracing::debug!("Files found from path: {:#?}", existing_files);
            tracing::info!("Gathering all mutations for files...");
        }
        existing_files
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
        cli_table::print_stdout(mutations.with_title()).unwrap();
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
                .ok_or(KodeKrakenError::MutationBuildTestError)?
                .to_str()
                .ok_or(KodeKrakenError::MutationBuildTestError)?;
            let backup_path = self.backup_dir.join(original_file_name);
            // Save a copy of the original file
            fs::copy(&original_file_path, &backup_path).unwrap();
        }
        Ok(())
    }

    fn build_and_test(
        &mut self,
        file_mutations: &HashMap<String, FileMutations>,
    ) -> Result<Vec<Mutation>> {
        let num_mutations = file_mutations
            .iter()
            .fold(0, |acc, (_, fm)| acc + fm.mutations.len());
        let progress_bar = Arc::new(ProgressBar::new(num_mutations as u64));
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} - Running tests...")
                .map_err(|e| KodeKrakenError::Error(e.to_string()))?
                .progress_chars("=> "),
        );

        self.copy_files(file_mutations).unwrap();

        // Merge all mutants into one vector
        let mut all_mutations: Vec<Mutation> = vec![];
        for (_, fm) in file_mutations.iter() {
            for mutation in fm.mutations.iter() {
                all_mutations.push(mutation.clone());
            }
        }
        // Partition the mutations into chunks
        let chunk_size = ((all_mutations.len() as f32) / MAX_BUILD_THREADS) as usize;
        let mut chunks: Vec<Vec<Mutation>> = all_mutations
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect();
        // Set up threading
        let path = Arc::new(self.config.path.clone());
        let mutation_dir = Arc::new(self.mutation_dir.clone());
        let backup_dir = Arc::new(self.backup_dir.clone());
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(chunks.len())
            .build()
            .map_err(|e| KodeKrakenError::Error(e.to_string()))?;
        thread_pool.scope(|s| {
            for chunck in chunks.iter_mut() {
                // Create unique temp directory
                let uuid = uuid::Uuid::new_v4();
                let mut td = Path::new(OUT_DIRECTORY).join(format!("temp/{}", uuid));
                fs::create_dir_all(&td).unwrap();
                // Create directory structure inside temp directory that matches the original project
                let dir = PathBuf::from(&self.config.path);
                let mut config_prefix = PathBuf::new();
                for c in dir.components() {
                    if let Component::Normal(dir) = c {
                        td = td.join(dir);
                        config_prefix = config_prefix.join(dir);
                    }
                }
                fs::create_dir_all(&td).unwrap();
                self.create_temp_directory(dir, &td).unwrap();
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
                                .expect("Failed to convert file name to string") // TODO: Remove unwrap
                        ));

                        if let Err(_err) = gradle::run(
                            &PathBuf::from(&td),
                            false,
                            &mutated_file_path,
                            &original_file_path,
                            mutation,
                        ) {
                            // Log here something?
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
        fs::remove_dir_all(Path::new(OUT_DIRECTORY).join("temp"))
            .expect("Failed to remove temp directory");
        Ok(chunks.into_iter().flatten().collect())
    }

    fn create_temp_directory(&self, dir: PathBuf, temp_dir: &Path) -> Result<()> {
        for entry in dir.read_dir()? {
            let path = entry.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            // Ignore the kode-kraken-dist folder
            // We can add to this to ignore more things.
            if file_name == "kode-kraken-dist" {
                continue;
            }
            if path.is_dir() {
                let temp_dir = temp_dir.join(file_name);
                fs::create_dir(&temp_dir).unwrap();
                self.create_temp_directory(path, &temp_dir)?;
            } else {
                let file_contents = fs::read(&path).unwrap();
                if file_name == "gradlew" || file_name == "gradlew.bat" {
                    // We copy here so that we keep the same permissions
                    fs::copy(&path, temp_dir.join(file_name)).unwrap();
                } else {
                    fs::write(temp_dir.join(file_name), file_contents).unwrap();
                }
            }
        }
        Ok(())
    }

    fn generate_mutations_per_file(
        &self,
        file_mutations: &HashMap<String, FileMutations>,
    ) -> Result<()> {
        if self.verbose {
            tracing::info!("Generating mutations per file");
        }
        self.thread_pool.scope(|s| {
            file_mutations.iter().for_each(|(file_name, fm)| {
                let file_str = fs::read_to_string(file_name).unwrap(); // TODO: Remove unwrap
                s.spawn(move |_| {
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
                        let mutated_file_name = self
                            .mutation_dir
                            .join(self.create_mutated_file_name(file_name, m).unwrap());
                        // Write the mutated file to the output directory
                        fs::write(mutated_file_name, file).unwrap(); // TODO: Remove unwrap
                    });
                });
            });
        });

        Ok(())
    }

    fn gather_mutations_per_file(
        &mut self,
        existing_files: &mut Vec<String>,
    ) -> Result<HashMap<String, FileMutations>> {
        let file_mutations: Arc<Mutex<HashMap<String, FileMutations>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let mutation_count = Arc::new(Mutex::new(0));
        self.thread_pool.scope(|s| {
            for file in existing_files.clone() {
                let mutation_count = mutation_count.clone();
                let file_mutations = file_mutations.clone();
                let parser = self.parser.clone();
                let mutation_operators = self.mutation_operators.clone();
                s.spawn(move |_| {
                    let ast = parser
                        .lock()
                        .unwrap()
                        .parse(fs::read_to_string(&file).expect("File Not Found!"), None)
                        .unwrap(); // TODO: Remove this unwrap
                    for mut_op in mutation_operators.iter() {
                        // Get a list of mutations that can be made
                        let mutations = mut_op.find_mutation(&ast, &file);
                        *mutation_count.lock().unwrap() += mutations.len();
                        file_mutations
                            .lock()
                            .unwrap()
                            .entry(file.clone())
                            .and_modify(|fm| fm.mutations.extend(mutations.clone()))
                            .or_insert(FileMutations {
                                mutations: mutations.clone(),
                            });
                    }
                });
            }
        });
        if self.verbose {
            let mutation_count = Arc::try_unwrap(mutation_count)
                .unwrap()
                .into_inner()
                .unwrap();
            tracing::info!("Mutations made to all files");
            tracing::info!("Total mutations made: {}", mutation_count);
        }
        Ok(Arc::try_unwrap(file_mutations)
            .unwrap()
            .into_inner()
            .unwrap())
    }

    /*
        Take in path to directory and get all files that end with .kt
    */
    fn get_files_from_directory(path: String, existing_files: &mut Vec<String>) -> Result<()> {
        // TODO: Consider adding src to this path.
        let directory = Path::new(path.as_str())
            .read_dir()
            .map_err(|_| KodeKrakenError::MutationGatheringError)?;
        for entry in directory {
            let entry = entry.map_err(|_| KodeKrakenError::MutationGatheringError)?;
            let path = entry.path();
            if path.file_name().unwrap().to_str().unwrap() == "kode-kraken-dist" {
                continue;
            }
            if path.is_dir() {
                Self::get_files_from_directory(
                    path.to_str()
                        .ok_or(KodeKrakenError::MutationGatheringError)?
                        .to_string(),
                    existing_files,
                )?;
                continue;
            }
            if path.extension() != Some("kt".as_ref()) {
                continue;
            }
            if path.components().any(|p| {
                // TODO: This will be where configuration file will be used
                p == Component::Normal(OsStr::new("test"))
                    || p == Component::Normal(OsStr::new("build"))
                    || p == Component::Normal(OsStr::new("bin"))
            }) {
                continue;
            }
            let file_name = entry.file_name();
            if file_name
                .to_str()
                .ok_or(KodeKrakenError::Error(
                    "Failed to convert os str to string".into(),
                ))?
                .ends_with("Test.kt")
            {
                continue;
            }
            existing_files.push(
                path.to_str()
                    .ok_or(KodeKrakenError::Error(
                        "Failed to convert os str to string".into(),
                    ))?
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use uuid::Uuid;

    use super::*;
    use crate::test_config::*;

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
            false,
            MutationCommandConfig {
                path: format!("./{}", mutation_test_id),
            },
            output_directory,
            operators,
            false,
            30,
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
            .gather_mutations_per_file(&mut mutator.get_files_from_project())
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
            .gather_mutations_per_file(&mut mutator.get_files_from_project())
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
                let diff = m.new_op.as_bytes().len() as isize - m.old_op.as_bytes().len() as isize;
                let mut_range = m.start_byte..(m.end_byte as isize + diff) as usize;
                // Checks that the mutated file does not have the same contents as the original file

                assert_eq!(m.new_op.as_bytes().to_vec(), mut_file[mut_range].to_vec());
            }
        }
        // Remove contents in temp directory
        remove_directory(mutation_test_id);
    }

    #[test]
    fn test_mutate_arithmetic_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ArthimeticOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_arithmetic_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::ArthimeticOperator],
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
            vec![MutationOperators::AssignmentOperator],
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
            vec![MutationOperators::AssignmentOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_logical_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LOGICAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::LogicalOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_logical_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LOGICAL_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::LogicalOperator],
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
            vec![MutationOperators::RelationalOperator],
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
            vec![MutationOperators::RelationalOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_unary_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_UNARY_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::UnaryOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id, output_directory);
    }

    #[test]
    fn test_mutate_unary_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_UNARY_TEST_CODE);
        let mut mutator = create_mutator_with_specific_operators(
            mutation_test_id,
            output_directory.clone(),
            vec![MutationOperators::UnaryOperator],
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
}
