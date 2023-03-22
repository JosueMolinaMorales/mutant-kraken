use std::{process::{Command, Stdio, Child}, path::PathBuf, fs, time::Duration};

use clap::{CommandFactory, error::ErrorKind};
use wait_timeout::ChildExt;

use crate::Cli;

pub struct Gradle {
    config_path: PathBuf,
}

impl Gradle {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path
        }
    }

    pub fn run(&mut self, mutated_file_path: PathBuf, original_file_path: PathBuf, backup_path: PathBuf) {
        // Check to see if gradlew exists in the root of the directory
        // TODO: How will testing work for this?
        if !self.config_path.join("gradlew").exists() {
            Cli::command()
                .error(ErrorKind::ArgumentConflict, "gradlew does not exist at the root of this project")
                .exit();
        }
        // Save a copy of the original file
        fs::copy(&original_file_path,&backup_path,).unwrap();
        // Copy the mutated file to the original file
        fs::copy(&mutated_file_path,&original_file_path,).unwrap();
        // Compile the project first, skip if compilation fails
        let res = self.build_gradle_command("assemble").wait().unwrap();
        if !res.success() {
            tracing::info!("Build failed");
            // Restore the original file
            self.restore_original_file(&backup_path, &original_file_path);
            return;
        }
        let mut child_process = self.build_gradle_command("test");
        // Will need to keep an eye on this timeout. The reason its here is because of infinite loops that
        // can occur from the mutations.
        let res = match child_process.wait_timeout(Duration::from_secs(10)) {
                Ok(Some(status)) => status,
                Ok(None) => {
                    child_process.kill().unwrap();
                    tracing::info!("Test timed out for: {}", mutated_file_path.display());
                    // Restore the original file
                    self.restore_original_file(&backup_path, &original_file_path,);
                    return;
                },
                Err(e) => {
                    tracing::info!("Test failed: {}", e);
                    child_process.kill().unwrap();
                    // Restore the original file
                    self.restore_original_file(&backup_path, &original_file_path,);
                    return;
                }
            };
        if res.success() {
            tracing::info!("Test successful for: {}", mutated_file_path.display());
        } else {
            tracing::info!("Test failed for: {}", mutated_file_path.display());
        }
        // Restore the original file
        self.restore_original_file(&backup_path, &original_file_path)
    }

    fn build_gradle_command(&mut self, command: &str) -> Child {
        Command::new("./gradlew")
            .arg(command)
            .arg("--parallel")
            .arg("--build-cache")
            .arg("--quiet")
            .current_dir(&self.config_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
    }

    fn restore_original_file(&self, backup_path: &PathBuf, original_file_path: &PathBuf) {
        fs::copy(
            backup_path,
            original_file_path,
        ).unwrap();
    }

}