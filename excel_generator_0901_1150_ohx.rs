use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::Serialize;
use async_stream::stream;
use excel::writer::ExcelWriter;
use excel::cell::Cell;
use excel::worksheet::Worksheet;
use excel::format::Format;
use excel::date_time::DateTime;
use excel::prelude::*;
use std::path::Path;

// 定义一个结构体来存储表格数据
#[derive(Serialize)]
struct Data {
    name: String,
    date: String,
    value: f64,
}

// 异步函数，生成Excel文件
async fn generate_excel<P: AsRef<Path> + Send + Sync>(data: Vec<Data>, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = ExcelWriter::new();
    let mut worksheet = workbook.book().add_worksheet().await?;

    // 添加标题
    worksheet.cell(0, 0).string("Name")?;
    worksheet.cell(0, 1).string("Date")?;
    worksheet.cell(0, 2).string("Value")?;

    // 添加数据
    for (row, item) in data.iter().enumerate().skip(1) {
        worksheet.cell(row + 1, 0).string(&item.name)?;
        worksheet.cell(row + 1, 1).string(&item.date)?;
        worksheet.cell(row + 1, 2).number(item.value)?;
    }

    // 设置保存路径
    let file = File::create(path).await?;
    file.write_all(&workbook.finish().await?).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 示例数据
    let data = vec![
        Data {
            name: "Item 1".to_string(),
            date: "2023-01-01".to_string(),
            value: 100.0,
        },
        Data {
            name: "Item 2".to_string(),
            date: "2023-01-02".to_string(),
            value: 200.0,
        },
    ];

    // 生成Excel文件
    generate_excel("example.xlsx", &data).await?;

    Ok(())
}
