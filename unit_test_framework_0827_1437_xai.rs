// unit_test_framework.rs
// 一个使用RUST和TOKIO框架创建的单元测试框架。

use tokio::task;
use std::sync::{Arc, Mutex};
use std::error::Error;

/// 测试用例结构体
struct TestCase {
    /// 测试名称
    name: String,
    /// 测试函数
    test_func: Arc<Mutex<dyn Fn() -> Result<(), Box<dyn Error>>>>,
}

impl TestCase {
    /// 创建一个新的测试用例
    fn new(name: &str, test_func: impl Fn() -> Result<(), Box<dyn Error>> + 'static + Send + Sync) -> Self {
        TestCase {
            name: name.to_owned(),
            test_func: Arc::new(Mutex::new(test_func)),
        }
    }

    /// 运行测试
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        let test_func = self.test_func.lock().unwrap().clone();
        (test_func)()
    }
}

/// 测试套件结构体
struct TestSuite {
    /// 包含的测试用例
    cases: Vec<TestCase>,
}

impl TestSuite {
    /// 创建一个新的测试套件
    fn new() -> Self {
        TestSuite { cases: Vec::new() }
    }

    /// 添加测试用例
    fn add_case(&mut self, case: TestCase) {
        self.cases.push(case);
    }

    /// 运行所有测试用例
    async fn run_all(&self) -> Result<(), Box<dyn Error>> {
        let mut handles = Vec::new();
        for case in &self.cases {
            let handle = task::spawn(async move {
                case.run().await
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await?;
        }

        Ok(())
    }
}

/// main函数，程序入口
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut suite = TestSuite::new();

    // 添加测试用例
    suite.add_case(TestCase::new("TestExample", || {
        // 测试逻辑
        assert_eq!(2 + 2, 4);
        Ok(())
    }));

    // 运行所有测试用例
    suite.run_all().await?;

    Ok(())
}
