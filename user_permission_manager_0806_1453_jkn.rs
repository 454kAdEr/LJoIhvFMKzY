use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

// 定义用户权限枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Permission {
    Read,
    Write,
    Execute,
}

// 用户结构体，包含用户名和权限
struct User {
    username: String,
    permissions: Vec<Permission>,
}

// 用户权限管理器
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
    async fn add_user(&self, username: String, permissions: Vec<Permission>) -> Result<(), String> {
        let mut users = self.users.lock().await;
        if users.contains_key(&username) {
            Err("User already exists.".to_string())
        } else {
            users.insert(username, User {
                username: username.clone(),
                permissions,
            });
            Ok(())
        }
    }

    // 移除用户
    async fn remove_user(&self, username: &str) -> Result<(), String> {
        let mut users = self.users.lock().await;
        if users.remove(username).is_some() {
            Ok(())
        } else {
            Err("User not found.".to_string())
        }
    }

    // 检查用户是否存在
    async fn user_exists(&self, username: &str) -> bool {
        let users = self.users.lock().await;
        users.contains_key(username)
    }

    // 检查用户是否有特定权限
    async fn has_permission(&self, username: &str, permission: Permission) -> bool {
        let users = self.users.lock().await;
        if let Some(user) = users.get(username) {
            user.permissions.contains(&permission)
        } else {
            false
        }
    }
}

#[tokio::main]
async fn main() {
    let manager = Arc::new(PermissionManager::new());

    // 添加用户
    match manager.add_user("alice".to_string(), vec![Permission::Read, Permission::Write]).await {
        Ok(_) => println!("User added successfully."),
        Err(e) => println!("Error: {}", e),
    }

    // 检查用户是否存在
    if manager.user_exists("alice").await {
        println!("User exists.")
    } else {
        println!("User does not exist.")
    }

    // 检查用户是否有特定权限
    if manager.has_permission("alice", Permission::Read).await {
        println!("User has read permission.")
    } else {
        println!("User does not have read permission.")
    }

    // 移除用户
    match manager.remove_user("alice").await {
        Ok(_) => println!("User removed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}
