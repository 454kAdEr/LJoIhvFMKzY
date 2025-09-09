// 引入必要的库
#[macro_use]
extern crate log;
extern crate tokio;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use log::info;

// 定义用户角色枚举
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum UserRole {
    Admin,
    User,
    Guest,
}

// 定义用户结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    role: UserRole,
}

// 用户权限管理器
struct PermissionManager {
    users: Mutex<HashMap<u32, User>>,
}

impl PermissionManager {
    // 创建新的权限管理器
    pub fn new() -> Self {
        PermissionManager {
            users: Mutex::new(HashMap::new()),
        }
    }

    // 添加用户
    pub async fn add_user(&self, user: User) -> Result<(), String> {
        let mut users = self.users.lock().await;
        if users.contains_key(&user.id) {
            Err("User already exists".to_string())
        } else {
            users.insert(user.id, user);
            Ok(())
        }
    }

    // 检查用户权限
    pub async fn check_permission(&self, user_id: u32, required_role: UserRole) -> Result<bool, String> {
        let users = self.users.lock().await;
        match users.get(&user_id) {
            Some(user) => Ok(user.role >= required_role),
            None => Err("User not found".to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    // 初始化日志
    println!("User Permission System");
    let permission_manager = PermissionManager::new();

    // 添加用户
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        role: UserRole::Admin,
    };
    if let Err(e) = permission_manager.add_user(user).await {
        eprintln!("Error adding user: {}", e);
        return;
    }

    // 检查权限
    match permission_manager.check_permission(1, UserRole::User).await {
        Ok(permission) => info!("Permission granted: {}", permission),
        Err(e) => eprintln!("Error checking permission: {}", e),
    }
}
