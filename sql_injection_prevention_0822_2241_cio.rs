use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::sync::Arc;
use anyhow::Result;

// 配置数据库连接池
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() -> Result<()> {
    // 从环境变量中获取数据库连接信息
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建数据库连接池
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // 监听TCP连接
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on localhost:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(socket, pool.clone()).await;
        });
    }
}

// 处理客户端请求
async fn handle_client(mut socket: TcpStream, pool: Arc<DbPool>) {
    let mut buf = [0; 1024];
    match socket.read(&mut buf).await {
        Ok(n) if n == 0 => return,
        Ok(_) => println!("Received: {:?}", &buf[..]),
        Err(e) => {
            eprintln!("Failed to read from socket; err = {:?}