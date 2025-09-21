// unit_test_example.rs
// 这是一个使用 Rust 和 Tokio 框架的单元测试示例程序。

use tokio::task;
use std::error::Error;

// 定义一个简单的结构体，用于演示单元测试。
#[derive(Debug)]
struct Calculator {
    value: i32,
}

impl Calculator {
    // 新建一个 Calculator 实例。
    fn new(value: i32) -> Self {
        Calculator { value }
    }

    // 演示加法功能。
    async fn add(&self, other: i32) -> Result<i32, Box<dyn Error>> {
        Ok(self.value + other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use std::future::Future;

    // 单元测试函数，测试 Calculator 的 add 方法。
    #[test]
    async fn test_add() {
        let calculator = Calculator::new(10);
        let result = calculator.add(5).await;
        assert_eq!(result.unwrap(), 15);
    }

    // 单元测试函数，测试 add 方法在输入为负数时的行为。
    #[test]
    async fn test_add_negative() {
        let calculator = Calculator::new(10);
        let result = calculator.add(-5).await;
        assert_eq!(result.unwrap(), 5);
    }
}

// 异步主函数，用于运行 Tokio 运行时。
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 在这里可以初始化你的程序，并启动 Tokio 任务。
    println!("Hello, Tokio!");
    Ok(())
}
