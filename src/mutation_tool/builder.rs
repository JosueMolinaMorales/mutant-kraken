use crate::{cli::MutationCommandConfig, config::KodeKrakenConfig};

use super::{AllMutationOperators, MutationOperators, MutationTool, OUT_DIRECTORY};

pub struct MutationToolBuilder {
    mutate_config: Option<MutationCommandConfig>,
    kodekraken_config: Option<KodeKrakenConfig>,
    mutation_operators: Option<Vec<MutationOperators>>,
    enable_mutation_comment: bool,
}

impl Default for MutationToolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for the MutationTool
impl MutationToolBuilder {
    pub fn new() -> Self {
        Self {
            mutate_config: None,
            kodekraken_config: None,
            mutation_operators: None,
            enable_mutation_comment: false,
        }
    }

    /// Sets the general config for the mutation tool
    pub fn set_general_config(mut self, config: KodeKrakenConfig) -> Self {
        self.kodekraken_config = Some(config);
        self
    }

    /// Set the path to the files to be mutated
    pub fn set_mutate_config(mut self, config: MutationCommandConfig) -> Self {
        self.mutate_config = Some(config);
        self
    }

    /// Set the mutation operators to be used
    pub fn set_mutation_operators(mut self, mutation_operators: Vec<MutationOperators>) -> Self {
        self.mutation_operators = Some(mutation_operators);
        self
    }

    /// Sets whether to enable the mutation comment
    pub fn set_mutation_comment(mut self, enable_mutation_comment: bool) -> Self {
        self.enable_mutation_comment = enable_mutation_comment;
        self
    }

    pub fn build(self) -> MutationTool {
        let mutate_config = self.mutate_config.unwrap_or_default();
        let kodekraken_config = self.kodekraken_config.unwrap_or_default();
        let mutation_operators = self
            .mutation_operators
            .unwrap_or(AllMutationOperators::new().get_mutation_operators());
        MutationTool::new(
            mutate_config,
            kodekraken_config,
            OUT_DIRECTORY.into(),
            mutation_operators,
            self.enable_mutation_comment,
        )
        .unwrap()
    }
}
