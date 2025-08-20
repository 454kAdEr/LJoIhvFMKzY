use std::fs::File;
use std::io::{self, BufRead, BufReader};
use tokio;
use tokio::fs::File as AsyncFile;
use tokio::io::{AsyncBufReadExt, BufReader as AsyncBufReader};
use log::{info, warn};

/// 解析日志文件并输出关键信息的结构体
pub struct LogParser {
    file_path: String,
}

impl LogParser {
    /// 创建一个新的LogParser实例
    pub fn new(file_path: &str) -> Self {
        LogParser {
            file_path: file_path.to_string(),
        }
    }

    /// 同步解析日志文件
    pub fn parse_logs_sync(&self) -> io::Result<Vec<String>> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let mut results = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if self.is_relevant(&line) {
                results.push(line);
            }
        }
        Ok(results)
    }

    /// 异步解析日志文件
    #[tokio::main]
    pub async fn parse_logs_async(&self) -> io::Result<Vec<String>> {
        let file = AsyncFile::open(&self.file_path).await?;
        let reader = AsyncBufReader::new(file);

        let mut results = Vec::new();
        while let Some(line) = reader.lines().await? {
            if self.is_relevant(&line) {
                results.push(line);
            }
        }
        Ok(results)
    }

    /// 判断日志行是否包含关键信息
    fn is_relevant(&self, line: &str) -> bool {
        // 此处可以根据实际日志格式和需求来实现具体的过滤逻辑
        // 例如，检查是否包含特定的关键词或错误代码等
        line.contains("ERROR") || line.contains("WARN")
    }
}

/// 程序入口点
fn main() {
    let log_parser = LogParser::new("./path/to/your/logfile.log");
    match log_parser.parse_logs_sync() {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        },
        Err(e) => {
            warn!("Failed to parse logs: {}", e);
        },
    }
    // 也可以使用异步版本
    // tokio::runtime::Runtime::new().unwrap().block_on(async {
    //     let log_parser = LogParser::new("./path/to/your/logfile.log");
    //     match log_parser.parse_logs_async().await {
    //         Ok(lines) => {
    //             for line in lines {
    //                 println!("{}", line);
    //             }
    //         },
    //         Err(e) => {
    //             warn!("Failed to parse logs asynchronously: {}", e);
    //         },
    //     }
    // });
}
