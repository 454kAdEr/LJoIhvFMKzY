use tokio::process::Command;
use tokio::signal;
use tokio::sync::mpsc;
use std::process::Command as StdCommand;
use tokio::io::{self, AsyncBufReadExt};
use tokio::time::{sleep, Duration};
use std::error::Error;
use std::time::Instant;

// 定义ProcessManager结构体，用于管理进程
pub struct ProcessManager {
    process: StdCommand,
    rx: mpsc::Receiver<String>,
}

impl ProcessManager {
    // 创建一个新的ProcessManager实例
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);

        let process = StdCommand::new("your_program")
            .arg("your_argument")
            .stdout(tx);

        ProcessManager {
            process,
            rx,
        }
    }

    // 启动进程
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut child = self.process.spawn()?;

        let mut stdout = self.rx;

        while let Some(line) = stdout.recv().await {
            println!("Received: {}", line);
        }

        Ok(())
    }

    // 停止进程
    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(child) = self.process.take() {
            child.kill()?;
            Ok(())
        } else {
            Err("Process is not running".into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut process_manager = ProcessManager::new();

    // 启动进程管理器
    process_manager.start().await?;

    // 监听Ctrl+C信号，以便优雅地停止进程
    let ctrl_c = async {
        signal::ctrl_c().await?;
        println!("Ctrl+C received, stopping process...");
        process_manager.stop().await?;
    };

    // 等待进程停止或Ctrl+C信号
    tokio::join!(ctrl_c);

    Ok(())
}