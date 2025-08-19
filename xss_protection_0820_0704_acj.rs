// xss_protection.rs

// 引入必要的库
use actix_web::{get, HttpResponse, Responder};
use regex::Regex;
use std::collections::HashSet;
use once_cell::sync::Lazy;

// 定义全局的转义HTML字符集合
static HTML_ESCAPE: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.extend(&['"', "'", "<", ">", "&", "/"]);
    s
});

// 定义一个函数来转义HTML字符，防止XSS攻击
fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        if HTML_ESCAPE.contains(c.escape_unicode().to_string().as_str()) {
            format!("&#x{:X};", c as u32)
        } else {
            c.to_string()
        }
    }).collect()
}

// 实现Actix Web的handler来处理请求
#[get("/")]
async fn index() -> impl Responder {
    let user_input = "<script>alert('xss');</script>";
    // 使用转义函数来防止XSS攻击
    let safe_input = escape_html(user_input);

    // 返回响应，显示转义后的输入
    HttpResponse::Ok().body(safe_input)
}

// main函数启动Actix Web服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动路由到index的get请求
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", actix_web::web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
