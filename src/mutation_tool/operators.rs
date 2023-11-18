use rand::Rng;

use crate::{
    error::{KodeKrakenError, Result},
    kotlin_types::KotlinTypes,
    mutation_tool::Mutation,
};
use std::{collections::HashSet, fmt::Display, fs};

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
    NotNullAssertionOperator,
    RemoveElvisOperator,
    ElvisOperator,
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
                MutationOperators::NotNullAssertionOperator => "NotNullAssertionOperator",
                MutationOperators::RemoveElvisOperator => "RemoveElvisOperator",
                MutationOperators::ElvisOperator => "ElvisOperator",
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
            MutationOperators::NotNullAssertionOperator => vec![
                KotlinTypes::NonNamedType("!!".to_string()),
                KotlinTypes::NonNamedType("?".to_string()),
                KotlinTypes::RemoveOperator,
            ]
            .into_iter()
            .collect(),
            MutationOperators::RemoveElvisOperator | MutationOperators::ElvisOperator => {
                vec![KotlinTypes::NonNamedType("?:".to_string())]
                    .into_iter()
                    .collect()
            }
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
            MutationOperators::NotNullAssertionOperator => vec![KotlinTypes::PostfixExpression],
            MutationOperators::RemoveElvisOperator | MutationOperators::ElvisOperator => {
                vec![KotlinTypes::ElvisExpression]
            }
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

    /// Mutates the given `root` node and its children using the provided `cursor`, `parent`, `mutations_made`, `file_name`, `operators`, and `parent_necessary_types`.
    ///
    /// # Arguments
    ///
    /// * `root` - A `tree_sitter::Node` representing the root node to mutate.
    /// * `cursor` - A mutable reference to a `tree_sitter::TreeCursor` used to traverse the syntax tree.
    /// * `parent` - An optional `tree_sitter::Node` representing the parent node of `root`.
    /// * `mutations_made` - A mutable reference to a `Vec<Mutation>` that will be populated with any mutations made during the function's execution.
    /// * `file_name` - A `String` representing the name of the file being mutated.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cursor = tree_sitter::TreeCursor::new();
    /// let mut mutations_made = Vec::new();
    /// let root_node = tree_sitter::Node::new();
    /// let parent_node = tree_sitter::Node::new();
    /// let file_name = String::from("example.kt");
    /// let operator = Operator::new();
    /// let parent_necessary_types = vec![KotlinTypes::IfStatement];
    /// operator.mutate(root_node, &mut cursor, Some(parent_node), &mut mutations_made, &file_name);
    /// ```
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
            if root_type == KotlinTypes::NonNamedType("?:".into()) {
                // println!("Found elvis operator");
                // println!("{:#?}", node);
                // println!("{:#?}", node.parent());
            }
            mutations_made.append(
                &mut self
                    .mutate_operator(&node, &root_type, &parent_type, file_name)
                    .expect("Failed to mutate an operator"),
            );
            self.mutate(node, cursor, Some(node), mutations_made, file_name);
        });
    }

    /// Mutates the given root node based on the specified mutation operators and returns a vector of mutations made.
    ///
    /// # Arguments
    ///
    /// * `root_node` - A reference to the root node of the AST.
    /// * `root` - A reference to the root type of the AST.
    /// * `parent` - An optional reference to the parent type of the AST.
    /// * `mutation_operators` - A HashSet of mutation operators to apply.
    /// * `parent_types` - A vector of parent types to check against.
    /// * `file_name` - The name of the file being mutated.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of mutations made.
    fn mutate_operator(
        &self,
        root_node: &tree_sitter::Node,
        root: &KotlinTypes,
        parent: &Option<KotlinTypes>,
        file_name: &str,
    ) -> Result<Vec<Mutation>> {
        let mut mutations_made = Vec::new();
        let mutation_operators = self.get_operators();
        let parent_types = self.get_parent_necessary_types();
        // Check to see if root type is in the mutation_operators
        // Check to see if the parent exists
        // Checks to see if the parent is the necessary kotlin type
        if !mutation_operators.contains(root)
            || parent.is_none()
            || !parent_types.contains(parent.as_ref().ok_or(KodeKrakenError::ConversionError)?)
        {
            return Ok(mutations_made);
        }

        match *self {
            MutationOperators::UnaryRemovalOperator => {
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
            }
            MutationOperators::RemoveElvisOperator => {
                // If the operator is a Remove elvis operator, we remove the operator and everything after it
                // Get the end byte of the end of the line
                let end_byte = root_node.parent().unwrap().end_byte(); // TODO: remove unwrap

                let mutation = Mutation::new(
                    root_node.start_byte(),
                    end_byte,
                    KotlinTypes::RemoveOperator.to_string(),
                    root.as_str(),
                    root_node.start_position().row + 1,
                    self.clone(),
                    file_name.to_string(),
                );
                mutations_made.push(mutation);
            }
            MutationOperators::ElvisOperator => {
                self.mutate_literal(&root_node.parent().unwrap(), &mut mutations_made, file_name)
            }
            _ => {
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
            }
        }

        Ok(mutations_made)
    }

    fn mutate_literal(
        &self,
        root_node: &tree_sitter::Node,
        mutations_made: &mut Vec<Mutation>,
        file_name: &str,
    ) {
        let file = fs::read(file_name).expect("Failed to read file");
        let file = file.as_slice();
        let children = root_node
            .children(&mut root_node.walk())
            .collect::<Vec<tree_sitter::Node>>();
        println!("Children: {:#?}", children);
        println!("Num of children: {}", children.len());
        let node = children.iter().last().unwrap();

        let child_type = KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");

        println!("Child type: {:#?}", child_type);

        // Change the literal to a different literal
        let mut val = node.utf8_text(file).unwrap();
        match child_type {
            KotlinTypes::IntegerLiteral => {
                let val = val.parse::<i32>().unwrap();
                // Change the value and create a mutation
                let rnd_val = rand::random::<i32>() % 100;
                let mutated_val = val & rnd_val;
                println!("new value is: {}", mutated_val);
                let mutation = Mutation::new(
                    node.start_byte(),
                    node.end_byte(),
                    mutated_val.to_string(),
                    val.to_string(),
                    node.start_position().row + 1,
                    self.clone(),
                    file_name.to_string(),
                );
                mutations_made.push(mutation);
            }
            KotlinTypes::PrefixExpression => {
                // In this case, we need to see the type of the prefix expression, so we need to
                // Recurse down to the literal
                self.mutate_literal(node, mutations_made, file_name)
            }
            KotlinTypes::LineStringLiteral => {
                // Replace the string with a different string
                let val = r#""Hello I am a Mutant!""#.to_string();

                let mutation = Mutation::new(
                    node.start_byte(),
                    node.end_byte(),
                    val,
                    node.utf8_text(file).unwrap().to_string(),
                    node.start_position().row + 1,
                    self.clone(),
                    file_name.to_string(),
                );
                mutations_made.push(mutation);
            }
            KotlinTypes::BooleanLiteral => {
                let val = val.parse::<bool>().unwrap();

                // Change the value and create a mutation
                let mutated_val = !val;

                let mutation = Mutation::new(
                    node.start_byte(),
                    node.end_byte(),
                    mutated_val.to_string(),
                    val.to_string(),
                    node.start_position().row + 1,
                    self.clone(),
                    file_name.to_string(),
                );
                mutations_made.push(mutation);
            }
            KotlinTypes::LongLiteral => {
                // Need to strip off the l at the end
                if val.ends_with("L") {
                    val = val.strip_suffix("L").unwrap();
                }

                let val = val.parse::<i64>().unwrap();
                println!("Value is: {}", val);
                println!("Found long literal");
            }
            KotlinTypes::RealLiteral => {
                // Need to strip off the f at the end
                if val.ends_with("f") {
                    val = val.strip_suffix("f").unwrap();
                }
                let val = val.parse::<f32>().unwrap();
                println!("Value is: {}", val);
                println!("Found real literal");
            }
            KotlinTypes::CharacterLiteral => {
                // Remove the single quotes and get the value
                let val = val
                    .strip_prefix("'")
                    .unwrap()
                    .strip_suffix("'")
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();

                // Get a random character between 'a' and 'z'
                let mut rnd_val = rand::thread_rng().gen_range(b'a'..b'z') as char;
                while rnd_val == val {
                    rnd_val = rand::thread_rng().gen_range(b'a'..b'z') as char;
                }
                let mut_val = format!("'{}'", rnd_val);
                let mutation = Mutation::new(
                    node.start_byte(),
                    node.end_byte(),
                    mut_val,
                    val.to_string(),
                    node.start_position().row + 1,
                    self.clone(),
                    file_name.to_string(),
                );
                mutations_made.push(mutation);
            }
            _ => {}
        }
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
                MutationOperators::NotNullAssertionOperator,
                // MutationOperators::RemoveElvisOperator,
                // MutationOperators::ElvisOperator,
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
    fn test_elvis_remove_operator() {
        let tree = get_ast(KOTLIN_ELVIS_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::RemoveElvisOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        assert_eq!(mutations_made.len(), 1);
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

    #[test]
    fn test_remove_elvis_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::RemoveElvisOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        assert_eq!(mutations_made.len(), 0);
    }
}
