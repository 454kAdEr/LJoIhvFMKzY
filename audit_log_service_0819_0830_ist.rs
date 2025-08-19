use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::prelude::*;
use log::info;
use log::error;
use std::io::Result;

// 定义一个异步的日志服务结构体
pub struct AuditLogService {
    file: Arc<Mutex<File>>,
}

impl AuditLogService {
    // 创建一个新的审计日志服务实例
    pub async fn new(log_file_path: &str) -> Result<Self> {
        let file = File::create(log_file_path).await?;
        Ok(Self {
            file: Arc::new(Mutex::new(file)),
        })
    }

    // 记录一条安全审计日志
    pub async fn log(&self, message: &str) -> Result<()> {
        let mut file = self.file.lock().await;
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!(""{}" - {}
", timestamp, message);
        file.write(log_entry.as_bytes()).await?;
        file.flush().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init().unwrap();

    let audit_log = AuditLogService::new("audit.log").await?;
    audit_log.log("User accessed the system").await?;
    audit_log.log("User performed an action").await?;

    Ok(())
}
