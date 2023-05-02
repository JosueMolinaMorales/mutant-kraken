use std::{fs, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct KodeKrakenConfig {
    pub ignore: IgnoreConfig,
    pub threading: ThreadingConfig,
    pub output: OutputConfig,
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
        let file = fs::File::open("kodekraken.config.json").unwrap();
        let buffer_reader = BufReader::new(file);
        let config: KodeKrakenConfig = serde_json::from_reader(buffer_reader).unwrap();
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
