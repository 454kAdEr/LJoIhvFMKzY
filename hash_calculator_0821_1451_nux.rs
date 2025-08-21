use sha2::{Sha256, Digest};
use tokio; // 引入tokio用于异步运行

/// 计算给定数据的SHA-256哈希值
/// 
/// 这个函数接受一个字节切片作为输入，并返回其SHA-256哈希值
/// 
/// # 参数
/// * `data` - 需要计算哈希的数据
/// 
/// # 返回值
/// 返回计算得到的SHA-256哈希值
#[must_use]
async fn calculate_sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.to_vec()
}

#[tokio::main] // 使用tokio宏来启动异步main函数
async fn main() {
    // 示例数据
    let data = b"Hello, world!";

    match calculate_sha256_hash(data).await {
        Ok(hash) => {
            println!("Hash: {}", hex::encode(hash)); // 打印哈希值
        },
        Err(e) => {
            eprintln!("Error calculating hash: {}", e); // 错误处理
        }
    }
}