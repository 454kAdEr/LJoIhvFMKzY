use tokio;
use std::collections::HashMap;
use std::sync::Mutex;

// 模拟数据库用户存储
lazy_static::lazy_static! {
    static ref USERS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 用户登录请求结构体
#[derive(Debug, Clone)]
struct LoginRequest {
    username: String,
    password: String,
}

// 异步处理用户登录请求
async fn handle_login(request: LoginRequest) -> Result<String, String> {
    // 从模拟数据库中获取用户密码
    let users_lock = USERS.lock().unwrap();
    let user_password = users_lock.get(&request.username).cloned();

    if user_password.is_none() {
        return Err("User not found".to_string());
    }

    let user_password = user_password.unwrap();
    if user_password != request.password {
        return Err("Incorrect password".to_string());
    }

    Ok("Login successful".to_string())
}

#[tokio::main]
async fn main() {
    // 初始化模拟数据库
    {
        let mut users_lock = USERS.lock().unwrap();
        users_lock.insert("user123".to_string(), "pass123".to_string());
    }

    // 模拟登录请求
    let login_requests = vec![
        LoginRequest { username: "user123".to_string(), password: "pass123".to_string() },
        LoginRequest { username: "user123".to_string(), password: "wrongpassword".to_string() },
    ];

    for request in login_requests {
        match handle_login(request).await {
            Ok(message) => println!("{}: {}", request.username, message),
            Err(error) => println!("{}: {}", request.username, error),
        }
    }
}

// 启用lazy_static宏
#[macro_use]
extern crate lazy_static;