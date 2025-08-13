use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use std::net::SocketAddr;
use anyhow::Result;
use tracing::{info, error};

// 定义检查网络连接状态的函数
async fn check_connection(addr: &SocketAddr) -> Result<bool> {
    let timeout_duration = Duration::from_secs(10); // 设置超时时间

    // 尝试建立TCP连接
    match TcpStream::connect_timeout(addr, timeout_duration).await {
        Ok(_) => {
            info!("Connection to {} is successful", addr);
            Ok(true)
        },
        Err(e) => {
            error!("Failed to connect to {}: {}", addr, e);
            Err(e).into()) // 将错误传播出去
        },
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 定义要检查的服务器地址
    let addr: SocketAddr = "www.google.com:80".parse()?;

    // 检查连接状态
    match check_connection(&addr).await {
        Ok(is_connected) => {
            if is_connected {
                println!("Successfully connected to {}!", addr);
            } else {
                println!("Failed to connect to {}!", addr);
            }
        },
        Err(e) => {
            eprintln!("An error occurred: {}", e);
        },
    }

    Ok(())
}
