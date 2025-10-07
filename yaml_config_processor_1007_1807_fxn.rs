use serde::Deserialize;
use serde_yaml;
use std::fs;
use tokio;

// 定义一个结构体来反序列化YAML配置文件
#[derive(Debug, Deserialize)]
struct Config {
    database: DatabaseConfig,
    // 可以根据需要添加更多配置字段
}

// 定义数据库配置结构体
#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    user: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取YAML配置文件
    let config_str = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_str)?;

    // 打印配置信息，实际应用中可以根据需要进行其他操作
    println!("Database URL: {}", config.database.url);
    println!("Database User: {}", config.database.user);
    println!("Database Password: {}", config.database.password);

    Ok(())
}
