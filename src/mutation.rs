use std::fmt::Display;

use uuid::Uuid;

use crate::mutation_operators::MutationOperators;

#[derive(Debug, Clone)]
pub struct Mutation {
    pub start_byte: usize,
    pub end_byte: usize,
    pub line_number: usize,
    pub new_op: String,
    pub old_op: String,
    pub mutation_type: MutationOperators,
    pub id: Uuid,
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
        }
    }
}

impl Display for Mutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/**
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