use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use sha2::{Sha256, Digest};
use std::path::Path;
use std::pin::Pin;
use tokio::fs::read;
use anyhow::Result;
use sha1::{Sha1, Digest as Sha1Digest};
use md5::Md5;

/// 异步计算文件的哈希值
///
/// 支持的哈希算法：SHA256, SHA1, MD5
async fn calculate_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new(); // 默认使用SHA256算法
    let mut buffer = [0; 1024];

    loop {
        let size = file.read(&mut buffer).await?;
        if size == 0 {
            break;
        }
        hasher.update(&buffer[..size]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// 异步计算文件的SHA1哈希值
async fn calculate_sha1_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let size = file.read(&mut buffer).await?;
        if size == 0 {
            break;
        }
        hasher.update(&buffer[..size]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// 异步计算文件的MD5哈希值
async fn calculate_md5_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = Md5::new();
    let mut buffer = [0; 1024];

    loop {
        let size = file.read(&mut buffer).await?;
        if size == 0 {
            break;
        }
        hasher.update(&buffer[..size]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = "example.txt"; // 文件路径
    println!("Calculating SHA256 hash...");
    let sha256_hash = calculate_hash(path).await?;
    println!("SHA256: "{}", sha256_hash);

    println!("Calculating SHA1 hash...");
    let sha1_hash = calculate_sha1_hash(path).await?;
    println!("SHA1: "{}", sha1_hash);

    println!("Calculating MD5 hash...");
    let md5_hash = calculate_md5_hash(path).await?;
    println!("MD5: "{}", md5_hash);

    Ok(())
}