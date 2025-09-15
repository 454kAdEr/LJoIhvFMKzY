use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use tokio;
use tokio::io::AsyncBufReadExt;
use tokio::fs::File as AsyncFile;
use tokio::io::BufReader as AsyncBufReader;

/// 文本文件内容分析器
/// 这个结构体负责打开和分析文本文件的内容
pub struct TextFileAnalyzer {
    file_path: String,
}

impl TextFileAnalyzer {
    /// 创建一个新的文本文件分析器实例
    ///
    /// 参数:
    /// * `file_path` - 文本文件的路径
    pub fn new(file_path: String) -> Self {
        TextFileAnalyzer { file_path }
    }

    /// 异步打开并分析文本文件的内容
    ///
    /// 返回:
    /// 一个包含文件行数的结果
    async fn analyze(&self) -> io::Result<usize> {
        let file = AsyncFile::open(&self.file_path).await?;
        let reader = AsyncBufReader::new(file);

        let mut line_count = 0;
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            line_count += 1;
            line.clear();
        }

        Ok(line_count)
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let analyzer = TextFileAnalyzer::new("path_to_your_file.txt".to_string());
    let line_count = analyzer.analyze().await?;
    println!("文件中的行数：{}", line_count);
    Ok(())
}