use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    io::BufRead,
    path::{Component, Path, PathBuf}, sync::{Arc, Mutex},
};

use clap::{error::ErrorKind, CommandFactory};

use crate::{
    gradle::Gradle,
    mutation::{Mutation, FileMutations, MutationResult},
    mutation_operators::{AllMutationOperators, MutationOperators, self},
    Cli, CliError, MutationCommandConfig,
};

use cli_table::{WithTitle, Table};

const OUT_DIRECTORY: &str = "./kode-kraken-dist";

pub struct MutationToolBuilder {
    verbose: bool,
    config: Option<MutationCommandConfig>,
    mutation_operators: Option<Vec<MutationOperators>>,
    enable_mutation_comment: bool,
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

    pub fn build(self) -> MutationTool {
        let config = self.config.unwrap_or_default();
        let mutation_operators = self
            .mutation_operators
            .unwrap_or(AllMutationOperators::new().get_mutation_operators());
        MutationTool::new(
            self.verbose,
            config,
            OUT_DIRECTORY.into(),
            mutation_operators,
            self.enable_mutation_comment,
        )
    }
}

pub struct MutationTool {
    parser: Arc<Mutex<tree_sitter::Parser>>,
    verbose: bool,
    config: MutationCommandConfig,
    mutation_operators: Arc<Vec<MutationOperators>>,
    mutation_dir: PathBuf,
    backup_dir: PathBuf,
    gradle: Gradle,
    enable_mutation_comment: bool,
}

impl Default for MutationTool {
    fn default() -> Self {
        Self::new(
            false,
            MutationCommandConfig::default(),
            OUT_DIRECTORY.into(),
            AllMutationOperators::new().get_mutation_operators(),
            false,
        )
    }
}

