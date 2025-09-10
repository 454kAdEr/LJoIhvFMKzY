It is designed to be extensible and maintainable, with clear code structure
and proper error handling.
*/

use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

// Define the User struct which holds user information and permissions
#[derive(Debug, Clone)]
struct User {
    id: u32,
# FIXME: 处理边界情况
    name: String,
    permissions: Vec<String>,
# 增强安全性
}

// Define the PermissionManager which manages user permissions
struct PermissionManager {
    users: Arc<Mutex<HashMap<u32, User>>>,
}
# NOTE: 重要实现细节

impl PermissionManager {
    // Creates a new PermissionManager
    pub fn new() -> Self {
        PermissionManager {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Adds a new user with their permissions
    pub async fn add_user(&self, user_id: u32, name: String, permissions: Vec<String>) -> Result<(), String> {
        let mut users = self.users.lock().await;
        if users.contains_key(&user_id) {
            Err("User already exists.".to_string())
        } else {
            let user = User {
                id: user_id,
                name,
                permissions,
            };
            users.insert(user_id, user);
            Ok(())
        }
    }

    // Gets a user by their ID
    pub async fn get_user(&self, user_id: u32) -> Result<User, String> {
        let users = self.users.lock().await;
        users.get(&user_id).cloned().map_or_else(
            || Err("User not found.".to_string()),
            |user| Ok(user),
        )
    }

    // Updates a user's permissions
    pub async fn update_permissions(&self, user_id: u32, new_permissions: Vec<String>) -> Result<(), String> {
        let mut users = self.users.lock().await;
        if let Some(user) = users.get_mut(&user_id) {
# 改进用户体验
            user.permissions = new_permissions;
            Ok(())
        } else {
            Err("User not found.".to_string())
        }
# 添加错误处理
    }
}

#[tokio::main]
async fn main() {
    // Initialize the PermissionManager
    let permission_manager = PermissionManager::new();

    // Add a new user
    if let Err(e) = permission_manager.add_user(1, "Alice".to_string(), vec!["read".to_string(), "write".to_string()]).await {
        println!("Error: {}", e);
    } else {
        println!("User added successfully!");
    }

    // Get the user and print their permissions
    if let Ok(user) = permission_manager.get_user(1).await {
# 添加错误处理
        println!("User: {:?}, Permissions: {:?}", user.name, user.permissions);
    } else {
        println!("User not found.");
    }

    // Update the user's permissions
    if let Err(e) = permission_manager.update_permissions(1, vec!["admin".to_string()]).await {
        println!("Error: {}", e);
    } else {
        println!("Permissions updated successfully!");
    }
}
