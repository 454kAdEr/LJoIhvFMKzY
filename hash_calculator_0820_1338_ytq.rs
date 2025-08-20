// hash_calculator.rs
// 一个使用RUST和TOKIO框架实现的哈希值计算工具

use std::io;
use std::collections::HashMap;
use tokio;
use sha2::{Sha256, Digest};
use hex::encode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建输入提示
    println!("Please enter the text to calculate its SHA256 hash value: ");
    // 从标准输入读取文本
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    // 计算输入文本的SHA256哈希值
    let hash_value = calculate_sha256(&input).await?;

    // 打印哈希值
    println!("The SHA256 hash value is: ");
    println!(