use std::fmt::Display;

use cli_table::Table;
use uuid::Uuid;

use crate::mutation_tool::MutationOperators;

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub enum MutationResult {
    InProgress,
    Survived,
    Killed,
    BuildFailed,
    Timeout,
    Failed,
}

impl Display for MutationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutationResult::InProgress => write!(f, "In Progress"),
            MutationResult::Survived => write!(f, "Mutant Survived"),
            MutationResult::Killed => write!(f, "Mutant Killed"),
            MutationResult::BuildFailed => write!(f, "Build Failed"),
            MutationResult::Timeout => write!(f, "Timeout"),
            MutationResult::Failed => write!(f, "Failed"),
        }
    }
}

impl Default for MutationResult {
    fn default() -> Self {
        Self::InProgress
    }
}

#[derive(Debug, Clone, Table, serde::Serialize)]
/// Represents a mutation applied to a code file.
pub struct Mutation {
    /// The unique identifier for the mutation.
    #[table(title = "Id")]
    #[serde(skip)]
    pub id: Uuid,
    /// The starting byte of the old operator.
    #[table(skip)]
    #[serde(skip)]
    pub start_byte: usize,
    /// The ending byte of the old operator.
    #[table(skip)]
    #[serde(skip)]
    pub end_byte: usize,
    /// The name of the file that was mutated.
    #[table(title = "File Name")]
    pub file_name: String,
    /// The line number where the mutation was applied.
    #[table(title = "Line Number")]
    pub line_number: usize,
    /// The new operator that was applied.
    #[table(title = "New Operator")]
    pub new_op: String,
    /// The old operator that was replaced.
    #[table(title = "Old Operator")]
    pub old_op: String,
    /// The type of mutation that was applied.
    #[table(title = "Mutation Type")]
    pub mutation_type: MutationOperators,
    /// The result of the mutation.
    #[table(title = "Result")]
    pub result: MutationResult,
}

impl Mutation {
    pub fn new(
        start_byte: usize,
        end_byte: usize,
        new_op: String,
        old_op: String,
        line_number: usize,
        mutation_type: MutationOperators,
        file_name: String,
    ) -> Self {
        Self {
            start_byte,
            end_byte,
            line_number,
            new_op,
            old_op,
            mutation_type,
            id: Uuid::new_v4(),
            result: MutationResult::default(),
            file_name,
        }
    }
}

impl Display for Mutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            /**
            AUTO GENERATED COMMENT
            Mutation Operator: {}
            Line number: {}
            Id: {},
            Old Operator: {},
            New Operator: {}
            */",
            self.mutation_type,
            (self.line_number + 9),
            self.id,
            self.old_op,
            self.new_op
        )
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileMutations {
    pub mutations: Vec<Mutation>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mutation_result() {
        let default_result = MutationResult::default();
        assert_eq!(default_result, MutationResult::InProgress);
    }

    #[test]
    fn test_display_mutation_result() {
        let result = MutationResult::Timeout;
        let formatted_result = format!("{}", result);
        assert_eq!(formatted_result, "Timeout");
    }

    #[test]
    fn test_create_mutation() {
        let mutation = Mutation::new(
            10, // start_byte
            20, // end_byte
            "new_op".to_string(),
            "old_op".to_string(),
            42, // line_number
            MutationOperators::ArithmeticReplacementOperator,
            "example.rs".to_string(),
        );

        assert_eq!(mutation.start_byte, 10);
        assert_eq!(mutation.end_byte, 20);
        assert_eq!(mutation.new_op, "new_op");
        assert_eq!(mutation.old_op, "old_op");
        assert_eq!(mutation.line_number, 42);
        assert_eq!(
            mutation.mutation_type,
            MutationOperators::ArithmeticReplacementOperator
        );
        assert_eq!(mutation.file_name, "example.rs");
        assert_ne!(mutation.id, Uuid::nil()); // Ensure that UUID is not nil
        assert_eq!(mutation.result, MutationResult::InProgress);
    }

    #[test]
    fn test_display_mutation() {
        let mutation = Mutation::new(
            10, // start_byte
            20, // end_byte
            "new_op".to_string(),
            "old_op".to_string(),
            42, // line_number
            MutationOperators::ArithmeticReplacementOperator,
            "example.rs".to_string(),
        );

        let expected_output = format!(
            "
            /**
            AUTO GENERATED COMMENT
            Mutation Operator: ArithmeticReplacementOperator
            Line number: {}
            Id: {},
            Old Operator: old_op,
            New Operator: new_op
            */",
            (42 + 9),
            mutation.id
        );

        assert_eq!(format!("{}", mutation), expected_output);
    }

    #[test]
    fn test_create_file_mutations() {
        let mutations = vec![
            Mutation::new(
                10, // start_byte
                20, // end_byte
                "new_op1".to_string(),
                "old_op1".to_string(),
                42, // line_number
                MutationOperators::ArithmeticReplacementOperator,
                "example.rs".to_string(),
            ),
            Mutation::new(
                30, // start_byte
                40, // end_byte
                "new_op2".to_string(),
                "old_op2".to_string(),
                56, // line_number
                MutationOperators::UnaryRemovalOperator,
                "example.rs".to_string(),
            ),
        ];

        let file_mutations = FileMutations { mutations };

        assert_eq!(file_mutations.mutations.len(), 2);
        // Add more assertions based on your specific requirements
    }
}
