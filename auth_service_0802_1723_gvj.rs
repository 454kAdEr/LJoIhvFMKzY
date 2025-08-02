// 使用必要的外部库和模块
use tokio;
use serde::Serialize;
use serde_json;
use std::sync::Arc;
use warp::Filter;

// 定义用户结构体，包含用户名和密码
#[derive(Serialize, Debug)]
struct User {
    username: String,
    password: String,
}

// 定义登录请求结构体
#[derive(Serialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

// 定义登录响应结构体
#[derive(Serialize, Debug)]
struct LoginResponse {
    success: bool,
    message: String,
}

// 模拟的用户数据库
static DB: Arc<[User]> = Arc::new(
    vec![
        User {
            username: "admin".to_string(),
            password: "password123".to_string(),
        },
    ],
);

// 异步函数，用于处理登录请求
async fn login(
    login_request: LoginRequest,
) -> Result<LoginResponse, warp::Rejection> {
    // 在模拟的数据库中查找用户
    for user in DB.iter() {
        if user.username == login_request.username && user.password == login_request.password {
            return Ok(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
            });
        }
    }
    // 如果用户不存在或密码错误，则返回错误
    Ok(LoginResponse {
        success: false,
        message: "Invalid username or password".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // 设置 warp 的日志过滤器
    let log = warp::log("auth_service");
    // 设置登录路由
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|login_request: LoginRequest| async move {
            match login(login_request).await {
                Ok(response) => {
                    warp::reply::json(&response)
                },
                Err(_) => {
                    warp::reject::not_found()
                },
            }
        });
    // 开始监听和处理请求
    warp::serve(log.and(login_route)).run(([127, 0, 0, 1], 3030)).await;
}
