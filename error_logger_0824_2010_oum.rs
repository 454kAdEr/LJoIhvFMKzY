use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{Record, Level, Log, Metadata, SetLoggerError};
use once_cell::sync::Lazy;
use std::time::SystemTime;
use backtrace::Backtrace;
use tokio::time::{sleep, Duration};
use std::error::Error;

// 创建一个全局的日志器
static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    let logger = Logger::new("error.log").await;
    Mutex::new(logger)
});

pub struct Logger {
    // 使用 Tokio 的异步文件写入器来写入日志文件
    writer: BufWriter<File>,
}

impl Logger {
    // 异步创建一个新的 Logger 实例
    async fn new<P: AsRef<Path> + Send + Sync + 'static>(path: P) -> Self {
        let file = File::create(path).await.expect("Unable to create log file");
        let writer = BufWriter::new(file);
        Logger { writer }
    }

    // 异步写入日志
    async fn write_log(&mut self, record: &Record) -> Result<(), Box<dyn Error>> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let level = record.metadata().level();
        let message = format!("[{}.{}] {}
", timestamp, level, record.args());

        self.writer
            .write_all(message.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Error
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut logger = LOGGER.lock().await;
            let result = logger.write_log(record).await;
            if let Err(e) = result {
                eprintln!("Failed to write log: {}", e);
            }
        }
    }

    fn flush(&self) {
        // 异步日志器不需要手动 flush，因为异步写入器已经处理了这一点
    }
}

#[tokio::main]
async fn main() {
    // 设置全局 logger
    log::set_logger(&*LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Error))
        .expect("Unable to set logger");

    // 测试日志记录
    log::error!("An error occurred");
    let result = std::panic::catch_unwind(|| {
        panic!("Panic occurred");
    });
    if let Err(e) = result {
        let backtrace = Backtrace::new();
        let error = e.downcast_ref::<String>().unwrap_or("Unknown panic");
        log::error!("Panic occurred: {}
Backtrace: {:?}