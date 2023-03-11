use std::collections::HashSet;
use rand::seq::IteratorRandom;
use crate::{Mutation, kotlin_types::KotlinTypes};

pub struct AllMutationOperators {
    mutation_operators: Vec<MutationOperators>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MutationOperators {
    ArthimeticOperator,
    ArthimeticRemovalOperator,
    LogicalOperator,
    RelationalOperator,
    AssignmentOperator,
    UnaryOperator,
}

impl MutationOperators {
    fn get_operators(&self) -> HashSet<KotlinTypes> {
        match self {
            MutationOperators::ArthimeticOperator => {
                vec![
                    KotlinTypes::NonNamedType("+".to_string()),
                    KotlinTypes::NonNamedType("-".to_string()),
                    KotlinTypes::NonNamedType("*".to_string()),
                    KotlinTypes::NonNamedType("/".to_string()),
                    KotlinTypes::NonNamedType("%".to_string()),
                ].into_iter().collect()
            },
            MutationOperators::ArthimeticRemovalOperator => {
                vec![
                    KotlinTypes::NonNamedType("+".to_string()),
                    KotlinTypes::NonNamedType("-".to_string()),
                    KotlinTypes::NonNamedType("*".to_string()),
                    KotlinTypes::NonNamedType("/".to_string()),
                    KotlinTypes::NonNamedType("%".to_string()),
                ].into_iter().collect()
            },
            MutationOperators::LogicalOperator => {
                vec![
                    KotlinTypes::NonNamedType("&&".to_string()),
                    KotlinTypes::NonNamedType("||".to_string()),
                ].into_iter().collect()
            },
            MutationOperators::RelationalOperator => {
                vec![
                    KotlinTypes::NonNamedType("==".to_string()),
                    KotlinTypes::NonNamedType("!=".to_string()),
                    KotlinTypes::NonNamedType("<".to_string()),
                    KotlinTypes::NonNamedType("<=".to_string()),
                    KotlinTypes::NonNamedType(">".to_string()),
                    KotlinTypes::NonNamedType(">=".to_string()),
                ].into_iter().collect()
            },
            MutationOperators::AssignmentOperator => {
                vec![
                    KotlinTypes::NonNamedType("=".to_string()),
                    KotlinTypes::NonNamedType("+=".to_string()),
                    KotlinTypes::NonNamedType("-=".to_string()),
                    KotlinTypes::NonNamedType("*=".to_string()),
                    KotlinTypes::NonNamedType("/=".to_string()),
                    KotlinTypes::NonNamedType("%=".to_string()),
                ].into_iter().collect()
            },
            MutationOperators::UnaryOperator => {
                vec![
                    KotlinTypes::NonNamedType("!".to_string()),
                    KotlinTypes::NonNamedType("++".to_string()),
                    KotlinTypes::NonNamedType("--".to_string()),
                    KotlinTypes::NonNamedType(" ".to_string())
                ].into_iter().collect()
            },
        }
    }

    fn get_parent_necessary_types(&self) -> Vec<KotlinTypes> {
        match self {
            MutationOperators::ArthimeticOperator => 
                vec![
                    KotlinTypes::AdditiveExpression,
                    KotlinTypes::MultiplicativeExpression,
                ],
            MutationOperators::ArthimeticRemovalOperator => todo!(),
            MutationOperators::LogicalOperator => 
                vec![
                    KotlinTypes::ConjunctionExpression,
                    KotlinTypes::DisjunctionExpression,
                ],
            MutationOperators::RelationalOperator => 
                vec![
                    KotlinTypes::EqualityExpression,
                    KotlinTypes::ComparisonExpression,
                ],
            MutationOperators::AssignmentOperator => 
                vec![
                    KotlinTypes::Assignment,
                ],
            MutationOperators::UnaryOperator => 
                vec![
                    KotlinTypes::PostfixExpression,
                    KotlinTypes::PrefixExpression,
                ],
        }
    }

    pub fn find_mutation(&self, ast: tree_sitter::Tree) -> Vec<Mutation> {
        // tracing::debug!("Finding mutations for {:?}", self);
        let mut mutations = Vec::new();
        let mut cursor = ast.walk();
        let root = ast.root_node();
        // tracing::debug!("Root node: {:?}", root);
        self.mutate(root, &mut cursor, None, &mut mutations);
        mutations
    }

