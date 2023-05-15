use std::{fs, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct KodeKrakenConfig {
    pub ignore: IgnoreConfig,
    pub threading: ThreadingConfig,
    pub output: OutputConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_level: String,
}

#[derive(Debug, Deserialize)]
pub struct IgnoreConfig {
    pub ignore_files: Vec<String>,
    pub ignore_directories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadingConfig {
    pub max_threads: usize,
}

#[derive(Debug, Deserialize, Default)]
pub struct OutputConfig {
    pub display_end_table: bool,
}

impl KodeKrakenConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_config() -> Self {
        match fs::File::open("kodekraken.config.json") {
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
                println!("[WARNING] ⚠️  Could not find kodekraken.config.json file, using default config.");
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
