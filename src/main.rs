use std::{fs, path::PathBuf};
use kotlin_types::KotlinTypes;
use tree_sitter::Parser;

pub mod kotlin_types;

enum Commands {
    Mutate(MutationCommandConfig),
    Help
}

struct MutationCommandConfig {
    file: String,
}

fn main() {
    let command = get_command(std::env::args().collect());
    match command {
        Commands::Mutate(config) => {
            mutate(config);
        },
        Commands::Help => {
            print_help_message();
        }
    }
}

/// Get the command from the command line arguments
fn get_command(args: Vec<String>) -> Commands {
    if args.len() < 2 {
        println!("No command provided");
        print_help_message();
        std::process::exit(1);
    }
    let command = args.get(1).expect("No command provided");
    let command = match command.as_str() {
        "mutate" => {
            let file = args.get(2).expect("No file provided");
            if file == "--help" || file == "-h" {
                print_mutate_help_message();
                std::process::exit(0);
            }
            // Check if file exists
            if !PathBuf::from(file).exists() {
                panic!("File does not exist");
            }
            Commands::Mutate(MutationCommandConfig {
                file: file.trim().to_string(),
            })
        },
        "help" => Commands::Help,
        _ => {
            println!("Unknown command: {}", command);
            print_help_message();
            std::process::exit(1);
        }
    };
    command

}

fn print_mutate_help_message() {
    println!("Usage: kotlin-mutator mutate <file>");
    println!("Mutate the given file");
}

fn print_help_message() {
    println!("Usage: kotlin-mutator <command> [options]");
    println!("Commands:");
    println!("    mutate <file> - Mutate the given file");
    println!("    help - Print this help message");
}

fn mutate(config: MutationCommandConfig) {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_kotlin::language()).unwrap();

    let kotlin_file = fs::read_to_string(&config.file).expect("File Not Found!");
    let parsed = parser.parse(&kotlin_file, None).unwrap();
    let root_node = parsed.root_node();
    let mut cursor = parsed.walk();

    let mut mutations_made = Vec::new();
    search_children(
        root_node, 
        &mut cursor, 
        " ", 
        false, 
        &mut mutations_made,
        kotlin_file.to_string(),
        format!("./examples/mutations/{}", config.file.split("/").last().expect("Failed to get file name"))
    );
    println!("Mutations made: {:#?}", mutations_made)
}

#[derive(Debug)]
#[allow(dead_code)]
struct Mutation {
    start_byte: usize,
    end_byte: usize,
    new_op: String,
    old_op: String,
}

impl Mutation {
    pub fn new(start_byte: usize, end_byte: usize, new_op: String, old_op: String) -> Self {
        Self {
            start_byte,
            end_byte,
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
    let mut kt_file = kt_file;
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
                kt_file = fs::read_to_string("./examples/mutations/bigger_examples_mutate.kt").unwrap();
                mutations_made.push(Mutation::new(node.start_byte(), node.end_byte(), "!=".to_string(), "==".to_string()));
            }
            println!("{}({} {} - {})", prefix, node.kind(), node.start_position(), node.end_position());
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

    #[test]
    fn test_get_command() {
        let args = vec!["kotlin-mutator".to_string(), "mutate".to_string(), "./examples/bigger_examples.kt".to_string()];
        let command = get_command(args);
        match command {
            Commands::Mutate(config) => {
                assert_eq!(config.file, "./examples/bigger_examples.kt");
            },
            _ => panic!("Expected mutate command"),
        }
    }

    #[test]
    #[should_panic]
    fn test_get_command_no_command() {
        let args = vec!["kotlin-mutator".to_string()];
        get_command(args);
    }

    #[test]
    #[should_panic]
    fn test_get_command_unknown_command() {
        let args = vec!["kotlin-mutator".to_string(), "unknown".to_string()];
        get_command(args);
    }

    #[test]
    #[should_panic]
    fn test_get_command_no_file() {
        let args = vec!["kotlin-mutator".to_string(), "mutate".to_string()];
        get_command(args);
    }

    #[test]
    #[should_panic]
    fn test_get_command_file_does_not_exist() {
        let args = vec!["kotlin-mutator".to_string(), "mutate".to_string(), "./examples/does_not_exist.kt".to_string()];
        get_command(args);
    }
}