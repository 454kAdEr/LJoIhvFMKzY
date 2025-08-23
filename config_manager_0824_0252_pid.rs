use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::Deserialize;
use serde_json;
use tokio::fs;
use tokio::task;

// 定义配置文件的结构
#[derive(Deserialize, Debug)]
struct Config {
    // 根据实际配置添加字段
    // example_key: String,
}

/// ConfigManager 负责读取和管理配置文件
struct ConfigManager {
    path: String,
}

impl ConfigManager {
    /// 创建一个新的 ConfigManager 实例
    pub fn new(path: String) -> Self {
        ConfigManager { path }
    }

    /// 异步加载配置文件
    pub async fn load_config(&self) -> io::Result<Config> {
        // 异步读取文件内容
        let content = fs::read_to_string(&self.path).await?;

        // 解析 JSON 内容到 Config 结构体
        serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 配置文件路径
    let config_path = "./config.json".to_string();
    let config_manager = ConfigManager::new(config_path);

    // 加载配置
    match config_manager.load_config().await {
        Ok(config) => {
            println!("配置加载成功: {:?}", config);
        },
        Err(e) => {
            eprintln!("配置加载失败: {}", e);
        },
    }

    Ok(())
}