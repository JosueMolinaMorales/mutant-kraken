use std::{
    fs,
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

use crate::{
    error::{KodeKrakenError, Result},
    mutation_tool::{Mutation, MutationResult},
};

#[derive(PartialEq, Eq)]
pub enum GradleCommand<'a> {
    Assemble,
    Clean,
    Test(&'a str),
}

impl<'a> ToString for GradleCommand<'a> {
    fn to_string(&self) -> String {
        match *self {
            GradleCommand::Assemble => "assemble".to_string(),
            GradleCommand::Clean => "clean".to_string(),
            GradleCommand::Test(_) => "test".to_string(),
        }
    }
}

/// Run the gradle commands, assemble and test
/// This will check to see if there is a gradlew file in the root of the directory
pub fn run(
    config_path: &PathBuf,
    mutated_file_path: &PathBuf,
    original_file_path: &PathBuf,
    mutation: &mut Mutation,
) -> Result<()> {
    // Check to see if gradlew exists in the root of the directory
    if !config_path.join("gradlew").exists() {
        return Err(KodeKrakenError::Error(
            "gradlew does not exist at the root of this project".into(),
        ));
    }
    // Run Clean Build
    build_gradle_command(config_path, GradleCommand::Clean)?
        .wait()
        .map_err(|e| KodeKrakenError::Error(format!("Failed to run gradle command: {}", e)))?;

    // Copy the mutated file to the original file
    fs::copy(mutated_file_path, original_file_path)?;

    // Compile the project first, skip if compilation fails
    let res = build_gradle_command(config_path, GradleCommand::Assemble)?
        .wait()
        .map_err(|e| KodeKrakenError::Error(format!("Failed to run gradle command: {}", e)))?;

    if !res.success() {
        tracing::info!("Build failed for: {}", mutated_file_path.display());
        mutation.result = MutationResult::BuildFailed;
        return Ok(());
    }

    let filter = original_file_path
        .file_name()
        .ok_or(KodeKrakenError::ConversionError)?
        .to_str()
        .ok_or(KodeKrakenError::ConversionError)?
        .strip_suffix(".kt")
        .ok_or(KodeKrakenError::ConversionError)?;

    let mut child_process = build_gradle_command(config_path, GradleCommand::Test(filter))?;
    tracing::debug!("Running test for mutation: {}", mutated_file_path.display());
    // Will need to keep an eye on this timeout. The reason its here is because of infinite loops that
    // can occur from the mutations.
    let res = match child_process.wait_timeout(Duration::from_secs(30)) {
        Ok(Some(status)) => status,
        Ok(None) => {
            child_process.kill().map_err(|e| {
                KodeKrakenError::Error(format!("Failed to kill child process: {}", e))
            })?;
            tracing::error!("Test timed out for: {}", mutated_file_path.display());

            mutation.result = MutationResult::Timeout;
            return Ok(());
        }
        Err(e) => {
            tracing::error!("Test failed: {}", e);
            child_process.kill().map_err(|e| {
                KodeKrakenError::Error(format!("Failed to kill child process: {}", e))
            })?;
            mutation.result = MutationResult::Failed;
            return Ok(());
        }
    };
    if res.success() {
        tracing::info!("Mutant survived for file: {}", mutated_file_path.display());
        mutation.result = MutationResult::Survived;
    } else {
        tracing::info!("Mutant killed for file: {}", mutated_file_path.display());
        mutation.result = MutationResult::Killed;
    }
    Ok(())
}

// Builds the gradle command to be ran
fn build_gradle_command(config_path: &PathBuf, command: GradleCommand) -> Result<Child> {
    let mut cmd = if cfg!(unix) {
        Command::new("./gradlew")
    } else if cfg!(windows) {
        Command::new("cmd")
    } else {
        panic!("Unsupported OS");
    };
    let mut args = vec![];
    if cfg!(windows) {
        args.append(&mut ["/C".into(), "gradlew.bat".into()].to_vec())
    }
    args.push(command.to_string());
    if let GradleCommand::Test(filter) = command {
        args.append(&mut ["--tests".to_string(), format!("{}Test", filter)].to_vec())
    }
    if command != GradleCommand::Clean {
        args.append(&mut ["--parallel".to_string(), "--quiet".to_string()].to_vec());
    }
    cmd.args(args)
        .current_dir(config_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| KodeKrakenError::Error(format!("Failed to run gradle command: {}", e)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "gradlew does not exist at the root of this project")]
    fn test() {
        run(
            &PathBuf::from("./kotlin-test-projects/no-gradle-project"),
            &PathBuf::new(),
            &PathBuf::new(),
            &mut Mutation::new(
                0,
                0,
                "new_op".into(),
                "old_op".into(),
                0,
                crate::mutation_tool::MutationOperators::ArithmeticReplacementOperator,
                "file_name".into(),
            ),
        )
        .unwrap()
    }

    #[test]
    fn run_mutations_should_all_pass() {
        let dir = PathBuf::from("./kotlin-test-projects/mutations")
            .read_dir()
            .unwrap();
        let file_backup =
            include_str!("../kotlin-test-projects/kotlin-project/src/main/kotlin/Calculator.kt");
        for entry in dir {
            let entry = entry.unwrap().path();
            let mut mutation = Mutation::new(
                0,
                0,
                "new_op".into(),
                "old_op".into(),
                0,
                crate::mutation_tool::MutationOperators::ArithmeticReplacementOperator,
                "file_name".into(),
            );
            run(
                &PathBuf::from("./kotlin-test-projects/kotlin-project"),
                &entry,
                &PathBuf::from(
                    "./kotlin-test-projects/kotlin-project/src/main/kotlin/Calculator.kt",
                ),
                &mut mutation,
            )
            .unwrap();
            // Reset File
            fs::write(
                &PathBuf::from(
                    "./kotlin-test-projects/kotlin-project/src/main/kotlin/Calculator.kt",
                ),
                file_backup,
            )
            .unwrap();
            // Get File Name
            let file_name = entry
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(".kt")
                .unwrap();
            let expected = match file_name {
                "BuildFails" => MutationResult::BuildFailed,
                "Killed" => MutationResult::Killed,
                "Survived" => MutationResult::Survived,
                _ => unreachable!(),
            };
            assert_eq!(expected, mutation.result, "Failed for: {}", file_name)
        }
    }
}
