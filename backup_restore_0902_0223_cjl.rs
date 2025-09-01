// backup_restore.rs
// This program demonstrates a simple data backup and restore functionality using Rust and Tokio framework.

use std::fs;
use tokio;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::io::Result;

// Define a struct to hold the configuration for the backup and restore operations.
struct DataBackupConfig {
    backup_dir: String,
    restore_dir: String,
}

// Implement the backup functionality.
async fn backup_data(config: &DataBackupConfig) -> Result<()> {
    // Open the file to backup.
    let backup_file = File::open(&config.backup_dir).await?;

    // Open the file to write the backup data.
    let mut backup = File::create(&config.backup_dir.replace("/", "_backup")).await?;

    // Copy the data from the original file to the backup file.
    let mut buffer = Vec::new();
    backup_file.read_to_end(&mut buffer).await?;
    backup.write_all(&buffer).await?;

    Ok(())
}

// Implement the restore functionality.
async fn restore_data(config: &DataBackupConfig) -> Result<()> {
    // Open the backup file to read the data.
    let backup_file = File::open(&config.backup_dir.replace("/", "_backup")).await?;

    // Open the file to restore the data.
    let mut restore = File::create(&config.restore_dir).await?;

    // Copy the data from the backup file to the original file.
    let mut buffer = Vec::new();
    backup_file.read_to_end(&mut buffer).await?;
    restore.write_all(&buffer).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // Define the configuration for the backup and restore operations.
    let config = DataBackupConfig {
        backup_dir: "./data".to_string(),
        restore_dir: "./data".to_string(),
    };

    // Perform the backup operation.
    match backup_data(&config).await {
        Ok(_) => println!("Data backed up successfully."),
        Err(e) => eprintln!("Failed to backup data: {}", e),
    }

    // Perform the restore operation.
    match restore_data(&config).await {
        Ok(_) => println!("Data restored successfully."),
        Err(e) => eprintln!("Failed to restore data: {}", e),
    }
}
