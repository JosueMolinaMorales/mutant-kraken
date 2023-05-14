use std::path::Path;

use clap::{Args, CommandFactory, Parser, Subcommand};
use mutation::Mutation;
use mutation_tool::{MutationToolBuilder, OUT_DIRECTORY};
use tracing_appender::non_blocking::WorkerGuard;

pub mod config;
pub mod error;
pub mod gradle;
pub mod html_gen;
pub mod kotlin_types;
pub mod mutation;
pub mod mutation_operators;
pub mod mutation_tool;

#[cfg(test)]
pub mod test_config;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Mutate the files in the given path
    /// If no path is given, the current directory will be used
    /// A csv file will be generated along with a html table
    Mutate(MutationCommandConfig),
    /// Display help text on how to setup the config file
    /// or create a config file in the current directory
    Config(ConfigCommandConfig),
    /// Clean the kode-kraken-dist directory
    /// This will delete all files in the directory
    /// This is useful if you want to remove all the files
    Clean,
}

const ABOUT: &str = include_str!("../assets/about.txt");

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = ABOUT,
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug, Clone)]
pub struct MutationCommandConfig {
    /// The path to the files to be mutated
    /// Error will be thrown if the path is not a directory
    #[clap(default_value = ".")]
    path: String,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigCommandConfig {
    /// Create a config file in the current directory
    #[clap(long, short, default_value = "false")]
    setup: bool,
}

impl Default for MutationCommandConfig {
    fn default() -> Self {
        Self {
            path: std::env::current_dir()
                .expect("Could not get the current working directory")
                .display()
                .to_string(),
        }
    }
}

fn main() {
    let config = config::KodeKrakenConfig::load_config();
    let _guard = setup_logging(&config.logging.log_level);
    tracing::info!("Starting Kode Kraken");
    let args = Cli::parse();
    let mutate_tool_builder = MutationToolBuilder::new();

    match args.command {
        Commands::Mutate(mutate_config) => {
            if let Err(e) = mutate_tool_builder
                .set_mutate_config(mutate_config)
                .set_general_config(config)
                .set_mutation_comment(true)
                .build()
                .mutate()
            {
                let error_msg = match e {
                    error::KodeKrakenError::FileReadingError(msg) => msg,
                    error::KodeKrakenError::MutationGenerationError => {
                        "Error Generating Mutations".into()
                    }
                    error::KodeKrakenError::MutationGatheringError => {
                        "Error Gathering Mutations".into()
                    }
                    error::KodeKrakenError::MutationBuildTestError => {
                        "Error Building and Testing Mutations".into()
                    }
                    error::KodeKrakenError::ConversionError => "Error Converting".into(),
                    error::KodeKrakenError::Error(msg) => msg,
                };
                Cli::command()
                    .error(clap::error::ErrorKind::Io, error_msg)
                    .exit();
            }
        }
        Commands::Config(config) => {
            if config.setup {
                let config_file_path = Path::new("kodekraken.config.json");
                if config_file_path.exists() {
                    println!("Config file already exists");
                } else {
                    std::fs::write(config_file_path, include_str!("../assets/config.json"))
                        .expect("Could not write config file");
                    println!("Config file created");
                }
            } else {
                println!("Config file setup instructions:");
                println!(
                    "1. Create a file named kodekraken.config.json in the root of your project"
                );
                println!("2. Copy the following into the file:");
                println!("{}", include_str!("../assets/config.json"));
                println!("3. Edit the config file to your liking");
            }
        }
        Commands::Clean => {
            // Check to see if the output directory exists
            let output_dir = Path::new(OUT_DIRECTORY);
            if output_dir.exists() {
                // Delete the output directory
                std::fs::remove_dir_all(output_dir).expect("Could not delete output directory");
            }
        }
    }
}

fn setup_logging(log_level: &str) -> WorkerGuard {
    let log_level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };
    // Create dist log folder if it doesn't exist
    let log_dir = Path::new(OUT_DIRECTORY).join("logs");
    std::fs::create_dir_all(&log_dir).expect("Could not create log directory");
    let file_appender = tracing_appender::rolling::hourly(log_dir, "kode-kraken.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_ansi(false)
        .with_target(false)
        .with_writer(non_blocking)
        .with_thread_ids(true)
        .init();
    guard
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_parse() {
        Cli::command().debug_assert();
    }
}