impl MutationTool {
    fn new(
        verbose: bool,
        config: MutationCommandConfig,
        output_directory: String,
        mutation_operators: Vec<MutationOperators>,
        enable_mutation_comment: bool,
    ) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_kotlin::language()).unwrap();

        // Validate config path
        // Check if it exists
        let config_path = Path::new(&config.path);
        if !config_path.exists() {
            Cli::command()
                .error(ErrorKind::ArgumentConflict, "Path does not exist")
                .exit();
        }
        if !config_path.is_dir() {
            Cli::command()
                .error(ErrorKind::ArgumentConflict, "Path is not a directory")
                .exit();
        }

        // If output directory exists, clear it
        if Path::new(&output_directory).exists() {
            fs::remove_dir_all(&output_directory).unwrap(); // TODO: Remove unwrap
        }
        // Create directories
        let mutation_dir = Path::new(&output_directory).join("mutations");
        let backup_dir = Path::new(&output_directory).join("backups");
        if !mutation_dir.exists() {
            fs::create_dir_all(&mutation_dir).unwrap(); // TODO: Remove unwrap
        }
        if !backup_dir.exists() {
            fs::create_dir(&backup_dir).unwrap(); // TODO: Remove unwrap
        }

        // Create gradle struct
        let gradle = Gradle::new(config_path.to_path_buf(), verbose);

        Self {
            verbose,
            config,
            parser: Arc::new(Mutex::new(parser)),
            mutation_operators: Arc::new(mutation_operators),
            mutation_dir,
            backup_dir,
            gradle,
            enable_mutation_comment,
        }
    }

    fn create_mutated_file_name(&self, file_name: &str, mutation: &Mutation) -> String {
        format!(
            "{}_{}",
            mutation.id,
            Path::new(&file_name).file_name().unwrap().to_str().unwrap() // TODO: Remove unwrap
        )
    }

    pub fn mutate(&mut self) {
        tracing::info!("Mutation tool started...");
        // Phase 1: Get files from project
        let mut existing_files = self.get_files_from_project();
        // Phase 2: Gather mutations per file
        let start_time = std::time::Instant::now();
        let mut file_mutations = self.gather_mutations_per_file(&mut existing_files);
        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time).as_millis();
        tracing::info!(
            "Gathered mutations per file in {} milliseconds",
            duration
        );
        // Phase 2: Generate mutations per file
        let start_time = std::time::Instant::now();
        self.generate_mutations_per_file(&file_mutations);
        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time).as_millis();
        tracing::info!(
            "Generated mutations per file in {} milliseconds",
            duration
        );
        // tracing::info!("Building and testing mutations...");
        // // Phase 4: Build and test
        // self.build_and_test(&mut file_mutations);
        // // Phase 5: Report results
        // self.report_results(&file_mutations);
    }

    fn get_files_from_project(&self) -> Vec<String> {
        let mut existing_files: Vec<String> = vec![];
        if let Some(error) =
            Self::get_files_from_directory(self.config.path.clone(), &mut existing_files).err()
        {
            Cli::command().error(error.kind, error.message).exit();
        }
        if self.verbose {
            tracing::debug!("Files found from path: {:#?}", existing_files);
            tracing::info!("Gathering all mutations for files...");
        }
        existing_files
    }

    fn report_results(&self, file_mutations: &HashMap<String, FileMutations>) {
        let mut mutations = vec![];
        let mut total_mutations = 0;
        let mut total_killed_mutants = 0;
        let mut total_survived_mutants = 0;
        let mut total_timeouts_or_build_fails = 0;
        file_mutations.iter().for_each(|(_, fm)| {
            if !fm.mutations.is_empty() {
                mutations.push(fm.mutations.clone());
            }
            total_mutations += fm.mutations.len();
            fm.mutations.iter().for_each(|m| {
                match m.result {
                    MutationResult::Killed => total_killed_mutants += 1,
                    MutationResult::Survived => total_survived_mutants += 1,
                    _ => total_timeouts_or_build_fails += 1,
                }
            })
        });
        cli_table::print_stdout(mutations.concat().with_title()).unwrap();
        let table = vec![
            vec![
                "Total mutations".to_string(),
                total_mutations.to_string(),
            ],
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
                    (total_killed_mutants as f32 / total_mutations as f32) * 100.0
                ),
            ]
        ].table();
        cli_table::print_stdout(table).unwrap();
    }

    fn build_and_test(&mut self, file_mutations: &mut HashMap<String, FileMutations>) {
        for (file_name, fm) in file_mutations.iter_mut() {
            let original_file_path = Path::new(file_name).to_path_buf();
            let original_file_name = original_file_path.file_name().unwrap().to_str().unwrap();
            let backup_path = self.backup_dir.join(original_file_name);
            // Save a copy of the original file
            fs::copy(&original_file_path, &backup_path).unwrap();

            for mutation in fm.mutations.iter_mut() {
                let mutated_file_path = self
                    .mutation_dir
                    .join(self.create_mutated_file_name(file_name, mutation));
                if self.verbose {
                    tracing::info!("Building and testing {}", mutated_file_path.display());
                }

                self.gradle.run(&mutated_file_path, &original_file_path, &backup_path, mutation);
            }
        }
    }

    fn generate_mutations_per_file(&self, file_mutations: &HashMap<String, FileMutations>) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(30)
            .build()
            .unwrap();
        if self.verbose {
            tracing::info!("Generating mutations per file");
        }
        pool.scope(|s| {
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
                            .join(self.create_mutated_file_name(file_name, m));
                        // Write the mutated file to the output directory
                        fs::write(mutated_file_name, file).unwrap(); // TODO: Remove unwrap
                    });
                });
            });
            println!("Num of threads: {}", pool.current_num_threads());
        })
        // std::thread::scope(|s| {
        //     let mut threads = vec![];
        //     file_mutations.iter().for_each(|(file_name, fm)| {   
        //         let file_str = fs::read_to_string(file_name).unwrap(); // TODO: Remove unwrap
        //         threads.push(s.spawn(move || {
        //             fm.mutations.iter().for_each(|m| {
        //                 let new_op_bytes = m.new_op.as_bytes();
        //                 let mut file = file_str.as_bytes().to_vec();
        
        //                 // Add the mutation to the vector of bytes
        //                 file.splice(m.start_byte..m.end_byte, new_op_bytes.iter().cloned());
        //                 // Add comment above mutation about the mutation
        //                 let file = file
        //                     .lines()
        //                     .enumerate()
        //                     .map(|(i, line)| {
        //                         let mut line = line.expect("Failed to convert line to string");
        //                         if i == m.line_number - 1 && self.enable_mutation_comment {
        //                             line = format!("{}\n{}", m, line);
        //                         }
        //                         line
        //                     })
        //                     .collect::<Vec<String>>()
        //                     .join("\n");
        
        //                 // Create a file name for the mutated file
        //                 let mutated_file_name = self
        //                     .mutation_dir
        //                     .join(self.create_mutated_file_name(file_name, m));
        //                 // Write the mutated file to the output directory
        //                 fs::write(mutated_file_name, file).unwrap(); // TODO: Remove unwrap
        //             });
        //         }));
        //     });
        //     for t in threads {
        //         t.join().unwrap();
        //     }
        // });
    }

    fn gather_mutations_per_file(&mut self, existing_files: &mut Vec<String>) -> HashMap<String, FileMutations> {
        let file_mutations: Arc<Mutex<HashMap<String, FileMutations>>> = Arc::new(Mutex::new(HashMap::new()));
        let mutation_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
        std::thread::scope(|s| {
            let mut threads = vec![];
            for file in existing_files.clone() {
                let mutation_count = mutation_count.clone();
                let file_mutations = file_mutations.clone();
                let parser = self.parser.clone();
                let mutation_operators = self.mutation_operators.clone();
                threads.push(s.spawn(move || {
                    let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap().to_string();
                    let ast = parser
                        .lock()
                        .unwrap()
                        .parse(
                            fs::read_to_string(&file).expect("File Not Found!"),
                            None,
                        )
                        .unwrap(); // TODO: Remove this unwrap
                    for mut_op in mutation_operators.iter() {
                        // Get a list of mutations that can be made
                        let mutations = mut_op.find_mutation(&ast, &file_name);
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
                }));
            }
            for t in threads {
                t.join().unwrap();
            }
        });
        if self.verbose {
            let mutation_count = Arc::try_unwrap(mutation_count).unwrap().into_inner().unwrap();
            tracing::info!("Mutations made to all files");
            tracing::info!("Total mutations made: {}", mutation_count);
        }
        Arc::try_unwrap(file_mutations).unwrap().into_inner().unwrap()
    }

    /*
        Take in path to directory and get all files that end with .kt
    */
    fn get_files_from_directory(
        path: String,
        existing_files: &mut Vec<String>,
    ) -> Result<(), CliError> {
        // TODO: Consider adding src to this path.
        let directory = Path::new(path.as_str()).read_dir().map_err(|_| CliError {
            kind: ErrorKind::Io,
            message: "Could not read directory".into(),
        })?;
        for entry in directory {
            let entry = entry.map_err(|_| CliError {
                kind: ErrorKind::Io,
                message: "Could not read directory".into(),
            })?;
            let path = entry.path();
            if path.is_dir() {
                Self::get_files_from_directory(
                    path.to_str()
                        .ok_or_else(|| CliError {
                            kind: ErrorKind::Io,
                            message: "Could not read directory".into(),
                        })?
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
            }) {
                continue;
            }
            let file_name = entry.file_name();
            if file_name.to_str().unwrap().ends_with("Test.kt") {
                continue;
            }
            existing_files.push(path.to_str().unwrap().to_string());
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
        fs::write(&file_id, file_contents).unwrap();

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
        )
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
        let fm = mutator.gather_mutations_per_file(&mut mutator.get_files_from_project());
        mutator.generate_mutations_per_file(&fm);
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
        let mut fm = mutator.gather_mutations_per_file(&mut mutator.get_files_from_project());
        mutator.generate_mutations_per_file(&mut fm);
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
