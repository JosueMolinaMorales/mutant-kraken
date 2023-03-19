use std::{
    collections::HashMap,
    fs,
    path::{self, Path},
};

use clap::{error::ErrorKind, CommandFactory};
use uuid::Uuid;

use crate::{
    mutation_operators::{AllMutationOperators, MutationOperators},
    Cli, CliError, FileMutations, MutationCommandConfig,
};

#[derive(Debug, Clone)]
pub struct Mutation {
    pub start_byte: usize,
    pub end_byte: usize,
    pub line_number: usize,
    pub new_op: String,
    pub old_op: String,
    pub mutation_type: MutationOperators,
    pub id: Uuid,
}

impl Mutation {
    pub fn new(
        start_byte: usize,
        end_byte: usize,
        new_op: String,
        old_op: String,
        line_number: usize,
        mutation_type: MutationOperators,
    ) -> Self {
        Self {
            start_byte,
            end_byte,
            line_number,
            new_op,
            old_op,
            mutation_type,
            id: Uuid::new_v4(),
        }
    }
}

pub struct MutationToolBuilder {
    verbose: bool,
    config: Option<MutationCommandConfig>,
    output_directory: Option<String>,
    mutation_operators: Option<Vec<MutationOperators>>,
}

impl MutationToolBuilder {
    pub fn new() -> Self {
        Self {
            verbose: false,
            config: None,
            output_directory: None,
            mutation_operators: None,
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
    pub fn set_output_directory(mut self, output_directory: String) -> Self {
        self.output_directory = Some(output_directory);
        if let Some(o_dir) = self.output_directory.as_ref() {
            // If path does not exist, create it
            if !path::Path::new(o_dir.as_str()).exists() {
                fs::create_dir_all(o_dir).unwrap(); // TODO: Remove unwrap
            }
        }
        self
    }
    pub fn set_mutation_operators(mut self, mutation_operators: Vec<MutationOperators>) -> Self {
        self.mutation_operators = Some(mutation_operators);
        self
    }
    pub fn build(self) -> MutationTool {
        let config = self.config.unwrap_or_default();
        let output_directory = self.output_directory.unwrap_or(".".into());
        let mutation_operators = self
            .mutation_operators
            .unwrap_or(AllMutationOperators::new().get_mutation_operators());
        MutationTool::new(self.verbose, config, output_directory, mutation_operators)
    }
}

pub struct MutationTool {
    parser: tree_sitter::Parser,
    verbose: bool,
    config: MutationCommandConfig,
    output_directory: String,
    mutation_operators: Vec<MutationOperators>,
}

impl Default for MutationTool {
    fn default() -> Self {
        Self::new(
            false,
            MutationCommandConfig::default(),
            ".".into(),
            AllMutationOperators::new().get_mutation_operators(),
        )
    }
}

impl MutationTool {
    fn new(
        verbose: bool,
        config: MutationCommandConfig,
        output_directory: String,
        mutation_operators: Vec<MutationOperators>,
    ) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_kotlin::language()).unwrap();

        // Validate config path
        // Check if it exists
        if !path::Path::new(config.path.as_str()).exists() {
            Cli::command()
                .error(ErrorKind::ArgumentConflict, "Path does not exist")
                .exit();
        }
        if !path::Path::new(config.path.as_str()).is_dir() {
            Cli::command()
                .error(ErrorKind::ArgumentConflict, "Path is not a directory")
                .exit();
        }

        // Validate output directory
        if !path::Path::new(output_directory.as_str()).is_dir() {
            Cli::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    "Output directory is not a directory",
                )
                .exit();
        }

