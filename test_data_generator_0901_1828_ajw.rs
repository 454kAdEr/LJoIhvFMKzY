use tokio::time::{self, Duration};

/// 定义一个结构体，用于生成测试数据
struct TestDataGenerator {
    interval: Duration, // 数据生成的时间间隔
}

impl TestDataGenerator {
    /// 创建一个新的测试数据生成器实例
    pub fn new(interval: Duration) -> Self {
        TestDataGenerator { interval }
    }

    /// 生成测试数据的异步函数
    pub async fn generate(&self) {
        loop {
            // 模拟数据生成
            let data = self.generate_data();
            println!("Generated test data: "{}"", data);
            // 等待指定的时间间隔
            time::sleep(self.interval).await;
        }
    }

    /// 模拟数据生成的函数
    fn generate_data(&self) -> String {
        // 这里可以添加更复杂的数据生成逻辑
        format!("Data at {:?}", time::Instant::now())
    }
}

#[tokio::main]
async fn main() {
    // 设置数据生成的时间间隔为1秒
    let interval = Duration::from_secs(1);
    let generator = TestDataGenerator::new(interval);
    // 开始生成测试数据
    if let Err(e) = generator.generate().await {
        eprintln!("Error generating test data: "{}"", e);
    }
}