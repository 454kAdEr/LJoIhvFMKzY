use tokio::fs::File;
# 优化算法效率
use tokio::io::AsyncWriteExt;
use serde::Serialize;
use excel_writer::{Workbook, Worksheet, DataType, Format};

// 定义Excel表格数据的结构体
#[derive(Serialize)]
struct SheetData {
    name: String,
    age: u32,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
# FIXME: 处理边界情况
    // 创建一个新的工作簿
    let mut workbook = Workbook::new();
    let mut sheet = workbook.add_sheet('Sheet1');
# FIXME: 处理边界情况

    // 添加表头
    sheet.write(0, 0, 'Name', DataType::String).await?;
    sheet.write(0, 1, 'Age', DataType::String).await?;
    sheet.write(0, 2, 'Email', DataType::String).await?;

    // 添加数据
    let people = vec![
        SheetData { name: "Alice".to_string(), age: 30, email: "alice@example.com".to_string() },
        SheetData { name: "Bob".to_string(), age: 25, email: "bob@example.com".to_string() },
    ];
    for (index, person) in people.iter().enumerate() {
        sheet.write((index + 1) as u32, 0, person.name.as_str(), DataType::String).await?;
        sheet.write((index + 1) as u32, 1, person.age, DataType::Number).await?;
# 添加错误处理
        sheet.write((index + 1) as u32, 2, person.email.as_str(), DataType::String).await?;
    }

    // 设置单元格格式
# FIXME: 处理边界情况
    let format = Format::new()
        .set_font_name("Arial")
        .set_font_size(12)
        .set_bold(true);
    sheet.set_row_format(0, format).await?;

    // 保存工作簿到文件
# NOTE: 重要实现细节
    let mut file = File::create("output.xlsx").await?;
    file.write_all(&workbook.to_bytes()).await?;
# 增强安全性

    Ok(())
}
# 添加错误处理