    fn mutate(
        &self,
        root: tree_sitter::Node,
        cursor: &mut tree_sitter::TreeCursor,
        parent: Option<tree_sitter::Node>,
        mutations_made: &mut Vec<Mutation>,
    ) {
        root
        .children(&mut cursor.clone())
        .for_each(|node| {
            let root_type = KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");
            let parent_type = parent.map(|p| KotlinTypes::new(p.kind()).expect("Failed to convert to KotlinType"));
            // tracing::debug!("Root: {:?}, Parent: {:?}", root_type, parent_type);
            self.mutate_operator(
                &root,
                &root_type,
                &parent_type,
                mutations_made,
                self.get_operators(),
                self.get_parent_necessary_types(),
            );
            self.mutate(
                node, 
                cursor, 
                Some(node), 
                mutations_made
            );
        });
    }

    fn mutate_operator(
        &self,
        root_node: &tree_sitter::Node,
        root: &KotlinTypes,
        parent: &Option<KotlinTypes>,
        mutations_made: &mut Vec<Mutation>,
        mutation_operators: HashSet<KotlinTypes>,
        parent_types: Vec<KotlinTypes>,
    ) {
        // Check to see if root type is in the mutation_operators
        if !mutation_operators.contains(root) {
            return;
        }
        // Check to see if the parent exists
        if parent.is_none() {
            return;
        }

        // Checks to see if the parent is the necessary kotlin type
        let parent = parent.as_ref().unwrap();
        if !parent_types.contains(parent) {
            return;
        }
        let mut random_operator;
        loop {
            // Get a random opertor from mutation_operators
            random_operator = mutation_operators.iter().choose(&mut rand::thread_rng()).unwrap();
            if random_operator != root {
                break;
            }
        }
        // Parent was a comparison operator, so we can mutate the root
        let mutation = Mutation::new(
            root_node.start_byte(),
            root_node.end_byte(),
            random_operator.to_string(),
            root.as_str(),
            root_node.start_position().row + 1
        );
        mutations_made.push(mutation)
    }
}

impl AllMutationOperators {
    pub fn new() -> Self {
        Self {
            mutation_operators: vec![
                MutationOperators::ArthimeticOperator,
                MutationOperators::ArthimeticRemovalOperator,
                MutationOperators::LogicalOperator,
                MutationOperators::RelationalOperator,
                MutationOperators::AssignmentOperator,
                MutationOperators::UnaryOperator,
            ],
        }
    }
}

impl Iterator for AllMutationOperators {
    type Item = MutationOperators;

    fn next(&mut self) -> Option<Self::Item> {
        self.mutation_operators.pop()
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::Parser;
    use super::*;
    
    fn get_ast(text: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_kotlin::language()).unwrap();
        let tree = parser.parse(text, None).unwrap();
        tree
    }
    
    const KOTLIN_TEST_CODE: &str = r#"
    fun main() {
        // Arithmetic expressions
        val a = 10
        val b = 3
        val c = a + b
        val d = a - b
        val e = a * b
        val f = a / b
        val g = a % b
    "#;

    #[test]
    fn test_arthimetic_operator() {
        let tree = get_ast(KOTLIN_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ArthimeticOperator.mutate(root, &mut root.walk(), None, &mut mutations_made);
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 5);
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    const KOTLIN_RELATIONAL_TEST_CODE: &str = r#"
    fun main() {
        // Relational expressions
        val a = 10
        val b = 3
        val c = a > b
        val d = a < b
        val e = a >= b
        val f = a <= b
        val g = a == b
        val h = a != b
    "#;

    #[test]
    fn test_relational_operator() {
        let tree = get_ast(KOTLIN_RELATIONAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::RelationalOperator.mutate(root, &mut root.walk(), None, &mut mutations_made);

        assert_eq!(mutations_made.len(), 6);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    const KOTLIN_LOGICAL_TEST_CODE: &str = r#"
    fun main() {
        // Logical expressions
        val a = true
        val b = false
        val c = a && b
        val d = a || b
    "#;

    #[test]
    fn test_logical_operator() {
        let tree = get_ast(KOTLIN_LOGICAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::LogicalOperator.mutate(root, &mut root.walk(), None, &mut mutations_made);
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 2);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    const KOTLIN_ASSIGNMENT_TEST_CODE: &str = r#"
        var h = 5
        h += 3
        h -= 1
        h *= 2
        h /= 4
        h %= 2
    "#;
    
    #[test]
    fn test_assignment_operator() {
        let tree = get_ast(KOTLIN_ASSIGNMENT_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::AssignmentOperator.mutate(root, &mut root.walk(), None, &mut mutations_made);
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 5);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    const KOTLIN_UNARY_TEST_CODE: &str = r#"
        var h = 5
        h++
        h--
        ++h
        --h
    "#;

    #[test]
    fn test_unary_operator() {
        let tree = get_ast(KOTLIN_UNARY_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::UnaryOperator.mutate(root, &mut root.walk(), None, &mut mutations_made);
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 4);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }
}