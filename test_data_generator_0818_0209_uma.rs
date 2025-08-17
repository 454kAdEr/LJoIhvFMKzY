use rand::Rng;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

/// 测试数据生成器结构体
struct TestDataGenerator {
    rng: rand::rngs::ThreadRng,
}

impl TestDataGenerator {
    /// 创建一个新的测试数据生成器
    pub fn new() -> Self {
        TestDataGenerator {
            rng: rand::thread_rng(),
        }
    }

    /// 生成随机字符串测试数据
    pub async fn generate_random_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let length = self.rng.gen_range(1..=10);
        let mut result = String::new();
        for _ in 0..length {
            result.push(self.rng.gen_range(b'a'..b'z') as char);
        }
        Ok(result)
    }

    /// 模拟生成测试数据的异步任务
    pub async fn simulate_data_generation(&mut self) {
        loop {
            match self.generate_random_string().await {
                Ok(data) => println!("Generated test data: "{}"", data),
                Err(e) => eprintln!("Error generating test data: "{}"", e),
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // 共享测试数据生成器实例
    let generator = Arc::new(Mutex::new(TestDataGenerator::new()));
    
    let mut handles = Vec::new();
    for _ in 0..5 {
        let generator_clone = Arc::clone(&generator);
        let handle = tokio::spawn(async move {
            let mut generator = generator_clone.lock().await;
            generator.simulate_data_generation().await;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}