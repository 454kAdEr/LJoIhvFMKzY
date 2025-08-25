// random_number_generator.rs
// 一个使用RUST和TOKIO框架的随机数生成器程序。

use rand::Rng; // 引入RUST的随机数生成库。
use tokio; // 引入TOKIO异步运行时库。

#[tokio::main]
async fn main() {
    // 随机数生成器的主入口函数。
    
    match generate_random_number() {
        Ok(number) => println!("随机数生成器结果: {}", number),
        Err(e) => println!("随机数生成器错误: {}", e),
    }
}

/// 生成一个随机数。
///
/// # 参数
/// * `lower` - 随机数的最小值。
/// * `upper` - 随机数的最大值。
///
/// # 返回值
/// 返回生成的随机数，如果生成失败，则返回错误。
fn generate_random_number() -> Result<u32, String> {
    // 使用RUST的随机数生成器生成一个随机数。
    let mut rng = rand::thread_rng(); // 获取一个线程局部的随机数生成器。
    let lower = 1; // 设置随机数的最小值为1。
    let upper = 100; // 设置随机数的最大值为100。
    
    // 尝试生成一个随机数，并处理可能的错误。
    match rng.gen_range(lower..=upper) {
        Ok(number) => Ok(number),
        Err(e) => Err(e.to_string()),
    }
}