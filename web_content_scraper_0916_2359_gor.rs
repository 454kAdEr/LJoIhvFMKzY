use reqwest::Error;
use tokio::main;
use std::env;
use std::error::Error;
use url::Url;
use console::Style;
use indicatif::ProgressBar;

// 主函数，用于执行网页内容抓取
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <URL>", args[0]);
        return Ok(());
    }

    let url_str = &args[1];
    let url = Url::parse(url_str).map_err(|_| format!("Invalid URL: {}", url_str))?;

    println!("Scraping content from: {}", url_str);

    let content = scrape_content(url).await?;
    println!("Scraped content:\
{}", content);

    Ok(())
}

// 异步函数，用于抓取网页内容
async fn scrape_content(url: Url) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let status = response.status();

    if status.is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err(Box::new(format!("Failed to retrieve content: {}", status)))
    }
}

// 简单的错误处理和日志记录
trait LogError {
    fn log_error(&self);
}

impl<E: Error> LogError for E {
    fn log_error(&self) {
        let styled_error = Style::new().red().paint(self.description());
        println!("Error: {}", styled_error);
    }
}
