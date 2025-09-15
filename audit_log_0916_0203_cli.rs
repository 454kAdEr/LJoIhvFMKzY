// audit_log.rs
// 这是一个使用Rust和Tokio框架实现的安全审计日志程序。
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};
use chrono::prelude::*;
use std::path::PathBuf;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;

// 日志记录器结构体
struct Logger {
    file_path: PathBuf,
}

impl Logger {
    // 创建一个新的日志记录器
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Logger {
            file_path: file_path.into(),
        }
    }

    // 异步写入日志
    pub async fn log(&self, message: &str) -> io::Result<()> {
        let now = Utc::now();
        let log_message = format!("\"{}\