use std::fmt::Display;

use cli_table::Table;
use uuid::Uuid;

use crate::mutation_operators::MutationOperators;

#[derive(Debug, Clone)]
pub enum MutationResult {
    InProgress,
    Success,
    TestFailed,
    BuildFailed,
    Timeout,
}

impl Display for MutationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutationResult::InProgress => write!(f, "In Progress"),
            MutationResult::Success => write!(f, "Mutant Survived"),
            MutationResult::TestFailed => write!(f, "Mutatan Killed"),
            MutationResult::BuildFailed => write!(f, "Build Failed"),
            MutationResult::Timeout => write!(f, "Timeout"),
        }
    }
}

impl Default for MutationResult {
    fn default() -> Self {
        Self::InProgress
    }
}

#[derive(Debug, Clone, Table)]
pub struct Mutation {
    #[table(title = "Id")]
    pub id: Uuid,
    #[table(skip)] pub start_byte: usize,
    #[table(skip)] pub end_byte: usize,
    #[table(title = "Line Number")]
    pub line_number: usize,
    #[table(title = "New Operator")]
    pub new_op: String,
    #[table(title = "Old Operator")]
    pub old_op: String,
    #[table(title = "Mutation Type")]
    pub mutation_type: MutationOperators,
    #[table(title = "Result")]
    pub result: MutationResult
}

impl Mutation {
    pub fn new(
        start_byte: usize,
        end_byte: usize,
        new_op: String,
        old_op: String,
        line_number: usize,
        mutation_type: MutationOperators,
    ) -> Self {
        Self {
            start_byte,
            end_byte,
            line_number,
            new_op,
            old_op,
            mutation_type,
            id: Uuid::new_v4(),
            result: MutationResult::default()
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
            Mutation:
            {}
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

#[derive(Debug, Clone)]
pub struct FileMutations {
    pub mutations: Vec<Mutation>,
}