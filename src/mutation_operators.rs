use std::collections::HashSet;
use rand::seq::{SliceRandom, IteratorRandom};
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
                ].into_iter().collect()
            },
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
            match &self {
                MutationOperators::ArthimeticOperator => todo!(),
                MutationOperators::ArthimeticRemovalOperator => todo!(),
                MutationOperators::LogicalOperator => todo!(),
                MutationOperators::RelationalOperator => self.mutate_relational_operator(&node, &root_type, &parent_type, mutations_made, self.get_operators()),
                MutationOperators::AssignmentOperator => todo!(),
                MutationOperators::UnaryOperator => todo!(),
            };
           
            self.mutate(
                node, 
                cursor, 
                Some(node), 
                mutations_made
            );
        });
    }

    fn mutate_relational_operator(
        &self,
        root_node: &tree_sitter::Node,
        root: &KotlinTypes,
        parent: &Option<KotlinTypes>,
        mutations_made: &mut Vec<Mutation>,
        mutation_operators: HashSet<KotlinTypes>,
    ) {
        // tracing::debug!("In mutate_relational_operator");
        // Check to see if root is a relational operator
        if !mutation_operators.contains(root) {
            return;
        }
        // tracing::debug!("Did not return for if !mutation_operators.contains(root)");
        // Check to see if the parent is a comparison operator
        if parent.is_none() {
            // tracing::debug!("Returning, parent is none");
            return;
        }

        let parent = parent.as_ref().unwrap();
        if *parent != KotlinTypes::ComparisonExpression && *parent != KotlinTypes::EqualityExpression {
            // tracing::debug!("Returning, parent is not a comparison or equality expression");
            return;
        }
        // tracing::debug!("Did not return for if *parent != KotlinTypes::ComparisonExpression && *parent != KotlinTypes::EqualityExpression");
        // tracing::debug!("Selecting a random operator from {:?}", mutation_operators);
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
        // tracing::debug!("Mutation made");
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
