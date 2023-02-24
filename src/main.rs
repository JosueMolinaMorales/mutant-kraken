use std::{fs, path::{self, Path}};
use clap::{Parser, Subcommand, Args, CommandFactory, error::ErrorKind};
use kotlin_types::KotlinTypes;

pub mod kotlin_types;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Mutate the files in the given path
    Mutate(MutationCommandConfig),

    /// Clear the output directory of all files
    ClearOutputDirectory,
}
const ABOUT: &str = include_str!("./about.txt");
#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about = ABOUT, 
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Print out verbose information
    #[clap(short, long, default_value = "false")]
    verbose: bool,

    /// The path to the output directory
    #[clap(short, long, default_value = "./examples/mutations/")]
    output_directory: String,
}

#[derive(Args, Debug, Clone)]
struct MutationCommandConfig {
    /// The path to the files to be mutated
    /// Error will be thrown if the path is not a directory
    path: String,
}

#[derive(Debug)]
struct FileMutations {
    mutations: Vec<Mutation>,
    file: String,
}

struct CliError {
    kind: ErrorKind,
    message: String,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Mutate(config) => {
            // Check if config.path is a directory
            if !path::Path::new(config.path.as_str()).is_dir() {
                Cli::command().error(ErrorKind::ArgumentConflict, "Path is not a directory").exit();
            }
            if let Some(error) = mutate(config, args.output_directory).err() {
                Cli::command().error(error.kind, error.message).exit();
            }
        },
        Commands::ClearOutputDirectory => clear_output_directory(args.output_directory),
    }
}

fn clear_output_directory(ouptut_directory: String) {
    let dir = Path::new(ouptut_directory.as_str());
    if dir.exists() {
        fs::remove_dir_all(dir).unwrap();
    }
}

fn mutate(config: MutationCommandConfig, ouptut_directory: String) -> Result<(), CliError> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_kotlin::language()).unwrap();

    let mut file_mutations: Vec<FileMutations> = vec![];
    let directory = Path::new(config.path.as_str())
        .read_dir()
        .map_err(|_| CliError { kind: ErrorKind::Io, message: "Could not read directory".into()})?;

    // Check to see if the output directory exists if not create it
    let output_dir = Path::new(ouptut_directory.as_str());
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    for entry in directory {
        let entry = entry.unwrap();
        let path = entry.path();
        // Refactoring for a directory will be needed
        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().unwrap().to_str().unwrap();
        if !file_name.ends_with(".kt") {
            continue;
        }
        // prepend mutation to file name
        let file_name = format!("mutation_{}", file_name);
        let file = fs::read_to_string(path.clone()).expect("File Not Found!");
        let parsed = parser.parse(&file, None).unwrap();
        let root_node = parsed.root_node();
        let mut cursor = parsed.walk();

        let mut mutations_made = Vec::new();
        println!("File: {}", output_dir.join(&file_name).display());
        // Write file to mutation folder
        fs::write(output_dir.join(&file_name), file.clone()).unwrap();
        search_children(
            root_node, 
            &mut cursor, 
            " ", 
            false, 
            &mut mutations_made,
            file.to_string(),
            format!("{}", output_dir.join(&file_name).display())
        );
        file_mutations.push(FileMutations {
            mutations: mutations_made,
            file: file_name.to_string(),
        });
    }

    println!("File Mutations: {:#?}", file_mutations);
    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct Mutation {
    start_byte: usize,
    end_byte: usize,
    line_number: usize,
    new_op: String,
    old_op: String,
}

impl Mutation {
    pub fn new(
        start_byte: usize, 
        end_byte: usize, 
        new_op: String,
        old_op: String,
        line_number: usize,
    ) -> Self {
        Self {
            start_byte,
            end_byte,
            line_number,
            new_op,
            old_op,
        }
    }

}

fn search_children(
    root: tree_sitter::Node, 
    cursor: &mut tree_sitter::TreeCursor, 
    prefix: &str,
    parent_was_comp_exp: bool,
    mutations_made: &mut Vec<Mutation>,
    kt_file: String,
    output_file: String,
) {
    root
        .children(&mut cursor.clone())
        .for_each(|node| {
            let node_type = KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");
            if parent_was_comp_exp && node_type == KotlinTypes::NonNamedType("==".to_string()) {
                // TODO: Inserting mutants will need to be updated
                //       to account for the fact that the start and end
                //       bytes will change as we insert new mutants
                let new_op = "!=".as_bytes();
                let mut mutated_file: Vec<u8> = kt_file.as_bytes().iter().map(|b| *b).collect();
                for (i, b) in mutated_file.iter_mut().skip(node.start_byte()).enumerate() {
                    if i >= (node.end_byte() - node.start_byte()) {
                        break;
                    }
                    *b = new_op[i];
                }
                fs::write(&output_file, mutated_file).unwrap();
                mutations_made.push(Mutation::new(
                    node.start_byte(), 
                    node.end_byte(), 
                    "!=".to_string(), 
                    "==".to_string(),
                    node.start_position().row + 1,
                ));
            }
            // println!("{}({} {} - {})", prefix, node.kind(), node.start_position(), node.end_position());
            let kt_file = fs::read_to_string(&output_file).unwrap();

            search_children(
                node, 
                &mut cursor.clone(), 
                &format!("    {}", prefix),
                node_type == KotlinTypes::ComparisonExpression || node_type == KotlinTypes::EqualityExpression,
                mutations_made,
                kt_file.clone(),
                output_file.clone()
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_parse() {
        Cli::command().debug_assert();
    }

}