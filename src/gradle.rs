use std::{
    fs,
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::Duration,
};

use clap::{error::ErrorKind, CommandFactory};
use wait_timeout::ChildExt;

use crate::{Cli, mutation::{Mutation, MutationResult}};

/// Gradle is a struct that will run gradle commands
pub struct Gradle {
    config_path: PathBuf,
    verbose: bool,
}

impl Gradle {

    /// Create a new instance of Gradle
    /// This will be used to run gradle commands
    pub fn new(config_path: PathBuf, verbose: bool) -> Self {
        Self { config_path, verbose }
    }

    /// Run the gradle commands, assemble and test
    /// This will check to see if there is a gradlew file in the root of the directory
    pub fn run(
        &mut self,
        mutated_file_path: &PathBuf,
        original_file_path: &PathBuf,
        backup_path: &PathBuf,
        mutation: &mut Mutation,
    ) {
        // Check to see if gradlew exists in the root of the directory
        // TODO: How will testing work for this?
        if !self.config_path.join("gradlew").exists() {
            Cli::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    "gradlew does not exist at the root of this project",
                )
                .exit();
        }
        // Copy the mutated file to the original file
        fs::copy(&mutated_file_path, &original_file_path).unwrap();
        // Compile the project first, skip if compilation fails
        let res = self.build_gradle_command("assemble").wait().unwrap();
        if !res.success() {
            if self.verbose {
                tracing::info!("Build failed for: {}", mutated_file_path.display());
            }
            // Restore the original file
            // self.restore_original_file(&backup_path, &original_file_path);
            mutation.result = MutationResult::BuildFailed;
            return;
        }
        let mut child_process = self.build_gradle_command("test");
        // Will need to keep an eye on this timeout. The reason its here is because of infinite loops that
        // can occur from the mutations.
        let res = match child_process.wait_timeout(Duration::from_secs(10)) {
            Ok(Some(status)) => status,
            Ok(None) => {
                child_process.kill().unwrap();
                if self.verbose {
                    tracing::info!("Test timed out for: {}", mutated_file_path.display());
                }
                // Restore the original file
                // self.restore_original_file(&backup_path, &original_file_path);
                mutation.result = MutationResult::Timeout;
                return;
            }
            Err(e) => {
                if self.verbose { 
                    tracing::info!("Test failed: {}", e);
                }
                child_process.kill().unwrap();
                // Restore the original file
                // self.restore_original_file(&backup_path, &original_file_path);
                mutation.result = MutationResult::Failed;
                return;
            }
        };
        if res.success() {
            if self.verbose {
                tracing::info!("Mutant survived for file: {}", mutated_file_path.display());
            }
            mutation.result = MutationResult::Survived;
        } else {
            if self.verbose {
                tracing::info!("Mutant killed for file: {}", mutated_file_path.display());
            }
            mutation.result = MutationResult::Killed;
        }
        // Restore the original file
        // self.restore_original_file(&backup_path, &original_file_path);
    }

    // Builds the gradle command to be ran
    fn build_gradle_command(&mut self, command: &str) -> Child {
        let mut cmd = if cfg!(unix) {
            Command::new("./gradlew")
        } else if cfg!(windows) {
            Command::new("./gradlew.bat")
        } else {
            panic!("Unsupported OS");
        };
        let std_out = if self.verbose {
            Stdio::inherit()
        } else {
            Stdio::null()
        };
        let std_err = if self.verbose {
            Stdio::inherit()
        } else {
            Stdio::null()
        };
        cmd
            .arg(command)
            .arg("--parallel")
            .arg("--build-cache")
            .arg("--quiet")
            .current_dir(&self.config_path)
            .stdout(std_out)
            .stderr(std_err)
            .spawn()
            .unwrap()
    }

    // Restores a file to its original state
    pub fn restore_original_file(&self, backup_path: &PathBuf, original_file_path: &PathBuf) {
        fs::copy(backup_path, original_file_path).unwrap();
    }
}


// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "gradlew does not exist at the root of this project")]
//     fn test() {
//         let mut gradle = Gradle::new(PathBuf::from("."));
//         gradle.run(PathBuf::from("."), PathBuf::from("."), PathBuf::from("."),);
//     }
// }