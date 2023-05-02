use std::path::Path;

use clap::{Args, CommandFactory, Parser, Subcommand};
use mutation::Mutation;
use mutation_tool::{MutationToolBuilder, OUT_DIRECTORY};
use tracing_appender::non_blocking::WorkerGuard;

pub mod config;
pub mod error;
pub mod gradle;
pub mod kotlin_types;
pub mod mutation;
pub mod mutation_operators;
pub mod mutation_tool;

#[cfg(test)]
pub mod test_config;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Mutate the files in the given path
    Mutate(MutationCommandConfig),
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

impl Default for MutationCommandConfig {
    fn default() -> Self {
        Self {
            path: std::env::current_dir().unwrap().display().to_string(),
        }
    }
}

fn main() {
    let guard = setup_logging();
    tracing::info!("Starting Kode Kraken");
    let args = Cli::parse();
    let mutate_tool_builder = MutationToolBuilder::new();

    let config = config::KodeKrakenConfig::load_config();
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
                drop(guard);
                Cli::command()
                    .error(clap::error::ErrorKind::Io, error_msg)
                    .exit();
            }
        }
    }
}

fn setup_logging() -> WorkerGuard {
    // Create dist log folder if it doesn't exist
    let log_dir = Path::new(OUT_DIRECTORY).join("logs");
    std::fs::create_dir_all(&log_dir).unwrap();
    let file_appender = tracing_appender::rolling::never(log_dir, "kode-kraken.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .with_target(false)
        .with_writer(non_blocking)
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
