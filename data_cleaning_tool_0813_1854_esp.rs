// data_cleaning_tool.rs

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use serde::{Deserialize, Serialize};
use regex::Regex;

// 定义一个结构体来表示清洗后的数据
#[derive(Serialize, Deserialize, Debug)]
struct CleanedData {
    id: u32,
    name: String,
    email: String,
}

// 定义一个数据清洗器
struct DataCleaner;

impl DataCleaner {
    // 构造函数
    fn new() -> Self {
        DataCleaner
    }

    // 数据清洗函数
    fn clean_data(&self, data: &str) -> Result<Vec<CleanedData>, String> {
        let mut cleaned_data = Vec::new();
        let lines = data.lines();
        for line in lines {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            match self.parse_line(trimmed_line) {
                Ok(data) => cleaned_data.push(data),
                Err(e) => return Err(format!("Error parsing line '{}': {}", line, e)),
            }
        }
        Ok(cleaned_data)
    }

    // 解析单行数据
    fn parse_line(&self, line: &str) -> Result<CleanedData, String> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return Err("Line does not contain exactly three parts".to_string());
        }
        let id: u32 = parts[0].parse().map_err(|e| e.to_string())?;
        let name = parts[1].to_string();
        let email = parts[2].to_string();
        let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        if !re.is_match(&email) {
            return Err("Invalid email format".to_string());
        }
        Ok(CleanedData { id, name, email })
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 创建数据清洗器实例
    let cleaner = DataCleaner::new();

    // 打开文件
    let file = File::open("data.csv").map_err(|e| io::Error::new(e.kind(), e.to_string()))?;
    let reader = BufReader::new(file);

    // 读取文件内容
    let mut data = String::new();
    reader.read_to_string(&mut data).await.map_err(|e| io::Error::new(e.kind(), e.to_string()))?;

    // 清洗数据并处理错误
    match cleaner.clean_data(&data) {
        Ok(cleaned) => {
            // 这里可以添加代码将清洗后的数据保存到文件或进行进一步处理
            println!("Cleaned Data: {:?}", cleaned);
        },
        Err(e) => println!("Error cleaning data: {}", e),
    }

    Ok(())
}
