use tokio::sync::Mutex;
use std::collections::HashMap;
# TODO: 优化性能
use std::sync::Arc;
use async_trait::async_trait;
use rand::{distributions::Alphanumeric, Rng};
# NOTE: 重要实现细节
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::Filter;

// 定义用户模型
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

// 定义登录请求模型
#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginRequest {
    username: String,
    password: String,
}

// 定义登录响应模型
# 扩展功能模块
#[derive(Serialize, Deserialize, Debug, Clone)]
# 扩展功能模块
struct LoginResponse {
    success: bool,
# 扩展功能模块
    token: Option<String>,
}

// 模拟数据库中的用户信息
#[derive(Clone)]
struct UserRepository {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl UserRepository {
    fn new() -> Self {
        let users = Arc::new(Mutex::new(HashMap::new()));
        let mut repo = UserRepository { users };
        // 预填充一些用户数据
# 优化算法效率
        repo.add_user(User {
            username: "admin".to_string(),
            password: "password123".to_string(),
        });
        repo
    }

    fn add_user(&self, user: User) {
        let mut users = self.users.lock().await;
        users.insert(user.username.clone(), user);
    }

    async fn authenticate(&self, username: &str, password: &str) -> Option<String> {
        let users = self.users.lock().await;
        users
            .get(username)
            .and_then(|user| {
                if user.password == password {
                    Some(self.generate_token(&user.username))
                } else {
                    None
                }
            })
    }

    fn generate_token(&self, username: &str) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
# 添加错误处理
            .map(char::from)
# 添加错误处理
            .collect()
    }
}

// 定义登录服务
#[async_trait]
trait LoginService {
    async fn login(&self, login_request: &LoginRequest) -> LoginResponse;
}
# NOTE: 重要实现细节

struct LoginServiceImpl;

#[async_trait]
impl LoginService for LoginServiceImpl {
    async fn login(&self, login_request: &LoginRequest) -> LoginResponse {
        let user_repository = UserRepository::new();
        match user_repository.authenticate(&login_request.username, &login_request.password).await {
            Some(token) => LoginResponse {
                success: true,
                token: Some(token),
# FIXME: 处理边界情况
            },
            None => LoginResponse {
# NOTE: 重要实现细节
                success: false,
                token: None,
            },
        }
    }
}

// 定义_warp_路由
# NOTE: 重要实现细节
fn login_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("login"))
# 改进用户体验
        .and(warp::body::json())
        .and(with_user_repository())
# FIXME: 处理边界情况
        .and_then(|login_request: LoginRequest, user_repo: UserRepository| async move {
            let login_service = LoginServiceImpl;
            let response = login_service.login(&login_request).await;
            warp::reply::json(&response)
        })
}

// 将UserRepository注入到路由处理中
async fn with_user_repository() -> UserRepository {
    UserRepository::new()
}

#[tokio::main]
async fn main() {
    let user_repo = UserRepository::new();

    let routes = login_routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
# FIXME: 处理边界情况
}
