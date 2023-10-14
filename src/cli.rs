use std::{path::Path, time::Duration};

use clap::{Args, CommandFactory, Parser, Subcommand};

use crate::{
    config::KodeKrakenConfig,
    error::{self, KodeKrakenError},
    mutation_tool::{MutationToolBuilder, OUT_DIRECTORY},
};

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
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
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug, Clone)]
pub struct MutationCommandConfig {
    /// The path to the files to be mutated
    /// Error will be thrown if the path is not a directory
    #[clap(default_value = ".")]
    pub path: String,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigCommandConfig {
    /// Create a config file in the current directory
    #[clap(long, short, default_value = "false")]
    pub setup: bool,
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

pub fn run_with_timeout<F>(mut f: F, timeout: Duration) -> error::Result<()>
where
    F: FnMut() -> error::Result<()> + Send + 'static,
{
    // Create a channel to send a message when the function is done
    let (sender, receiver) = std::sync::mpsc::channel();
    // Spawn a thread to run the function
    std::thread::spawn(move || {
        sender.send(f()).expect("Could not send message");
    });
    // Wait for the function to finish or timeout
    match receiver.recv_timeout(timeout) {
        Ok(res) => res,
        Err(_) => Err(KodeKrakenError::Error(
            format!(
                "Timeout reached, mutation tool took longer than {} seconds to finish",
                timeout.as_secs()
            )
            .into(),
        )),
    }
}

pub fn run_cli(config: KodeKrakenConfig) {
    let args = Cli::parse();
    let mutate_tool_builder = MutationToolBuilder::new();

    match args.command {
        Commands::Mutate(mutate_config) => {
            let mut tool = mutate_tool_builder
                .set_mutate_config(mutate_config)
                .set_general_config(config)
                .set_mutation_comment(true)
                .build();
            println!("{:#?}", tool.kodekraken_config.general.timeout);
            let res = match tool.kodekraken_config.general.timeout {
                Some(timeout) => {
                    println!("Timeout set to {} seconds", timeout);
                    run_with_timeout(move || tool.mutate(), Duration::from_secs(timeout))
                }
                None => tool.mutate(),
            };
            if let Err(e) = res {
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_parse() {
        Cli::command().debug_assert();
    }
}
