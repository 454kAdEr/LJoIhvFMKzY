// excel_generator.rs
//
// 一个使用 Rust 和 Tokio 框架实现的 Excel 表格自动生成器。

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::Serialize;
use serde_json::json;
use std::error::Error;
use excel_writer::{Workbook, Worksheet};
use serde_json::Value;
use std::path::Path;

#[derive(Serialize)]
struct Data {
    // 定义需要写入 Excel 的数据结构
    name: String,
    age: u32,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 定义一些示例数据
    let data = vec![
        Data { name: "Alice".to_string(), age: 30, email: "alice@example.com".to_string() },
        Data { name: "Bob".to_string(), age: 25, email: "bob@example.com".to_string() },
        Data { name: "Charlie".to_string(), age: 35, email: "charlie@example.com".to_string() },
    ];

    // 创建 Workbook 实例
    let mut workbook = Workbook::new();
    let mut sheet = workbook.add_sheet("Sheet1");

    // 添加标题行
    sheet.write(0, 0, "Name")?;
    sheet.write(0, 1, "Age")?;
    sheet.write(0, 2, "Email")?;

    // 将数据写入 Excel
    for (index, item) in data.iter().enumerate() {
        sheet.write((index + 1) as u32, 0, &item.name)?;
        sheet.write((index + 1) as u32, 1, &item.age)?;
        sheet.write((index + 1) as u32, 2, &item.email)?;
    }

    // 将 Workbook 保存为 Excel 文件
    let path = Path::new("output.xlsx");
    let mut file = File::create(path).await?;
    file.write_all(&workbook.to_bytes()?).await?;

    println!("Excel file generated successfully!");
    Ok(())
}
