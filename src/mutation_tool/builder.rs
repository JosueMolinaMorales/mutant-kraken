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
#[cfg(test)]
mod tests {
    use crate::config::GeneralConfig;
    use std::env::temp_dir;

    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_default_builder() {
        let builder = MutationToolBuilder::new();
        let mutation_tool = builder.build();

        // Add assertions based on your specific default values
        assert_eq!(mutation_tool.enable_mutation_comment, false);
        assert_eq!(mutation_tool.kodekraken_config, KodeKrakenConfig::new());
        assert_eq!(
            mutation_tool.mutate_config,
            MutationCommandConfig::default()
        );
        assert_eq!(
            Arc::into_inner(mutation_tool.mutation_operators).unwrap(),
            AllMutationOperators::new().get_mutation_operators()
        );
    }

    #[test]
    fn test_set_general_config() {
        let general_config = KodeKrakenConfig {
            general: GeneralConfig {
                timeout: Some(10),
                operators: vec![
                    MutationOperators::AssignmentReplacementOperator,
                    MutationOperators::UnaryRemovalOperator,
                ],
            },
            ..Default::default()
        };

        let builder = MutationToolBuilder::new().set_general_config(general_config.clone());
        let mutation_tool = builder.build();

        assert_eq!(mutation_tool.kodekraken_config.general.timeout, Some(10));
        assert_eq!(
            mutation_tool.kodekraken_config.general.operators,
            vec![
                MutationOperators::AssignmentReplacementOperator,
                MutationOperators::UnaryRemovalOperator
            ]
        );
    }

    #[test]
    fn test_set_mutate_config() {
        // Create a temp directory
        let temp_dir = temp_dir().join("set_mutate_config");
        // Create the temp directory
        std::fs::create_dir_all(&temp_dir).unwrap();

        let mutate_config = MutationCommandConfig {
            path: temp_dir.to_str().unwrap().to_string(),
        };

        let builder = MutationToolBuilder::new().set_mutate_config(mutate_config.clone());
        let mutation_tool = builder.build();

        // Add assertions based on your specific mutate_config fields
        assert_eq!(mutation_tool.mutate_config, mutate_config);
        assert_eq!(
            mutation_tool.mutate_config.path,
            temp_dir.to_str().unwrap().to_string()
        );
    }

    #[test]
    fn test_set_mutation_operators() {
        let mutation_operators = vec![
            MutationOperators::AssignmentReplacementOperator,
            MutationOperators::ArithmeticReplacementOperator,
        ];

        let builder = MutationToolBuilder::new().set_mutation_operators(mutation_operators.clone());
        let mutation_tool = builder.build();

        assert_eq!(
            Arc::into_inner(mutation_tool.mutation_operators).unwrap(),
            mutation_operators
        );
    }

    #[test]
    fn test_set_mutation_comment() {
        let builder = MutationToolBuilder::new().set_mutation_comment(true);
        let mutation_tool = builder.build();

        assert_eq!(mutation_tool.enable_mutation_comment, true);
    }

    #[test]
    fn test_build_with_defaults() {
        let builder = MutationToolBuilder::new();
        let mutation_tool = builder.build();

        // Add assertions based on your specific default values
        assert_eq!(mutation_tool.enable_mutation_comment, false);
        assert_eq!(mutation_tool.kodekraken_config, KodeKrakenConfig::default());
        assert_eq!(
            mutation_tool.mutate_config,
            MutationCommandConfig::default()
        );
        assert_eq!(
            Arc::into_inner(mutation_tool.mutation_operators).unwrap(),
            AllMutationOperators::new().get_mutation_operators()
        );
    }
}
