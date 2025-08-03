use reqwest::Error;
use tokio;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use scraper::{Html, Selector};

/// 定义一个结构体，用于网页内容抓取
struct WebScraper;

impl WebScraper {
    /// 抓取网页内容的函数
    /// \u0026url: 需要抓取的网页URL
    async fn scrape(&self, url: &str) -> Result<String, Error> {
        // 发起网络请求，获取网页内容
        let res = reqwest::get(url).await?;
        // 确保响应状态码为200
        res.error_for_status()?;
        // 获取响应体并转换为String
        let body = res.text().await?;
        Ok(body)
    }

    /// 保存网页内容到文件的函数
    /// \u0026content: 网页内容
    /// path: 文件保存路径
    fn save_to_file(&self, content: &str, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// 从HTML中提取文本的函数
    /// html: Html对象
    /// selector: CSS选择器
    fn extract_text_from_html(html: &Html, selector: &Selector) -> Vec<String> {
        html.select(selector)
            .filter_map(|element| element.text().collect::<Vec<_>>().into_iter().next())
            .map(|text| text.trim().to_string())
            .collect()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化WebScraper实例
    let scraper = WebScraper;

    // 需要抓取的网页URL
    let url = "https://example.com";
    // 文件保存路径
    let path = Path::new("output.html");

    // 抓取网页内容
    let content = scraper.scrape(url).await?;

    // 保存网页内容到文件
    scraper.save_to_file(&content, path)?;

    // 从HTML中提取文本
    let html = Html::parse_document(&content);
    let selector = Selector::parse("p").unwrap(); // 假设我们提取所有的<p>标签
    let texts = scraper.extract_text_from_html(&html, &selector);

    // 打印提取的文本
    for text in texts {
        println!("{}", text);
    }

    Ok(())
}