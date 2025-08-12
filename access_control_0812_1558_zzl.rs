use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
# 扩展功能模块
use lazy_static::lazy_static;
use regex::Regex;

// 定义全局的访问控制列表，使用RwLock确保线程安全
lazy_static! {
# 优化算法效率
    static ref ACCESS_CONTROL: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
}

// 定义一个函数来添加到访问控制列表
pub fn add_to_access_control(user_id: &str) {
    let mut access_control = ACCESS_CONTROL.write().unwrap();
    access_control.insert(user_id.to_string());
}
# TODO: 优化性能

// 定义一个函数来检查用户ID是否在访问控制列表中
pub async fn check_access(user_id: &str) -> bool {
    let access_control = ACCESS_CONTROL.read().unwrap();
    access_control.contains(user_id)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 监听TCP端口
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on port 8080");

    // 将访问控制列表中的用户ID添加到HashSet中
    add_to_access_control("user123");
    add_to_access_control("user456");

    loop {
        let (mut socket, _) = listener.accept().await?;
        handle_client(socket).await;
    }
# NOTE: 重要实现细节
}

// 定义处理客户端连接的异步函数
async fn handle_client(mut socket: tokio::net::TcpStream) {
    // 读取客户端发送的数据，这里简单假定为用户ID
    let mut buffer = [0; 1024];
    let nbytes = socket.read(&mut buffer).await.unwrap();
    let user_id = String::from_utf8_lossy(&buffer[..nbytes]);

    // 使用正则表达式检查用户ID格式
    let re = Regex::new(r"^user\d+$").unwrap();
    if re.is_match(&user_id) {
        if check_access(&user_id).await {
# FIXME: 处理边界情况
            socket.write_all(b"Access granted
").await.unwrap();
        } else {
            socket.write_all(b"Access denied
").await.unwrap();
        }
    } else {
        socket.write_all(b"Invalid user ID
").await.unwrap();
    }
}
# 优化算法效率
