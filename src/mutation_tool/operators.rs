use rand::{distributions::uniform::SampleUniform, seq::SliceRandom, Rng};
use tree_sitter::Node;

use crate::{
    error::{MutantKrakenError, Result},
    kotlin_types::{KotlinExceptions, KotlinTypes},
    mutation_tool::Mutation,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

// The different types of mutation operators that can be performed on a file
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum MutationOperators {
    /// Replaces an arithmetic operator with a different arithmetic operator
    ArithmeticReplacementOperator,
    /// Removes a unary operator
    UnaryRemovalOperator,
    /// Replaces a logical operator with a different logical operator
    LogicalReplacementOperator,
    /// Replaces a relational operator with a different relational operator
    RelationalReplacementOperator,
    /// Replaces an assignment operator with a different assignment operator
    AssignmentReplacementOperator,
    /// Replaces a unary operator with a different unary operator
    UnaryReplacementOperator,
    /// Replaces a not null assertion operator with a different not null assertion operator
    NotNullAssertionOperator,
    /// Removes an elvis operator
    ElvisRemoveOperator,
    /// Changes the literal of an elvis operator
    ElvisLiteralChangeOperator,
    /// Changes the literal of a literal
    LiteralChangeOperator,
    /// Changes the exception thrown
    ExceptionChangeOperator,
    /// Removes a branch from the when statement if the statement has more than one branch
    WhenRemoveBranchOperator,
    /// Removes a label when continuing, breaking, or returning
    RemoveLabelOperator,
    /// Changes first() to last() and vice versa or find() to findLast() and vice versa
    FunctionalBinaryReplacementOperator,
    /// Changes Any() to All() or None() and vice versa or ForEach() to Map() or Filter() and vice versa
    FunctionalReplacementOperator,
}

impl Display for MutationOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MutationOperators::ArithmeticReplacementOperator => "ArithmeticReplacementOperator",
                MutationOperators::UnaryRemovalOperator => "UnaryRemovalOperator",
                MutationOperators::LogicalReplacementOperator => "LogicalReplacementOperator",
                MutationOperators::RelationalReplacementOperator => "RelationalReplacementOperator",
                MutationOperators::AssignmentReplacementOperator => "AssignmentReplacementOperator",
                MutationOperators::UnaryReplacementOperator => "UnaryReplacementOperator",
                MutationOperators::NotNullAssertionOperator => "NotNullAssertionOperator",
                MutationOperators::ElvisRemoveOperator => "ElvisRemoveOperator",
                MutationOperators::ElvisLiteralChangeOperator => "ElvisLiteralChangeOperator",
                MutationOperators::LiteralChangeOperator => "LiteralChangeOperator",
                MutationOperators::ExceptionChangeOperator => "ExceptionChangeOperator",
                MutationOperators::WhenRemoveBranchOperator => "WhenRemoveBranchOperator",
                MutationOperators::RemoveLabelOperator => "RemoveLabelOperator",
                MutationOperators::FunctionalBinaryReplacementOperator => {
                    "FunctionalBinaryReplacementOperator"
                }
                MutationOperators::FunctionalReplacementOperator => "FunctionalReplacementOperator",
            }
        )
    }
}