        Self {
            verbose,
            config,
            parser,
            output_directory,
            mutation_operators,
        }
    }

    pub fn mutate(&mut self) {
        let file_mutations = self.gather_mutations_per_file();
        self.generate_mutations_per_file(file_mutations);
    }

    fn generate_mutations_per_file(&self, file_mutations: HashMap<String, FileMutations>) {
        if self.verbose {
            tracing::info!("Generating mutations per file");
        }
        for (file_name, fm) in file_mutations {
            let mut file_str = fs::read_to_string(file_name.clone()).unwrap(); // TODO: Remove unwrap
            for m in fm.mutations.iter() {
                let new_op_bytes = m.new_op.as_bytes();
                let mut file = file_str.as_bytes().to_vec();

                // Add the mutation to the vector of bytes
                file.splice(m.start_byte..m.end_byte, new_op_bytes.iter().cloned());
                // Create a file name for the mutated file
                // Prepend 'mut' to the file name
                let mutated_file_name = Path::new(&self.output_directory).join(format!(
                    "mut_{}_{}",
                    m.id,
                    Path::new(&file_name).file_name().unwrap().to_str().unwrap() // TODO: Remove unwrap
                ));
                // Write the mutated file to the output directory
                fs::write(mutated_file_name, file).unwrap(); // TODO: Remove unwrap
                // THIS IS WHERE COMPILILNG AND TESTING HAPPENS
                // THIS WILL BE WHERE WE GET THE OUTCOMES OF THE COMPILATION AND TESTING
                // Read the original file again
                file_str = fs::read_to_string(&file_name).unwrap(); // TODO: Remove unwrap
            }
        }
    }

    fn gather_mutations_per_file(&mut self) -> HashMap<String, FileMutations> {
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

        let mut file_mutations: HashMap<String, FileMutations> = HashMap::new();
        let mut mutation_count = 0;
        for file in existing_files.clone() {
            for mut_op in self.mutation_operators.clone() {
                // Get a list of mutations that can be made
                let ast = self
                    .parser
                    .parse(
                        fs::read_to_string(file.clone()).expect("File Not Found!"),
                        None,
                    )
                    .unwrap(); // TODO: Remove this unwrap
                let mutations = mut_op.find_mutation(ast);
                mutation_count += mutations.len();
                file_mutations
                    .entry(file.clone())
                    .and_modify(|fm| fm.mutations.extend(mutations.clone()))
                    .or_insert(FileMutations {
                        mutations: mutations.clone(),
                    });
            }
        }
        if self.verbose {
            tracing::info!("Mutations made to all files");
            tracing::info!("Total mutations made: {}", mutation_count);
        }
        file_mutations
    }

    /*
        Take in path to directory and get all files that end with .kt
    */
    fn get_files_from_directory(
        path: String,
        existing_files: &mut Vec<String>,
    ) -> Result<(), CliError> {
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
            existing_files.push(path.to_str().unwrap().to_string());
        }

        Ok(())
    }

    pub fn clear_output_directory(&self, ouptut_directory: String) {
        // TODO: Remove contents of directory instead of the entire directory
        let dir = Path::new(ouptut_directory.as_str());
        if self.verbose {
            tracing::info!("Removing directory: {:#?}", dir);
        }
        if dir.exists() {
            fs::remove_dir_all(dir).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::test_config::*;

    fn create_temp_directory(file_contents: &str) -> (Uuid, String) {
        let mutation_test_id = Uuid::new_v4();
        let file_id = format!("./{}/{}.kt", mutation_test_id, mutation_test_id);
        let output_directory = format!("./{}/mutations/", mutation_test_id);
        // Create temp output directory
        std::fs::create_dir_all(&output_directory).unwrap();
        // Add test files to directory
        std::fs::write(&file_id, file_contents).unwrap();

        (mutation_test_id, output_directory)
    }

    fn remove_directory(mutation_test_id: Uuid) {
        fs::remove_dir_all(format!("./{}", mutation_test_id)).unwrap()
    }

    fn create_mutator_with_specifc_operators(
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
        )
    }

    fn get_mutated_file_name(mutator: &MutationTool, file_name: &str, m: &Mutation) -> PathBuf {
        Path::new(&mutator.output_directory).join(format!(
            "mut_{}_{}",
            m.id,
            Path::new(&file_name).file_name().unwrap().to_str().unwrap()
        ))
    }

    fn assert_all_mutation_files_were_created(mutator: &mut MutationTool, mutation_test_id: Uuid) {
        let fm = mutator.gather_mutations_per_file();
        mutator.generate_mutations_per_file(fm.clone());
        // Check that the mutated files were created
        for (file_name, fm) in fm {
            for m in fm.mutations.clone() {
                let mutated_file_name = get_mutated_file_name(&mutator, &file_name, &m);
                assert!(Path::new(mutated_file_name.to_str().unwrap()).exists());
            }
        }
        // Remove directory
        remove_directory(mutation_test_id);
    }

    fn assert_all_mutations_are_correct(mutator: &mut MutationTool, mutation_test_id: Uuid) {
        let fm = mutator.gather_mutations_per_file();
        mutator.generate_mutations_per_file(fm.clone());
        // Check that the mutated files were created
        for (file_name, fm) in fm {
            for m in fm.mutations {
                let mutated_file_name = get_mutated_file_name(&mutator, &file_name, &m);
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
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::ArthimeticOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_arithmetic_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::ArthimeticOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_assignment_mutated_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_ASSIGNMENT_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::AssignmentOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_assignment_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_ASSIGNMENT_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::AssignmentOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_logical_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LOGICAL_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::LogicalOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_logical_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_LOGICAL_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::LogicalOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_relational_mutated_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_RELATIONAL_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::RelationalOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_relational_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_RELATIONAL_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::RelationalOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_unary_mutated_files_exist() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_UNARY_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::UnaryOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_unary_mutations_are_correct() {
        let (mutation_test_id, output_directory) = create_temp_directory(KOTLIN_UNARY_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::UnaryOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_unary_removal_mutated_files_exist() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::UnaryRemovalOperator],
        );
        assert_all_mutation_files_were_created(&mut mutator, mutation_test_id);
    }

    #[test]
    fn test_mutate_unary_removal_mutations_are_correct() {
        let (mutation_test_id, output_directory) =
            create_temp_directory(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let mut mutator = create_mutator_with_specifc_operators(
            mutation_test_id,
            output_directory,
            vec![MutationOperators::UnaryRemovalOperator],
        );
        assert_all_mutations_are_correct(&mut mutator, mutation_test_id);
    }
}
