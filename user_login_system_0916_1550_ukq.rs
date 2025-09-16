use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::sync::Mutex as AsyncMutex;

// 定义全局的用户数据库
static GLOBAL_USER_DB: Lazy<AsyncMutex<HashMap<String, String>>> = Lazy::new(|| AsyncMutex::new(HashMap::new()));

#[tokio::main]
async fn main() -> io::Result<()> {
    // 启动监听
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is running on port 8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(handle_connection(socket));
    }
}

// 处理单个连接的异步函数
async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    // 读取客户端发送的数据
    let n = socket.read(&mut buf).await?;
    let user_input = String::from_utf8_lossy(&buf[..n]);
    println!("Received: {}", user_input);

    // 将用户输入拆分为用户名和密码
    let parts: Vec<&str> = user_input.split(":").collect();
    if parts.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input format"));
    }
    let username = parts[0].to_string();
    let password = parts[1].to_string();

    // 验证用户
    let global_user_db = GLOBAL_USER_DB.lock().await;
    if let Some(correct_password) = global_user_db.get(&username) {
        if password == correct_password {
            socket.write_all(b"Login successful
").await?;
        } else {
            socket.write_all(b"Incorrect password
").await?;
        }
    } else {
        socket.write_all(b"User not found
").await?;
    }

    Ok(())
}

// 初始化用户数据库示例
fn init_user_db() {
    let mut db = GLOBAL_USER_DB.lock().unwrap();
    db.insert("user1".to_string(), "password1".to_string());
}

// 在main函数中调用init_user_db来初始化用户数据库
