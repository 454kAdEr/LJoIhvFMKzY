// integration_tests.rs

// 引入必要的库
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::io::Result;
use std::pin::Pin;
use tokio::stream::StreamExt;

// 异步处理连接的函数
async fn handle_connection(mut socket: tokio::net::TcpStream) -> Result<()> {
    // 读取客户端发送的数据
    let mut buffer = [0; 1024];
    let n = socket.read(&mut buffer).await?;

    // 处理接收到的数据（这里只是简单地回显）
    if n == 0 {
        return Ok(());
    }

    // 将相同的数据发送回客户端
    socket.write_all(&buffer[0..n]).await?;
    socket.flush().await?;

    Ok(())
}

// 异步主函数，设置服务器
#[tokio::main]
async fn main() -> Result<()> {
    // 绑定服务器地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;

    // 无限循环，接受连接
    loop {
        let (socket, _) = listener.accept().await?;

        // 为每个连接创建一个处理任务
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                // 错误处理
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}
