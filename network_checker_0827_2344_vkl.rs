// network_checker.rs
// 一个使用RUST和TOKIO框架的网络连接状态检查器

use std::net::ToSocketAddrs;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use anyhow::Result;

/// 检查给定主机的网络连接状态
///
/// # 参数
/// * `host` - 要检查的主机地址
/// * `port` - 要检查的端口
///
/// # 返回值
/// 一个`Result`，包含连接状态或者错误信息
async fn check_connection(host: &str, port: u16) -> Result<(), io::Error> {
    let addr = (host, port).to_socket_addrs()?;
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b"HELLO").await?;
    let mut buf = [0; 1024];
    let _ = stream.read_exact(&mut buf).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // 检查的主机和端口
    let host = "www.example.com";
    let port = 80;
    
    // 尝试连接到指定的主机和端口
    match check_connection(host, port).await {
        Ok(_) => println!("Connection to {}:{} is successful", host, port),
        Err(e) => println!("Failed to connect to {}:{}. Error: {}", host, port, e),
    }

    Ok(())
}
