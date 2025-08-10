use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::prelude::*;
use log::{info, error, warn};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize the logger
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    // Path to the log file
    let log_file_path = "error_log.txt";

    // Shared access to the log file
    let log_file = Arc::new(Mutex::new(log_file_path));

    // Simulate error logging
    simulate_error_logging(log_file.clone()).await;

    Ok(())
}

/// Logs errors to the specified log file asynchronously.
///
/// # Arguments
///
/// * `log_file` - An Arc<Mutex<&str>> containing the path to the log file.
async fn simulate_error_logging(log_file: Arc<Mutex<&str>>) -> io::Result<()> {
    // Simulate an error that we want to log
    let error_message = "An error occurred at: ".to_string() + &Utc::now().to_string_with Seconds();
    error!(target: "error_logger", "{}", error_message);

    // Acquire a lock on the log file and write the error message
    let mut file = File::create(log_file.lock().await).await?;
    file.write_all(error_message.as_bytes()).await?;
    file.write_all(b"
").await?;

    Ok(())
}
