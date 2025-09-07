use futures::FutureExt;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

/// 模拟用户数据库
struct MockUserDatabase {
    users: HashMap<String, String>,
}

impl MockUserDatabase {
    fn new() -> Self {
        MockUserDatabase {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, username: String, password: String) {
        self.users.insert(username, password);
    }

    fn authenticate(&self, username: &str, password: &str) -> bool {
        self.users.get(username).map_or(false, |stored_password| stored_password == password)
    }
}

/// 用户认证服务
struct AuthService {
    user_db: Arc<Mutex<MockUserDatabase>>,
}

impl AuthService {
    /// 创建一个新的AuthService实例
    fn new() -> Self {
        AuthService {
            user_db: Arc::new(Mutex::new(MockUserDatabase::new())),
        }
    }

    /// 添加用户到数据库
    async fn add_user(&self, username: String, password: String) {
        let mut user_db = self.user_db.lock().await;
        user_db.add_user(username, password);
    }

    /// 用户身份认证
    async fn authenticate(&self, username: String, password: String) -> Result<bool, String> {
        let user_db = self.user_db.lock().await;
        let is_authenticated = user_db.authenticate(&username, &password);
        if is_authenticated {
            Ok(true)
        } else {
            Err("Authentication failed".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    let auth_service = AuthService::new();

    // 添加用户到数据库
    auth_service.add_user("user1".to_string(), "password123".to_string()).await;

    // 尝试身份认证
    match auth_service.authenticate("user1".to_string(), "password123".to_string()).await {
        Ok(is_authenticated) => println!("Authenticated: {}", is_authenticated),
        Err(e) => println!("Error: {}", e),
    }

    // 尝试失败的身份认证
    match auth_service.authenticate("user1".to_string(), "wrongpassword".to_string()).await {
        Ok(is_authenticated) => println!("Authenticated: {}", is_authenticated),
        Err(e) => println!("Error: {}", e),
    }
}