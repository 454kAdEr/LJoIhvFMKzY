use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::path::Path;
use csv::ReaderBuilder;
use serde::Deserialize;

// 定义CSV文件中的数据结构
#[derive(Debug, Deserialize)]
struct CsvRecord {
    column1: String,
    column2: String,
    // 可以根据实际CSV文件内容添加更多字段
}

// 主函数，程序入口
#[tokio::main]
async fn main() -> io::Result<()> {
    // 路径到包含CSV文件的目录
    let directory_path = "./csv_files";
    let entries = std::fs::read_dir(directory_path)?;
    let mut tasks = Vec::new();

    for entry in entries {
        let path = entry?.path();
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("csv") {
            let task = tokio::spawn(async move {
                process_csv_file(&path).await
            });
            tasks.push(task);
        }
    }

    // 等待所有任务完成
    for task in tasks {
        task.await.expect("Failed to process CSV file");
    }
    Ok(())
}

// 处理单个CSV文件的函数
async fn process_csv_file(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().from_reader(reader);

    for result in csv_reader.deserialize() {
        let record: CsvRecord = result?;
        // 处理CSV记录，例如打印出来
        println!("Record: {:?}", record);
    }
    Ok(())
}
