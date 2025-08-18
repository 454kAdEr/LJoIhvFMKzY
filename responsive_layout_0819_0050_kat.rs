use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::rt::Future;
use hyper::header::{CONTENT_TYPE};
use hyper::body::HttpBody;
use serde::Serialize;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

// 定义响应式布局的参数结构体
#[derive(Serialize, Clone)]
struct LayoutParams {
    breakpoint: f64,
    columns: u32,
    gap: f64,
}

// 创建一个线程安全的响应式布局参数结构体
type SharedParams = Arc<Mutex<LayoutParams>>;

// Hyper服务处理函数
async fn handle_request(req: Request<Body>, params: SharedParams) -> Result<Response<Body>, hyper::Error> {
    let params = params.lock().await;

    let layout = match params.breakpoint {
        0.0..=599.0 => "Mobile layout",
        600.0..=767.0 => "Tablet layout",
        768.0..=1199.0 => "Desktop layout",
        _ => "Large desktop layout",
    };

    let response_body = format!("<html><body>Current layout: {}\</body></html>", layout);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(response_body))?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 设置响应式布局参数
    let params = LayoutParams {
        breakpoint: 768.0,
        columns: 12,
        gap: 15.0,
    };
    let shared_params = Arc::new(Mutex::new(params));

    // 创建Hyper服务器
    let make_svc = make_service_fn(|_conn| {
        let shared_params = shared_params.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, shared_params.clone())))
        }
    });

    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}