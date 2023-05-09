use crate::{
    error::{KodeKrakenError, Result},
    kotlin_types::KotlinTypes,
    Mutation,
};
use std::{collections::HashSet, fmt::Display};

// Struct that stores all the mutations operators by default
#[derive(Clone)]
pub struct AllMutationOperators {
    mutation_operators: Vec<MutationOperators>,
}

// The different types of mutation operators that can be performed on a file
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum MutationOperators {
    ArthimeticOperator,
    UnaryRemovalOperator,
    LogicalOperator,
    RelationalOperator,
    AssignmentOperator,
    UnaryOperator,
    SafeCallOperator,
}

impl Display for MutationOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MutationOperators::ArthimeticOperator => "ArthimeticOperator",
                MutationOperators::UnaryRemovalOperator => "UnaryRemovalOperator",
                MutationOperators::LogicalOperator => "LogicalOperator",
                MutationOperators::RelationalOperator => "RelationalOperator",
                MutationOperators::AssignmentOperator => "AssignmentOperator",
                MutationOperators::UnaryOperator => "UnaryOperator",
                MutationOperators::SafeCallOperator => "SafeCallOperator",
            }
        )
    }
}

impl MutationOperators {
    /// Get the operators that correspond to the mutation operator
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
            MutationOperators::SafeCallOperator => vec![
                KotlinTypes::NonNamedType("?.".to_string()),
                KotlinTypes::NonNamedType(".".to_string()),
                KotlinTypes::RemoveOperator,
            ] 
            .into_iter()
            .collect(),
        }
    }

    /// Get the parent types that are necessary for the mutation operator
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
            MutationOperators::SafeCallOperator => vec! [KotlinTypes::CallExpression]
        }
    }

    /// Gets all the muatations that can be made to the file based on the the mutation operator
    pub fn find_mutation(&self, ast: &tree_sitter::Tree, file_name: &String) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let mut cursor = ast.walk();
        let root = ast.root_node();
        self.mutate(root, &mut cursor, None, &mut mutations, file_name);
        mutations
    }

    /// Recursive function that finds all the mutations that can be made to the file
    fn mutate(
        &self,
        root: tree_sitter::Node,
        cursor: &mut tree_sitter::TreeCursor,
        parent: Option<tree_sitter::Node>,
        mutations_made: &mut Vec<Mutation>,
        file_name: &String,
    ) {
        root.children(&mut cursor.clone()).for_each(|node| {
            let root_type = KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");
            let parent_type = parent
                .map(|p| KotlinTypes::new(p.kind()).expect("Failed to convert to KotlinType"));
            mutations_made.append(
                &mut self
                    .mutate_operator(
                        &node,
                        &root_type,
                        &parent_type,
                        self.get_operators(),
                        self.get_parent_necessary_types(),
                        file_name,
                    )
                    .expect("Failed to mutate an operator"),
            );
            self.mutate(node, cursor, Some(node), mutations_made, file_name);
        });
    }

    /// Checks to see if the mutation operator can be applied to the node
    fn mutate_operator(
        &self,
        root_node: &tree_sitter::Node,
        root: &KotlinTypes,
        parent: &Option<KotlinTypes>,
        mutation_operators: HashSet<KotlinTypes>,
        parent_types: Vec<KotlinTypes>,
        file_name: &str,
    ) -> Result<Vec<Mutation>> {
        let mut mutations_made = Vec::new();
        // Check to see if root type is in the mutation_operators
        // Check to see if the parent exists
        // Checks to see if the parent is the necessary kotlin type
        if !mutation_operators.contains(root)
            || parent.is_none()
            || !parent_types.contains(parent.as_ref().ok_or(KodeKrakenError::ConversionError)?)
        {
            return Ok(mutations_made);
        }

        if *self == MutationOperators::UnaryRemovalOperator {
            // If the operator is a unary removal operator, we just remove the operator
            let mutation = Mutation::new(
                root_node.start_byte(),
                root_node.end_byte(),
                KotlinTypes::RemoveOperator.to_string(),
                root.as_str(),
                root_node.start_position().row + 1,
                self.clone(),
                file_name.to_string(),
            );
            mutations_made.push(mutation);
            return Ok(mutations_made);
        }

        // Create a mutant for all mutation operators
        mutation_operators.iter().for_each(|operator| {
            if operator != root {
                let mutation = Mutation::new(
                    root_node.start_byte(),
                    root_node.end_byte(),
                    operator.to_string(),
                    root.as_str(),
                    root_node.start_position().row + 1,
                    self.clone(),
                    file_name.to_string(),
                );
                mutations_made.push(mutation)
            }
        });
        Ok(mutations_made)
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
                MutationOperators::SafeCallOperator,
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
        parser
            .set_language(tree_sitter_kotlin::language())
            .expect("Failed to set language for parser");
        parser.parse(text, None).expect("Failed to parse text")
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
            &"".into(),
        );
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 20);
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
            &"".into(),
        );

        assert_eq!(mutations_made.len(), 30);
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
            &"".into(),
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
            &"".into(),
        );
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 25);
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
        MutationOperators::UnaryOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        dbg!(&mutations_made);
        assert_eq!(mutations_made.len(), 12);
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
            &"".into(),
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
            &"".into(),
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
            &"".into(),
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
            &"".into(),
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
            &"".into(),
        );
        assert_eq!(mutations_made.len(), 0);
    }
}
