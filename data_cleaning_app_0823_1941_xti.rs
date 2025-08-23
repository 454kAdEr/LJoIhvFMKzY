use std::collections::HashMap;
use tokio::fs;
use serde::{Serialize, Deserialize};
use serde_json::Result;

// 定义一个结构体来表示需要被清洗的数据
#[derive(Serialize, Deserialize, Debug)]
struct DataRecord {
    name: String,
    age: Option<u32>,
    email: Option<String>,
}

// 定义一个数据清洗器
struct DataCleaner;

impl DataCleaner {
    // 创建一个新的数据清洗器实例
    fn new() -> Self {
        DataCleaner
    }

    // 清洗数据记录，预处理数据
    async fn clean_data(&self, data: &DataRecord) -> Result<DataRecord> {
        let mut clean_data = data.clone();

        // 清洗名字字段，去除前后空格
        clean_data.name = clean_data.name.trim().to_string();

        // 清洗年龄字段，确保是有效的年龄值
        if let Some(age) = &mut clean_data.age {
            if *age < 0 || *age > 150 {
                return Err(serde_json::Error::custom("Invalid age"));
            }
        }

        // 清洗邮箱字段，确保是有效的邮箱格式
        if let Some(email) = &mut clean_data.email {
            if !email.contains("@") {
                return Err(serde_json::Error::custom("Invalid email format"));
            }
        }

        Ok(clean_data)
    }
}

#[tokio::main]
async fn main() {
    // 加载数据文件
    let file_contents = fs::read_to_string("data.json").await.unwrap();
    let data_records: Vec<DataRecord> = serde_json::from_str(&file_contents).unwrap();

    // 实例化数据清洗器
    let cleaner = DataCleaner::new();

    for record in data_records {
        match cleaner.clean_data(&record).await {
            Ok(cleaned_record) => println!("Cleaned data: {:?}