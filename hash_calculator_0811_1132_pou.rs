use sha2::{Sha256, Digest};
use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;
use std::path::Path;
use std::fs::read_to_string;
use std::io::{self, Error};

/// 计算一个字符串的SHA-256哈希值
///
/// # 参数
/// * `input` - 需要计算哈希值的字符串
///
/// # 返回值
/// 计算出的哈希值字符串
async fn calculate_sha256(input: &str) -> Result<String, Error> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    Ok(format!("%x", hasher.finalize()))
}

/// 计算文件的SHA-256哈希值
///
/// # 参数
/// * `path` - 文件的路径
///
/// # 返回值
/// 计算出的哈希值字符串
async fn calculate_file_sha256(path: impl AsRef<Path>) -> Result<String, Error> {
    let file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buf = Vec::new();
    let mut reader = io::BufReader::new(file);
    loop {
        match reader.read_until(b'
', &mut buf).await? {
            0 => break,
            _ => hasher.update(&buf),
        }
        buf.clear();
    }
    Ok(format!("%x", hasher.finalize()))
}

/// 程序的主函数
#[tokio::main]
async fn main() {
    let input_string = "Hello, world!";
    match calculate_sha256(input_string).await {
        Ok(hash) => println!("String hash: {}", hash),
        Err(e) => println!("Error calculating string hash: {}", e),
    }

    let file_path = "path_to_your_file.txt";
    match calculate_file_sha256(file_path).await {
        Ok(hash) => println!("File hash: {}", hash),
        Err(e) => println!("Error calculating file hash: {}", e),
    }
}
