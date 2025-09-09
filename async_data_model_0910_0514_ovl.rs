use tokio;
use serde::{Serialize, Deserialize};
use std::error::Error;

// 定义一个简单的用户数据模型
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 定义一个数据存储接口
trait DataStore {
    fn save(&self, user: User) -> Result<(), Box<dyn Error>>;
    fn find(&self, id: u32) -> Result<User, Box<dyn Error>>;
}

// 实现一个内存中的数据存储
struct InMemoryDataStore {
    users: Vec<User>,
}

impl InMemoryDataStore {
    pub fn new() -> Self {
        InMemoryDataStore { users: Vec::new() }
    }
}

impl DataStore for InMemoryDataStore {
    fn save(&self, user: User) -> Result<(), Box<dyn Error>> {
        self.users.push(user);
        Ok(())
    }

    fn find(&self, id: u32) -> Result<User, Box<dyn Error>> {
        for user in &self.users {
            if user.id == id {
                return Ok(user.clone());
            }
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found")))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let store = InMemoryDataStore::new();

    let user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };

    // 保存用户
    store.save(user.clone())?;

    // 查找用户
    let found = store.find(1)?;
    println!("Found user: {:?}", found);

    Ok(())
}
