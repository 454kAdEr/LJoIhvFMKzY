use tokio::time::{sleep, Duration};
use rand::Rng;
use std::time::Instant;

/// 测试数据生成器
/// 此函数生成随机测试数据，并模拟数据生成过程可能的延迟。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个测试数据生成器实例
    let mut generator = TestDataGenerator::new();

    // 模拟数据生成过程
    for _ in 0..5 {
        let test_data = generator.generate_data().await?;
        println!("Generated test data: "{}"", test_data);

        // 模拟数据生成的延迟
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

/// 测试数据生成器结构体
struct TestDataGenerator;

impl TestDataGenerator {
    /// 创建一个新的测试数据生成器实例
    pub fn new() -> Self {
        TestDataGenerator
    }

    /// 生成随机测试数据
    /// 此函数生成一个随机字符串，模拟测试数据的生成过程。
    pub async fn generate_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        // 模拟数据生成的随机延迟
        sleep(Duration::from_millis(rand::thread_rng().gen_range(100..1000))).await;

        // 生成一个随机字符串作为测试数据
        let data: String = rand::thread_rng().gen_ascii_chars()
            .take(10)
            .map(char::from)
            .collect();

        Ok(data)
    }
}

/// 随机数生成器
/// 此函数用于生成随机数，模拟数据生成过程中的随机性。
fn random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