impl MutationOperators {
    /// Get the operators that correspond to the mutation operator
    fn get_operators(&self) -> HashSet<KotlinTypes> {
        match self {
            MutationOperators::ArithmeticReplacementOperator => vec![
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
            MutationOperators::LogicalReplacementOperator => vec![
                KotlinTypes::NonNamedType("&&".to_string()),
                KotlinTypes::NonNamedType("||".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::RelationalReplacementOperator => vec![
                KotlinTypes::NonNamedType("==".to_string()),
                KotlinTypes::NonNamedType("!=".to_string()),
                KotlinTypes::NonNamedType("<".to_string()),
                KotlinTypes::NonNamedType("<=".to_string()),
                KotlinTypes::NonNamedType(">".to_string()),
                KotlinTypes::NonNamedType(">=".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::AssignmentReplacementOperator => vec![
                KotlinTypes::NonNamedType("=".to_string()),
                KotlinTypes::NonNamedType("+=".to_string()),
                KotlinTypes::NonNamedType("-=".to_string()),
                KotlinTypes::NonNamedType("*=".to_string()),
                KotlinTypes::NonNamedType("/=".to_string()),
                KotlinTypes::NonNamedType("%=".to_string()),
            ]
            .into_iter()
            .collect(),
            MutationOperators::UnaryReplacementOperator => vec![
                KotlinTypes::NonNamedType("!".to_string()),
                KotlinTypes::NonNamedType("++".to_string()),
                KotlinTypes::NonNamedType("--".to_string()),
                KotlinTypes::RemoveOperator,
            ]
            .into_iter()
            .collect(),
            MutationOperators::NotNullAssertionOperator => vec![
                KotlinTypes::NonNamedType("!!".to_string()),
                KotlinTypes::NonNamedType("?.".to_string()),
                KotlinTypes::RemoveOperator,
            ]
            .into_iter()
            .collect(),
            MutationOperators::ElvisRemoveOperator
            | MutationOperators::ElvisLiteralChangeOperator => {
                vec![KotlinTypes::NonNamedType("?:".to_string())]
                    .into_iter()
                    .collect()
            }
            MutationOperators::LiteralChangeOperator => vec![
                KotlinTypes::IntegerLiteral,
                KotlinTypes::LineStringLiteral,
                KotlinTypes::StringLiteral,
                KotlinTypes::BooleanLiteral,
                KotlinTypes::LongLiteral,
                KotlinTypes::RealLiteral,
                KotlinTypes::CharacterLiteral,
            ]
            .into_iter()
            .collect(),
            MutationOperators::ExceptionChangeOperator => {
                vec![KotlinTypes::NonNamedType("throw".to_string())]
                    .into_iter()
                    .collect()
            }
            MutationOperators::WhenRemoveBranchOperator => {
                vec![KotlinTypes::WhenExpression].into_iter().collect()
            }
            MutationOperators::RemoveLabelOperator => {
                vec![KotlinTypes::JumpExpression].into_iter().collect()
            }
            MutationOperators::FunctionalBinaryReplacementOperator
            | MutationOperators::FunctionalReplacementOperator => {
                vec![KotlinTypes::SimpleIdentifier].into_iter().collect()
            }
        }
    }

    /// Get the parent types that are necessary for the mutation operator
    fn get_parent_necessary_types(&self) -> Vec<KotlinTypes> {
        match self {
            MutationOperators::ArithmeticReplacementOperator => vec![
                KotlinTypes::AdditiveExpression,
                KotlinTypes::MultiplicativeExpression,
            ],
            MutationOperators::UnaryRemovalOperator => vec![KotlinTypes::PrefixExpression],
            MutationOperators::LogicalReplacementOperator => vec![
                KotlinTypes::ConjunctionExpression,
                KotlinTypes::DisjunctionExpression,
            ],
            MutationOperators::RelationalReplacementOperator => vec![
                KotlinTypes::EqualityExpression,
                KotlinTypes::ComparisonExpression,
            ],
            MutationOperators::AssignmentReplacementOperator => vec![KotlinTypes::Assignment],
            MutationOperators::UnaryReplacementOperator => vec![
                KotlinTypes::PostfixExpression,
                KotlinTypes::PrefixExpression,
            ],
            MutationOperators::NotNullAssertionOperator => vec![KotlinTypes::PostfixExpression],
            MutationOperators::ElvisRemoveOperator
            | MutationOperators::ElvisLiteralChangeOperator => {
                vec![KotlinTypes::ElvisExpression]
            }
            MutationOperators::LiteralChangeOperator => vec![
                KotlinTypes::IntegerLiteral,
                KotlinTypes::LineStringLiteral,
                KotlinTypes::StringLiteral,
                KotlinTypes::BooleanLiteral,
                KotlinTypes::LongLiteral,
                KotlinTypes::RealLiteral,
                KotlinTypes::CharacterLiteral,
                KotlinTypes::PrefixExpression,
                KotlinTypes::PostfixExpression,
                KotlinTypes::AdditiveExpression,
                KotlinTypes::MultiplicativeExpression,
                KotlinTypes::ConjunctionExpression,
                KotlinTypes::DisjunctionExpression,
                KotlinTypes::EqualityExpression,
                KotlinTypes::ComparisonExpression,
                KotlinTypes::PropertyDeclaration,
                KotlinTypes::VariableDeclaration,
            ],
            MutationOperators::ExceptionChangeOperator => {
                vec![KotlinTypes::JumpExpression]
            }
            MutationOperators::WhenRemoveBranchOperator
            | MutationOperators::RemoveLabelOperator => vec![KotlinTypes::AnyParent],
            MutationOperators::FunctionalBinaryReplacementOperator
            | MutationOperators::FunctionalReplacementOperator => {
                vec![KotlinTypes::NavigationSuffix]
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
    fn mutate(
        &self,
        root: tree_sitter::Node,
        cursor: &mut tree_sitter::TreeCursor,
        parent: Option<tree_sitter::Node>,
        mutations_made: &mut Vec<Mutation>,
        file_name: &String,
    ) {
        root.children(&mut cursor.clone()).for_each(|node| {
            let root_type = KotlinTypes::new(node.kind())
                .expect(format!("Failed to convert to KotlinType: {:?}", node.kind()).as_str());
            let parent_type = parent
                .map(|p| KotlinTypes::new(p.kind()).expect("Failed to convert to KotlinType"));
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
            || (!parent_types.contains(&KotlinTypes::AnyParent)
                && !parent_types
                    .contains(parent.as_ref().ok_or(MutantKrakenError::ConversionError)?))
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
            MutationOperators::ElvisRemoveOperator => {
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
            MutationOperators::ElvisLiteralChangeOperator
            | MutationOperators::LiteralChangeOperator => {
                self.mutate_literal(&root_node.parent().unwrap(), &mut mutations_made, file_name)
            }
            MutationOperators::ExceptionChangeOperator => {
                self.mutate_exception(root_node, &mut mutations_made, file_name)
            }
            MutationOperators::WhenRemoveBranchOperator => {
                self.mutate_when(root_node, &mut mutations_made, file_name)
            }
            MutationOperators::RemoveLabelOperator => {
                self.mutate_label(root_node, &mut mutations_made, file_name)
            }
            MutationOperators::FunctionalBinaryReplacementOperator => {
                self.mutate_functional_binary(root_node, &mut mutations_made, file_name)
            }
            MutationOperators::FunctionalReplacementOperator => {
                self.mutate_functional(root_node, &mut mutations_made, file_name)
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

    fn mutate_functional(
        &self,
        root_node: &tree_sitter::Node,
        mutations_made: &mut Vec<Mutation>,
        file_name: &str,
    ) {
        let predicates = ["any", "all", "none"];
        let transform = ["forEach", "map", "filter"];

        // Need to make sure the current node is either "any", "all", "none", "forEach", "map", or "filter"
        // If it is, then we need to change it to the other one
        let file = fs::read(file_name).expect("Failed to read file");
        let file = file.as_slice();
        let val = root_node.utf8_text(file).unwrap();

        if !predicates.contains(&val) && !transform.contains(&val) {
            return;
        }

        // Pick a random method that is not the same as the original method
        let mut rng = rand::thread_rng();
        let mut mut_val = val;
        let type_val = if predicates.contains(&val) {
            "predicate"
        } else {
            "transform"
        };
        while mut_val == val {
            mut_val = match type_val {
                "predicate" => predicates.choose(&mut rng).unwrap(),
                "transform" => transform.choose(&mut rng).unwrap(),
                _ => panic!("Invalid type"),
            };
        }

        let mutation = Mutation::new(
            root_node.start_byte(),
            root_node.end_byte(),
            mut_val.into(),
            val.to_string(),
            root_node.start_position().row + 1,
            self.clone(),
            file_name.to_string(),
        );
        mutations_made.push(mutation);
    }

    fn mutate_functional_binary(
        &self,
        root_node: &tree_sitter::Node,
        mutations_made: &mut Vec<Mutation>,
        file_name: &str,
    ) {
        let changes = HashMap::from([
            ("first", "last"),
            ("last", "first"),
            ("firstOrNull", "lastOrNull"),
            ("lastOrNull", "firstOrNull"),
            ("find", "findLast"),
            ("findLast", "find"),
        ]);

        // Need to make sure the current node is either "first", "last", "firstOrNull", or "lastOrNull"
        // If it is, then we need to change it to the other one
        let file = fs::read(file_name).expect("Failed to read file");
        let file = file.as_slice();
        let val = root_node.utf8_text(file).unwrap();

        if !changes.contains_key(val) {
            return;
        }

        let mut_val = changes[val];

        let mutation = Mutation::new(
            root_node.start_byte(),
            root_node.end_byte(),
            mut_val.into(),
            val.to_string(),
            root_node.start_position().row + 1,
            self.clone(),
            file_name.to_string(),
        );
        mutations_made.push(mutation);
    }

    fn mutate_label(
        &self,
        root_node: &tree_sitter::Node,
        mutations_made: &mut Vec<Mutation>,
        file_name: &str,
    ) {
        // Get the value of the node
        let file = fs::read(file_name).expect("Failed to read file");
        let file = file.as_slice();
        let val = root_node.utf8_text(file).unwrap();

        // If the value is not a label, return
        if !val.contains("@")
            && (!val.starts_with("return")
                || !val.starts_with("continue")
                || !val.starts_with("break"))
        {
            return;
        }

        // Remove the label
        let label_start = val.find("@").unwrap();
        let mut_value = val[0..label_start].to_string();

        let mutation = Mutation::new(
            root_node.start_byte(),
            root_node.end_byte(),
            mut_value,
            val.to_string(),
            root_node.start_position().row + 1,
            self.clone(),
            file_name.to_string(),
        );

        mutations_made.push(mutation);
    }

    fn mutate_when(
        &self,
        root_node: &tree_sitter::Node,
        mutations_made: &mut Vec<Mutation>,
        file_name: &str,
    ) {
        // Get the when entry list
        let when_entry_list = root_node
            .children(&mut root_node.walk())
            .filter_map(|node| {
                let kt_node =
                    KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");
                if kt_node == KotlinTypes::WhenEntry {
                    Some(node)
                } else {
                    None
                }
            })
            .collect::<Vec<Node<'_>>>();

        // If the when entry list has more than one entry, remove one of the entries
        if when_entry_list.len() < 2 {
            return;
        }

        // Get a random branch other than the last branch (this is usually the else branch)
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..when_entry_list.len() - 1);
        let node = when_entry_list.get(index).unwrap();

        // Remove the node
        let mutation = Mutation::new(
            node.start_byte(),
            node.end_byte(),
            KotlinTypes::RemoveOperator.to_string(),
            "When Branch".to_string(),
            node.start_position().row + 1,
            self.clone(),
            file_name.to_string(),
        );

        mutations_made.push(mutation);
    }

    fn mutate_exception(
        &self,
        root_node: &tree_sitter::Node,
        mutations_made: &mut Vec<Mutation>,
        file_name: &str,
    ) {
        // Get the sibling of the root node
        let sibling = root_node.next_sibling().unwrap();
        let sibling_type =
            KotlinTypes::new(sibling.kind()).expect("Failed to convert to KotlinType");
        // If the sibling is a call expression, i want to grab the simple identifier (its child)
        if sibling_type != KotlinTypes::CallExpression {
            return;
        }

        let child = sibling.children(&mut sibling.walk()).next().unwrap();
        let child_type = KotlinTypes::new(child.kind()).expect("Failed to convert to KotlinType");
        // Check to see if the child is a simple identifier
        if child_type != KotlinTypes::SimpleIdentifier {
            return;
        }
        // Get the value
        let file = fs::read(file_name).expect("Failed to read file");
        let file = file.as_slice();
        let val = child.utf8_text(file).unwrap();
        let exception: KotlinExceptions = val
            .parse()
            .unwrap_or(KotlinExceptions::ArithmArithmeticException);

        // Pick a random exception that is not the same as the original exception
        let mut_val = exception.get_random_exception().to_string();

        let mutation = Mutation::new(
            child.start_byte(),
            child.end_byte(),
            mut_val,
            val.to_string(),
            child.start_position().row + 1,
            self.clone(),
            file_name.to_string(),
        );

        mutations_made.push(mutation);
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
        let node = match children.iter().last() {
            Some(node) => node,
            None => root_node,
        };

        let child_type = KotlinTypes::new(node.kind())
            .expect(format!("Failed to convert to KotlinType: {:?}", node.kind()).as_str());
        // Change the literal to a different literal
        let mut val = node.utf8_text(file).unwrap();
        match child_type {
            KotlinTypes::IntegerLiteral => {
                let val = val.replace("_", "").parse::<i32>();
                if val.is_err() {
                    return;
                }
                let val = val.unwrap();
                // Change the value and create a mutation
                let mutated_val = generate_random_literal(val, i32::MIN, i32::MAX);
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
            KotlinTypes::StringLiteral | KotlinTypes::LineStringLiteral => {
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

                let val = val.replace("_", "").parse::<i64>();
                if val.is_err() {
                    return;
                }
                let val = val.unwrap();
                // Change the value and create a mutation
                let mutated_val = generate_random_literal(val, i64::MIN, i64::MAX);

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
            KotlinTypes::RealLiteral => {
                // Need to strip off the f at the end
                if val.ends_with("f") {
                    val = val.strip_suffix("f").unwrap();
                }
                let val = val.parse::<f32>();
                if val.is_err() {
                    return;
                }
                let val = val.unwrap();
                // Change the value and create a mutation
                let mutated_val = generate_random_literal(val, 0.0, 1.0);
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

fn generate_random_literal<T>(original_literal: T, min: T, max: T) -> T
where
    T: std::cmp::PartialOrd + std::cmp::PartialEq + Copy + SampleUniform,
{
    let mut rng = rand::thread_rng();

    // Generate a random integer different from the original literal
    let mut random_literal = rng.gen_range(min..max);

    // Ensure the random literal is different from the original
    while random_literal == original_literal {
        random_literal = rng.gen_range(min..max);
    }

    random_literal
}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, io::Write};

    use crate::mutation_tool::test_util::*;

    use super::*;
    use crate::mutation_tool::debug_print_ast;
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
        MutationOperators::ArithmeticReplacementOperator.mutate(
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
        MutationOperators::RelationalReplacementOperator.mutate(
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
        MutationOperators::LogicalReplacementOperator.mutate(
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
        MutationOperators::AssignmentReplacementOperator.mutate(
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
        MutationOperators::UnaryReplacementOperator.mutate(
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
    fn test_not_null_assetion_operator() {
        let tree = get_ast(KOTLIN_TEST_NULL_ASSERTION_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::NotNullAssertionOperator.mutate(
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
    fn test_elvis_remove_operator() {
        let tree = get_ast(KOTLIN_ELVIS_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ElvisRemoveOperator.mutate(
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
    fn test_elvis_literal_operator() {
        // Create a temp file
        let temp_dir = temp_dir();
        let temp_file = temp_dir.join("temp_file.kt");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(KOTLIN_ELVIS_LITERAL_TEST_CODE.as_bytes())
            .expect("Failed to write to temp file");

        let tree = get_ast(KOTLIN_ELVIS_LITERAL_TEST_CODE);

        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ElvisLiteralChangeOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &temp_file.to_str().unwrap().to_string(),
        );
        assert_eq!(mutations_made.len(), 12);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    #[test]
    fn test_literal_change_operator() {
        // Create a temp file
        let temp_dir = temp_dir();
        let temp_file = temp_dir.join("literal_change_temp_file.kt");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(KOTLIN_LITERAL_TEST_CODE.as_bytes())
            .expect("Failed to write to temp file");
        let tree = get_ast(KOTLIN_LITERAL_TEST_CODE);
        let root = tree.root_node();
        debug_print_ast(&root, 0);
        let mut mutations_made = Vec::new();
        MutationOperators::LiteralChangeOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &temp_file.to_str().unwrap().to_string(),
        );
        assert_eq!(mutations_made.len(), 12);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    #[test]
    fn test_exception_change_operator() {
        // Create a temp file
        let temp_dir = temp_dir();
        let temp_file = temp_dir.join("exception_change_temp_file.kt");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(KOTLIN_EXCEPTION_TEST_CODE.as_bytes())
            .expect("Failed to write to temp file");
        let tree = get_ast(KOTLIN_EXCEPTION_TEST_CODE);
        let root = tree.root_node();
        debug_print_ast(&root, 0);
        let mut mutations_made = Vec::new();
        MutationOperators::ExceptionChangeOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &temp_file.to_str().unwrap().to_string(),
        );
        assert_eq!(mutations_made.len(), 3);
        // Assert that the old operator is not the same as the new operator
        for mutation in mutations_made {
            assert_ne!(mutation.old_op, mutation.new_op);
        }
    }

    #[test]
    fn test_when_remove_branch_operator() {
        // Create a temp file
        let temp_dir = temp_dir();
        let temp_file = temp_dir.join("when_remove_branch_temp_file.kt");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(KOTLIN_WHEN_EXPRESSION_TEST_CODE.as_bytes())
            .expect("Failed to write to temp file");
        let tree = get_ast(KOTLIN_WHEN_EXPRESSION_TEST_CODE);
        let root = tree.root_node();
        debug_print_ast(&root, 0);
        let mut mutations_made = Vec::new();
        MutationOperators::WhenRemoveBranchOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &temp_file.to_str().unwrap().to_string(),
        );
        assert_eq!(mutations_made.len(), 2);
    }

    #[test]
    fn test_label_remove_operator() {
        // Create a temp file
        let temp_dir = temp_dir();
        let temp_file = temp_dir.join("label_remove_temp_file.kt");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(KOTLIN_LABEL_REMOVING_TEST_CODE.as_bytes())
            .expect("Failed to write to temp file");
        let tree = get_ast(KOTLIN_LABEL_REMOVING_TEST_CODE);
        let root = tree.root_node();
        debug_print_ast(&root, 0);
        let mut mutations_made = Vec::new();
        MutationOperators::RemoveLabelOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &temp_file.to_str().unwrap().to_string(),
        );
        assert_eq!(mutations_made.len(), 3);
    }

    #[test]
    fn test_functional_binary_replacement_operator() {
        // Create a temp file
        let temp_dir = temp_dir();
        let temp_file = temp_dir.join("functional_find_find_last_temp_file.kt");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(KOTLIN_FUNCTIONAL_BINARY_REPLACEMENT_CODE.as_bytes())
            .expect("Failed to write to temp file");
        let tree = get_ast(KOTLIN_FUNCTIONAL_BINARY_REPLACEMENT_CODE);
        let root = tree.root_node();
        debug_print_ast(&root, 0);
        let mut mutations_made = Vec::new();
        MutationOperators::FunctionalBinaryReplacementOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &temp_file.to_str().unwrap().to_string(),
        );
        assert_eq!(mutations_made.len(), 6);
    }

    #[test]
    fn test_functional_replacement_operator() {
        // Create a temp file
        let temp_dir = temp_dir();
        let temp_file = temp_dir.join("functional_any_all_none_temp_file.kt");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(KOTLIN_FUNCTIONAL_REPLACEMENT_CODE.as_bytes())
            .expect("Failed to write to temp file");
        let tree = get_ast(KOTLIN_FUNCTIONAL_REPLACEMENT_CODE);
        let root = tree.root_node();
        debug_print_ast(&root, 0);
        let mut mutations_made = Vec::new();
        MutationOperators::FunctionalReplacementOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &temp_file.to_str().unwrap().to_string(),
        );
        assert_eq!(mutations_made.len(), 6);
    }

    #[test]
    fn test_arthimetic_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ArithmeticReplacementOperator.mutate(
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
        MutationOperators::RelationalReplacementOperator.mutate(
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
        MutationOperators::LogicalReplacementOperator.mutate(
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
        MutationOperators::AssignmentReplacementOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_when_remove_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::WhenRemoveBranchOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_unary_removal_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_ASSIGNMENT_TEST_CODE);
        let mutations_made =
            MutationOperators::UnaryRemovalOperator.find_mutation(&tree, &"file_name".into());
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_unary_replacement_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_ASSIGNMENT_TEST_CODE);
        let mutations_made =
            MutationOperators::UnaryReplacementOperator.find_mutation(&tree, &"file_name".into());
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_not_null_assertion_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_ASSIGNMENT_TEST_CODE);
        let mutations_made =
            MutationOperators::NotNullAssertionOperator.find_mutation(&tree, &"file_name".into());
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_remove_elvis_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ElvisRemoveOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_elvis_literal_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::ElvisLiteralChangeOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        assert_eq!(mutations_made.len(), 0);
    }

    #[test]
    fn test_label_removal_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let root = tree.root_node();
        let mut mutations_made = Vec::new();
        MutationOperators::RemoveLabelOperator.mutate(
            root,
            &mut root.walk(),
            None,
            &mut mutations_made,
            &"".into(),
        );
        assert!(mutations_made.is_empty());
    }

    #[test]
    fn test_functional_binary_replacement_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let mutations_made = MutationOperators::FunctionalBinaryReplacementOperator
            .find_mutation(&tree, &"file_name".into());
        assert!(mutations_made.is_empty());
    }

    #[test]
    fn test_functional_replacement_operator_does_not_create_mutations() {
        let tree = get_ast(KOTLIN_UNARY_REMOVAL_TEST_CODE);
        let mutations_made = MutationOperators::FunctionalReplacementOperator
            .find_mutation(&tree, &"file_name".into());
        assert!(mutations_made.is_empty());
    }
}
