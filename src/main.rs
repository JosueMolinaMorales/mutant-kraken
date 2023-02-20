use std::fs;
use kotlin_types::KotlinTypes;
use tree_sitter::Parser;

pub mod kotlin_types;

const KOTLIN_FILE: &str = include_str!("../examples/bigger_examples.kt");

fn main() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_kotlin::language()).unwrap();
    let parsed = parser.parse(KOTLIN_FILE, None).unwrap();
    let root_node = parsed.root_node();
    let mut cursor = parsed.walk();
    fs::copy("./examples/bigger_examples.kt", "./examples/bigger_examples_mutate.kt").unwrap();
    let mut mutations_made = Vec::new();
    search_children(
        root_node, 
        &mut cursor, 
        " ", 
        false, 
        &mut mutations_made,
        KOTLIN_FILE.to_string(),
    );
    println!("Mutations made: {:#?}", mutations_made)
}

#[derive(Debug)]
struct Mutation {
    start_byte: usize,
    end_byte: usize,
    new_op: String,
    old_op: String,
}

fn search_children(
    root: tree_sitter::Node, 
    cursor: &mut tree_sitter::TreeCursor, 
    prefix: &str,
    parent_was_comp_exp: bool,
    mutations_made: &mut Vec<Mutation>,
    kt_file: String
) {
    let mut kt_file = kt_file;
    root
        .children(&mut cursor.clone())
        .for_each(|node| {
            let node_type = KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");
            if parent_was_comp_exp && node_type == KotlinTypes::NonNamedType("==".to_string()) {
                // TODO: Inserting mutants will need to be updated
                let new_op = "!=".as_bytes();
                let mut mutated_file: Vec<u8> = kt_file.as_bytes().iter().map(|b| *b).collect();
                for (i, b) in mutated_file.iter_mut().skip(node.start_byte()).enumerate() {
                    if i >= (node.end_byte() - node.start_byte()) {
                        break;
                    }
                    *b = new_op[i];
                }
                fs::write("./examples/mutations/bigger_examples_mutate.kt", mutated_file).unwrap();
                kt_file = fs::read_to_string("./examples/mutations/bigger_examples_mutate.kt").unwrap();
                mutations_made.push(Mutation {
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    new_op: "!=".to_string(),
                    old_op: "==".to_string(),
                });
            }
            println!("{}({} {} - {})", prefix, node.kind(), node.start_position(), node.end_position());
            search_children(
                node, 
                &mut cursor.clone(), 
                &format!("    {}", prefix),
                node_type == KotlinTypes::ComparisonExpression || node_type == KotlinTypes::EqualityExpression,
                mutations_made,
                kt_file.clone(),
            )
        })
}
