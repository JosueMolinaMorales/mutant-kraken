use std::{fs::{File, self}, mem};

use kotlin_types::KotlinTypes;
use tree_sitter::{Parser, InputEdit};

pub mod kotlin_types;

const KOTLIN_FILE: &str = include_str!("bigger_examples.kt");

fn main() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_kotlin::language()).unwrap();
    let parsed = parser.parse(KOTLIN_FILE, None).unwrap();
    let root_node = parsed.root_node();
    let mut cursor = parsed.walk();
    fs::copy("./src/stack.kt", "./stack_mutate.kt").unwrap();
    search_children(root_node, &mut cursor, " ", false);
    println!("{}", KotlinTypes::AdditiveExpression.as_str());
}


fn search_children(
    root: tree_sitter::Node, 
    cursor: &mut tree_sitter::TreeCursor, 
    prefix: &str,
    parent_was_comp_exp: bool
) {
    root
        .children(&mut cursor.clone())
        .for_each(|node| {
            if parent_was_comp_exp && node.kind() == "<" {
                let new_op = ">".as_bytes();
                let mut kt_file: Vec<u8> = KOTLIN_FILE.as_bytes().iter().map(|b| *b).collect();
                for (i, b) in kt_file.iter_mut().skip(node.start_byte()).enumerate() {
                    if i >= (node.end_byte() - node.start_byte()) {
                        break;
                    }
                    *b = new_op[i];
                }
                fs::write("./stack_mutate.kt", kt_file).unwrap();
            }
            println!("{}({} {} - {})", prefix, node.kind(), node.start_position(), node.end_position());
            search_children(
                node, 
                &mut cursor.clone(), 
                &format!("    {}", prefix),
                node.kind() == "comparison_expression" || node.kind() == "equality_expression"
            )
        })
}
