use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::Result;
# FIXME: 处理边界情况
use std::net::SocketAddr;
use std::collections::HashMap;
# NOTE: 重要实现细节
use std::fmt::Write;
# NOTE: 重要实现细节

// 定义一个结构体来存储响应内容
# NOTE: 重要实现细节
struct Response {
# TODO: 优化性能
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

// 定义一个结构体来表示服务器
struct Server {
    addr: SocketAddr,
    listener: TcpListener,
}

impl Server {
    // 创建一个新的服务器实例
    async fn new(addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Server { addr, listener })
    }

    // 处理客户端连接
    async fn handle_client(&self, mut socket: tokio::net::TcpStream) -> Result<()> {
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await?;
        let request = match String::from_utf8(buf[..n].to_vec()) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        };

        // 模拟响应式布局的逻辑
        let response = self.responsive_layout(&request);

        // 发送响应
        let response_bytes = response.into_bytes();
        socket.write_all(&response_bytes).await?;
        socket.shutdown().await?;
        Ok(())
    }

    // 模拟响应式布局的处理逻辑
# 增强安全性
    fn responsive_layout(&self, request: &str) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());

        let mut body = "<html><head><title>Responsive Layout</title></head>
".to_string();
        body.push_str("<body>
");
        body.push_str("<p>Your request was: <strong>");
        write!(body, "{}", request).unwrap();
        body.push_str("</strong></p>
");
        body.push_str("<p>This is a responsive layout example.</p>
");
        body.push_str("</body></html>
");
# 改进用户体验

        Response {
            status_code: 200,
            headers,
# NOTE: 重要实现细节
            body,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Server::new(addr).await?;

    loop {
# 优化算法效率
        let (socket, _) = server.listener.accept().await?;
        tokio::spawn(async move {
            server.handle_client(socket).await;
        });
    }
}
