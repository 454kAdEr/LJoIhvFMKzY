use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::path::Path;
use std::error::Error;
use log::info;
use serde::Deserialize;
use serde_json::Value;
use regex::Regex;

// 日志条目结构体
#[derive(Debug, Deserialize, Clone)]
struct LogEntry {
    // 日志级别
    level: String,
    // 日志消息
    message: String,
    // 其他自定义字段
    // ...
}

// 解析日志文件的工具
struct LogParser {
    file_path: String,
    pattern: Regex,
}

impl LogParser {
    // 创建一个新的LogParser实例
    fn new(file_path: String, pattern: Regex) -> Self {
        LogParser {
            file_path,
            pattern,
        }
    }

    // 解析日志文件中的条目
    async fn parse_log(&self) -> io::Result<Vec<LogEntry>> {
        let file = File::open(Path::new(&self.file_path)).await?;
        let reader = BufReader::new(file);
        let mut log_entries = Vec::new();

        // 逐行读取并解析日志文件
        let mut line = String::new();
        while reader.read_line(&mut line).await? > 0 {
            if let Some(captures) = self.pattern.captures(&line) {
                let log_entry = LogEntry {
                    level: captures.get(1).map_or_default(|m| m.as_str().to_string()),
                    message: captures.get(2).map_or_default(|m| m.as_str().to_string()),
                    // 其他字段的解析
                    // ...
                };
                log_entries.push(log_entry);
            }
            line.clear();
        }

        Ok(log_entries)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 日志文件路径
    let file_path = "./log_file.log".to_string();
    // 正则表达式模式，用于匹配日志条目
    let pattern = Regex::new(r"^(\[INFO\] |\[ERROR\] |\[DEBUG\] )([\s\S]*?)$")?;

    // 创建LogParser实例
    let log_parser = LogParser::new(file_path, pattern);

    // 解析日志文件
    let log_entries = log_parser.parse_log().await?;

    // 打印解析出的日志条目
    for entry in log_entries {
        info!("Level: {}, Message: {}", entry.level, entry.message);
    }

    Ok(())
}
