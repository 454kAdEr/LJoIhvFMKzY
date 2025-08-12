use tokio::net::UdpSocket;
use tokio::io::{self, AsyncBufReadExt};
use std::net::SocketAddr;
use std::env;
use std::error::Error;
use tokio::time::{Duration, Instant};
use std::time::Duration as StdDuration;
use std::str::FromStr;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 获取要检查的服务器地址和端口
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <server_ip> <port>", args[0]);
        return Err("Missing arguments".into());
    }
    let server_ip = &args[1];
    let port = u16::from_str(&args[2])?;

    // 创建UDP套接字
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    // 构造目标地址
    let target_addr = SocketAddr::from((server_ip, port));

    // 定义检查间隔
    let check_interval = StdDuration::from_secs(5);
    // 定义超时时间
    let timeout = Duration::from_secs(1);

    // 循环检查网络连接状态
    loop {
        let start = Instant::now();
        match socket.send_to(b"ping", &target_addr).await {
            Ok(_) => println!("Ping sent to {}", target_addr),
            Err(e) => eprintln!("Failed to send ping: {}", e),
        };

        match socket.recv_from(&mut [0u8; 1024]).await {
            Ok((size, src)) => {
                if src == target_addr {
                    println!("Received response from {}: size = {}", src, size);
                } else {
                    println!("Received packet from {} (expected {})