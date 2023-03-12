use crate::{kotlin_types::KotlinTypes, Mutation};
use rand::seq::IteratorRandom;
use std::collections::HashSet;

#[derive(Clone)]
pub struct AllMutationOperators {
    mutation_operators: Vec<MutationOperators>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord,)]
pub enum MutationOperators {
    ArthimeticOperator,
    UnaryRemovalOperator,
    LogicalOperator,
    RelationalOperator,
    AssignmentOperator,
    UnaryOperator,
}

impl MutationOperators {
    fn get_operators(&self) -> HashSet<KotlinTypes> {
        match self {
            MutationOperators::ArthimeticOperator => vec![
                KotlinTypes::NonNamedType("+".to_string()),
                KotlinTypes::NonNamedType("-".to_string()),
                KotlinTypes::NonNamedType("*".to_string()),
                KotlinTypes::NonNamedType("/".to_string()),
                KotlinTypes::NonNamedType("%".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::UnaryRemovalOperator => vec![
                KotlinTypes::NonNamedType("+".to_string()),
                KotlinTypes::NonNamedType("-".to_string()),
                KotlinTypes::NonNamedType("!".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::LogicalOperator => vec![
                KotlinTypes::NonNamedType("&&".to_string()),
                KotlinTypes::NonNamedType("||".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::RelationalOperator => vec![
                KotlinTypes::NonNamedType("==".to_string()),
                KotlinTypes::NonNamedType("!=".to_string()),
                KotlinTypes::NonNamedType("<".to_string()),
                KotlinTypes::NonNamedType("<=".to_string()),
                KotlinTypes::NonNamedType(">".to_string()),
                KotlinTypes::NonNamedType(">=".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::AssignmentOperator => vec![
                KotlinTypes::NonNamedType("=".to_string()),
                KotlinTypes::NonNamedType("+=".to_string()),
                KotlinTypes::NonNamedType("-=".to_string()),
                KotlinTypes::NonNamedType("*=".to_string()),
                KotlinTypes::NonNamedType("/=".to_string()),
                KotlinTypes::NonNamedType("%=".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::UnaryOperator => vec![
                KotlinTypes::NonNamedType("!".to_string()),
                KotlinTypes::NonNamedType("++".to_string()),
                KotlinTypes::NonNamedType("--".to_string()),
                KotlinTypes::RemoveOperator,
            ]
            .into_iter()
            .collect(),
        }
    }

    fn get_parent_necessary_types(&self) -> Vec<KotlinTypes> {
        match self {
            MutationOperators::ArthimeticOperator => vec![
                KotlinTypes::AdditiveExpression,
                KotlinTypes::MultiplicativeExpression,
            ],
            MutationOperators::UnaryRemovalOperator => vec![KotlinTypes::PrefixExpression],
            MutationOperators::LogicalOperator => vec![
                KotlinTypes::ConjunctionExpression,
                KotlinTypes::DisjunctionExpression,
            ],
            MutationOperators::RelationalOperator => vec![
                KotlinTypes::EqualityExpression,
                KotlinTypes::ComparisonExpression,
            ],
            MutationOperators::AssignmentOperator => vec![KotlinTypes::Assignment],
            MutationOperators::UnaryOperator => vec![
                KotlinTypes::PostfixExpression,
                KotlinTypes::PrefixExpression,
            ],
        }
    }

    pub fn find_mutation(&self, ast: tree_sitter::Tree) -> Vec<Mutation> {
        tracing::debug!("Finding mutations for {:?}", self);
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
        root.children(&mut cursor.clone()).for_each(|node| {
            let root_type = KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");
            let parent_type = parent
                .map(|p| KotlinTypes::new(p.kind()).expect("Failed to convert to KotlinType"));
            self.mutate_operator(
                &node,
                &root_type,
                &parent_type,
                mutations_made,
                self.get_operators(),
                self.get_parent_necessary_types(),
            );
            self.mutate(node, cursor, Some(node), mutations_made);
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
            if *self == MutationOperators::UnaryRemovalOperator {
                // If the operator is a unary removal operator, we just remove the operator
                random_operator = &KotlinTypes::RemoveOperator;
                break;
            }
            // Get a random opertor from mutation_operators
            random_operator = mutation_operators
                .iter()
                .choose(&mut rand::thread_rng())
                .unwrap();
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
            root_node.start_position().row + 1,
            self.clone(),
        );
        mutations_made.push(mutation)
    }
}

impl AllMutationOperators {
    pub fn new() -> Self {
        Self {
            mutation_operators: vec![
                MutationOperators::ArthimeticOperator,
                MutationOperators::UnaryRemovalOperator,
                MutationOperators::LogicalOperator,
                MutationOperators::RelationalOperator,
                MutationOperators::AssignmentOperator,
                MutationOperators::UnaryOperator,
            ],
        }
    }

    pub fn get_mutation_operators(&self) -> Vec<MutationOperators> {
        self.mutation_operators.clone()
    }
}

impl Default for AllMutationOperators {
    fn default() -> Self {
        Self::new()
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
    use super::*;
    use crate::test_config::*;
    use tree_sitter::Parser;

    fn get_ast(text: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_kotlin::language()).unwrap();
        let tree = parser.parse(text, None).unwrap();
        tree
    }

    #[test]
    fn test_arthimetic_operator() {
        let tree = get_ast(KOTLIN_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ArthimeticOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 5);
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    #[test]
    fn test_relational_operator() {
        let tree = get_ast(KOTLIN_RELATIONAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::RelationalOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );

        assert_eq!(mutations_made.len(), 6);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    #[test]
    fn test_logical_operator() {
        let tree = get_ast(KOTLIN_LOGICAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::LogicalOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 2);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    #[test]
    fn test_assignment_operator() {
        let tree = get_ast(KOTLIN_ASSIGNMENT_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::AssignmentOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 5);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

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

    #[test]
    fn test_unary_removal_operator() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::UnaryRemovalOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 3);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
            assert_eq!(mutation.new_op, "".to_string());
        }
    }

    #[test]
    fn test_arthimetic_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ArthimeticOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_relational_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::RelationalOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_logical_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::LogicalOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_assignment_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::AssignmentOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
        );
        assert_eq!(mutations_made.len(), 0);
    }
}
