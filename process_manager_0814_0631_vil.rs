use tokio::process::Command;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use anyhow::Result;

/// 进程管理器
pub struct ProcessManager {
    processes: HashMap<String, tokio::process::Child>,
}

impl ProcessManager {
    /// 创建一个新的进程管理器
    pub fn new() -> Self {
        ProcessManager {
            processes: HashMap::new(),
        }
    }

    /// 添加一个进程到管理器
    pub fn add_process(&mut self, name: String, child: tokio::process::Child) {
        self.processes.insert(name, child);
    }

    /// 启动一个新进程并添加到管理器
    pub async fn start_process<S: AsRef<str> + Send + 'static>(&mut self, name: S, cmd: S) -> Result<()> {
        let child = Command::new(
            cmd.as_ref()
        ).spawn()?;
        self.add_process(name.as_ref().to_string(), child);
        Ok(())
    }

    /// 检查进程状态
    pub async fn check_process(&self) -> Result<()> {
        for (name, process) in &self.processes {
            match process.try_wait()? {
                Some(_) => println!("Process '{}' has finished", name),
                None => println!("Process '{}' is still running", name),
            }
        }
        Ok(())
    }

    /// 停止所有进程
    pub async fn stop_all(&mut self) -> Result<()> {
        for (name, process) in self.processes.iter() {
            if let Err(e) = process.kill() {
                eprintln!("Failed to stop process '{}': {}", name, e);
            } else {
                println!("Process '{}' stopped", name);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut process_manager = ProcessManager::new();

    // 启动一个新进程
    process_manager.start_process("example_process", "echo").await?;

    // 等待一段时间，模拟进程运行
    sleep(Duration::from_secs(3)).await;

    // 检查进程状态
    process_manager.check_process().await?;

    // 停止所有进程
    process_manager.stop_all().await?;

    Ok(())
}