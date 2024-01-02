use std::{fs, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

use crate::mutation_tool::MutationOperators;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct KodeKrakenConfig {
    pub general: GeneralConfig,
    pub ignore: IgnoreConfig,
    pub threading: ThreadingConfig,
    pub output: OutputConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GeneralConfig {
    /// The time in seconds to wait for the mutation tool to finish
    /// before killing the process
    pub timeout: Option<u64>,

    #[serde(default)]
    pub operators: Vec<MutationOperators>,
}

impl Default for GeneralConfig {
    /// Set Default timeout of 5 minutes
    fn default() -> Self {
        use MutationOperators::*;
        Self {
            timeout: None,
            operators: vec![
                ArithmeticReplacementOperator,
                UnaryRemovalOperator,
                LogicalReplacementOperator,
                RelationalReplacementOperator,
                AssignmentReplacementOperator,
                UnaryReplacementOperator,
                NotNullAssertionOperator,
                ElvisRemoveOperator,
                ElvisLiteralChangeOperator,
                LiteralChangeOpeator,
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LoggingConfig {
    pub log_level: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct IgnoreConfig {
    pub ignore_files: Vec<String>,
    pub ignore_directories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ThreadingConfig {
    pub max_threads: usize,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct OutputConfig {
    pub display_end_table: bool,
}

impl KodeKrakenConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_config<P: AsRef<Path>>(path: P) -> Self {
        let mut config = match fs::File::open(path.as_ref().join("kodekraken.config.json")) {
            Ok(file) => {
                let buffer_reader = BufReader::new(file);
                match serde_json::from_reader(buffer_reader) {
                    Ok(config) => config,
                    Err(e) => {
                        println!("[WARNING] ⚠️  Could not parse config file, view logs for error.");
                        tracing::warn!(
                            "Could not parse config file, using default config. Error: {}",
                            e
                        );
                        println!("{:?}", e);
                        Self::default()
                    }
                }
            }
            Err(_) => {
                println!("[WARNING] ⚠️  Could not find kodekraken.config.json file in root directory, using default config.");
                Self::default()
            }
        };

        if config.general.operators.is_empty() {
            println!("[WARNING] ⚠️  No mutation operators specified in config file, using default operators.");
            config.general.operators = GeneralConfig::default().operators;
        }

        config
    }
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        Self {
            ignore_files: vec![
                r#"^.*Test\.[^.]*$"#.to_string(), // Ignore all files that end with Test.*
            ],
            ignore_directories: vec![
                "dist".into(),
                "build".into(),
                "bin".into(),
                ".gradle".into(),
                ".idea".into(),
                "gradle".into(),
            ],
        }
    }
}

impl Default for ThreadingConfig {
    fn default() -> Self {
        Self { max_threads: 30 }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::temp_dir, fs::File, io::Write};

    #[test]
    fn test_default_general_config() {
        let default_general = GeneralConfig::default();
        assert_eq!(default_general.timeout, None);
        assert_eq!(
            default_general.operators,
            vec![
                MutationOperators::ArithmeticReplacementOperator,
                MutationOperators::UnaryRemovalOperator,
                MutationOperators::LogicalReplacementOperator,
                MutationOperators::RelationalReplacementOperator,
                MutationOperators::AssignmentReplacementOperator,
                MutationOperators::UnaryReplacementOperator,
                MutationOperators::NotNullAssertionOperator,
                MutationOperators::ElvisRemoveOperator,
                MutationOperators::ElvisLiteralChangeOperator,
                MutationOperators::LiteralChangeOpeator,
            ]
        );
    }

    #[test]
    fn test_default_ignore_config() {
        let default_ignore = IgnoreConfig::default();
        assert_eq!(default_ignore.ignore_files.len(), 1);
        assert_eq!(default_ignore.ignore_directories.len(), 6);
    }

    #[test]
    fn test_default_threading_config() {
        let default_threading = ThreadingConfig::default();
        assert_eq!(default_threading.max_threads, 30);
    }

    #[test]
    fn test_default_logging_config() {
        let default_logging = LoggingConfig::default();
        assert_eq!(default_logging.log_level, "info");
    }

    #[test]
    fn test_default_output_config() {
        let default_output = OutputConfig::default();
        assert_eq!(default_output.display_end_table, false);
    }

    #[test]
    fn test_new_kodekraken_config() {
        let config = KodeKrakenConfig::new();
        assert_eq!(config.general.timeout, None);
        assert_eq!(
            config.general.operators,
            vec![
                MutationOperators::ArithmeticReplacementOperator,
                MutationOperators::UnaryRemovalOperator,
                MutationOperators::LogicalReplacementOperator,
                MutationOperators::RelationalReplacementOperator,
                MutationOperators::AssignmentReplacementOperator,
                MutationOperators::UnaryReplacementOperator,
                MutationOperators::NotNullAssertionOperator,
                MutationOperators::ElvisRemoveOperator,
                MutationOperators::ElvisLiteralChangeOperator,
                MutationOperators::LiteralChangeOpeator,
            ]
        );
        assert_eq!(config.ignore.ignore_files.len(), 1);
        assert_eq!(config.ignore.ignore_directories.len(), 6);
        assert_eq!(config.threading.max_threads, 30);
        assert_eq!(config.output.display_end_table, false);
        assert_eq!(config.logging.log_level, "info");
    }

    #[test]
    fn test_load_config_from_valid_file() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.join("kodekraken.config.json");
        let mut file = File::create(file_path).expect("Failed to create temporary file");

        // Create a valid JSON content
        let config = KodeKrakenConfig {
            general: GeneralConfig {
                timeout: Some(10),
                operators: vec![
                    MutationOperators::UnaryRemovalOperator,
                    MutationOperators::AssignmentReplacementOperator,
                ],
            },
            ignore: IgnoreConfig {
                ignore_files: vec!["file1".into(), "file2".into()],
                ignore_directories: vec!["dir1".into(), "dir2".into()],
            },
            threading: ThreadingConfig { max_threads: 42 },
            output: OutputConfig {
                display_end_table: true,
            },
            logging: LoggingConfig {
                log_level: "debug".into(),
            },
        };

        let config_json = serde_json::to_string_pretty(&config).unwrap();
        file.write_all(config_json.as_bytes())
            .expect("Failed to write to temporary file");

        let config = KodeKrakenConfig::load_config(temp_dir);
        assert_eq!(config.general.timeout, Some(10));
        assert_eq!(
            config.general.operators,
            vec![
                MutationOperators::UnaryRemovalOperator,
                MutationOperators::AssignmentReplacementOperator
            ]
        );
        assert_eq!(config.ignore.ignore_files, vec!["file1", "file2"]);
        assert_eq!(config.ignore.ignore_directories, vec!["dir1", "dir2"]);
        assert_eq!(config.threading.max_threads, 42);
        assert_eq!(config.output.display_end_table, true);
        assert_eq!(config.logging.log_level, "debug");
    }

    #[test]
    fn test_load_config_from_invalid_file() {
        // Create an invalid JSON content
        let temp_dir = temp_dir().join("invalid_config");
        // Create the temporary directory
        fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");
        // Create the temporary file
        let file_path = temp_dir.join("kodekraken.config.json");
        let mut file = File::create(file_path).expect("Failed to create temporary file");

        let invalid_json_content = r#"
            {
                "general": {
                    "timeout": "invalid_timeout_value"
                }
            }
        "#;

        writeln!(file, "{}", invalid_json_content).expect("Failed to write to temporary file");
        let config = KodeKrakenConfig::load_config(temp_dir);
        // Since the JSON is invalid, it should fall back to default values
        assert_eq!(config.general.timeout, None);
        assert_eq!(
            config.general.operators,
            vec![
                MutationOperators::ArithmeticReplacementOperator,
                MutationOperators::UnaryRemovalOperator,
                MutationOperators::LogicalReplacementOperator,
                MutationOperators::RelationalReplacementOperator,
                MutationOperators::AssignmentReplacementOperator,
                MutationOperators::UnaryReplacementOperator,
                MutationOperators::NotNullAssertionOperator,
                MutationOperators::ElvisRemoveOperator,
                MutationOperators::ElvisLiteralChangeOperator,
                MutationOperators::LiteralChangeOpeator,
            ]
        );
    }

    #[test]
    fn test_load_config_from_missing_file() {
        // Create a temp directory
        let temp_dir = temp_dir().join("missing_config");
        // Create the temp directory
        fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");
        // Test loading config from a missing file
        let config = KodeKrakenConfig::load_config(temp_dir);
        // Since the file is missing, it should fall back to default values
        assert_eq!(config.general.timeout, None);
        assert_eq!(
            config.general.operators,
            vec![
                MutationOperators::ArithmeticReplacementOperator,
                MutationOperators::UnaryRemovalOperator,
                MutationOperators::LogicalReplacementOperator,
                MutationOperators::RelationalReplacementOperator,
                MutationOperators::AssignmentReplacementOperator,
                MutationOperators::UnaryReplacementOperator,
                MutationOperators::NotNullAssertionOperator,
                MutationOperators::ElvisRemoveOperator,
                MutationOperators::ElvisLiteralChangeOperator,
                MutationOperators::LiteralChangeOpeator,
            ]
        );
    }
}
