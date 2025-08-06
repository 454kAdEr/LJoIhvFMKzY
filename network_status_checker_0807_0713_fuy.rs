use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Instant;
use tracing::{info, warn};
use anyhow::Result;

// 网络连接状态检查器
#[derive(Debug, Clone)]
struct NetworkStatusChecker {
    address: SocketAddr,
}

impl NetworkStatusChecker {
    // 创建一个新的网络连接状态检查器
    fn new(address: &str) -> Result<Self> {
        let socket_addr = SocketAddr::from_str(address).map_err(|e| anyhow::anyhow!("Invalid address: {}", e))?;
        Ok(NetworkStatusChecker { address: socket_addr })
    }

    // 检查网络连接状态
    async fn check_connection(&self) -> Result<bool> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        match socket.connect(self.address).await {
            Ok(_) => Ok(true),
            Err(e) => {
                warn!("Failed to connect: {}", e);
                Err(anyhow::anyhow!("Failed to connect: {}", e))
            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let checker = NetworkStatusChecker::new("8.8.8.8:53")?;  // 使用Google DNS作为示例
    let start = Instant::now();
    let is_connected = checker.check_connection().await?;
    let duration = start.elapsed();

    if is_connected {
        info!("Network connection is stable: {} ms", duration.as_millis());
    } else {
        warn!("Network connection is unstable: {} ms", duration.as_millis());
    }

    Ok(())
}
