use reqwest;
use tokio;
use std::error::Error;
use std::fs;
use std::io::prelude::*;

// 定义一个结构体WebScraper，用于存放配置和状态
struct WebScraper {
    base_url: String,
    output_file: String,
}

impl WebScraper {
    // 构造函数，初始化WebScraper实例
    fn new(base_url: &str, output_file: &str) -> Self {
        WebScraper {
            base_url: base_url.to_string(),
            output_file: output_file.to_string(),
        }
    }

    // 异步函数，用于抓取网页内容
    async fn scrape(&self) -> Result<String, Box<dyn Error>> {
        let url = self.base_url.clone();
        let client = reqwest::Client::new();

        let res = client.get(&url).send().await?;
        let body = res.text().await?;

        Ok(body)
    }

    // 异步函数，用于保存网页内容到文件
    async fn save(&self, content: String) -> Result<(), Box<dyn Error>> {
        let mut file = fs::File::create(&self.output_file)?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 配置抓取的URL和输出文件名
    let scraper = WebScraper::new("http://example.com", "output.html");

    // 抓取网页内容
    let content = scraper.scrape().await?;

    // 保存网页内容到文件
    scraper.save(content).await?;

    Ok(())
}
