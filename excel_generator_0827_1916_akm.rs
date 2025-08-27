use tokio::io::AsyncWriteExt;
use excelwriter::{Workbook, Worksheet, Error};
use std::path::Path;
use tokio::fs::File;
use tokio::fs::OpenOptions;

/// 创建一个新的Excel工作簿并保存到指定路径
///
/// # 参数
/// * `filename` - Excel文件的路径
/// * `data` - 需要写入的数据，以二维数组的形式表示
///
/// # 返回值
/// * `Result<(), Error>` - 成功则返回`()`，失败则返回错误信息
async fn create_excel(filename: &str, data: Vec<Vec<String>>) -> Result<(), Error> {
    let mut workbook = Workbook::new();
    let mut worksheet = workbook.add_worksheet();

    for (row_index, row) in data.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            worksheet.write_string(row_index, col_index, value)?;
        }
    }

    workbook.save_to_file(filename).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // 示例数据
    let data = vec![
        vec!["姓名".to_string(), "年龄".to_string()],
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];

    // Excel文件保存路径
    let filename = "example.xlsx";

    // 尝试创建Excel文件
    if let Err(e) = create_excel(filename, data).await {
        eprintln!("Error occurred: {}", e);
    } else {
        println!("Excel file created successfully!");
    }
}
