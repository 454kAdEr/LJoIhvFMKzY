use tokio::net::TcpListener;
use warp::Filter;

// 使用warp库来创建RESTful API接口
#[tokio::main]
async fn main() {
    // 创建一个warp过滤器，用于处理HTTP请求
    let api = warp::path("api")
        .and(warp::get()) // 只处理GET请求
        .and_then(|| async {
            // 处理请求并返回响应
            Ok::<_, warp::Rejection>(warp::reply::json(&"Hello, world!"))
        });

    // 启动服务器监听3000端口
    let (addr, server) = warp::serve(api).bind_with_graceful_shutdown(([0, 0, 0, 0], 3000), async {
        // 等待3秒后关闭服务器
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    });
    println!("Server started at http://");
    println!("Ctrl-C to shutdown");
    server.await;
}

// 允许使用warp库的宏和函数
#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::Request;

    #[tokio::test]
    async fn test_api() {
        let api = warp::path("api")
            .and(warp::get())
            .and_then(|| async {
                Ok::<_, warp::Rejection>(warp::reply::json(&"Hello, world!"))
            });

        let response = Request::get().path("/api").reply(&api).await;
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), ""Hello, world!"");
    }
}