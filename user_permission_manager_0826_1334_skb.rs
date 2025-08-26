use std::collections::HashMap;
use tokio;
use tokio::sync::Mutex;
use std::sync::Arc;

// 定义用户结构体
#[derive(Debug, Clone)]
struct User {
    username: String,
    permissions: Vec<String>,
}

// 定义权限管理器
struct PermissionManager {
    users: Mutex<HashMap<String, User>>,
}

impl PermissionManager {
    // 创建新的权限管理器
    fn new() -> Self {
        PermissionManager {
            users: Mutex::new(HashMap::new()),
        }
    }

    // 添加用户
    async fn add_user(&self, username: String, permissions: Vec<String>) -> Result<(), String> {
        let mut users = self.users.lock().await;
        if users.contains_key(&username) {
            Err("User already exists".to_string())
        } else {
            let user = User {
                username: username.clone(),
                permissions,
            };
            users.insert(username, user);
            Ok(())
        }
    }

    // 删除用户
    async fn remove_user(&self, username: &str) -> Result<(), String> {
        let mut users = self.users.lock().await;
        if users.remove(username).is_some() {
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    // 检查用户权限
    async fn check_permission(&self, username: &str, permission: &str) -> Result<bool, String> {
        let users = self.users.lock().await;
        if let Some(user) = users.get(username) {
            Ok(user.permissions.contains(permission))
        } else {
            Err("User not found".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    let manager = Arc::new(PermissionManager::new());

    // 添加用户
    let user1 = manager.clone();
    tokio::spawn(async move {
        if let Err(e) = user1.add_user("alice".to_string(), vec!["read".to_string(), "write".to_string()]).await {
            eprintln!("Error: {}", e);
        }
    });

    // 删除用户
    let user2 = manager.clone();
    tokio::spawn(async move {
        if let Err(e) = user2.remove_user("alice").await {
            eprintln!("Error: {}", e);
        }
    });

    // 检查用户权限
    let user3 = manager.clone();
    tokio::spawn(async move {
        match user3.check_permission("alice", "read").await {
            Ok(true) => println!("User has read permission"),
            Ok(false) => println!("User does not have read permission"),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
}
