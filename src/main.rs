use kode_kraken::{cli::run_cli, config::KodeKrakenConfig, mutation_tool::OUT_DIRECTORY};
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;

fn main() {
    let config = KodeKrakenConfig::load_config();

    let _guard = setup_logging(&config.logging.log_level);
    tracing::info!("Starting Kode Kraken");
    run_cli(config);
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
    let file_appender = tracing_appender::rolling::never(log_dir, "kode-kraken.log");
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
