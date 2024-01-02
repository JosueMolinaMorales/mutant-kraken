use assert_cmd::prelude::*;
use kode_kraken::config::KodeKrakenConfig;
use std::{fs, path::Path, process::Command};

#[test]
fn test_tool_runs_correctly() {
    // Set the config
    let config = KodeKrakenConfig::default();
    // Create or replace the config file
    fs::write(
        "tests/kotlin-test-projects/demo/kodekraken.config.json",
        serde_json::to_string(&config).unwrap(),
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("kode-kraken").unwrap();

    // Create command
    cmd.arg("mutate").arg("tests/kotlin-test-projects/demo");

    // Assert that the command runs successfully
    cmd.assert().success();

    let dir_path =
        Path::new("tests/kotlin-test-projects/demo/kodekraken.config.json/kode-kraken-dist");
    // Assert that kode-kraken-dist was created
    assert!(dir_path.exists());
    // Assert that the backups directory was created and that the backup file exists
    let backup_path = dir_path.join("backups");
    assert!(backup_path.exists());
    // Get the files in the backups directory
    let backup_files = fs::read_dir(backup_path).unwrap();
    // Get the files in the mutations directory
    let real_files = fs::read_dir("tests/kotlin-test-projects/demo/src/main/kotlin").unwrap();
    // Assert that the number of files in the backups directory is the same as the number of files in the mutations directory
    assert_eq!(backup_files.count(), real_files.count());

    // Assert that the logs directory was created and that the log file exists
    let log_path = dir_path.join("logs");
    assert!(log_path.exists());
    assert!(log_path.join("kode-kraken.log").exists());
    // Assert that the mutations directory was created and that the mutations file exists
    let mutations_path = dir_path.join("mutations");
    assert!(mutations_path.exists());
    let mutation_files = fs::read_dir(mutations_path).unwrap();
    assert!(mutation_files.count() > 0);
    // Assert that the output.csv file was created
    let output_path = dir_path.join("output.csv");
    assert!(output_path.exists());
    // Assert that the report.html file was created
    let report_path = dir_path.join("report.html");
    assert!(report_path.exists());
}
