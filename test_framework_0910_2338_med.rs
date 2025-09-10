use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use tokio::test;

// 定义一个测试用例结构体
struct TestCase<T>
where
    T: Fn() -> Result<(), Box<dyn Error>> + Send + 'static,
{
    description: String,
    test_function: T,
}

impl<T> TestCase<T>
where
    T: Fn() -> Result<(), Box<dyn Error>> + Send + 'static,
{
    // 创建一个新的测试用例
    fn new(description: &str, test_function: T) -> Self {
        TestCase {
            description: description.to_string(),
            test_function,
        }
    }

    // 执行测试用例
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Running test: {}", self.description);
        (self.test_function)().await?;
        println!("Test passed: {}", self.description);
        Ok(())
    }
}

// 定义一个测试框架结构体
struct TestFramework {
    test_cases: Vec<Arc<Mutex<TestCase<Box<dyn Fn() -> Result<(), Box<dyn Error>> + Send + 'static>>>>>,
}

impl TestFramework {
    // 创建一个新的测试框架
    fn new() -> Self {
        TestFramework {
            test_cases: Vec::new(),
        }
    }

    // 添加一个新的测试用例到框架中
    fn add_test_case(&mut self, test_case: TestCase<Box<dyn Fn() -> Result<(), Box<dyn Error>> + Send + 'static>>) {
        let arc_mutex_test_case = Arc::new(Mutex::new(test_case));
        self.test_cases.push(arc_mutex_test_case);
    }

    // 运行所有测试用例
    async fn run_all_tests(&self) -> Result<(), Box<dyn Error>> {
        for test_case in &self.test_cases {
            let test_case = test_case.lock().await;
            test_case.run().await?;
        }
        Ok(())
    }
}

// 定义一个自定义错误类型
#[derive(Debug)]
struct FrameworkError;

impl fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred in the test framework")
    }
}

impl Error for FrameworkError {}

// 使用单元测试框架的示例
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建一个测试框架实例
    let mut framework = TestFramework::new();

    // 添加测试用例
    framework.add_test_case(TestCase::new(
        "Test example",
        || async {
            // 这里放置测试代码
            // 例如，一个简单的断言
            assert_eq!(2 + 2, 4);
            Ok(())
        },
    ));

    // 运行所有测试用例
    framework.run_all_tests().await?;

    Ok(())
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_framework_example() -> Result<(), Box<dyn Error>> {
        let mut framework = TestFramework::new();

        // 添加测试用例
        framework.add_test_case(TestCase::new(
            "Test framework example",
            || async {
                // 这里放置测试代码
                // 例如，一个简单的断言
                assert_eq!(2 + 2, 4);
                Ok(())
            },
        ));

        // 运行所有测试用例
        framework.run_all_tests().await?;

        Ok(())
    }
}
