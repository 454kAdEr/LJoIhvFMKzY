// memory_usage_analyzer.rs

use std::process::Command;
use tokio::process::Command as AsyncCommand;
use tokio::io::{self, AsyncBufReadExt};
use tokio::runtime::Runtime;
use std::time::Duration;
use anyhow::Result;

/// Analyzes memory usage of a given process by its pid.
/// This function runs asynchronously and uses the tokio runtime.
/// It returns a tuple containing the rss (resident set size) and peak rss values in bytes.
async fn analyze_memory_usage(pid: u32) -> Result<(u64, u64)> {
    let mut command = AsyncCommand::new("ps")
        .arg("-o")
        .arg("rss,peak_rss")
        .arg("-p")
        .arg(pid.to_string())
        .spawn()
        .await?;

    let mut output = String::new();
    while let Ok(Some(line)) = command.stdout().await.map(|mut line| line.read_line(&mut output)) {
        if line.trim().is_empty() {
            break;
        }
    }
    let status = command.wait().await?;
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to execute ps command"));
    }

    let parts: Vec<&str> = output.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Failed to parse output"));
    }

    let rss = parts[0].parse::<u64>().map_err(|_| anyhow::anyhow!("Failed to parse rss value"))?;
    let peak_rss = parts[1].parse::<u64>().map_err(|_| anyhow::anyhow!("Failed to parse peak rss value"))?;

    Ok((rss, peak_rss))
}

/// Entry point of the program.
#[tokio::main]
async fn main() -> Result<()> {
    let pid = 1234; // Replace with the actual process id you want to analyze.
    match analyze_memory_usage(pid).await {
        Ok((rss, peak_rss)) => {
            println!("RSS: {}, Peak RSS: {}", rss, peak_rss);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
    Ok(())
}
