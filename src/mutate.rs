use std::{
    collections::HashMap,
    fs,
    path::{self, Path},
};

use clap::{error::ErrorKind, CommandFactory};

use crate::{
    mutation_operators::{self, MutationOperators},
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
        }
    }
}

pub struct MutationTool {
    parser: tree_sitter::Parser,
    verbose: bool,
    config: MutationCommandConfig,
}

impl MutationTool {
    pub fn new(verbose: bool, config: MutationCommandConfig) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_kotlin::language()).unwrap();

        Self {
            verbose,
            config,
            parser,
        }
    }

    pub fn mutate(&mut self) {
        // Check if config.path is a directory
        if !path::Path::new(self.config.path.as_str()).is_dir() {
            Cli::command()
                .error(ErrorKind::ArgumentConflict, "Path is not a directory")
                .exit();
        }
        let mut existing_files: Vec<String> = vec![];
        if let Some(error) =
            Self::get_files_from_directory(self.config.path.clone(), &mut existing_files).err()
        {
            Cli::command().error(error.kind, error.message).exit();
        }
        if self.verbose {
            tracing::debug!("Files found from path: {:#?}", existing_files);
        }

        let mut file_mutations: HashMap<String, FileMutations> = HashMap::new();
        let mutation_operators = mutation_operators::AllMutationOperators::new();
        for mut_op in mutation_operators {
            for file in existing_files.clone() {
                // Get a list of mutations that can be made
                let ast = self
                    .parser
                    .parse(
                        fs::read_to_string(file.clone()).expect("File Not Found!"),
                        None,
                    )
                    .unwrap();
                let mutations = mut_op.find_mutation(ast);
                file_mutations
                    .entry(file.clone())
                    .or_insert(FileMutations {
                        mutations: mutations.clone(),
                    })
                    .mutations
                    .extend(mutations);
            }
        }
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

    pub fn clear_output_directory(ouptut_directory: String, verbose: bool) {
        let dir = Path::new(ouptut_directory.as_str());
        if verbose {
            tracing::info!("Removing directory: {:#?}", dir);
        }
        if dir.exists() {
            fs::remove_dir_all(dir).unwrap();
        }
    }
}
