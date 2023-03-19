use clap::{error::ErrorKind, Args, Parser, Subcommand};
use mutate::{Mutation, MutationToolBuilder};

pub mod kotlin_types;
pub mod mutate;
pub mod mutation_operators;
#[cfg(test)]
pub mod test_config;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Mutate the files in the given path
    Mutate(MutationCommandConfig),

    /// Clear the output directory of all files
    ClearOutputDirectory,
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

    /// Print out verbose information
    #[clap(short, long, default_value = "false")]
    verbose: bool,

    /// The path to the output directory
    #[clap(short, long, default_value = "./mutations/")]
    output_directory: String,
}

#[derive(Args, Debug, Clone)]
pub struct MutationCommandConfig {
    /// The path to the files to be mutated
    /// Error will be thrown if the path is not a directory
    path: String,
}

impl Default for MutationCommandConfig {
    fn default() -> Self {
        Self {
            path: ".".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct FileMutations {
    mutations: Vec<Mutation>,
}

#[derive(Debug)]
pub struct CliError {
    kind: ErrorKind,
    message: String,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    let args = Cli::parse();
    let verbose = args.verbose;
    let mutate_tool_builder = MutationToolBuilder::new();
    if verbose {
        tracing::info!("Verbose Mode Enabled");
        tracing::info!("Starting Mutation Testing Tool");
    }
    match args.command {
        Commands::Mutate(config) => {
            mutate_tool_builder
                .set_verbose(verbose)
                .set_output_directory(args.output_directory)
                .set_config(config)
                .build()
                .mutate();
        }
        Commands::ClearOutputDirectory => {
            mutate_tool_builder
                .set_verbose(args.verbose)
                .build()
                .clear_output_directory(args.output_directory)
        }
    }
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
