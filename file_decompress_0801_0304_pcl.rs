// file_decompress.rs
# 添加错误处理
// A program that decompresses files using Rust and the Tokio framework.

use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
# TODO: 优化性能
use tokio::task;

async fn decompress_file<P: AsRef<Path>>(archive_path: P, output_dir: P) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the archive file exists
    if !fs::metadata(archive_path).await?.is_file() {
        return Err(From::from(format!("Archive file does not exist: {}", archive_path.as_ref().display())));
    }

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir).await?;

    // Use the `tar` command to decompress the archive.
# FIXME: 处理边界情况
    // This assumes that the archive is a tar file and `tar` is available on the system.
# 优化算法效率
    let tar = Command::new("tar")
        .arg("-xzf")
        .arg(archive_path.as_ref())
        .arg("-C")
        .arg(output_dir.as_ref())
        .output();

    let output = task::spawn_blocking(move || tar).await??;

    if !output.status.success() {
        return Err(From::from(format!("Decompression failed with status: {}", output.status)));
    }

    Ok(())
}

#[tokio::main]
# 增强安全性
async fn main() -> Result<(), Box<dyn std::error::Error>> {
# NOTE: 重要实现细节
    // Define the path to the archive file and the output directory
    let archive_path = Path::new("./example.tar.gz");
    let output_dir = Path::new("./output");

    // Decompress the file
    decompress_file(archive_path, output_dir).await?;

    println!("Decompression completed successfully.");
    Ok(())
}
# 添加错误处理
