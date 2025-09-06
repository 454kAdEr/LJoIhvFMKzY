use std::fs;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// 定义配置数据结构
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    // 这里可以根据你的配置需求添加字段
    pub example_setting: String,
}

// 配置文件管理器
pub struct ConfigManager {
    pub file_path: String,
}

impl ConfigManager {
    // 创建一个新的 ConfigManager 实例
    pub fn new(file_path: String) -> Self {
        ConfigManager { file_path }
    }

    // 异步读取配置文件
    pub async fn load_config(&self) -> io::Result<AppConfig> {
        let mut file = File::open(self.file_path.clone()).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let config: AppConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }

    // 异步写入配置文件
    pub async fn save_config(&self, config: &AppConfig) -> io::Result<()> {
        let contents = serde_json::to_string(&config)?;
        let mut file = File::create(self.file_path.clone()).await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let config_manager = ConfigManager::new("config.json".to_string());

    // 加载配置文件
    match config_manager.load_config().await {
        Ok(config) => println!("Loaded config: {:?}", config),
        Err(e) => println!("Failed to load config: {}", e),
    }

    // 示例：更新配置并保存
    let mut config = config_manager.load_config().await?;
    config.example_setting = "new_value".to_string();
    config_manager.save_config(&config).await?;

    Ok(())
}