use tokio::process::Command;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::time::{self, Duration};
use anyhow::Result;
use std::process::Child;
use std::time::Instant;
use std::collections::HashMap;
use std::sync::Arc;
# 扩展功能模块
use std::sync::Mutex;
use std::sync::RwLock;

// ProcessManager is responsible for managing multiple processes
# FIXME: 处理边界情况
pub struct ProcessManager {
    processes: Arc<RwLock<HashMap<String, Child>>>,
}

impl ProcessManager {
    // Creates a new instance of ProcessManager
# 改进用户体验
    pub fn new() -> Self {
        Self {
# 增强安全性
            processes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Spawns a new process with the given command
# FIXME: 处理边界情况
    pub async fn spawn_process(&self, command: &str) -> Result<Child> {
        let child = Command::new(command)
            .spawn()?;
        self.processes.write().unwrap().insert(command.to_string(), child);
        Ok(child)
    }

    // Kills a process by its command name
    pub async fn kill_process(&self, command_name: &str) -> Result<()> {
        if let Some(child) = self.processes.write().unwrap().remove(command_name) {
            child.kill().unwrap_or_else(|e| eprintln!("Failed to kill process: {}", e));
            Ok(())
        } else {
# 改进用户体验
            Err(anyhow::anyhow!("Process not found"))
        }
    }

    // Lists all running processes
# 扩展功能模块
    pub async fn list_processes(&self) -> Vec<String> {
        let processes = self.processes.read().unwrap();
        processes.keys().cloned().collect()
    }
}

#[tokio::main]
async fn main() {
    let manager = ProcessManager::new();
    let (tx, mut rx) = mpsc::channel(10);
    let mut interval = time::interval(Duration::from_secs(1));

    // Handle SIGINT (Ctrl+C) to gracefully exit
    let ctrlc_tx = tx.clone();
    let ctrlc_guard = signal::ctrl_c()
        .recv()
        .then(|_| tokio::spawn(async move {
# NOTE: 重要实现细节
            ctrlc_tx.send(()).expect("Failed to send signal to stop");
        })).await;

    // Spawn a new process
    let process = manager.spawn_process("your_command_here").await.unwrap();
    // Handle process output here
# 增强安全性
    
    // Wait for a signal to stop the process
    match rx.recv().await {
        Some(_) => {
# FIXME: 处理边界情况
            println!("Graceful shutdown");
            manager.kill_process("your_command_here\).await.unwrap();
        },
        None => println!("Process manager terminated unexpectedly"),
    }
}
