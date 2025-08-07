use reqwest::Url;
use tokio;
use std::error::Error;

/// 检查URL链接是否有效
/// 
/// 这个函数接收一个URL字符串，尝试解析并检查这个URL是否有效。
/// 
/// # 参数
/// * `url` - 要检查的URL字符串
/// 
/// # 返回值
/// 成功时返回`Result<Url, Box<dyn Error>>`，其中包含解析后的`Url`对象或错误。
async fn validate_url(url: &str) -> Result<Url, Box<dyn Error>> {
    // 尝试解析URL
    let parsed_url = Url::parse(url)?;

    // 检查URL是否有效
    if !parsed_url.is_valid() {
        return Err("Invalid URL".into());
    }

    Ok(parsed_url)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 示例URL
    let test_url = "https://www.example.com";

    // 调用函数检查URL是否有效
    match validate_url(test_url).await {
        Ok(valid_url) => {
            println!("Valid URL: {}", valid_url);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }

    Ok(())
}
