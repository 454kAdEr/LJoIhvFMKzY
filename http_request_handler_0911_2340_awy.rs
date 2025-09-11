use tokio::net::TcpListener;
use hyper::{service::{make_service_fn, service_fn}, Server};
use hyper::Request;
# 增强安全性
use hyper::Response;
use hyper::Body;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::task::{Context, Poll};

// 定义一个简单的请求处理器
async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // 这里可以根据实际需求处理请求，并返回响应
    let res = Response::new(Body::from("Hello, World!"));
    Ok(res)
}

// 定义一个服务，用于处理请求
async fn service_fn_svc(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    handle_request(req).await
}

// 定义一个异步函数，用于启动HTTP服务器
async fn run_server(addr: SocketAddr) {
    let make_svc = make_service_fn(|_conn| {
        // 提供一个服务，用来处理每个连接
        async {
            Ok::<_, Infallible>(service_fn(service_fn_svc))
# 优化算法效率
        }
# TODO: 优化性能
    });

    let server = Server::bind(&addr).serve(make_svc);
    // 启动服务器
    if let Err(e) = server.await {
        eprintln!("server error: {}