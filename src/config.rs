use std::{fs, io::BufReader, path::Path};

use serde::Deserialize;

use crate::mutation_tool::MutationOperators;

#[derive(Debug, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct KodeKrakenConfig {
    pub general: GeneralConfig,
    pub ignore: IgnoreConfig,
    pub threading: ThreadingConfig,
    pub output: OutputConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct GeneralConfig {
    /// The time in seconds to wait for the mutation tool to finish
    /// before killing the process
    pub timeout: Option<u64>,
    pub operators: Vec<MutationOperators>,
}

impl Default for GeneralConfig {
    /// Set Default timeout of 5 minutes
    fn default() -> Self {
        Self {
            timeout: None,
            operators: vec![],
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct LoggingConfig {
    pub log_level: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IgnoreConfig {
    pub ignore_files: Vec<String>,
    pub ignore_directories: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ThreadingConfig {
    pub max_threads: usize,
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct OutputConfig {
    pub display_end_table: bool,
}

impl KodeKrakenConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_config<P: AsRef<Path>>(path: P) -> Self {
        match fs::File::open(path.as_ref().join("kodekraken.config.json")) {
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
                        Self::default()
                    }
                }
            }
            Err(_) => {
                println!("[WARNING] ⚠️  Could not find kodekraken.config.json file in root directory, using default config.");
                Self::default()
            }
        }
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
        assert_eq!(default_general.operators, vec![]);
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
        assert_eq!(config.general.operators, vec![]);
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
        let json_content = r#"
            {
                "general": {
                    "timeout": 10,
                    "operators": ["UnaryRemovalOperator", "AssignmentReplacementOperator"]
                },
                "ignore": {
                    "ignore_files": ["file1", "file2"],
                    "ignore_directories": ["dir1", "dir2"]
                },
                "threading": {
                    "max_threads": 42
                },
                "output": {
                    "display_end_table": true
                },
                "logging": {
                    "log_level": "debug"
                }
            }
        "#;

        writeln!(file, "{}", json_content).expect("Failed to write to temporary file");
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
        let temp_dir = temp_dir();
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
        assert_eq!(config.general.operators, vec![]);
    }

    #[test]
    fn test_load_config_from_missing_file() {
        // Test loading config from a missing file
        let config = KodeKrakenConfig::load_config("/tmp");
        // Since the file is missing, it should fall back to default values
        assert_eq!(config.general.timeout, None);
        assert_eq!(config.general.operators, vec![]);
    }
}
