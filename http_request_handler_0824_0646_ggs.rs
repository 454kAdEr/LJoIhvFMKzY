use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
# 增强安全性
use std::net::SocketAddr;

#[tokio::main]
# 增强安全性
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 定义服务器要监听的地址。
# TODO: 优化性能
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // 配置服务。
# 添加错误处理
    let make_svc = make_service_fn(|_conn| async {
        // 服务函数返回一个 `service_fn`，该函数接受一个请求并产生一个响应。
        Ok::<_, Infallible>(service_fn(|req| handle_request(req)))
    });

    // 创建一个服务器，监听上述地址，并使用提供的服务。
    let server = Server::bind(&addr).serve(make_svc);

    // 运行服务器。
# 添加错误处理
    if let Err(e) = server.await {
        eprintln!("server error: {}