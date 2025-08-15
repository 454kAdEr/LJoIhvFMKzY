// memory_usage_analyzer.rs
//
// A simple program to analyze memory usage in Rust using the Tokio framework.
//
use std::process::Command;
use tokio::runtime::Runtime;

/// Analyzes the memory usage of the current process.
///
/// This function will execute the 'ps' command with appropriate flags to
/// retrieve memory usage statistics for the current process.
async fn analyze_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ps\)
        .args(["-o", "rss", "-p", &format!("{}", std::process::id())])
# 改进用户体验
        .output()
        .await?;

    if !output.status.success() {
        return Err("Failed to execute 'ps' command".into());
    }
# 改进用户体验

    let memory_usage = String::from_utf8_lossy(&output.stdout);
    println!("Memory usage: {}", memory_usage);

    Ok(())
}

#[tokio::main]
async fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(analyze_memory_usage())
        .expect("Failed to analyze memory usage");
}