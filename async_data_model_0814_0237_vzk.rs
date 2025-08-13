use serde::{Deserialize, Serialize};
use serde_json::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// 数据模型
# NOTE: 重要实现细节
#[derive(Serialize, Deserialize, Debug)]
# 扩展功能模块
struct User {
    id: u32,
    name: String,
    email: String,
}

// 异步保存用户的函数
async fn save_user(user: &User) -> Result<()> {
    // 将User对象序列化为JSON字符串
    let user_json = serde_json::to_string(user)?;

    // 打开或创建文件
    let mut file = File::create("users.json").await?;
# 增强安全性

    // 将JSON字符串写入文件
    file.write_all(user_json.as_bytes()).await?;

    // 刷新文件缓冲区以确保数据写入磁盘
    file.flush().await?;
# FIXME: 处理边界情况

    Ok(())
}

// 异步从文件加载用户的函数
async fn load_users() -> Result<Vec<User>> {
    // 读取文件内容
    let mut file = File::open("users.json").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // 将JSON字符串反序列化为User对象的Vec
    let users: Vec<User> = serde_json::from_str(&contents)?;

    Ok(users)
# TODO: 优化性能
}

#[tokio::main]
# FIXME: 处理边界情况
async fn main() -> Result<()> {
    // 创建一个用户示例
    let user = User {
        id: 1,
        name: "John Doe".to_string(),
# 改进用户体验
        email: "john.doe@example.com".to_string(),
    };
# 优化算法效率

    // 保存用户到文件
    save_user(&user).await?;

    // 加载用户列表
    let users = load_users().await?;

    // 打印加载的用户列表
    for user in users {
        println!("User: {:?}", user);
    }

    Ok(())
# 优化算法效率
}
