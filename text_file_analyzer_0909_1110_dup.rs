use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use std::path::Path;
use std::collections::HashMap;
use std::sync::Arc;
# 优化算法效率
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use regex::Regex;
# 添加错误处理
use tokio::task;

// 定义一个结构体来存储分析结果
struct AnalysisResult {
    file_name: String,
    word_count: HashMap<String, usize>,
}

// 异步函数来分析文本文件的内容
async fn analyze_text_file<P: AsRef<Path>>(path: P) -> io::Result<AnalysisResult> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let word_count = count_words(&contents);
    Ok(AnalysisResult {
        file_name: path.as_ref().to_str().unwrap_or_default().to_string(),
        word_count,
# 增强安全性
    })
}

// 函数来计算单词出现的次数
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in words {
# FIXME: 处理边界情况
        let trimmed_word = word.trim_matches(|c: char| !c.is_alphabetic());
        let count = counts.entry(trimmed_word.to_lowercase().to_string()).or_insert(0);
        *count += 1;
    }

    counts
# 优化算法效率
}

// 程序的入口函数
#[tokio::main]
async fn main() {
    // 定义要分析的文件路径
    let file_path = "path/to/your/text/file.txt";

    // 使用异步任务来分析文件
# FIXME: 处理边界情况
    let analysis_result = task::spawn_blocking(move || {
        analyze_text_file(file_path)
    }).await;
    
    // 错误处理
    match analysis_result {
        Ok(result) => {
            println!("Analysis for file '{}' completed.", result.file_name);
            for (word, count) in result.word_count.iter() {
                println!("'{}' occurs {} times.", word, count);
            }
        },
        Err(e) => {
            eprintln!("Error analyzing the file: {}", e);
# FIXME: 处理边界情况
        },
# 改进用户体验
    }
}
