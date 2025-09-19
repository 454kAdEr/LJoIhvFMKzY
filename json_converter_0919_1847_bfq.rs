use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::fs::File as AsyncFile;
use tokio::io::BufReader as AsyncBufReader;
use tokio::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    // 配置结构体，可根据需要添加配置字段
    indent: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 读取配置文件
    let config = read_config("config.json").await?;

    // 打开待转换的JSON文件
    let file_path = "input.json";
    let mut input_file = AsyncFile::open(file_path).await?;
    let mut reader = AsyncBufReader::new(input_file);

    // 读取JSON文件内容
    let mut content = String::new();
    reader.read_to_string(&mut content).await?;

    // 反序列化JSON内容
    let value: Value = serde_json::from_str(&content).map_err(|e|
        io::Error::new(io::ErrorKind::InvalidData, e)
    )?;

    // 序列化JSON内容为新格式
    let formatted_json = match config.indent.as_str() {
        "" => serde_json::to_string(&value).unwrap(),
        _ => serde_json::to_string_pretty(&value).unwrap(),
    };

    // 写入转换后的JSON文件
    let output_file_path = "output.json";
    let mut output_file = AsyncFile::create(output_file_path).await?;
    output_file.write_all(formatted_json.as_bytes()).await?;

    Ok(())
}

// 读取配置文件函数
async fn read_config(file_path: &str) -> io::Result<Config> {
    let mut file = AsyncFile::open(file_path).await?;
    let mut reader = AsyncBufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).await?;
    let config: Config = serde_json::from_str(&content).map_err(|e|
        io::Error::new(io::ErrorKind::InvalidData, e)
    )?;
    Ok(config)
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_json_conversion() -> io::Result<()> {
        // 创建测试文件
        let test_file_path = "test_input.json";
        let mut file = File::create(test_file_path).await?;
        file.write_all(b"{"key": "value"}")..await?;

        // 调用主函数
        main().await?;

        // 读取转换后的文件
        let mut file = AsyncFile::open("output.json").await?;
        let mut reader = AsyncBufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content).await?;

        // 验证转换结果
        assert_eq!(content, "{"key": "value"}
");

        Ok(())
    }
}
