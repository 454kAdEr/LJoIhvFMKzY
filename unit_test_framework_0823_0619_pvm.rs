use tokio::test;

/// 单元测试框架的入口点，用于运行所有的测试用例。
#[tokio::main]
async fn main() {
    let tests = vec![
        // 这里可以注册多个测试用例
        test_case::test_example,
    ];

    for test in tests {
        test.await.expect("测试失败")
    }
}

/// 一个示例测试模块，用于演示如何编写测试用例。
mod test_case {
    use super::*;
    use tokio::test;

    /// 一个简单的测试用例，演示了基本的测试结构。
    #[test]
    async fn test_example() {
        // 这里是测试代码
        let actual = 2 + 2;
        let expected = 4;

        assert_eq!(actual, expected, "testing that 2 + 2 equals 4");
    }
}
