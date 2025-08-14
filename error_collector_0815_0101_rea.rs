use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use log::{info, error, warn, debug};
use std::sync::Arc;
use std::collections::HashSet;

// Define a structure to hold the state of the error collector.
struct ErrorCollector {
    errors: HashSet<String>,
    sender: mpsc::Sender<String>,
}

impl ErrorCollector {
    // Create a new ErrorCollector with a channel for sending errors.
    fn new(sender: mpsc::Sender<String>) -> Self {
        ErrorCollector {
            errors: HashSet::new(),
            sender,
        }
    }

    // Add an error to the collector and to the channel.
    async fn log_error(&mut self, error: String) {
        if self.errors.insert(error.clone()) {
            // If the error is new, send it to the channel.
            self.sender.send(error).await.unwrap_or_else(|e| {
                error!("Failed to send error: {}", e);
            });
        } else {
            // If the error has already been logged, log a warning.
            warn!("Duplicate error: {}", error);
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize a Tokio runtime.
    let (tx, mut rx) = mpsc::channel(100); // 100 is the buffer size.
    let collector = Arc::new(ErrorCollector::new(tx));

    // Start a background task to handle incoming errors.
    let collector_clone = collector.clone();
    tokio::spawn(async move {
        while let Some(error) = rx.recv().await {
            info!("Handling error: {}", error);
            // Here you would normally process the error.
        }
    });

    // Simulate error generation.
    let errors = vec!["Error 1", "Error 2", "Error 1"];
    for error in errors {
        collector.log_error(error.to_string()).await;
        time::sleep(Duration::from_millis(100)).await; // Simulate delay between errors.
    }
}
