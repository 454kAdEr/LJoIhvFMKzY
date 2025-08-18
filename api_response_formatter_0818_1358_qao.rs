use actix_web::{get, HttpResponse, Responder};
use serde_json::{json, Value};
use std::collections::HashMap;

// 定义响应体结构
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    status: String,
    data: Value,
    message: String,
}

// ApiResponseFormatter 是一个用于处理API响应的组件
#[derive(Debug)]
struct ApiResponseFormatter;

impl ApiResponseFormatter {
    // 构造一个ApiResponse实例
    fn new(data: Value, message: String) -> ApiResponse {
        ApiResponse {
            status: "success".to_string(),
            data,
            message,
        }
    }

    // 格式化响应体
    #[allow(dead_code)]
    fn format_response(&self, response: ApiResponse) -> impl Responder {
        HttpResponse::Ok().json(response)
    }
}

// 定义一个简单的API，使用ApiResponseFormatter来格式化响应
#[get("/")]
async fn index() -> impl Responder {
    let data = json!({
        "key": "value"
    });
    let message = "Hello, world!".to_string();
    let formatter = ApiResponseFormatter;
    let response = formatter.new(data, message);
    formatter.format_response(response)
}

// 程序入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置日志级别为Info
    env_logger::builder().format_timestamp_millis().init();

    // 启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
