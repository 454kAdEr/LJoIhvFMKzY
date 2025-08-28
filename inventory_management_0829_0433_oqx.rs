use tokio::sync::{Mutex, mpsc};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

// InventoryError 枚举定义了可能发生的错误类型
#[derive(Debug)]
enum InventoryError {
    ItemNotFound(String),
    InsufficientStock(String),
}

// 实现 `std::fmt::Display` 为 InventoryError 添加信息输出功能
impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::ItemNotFound(item) => write!(f, "Item not found: \x22{}\x22", item),
            InventoryError::InsufficientStock(item) => write!(f, "Insufficient stock for item: \x22{}\x22", item),
        }
    }
}

// 实现 `std::error::Error` 为 InventoryError 提供错误处理功能
impl Error for InventoryError {}

// InventoryItem 结构体定义库存条目
struct InventoryItem {
    id: String,
    name: String,
    quantity: i32,
}

// Inventory 结构体定义库存管理系统
struct Inventory {
    items: HashMap<String, InventoryItem>,
    // 使用互斥锁来保证线程安全
    lock: Mutex<()>,
}

impl Inventory {
    // 创建一个新的 Inventory 实例
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
            lock: Mutex::new(()),
        }
    }

    // 添加库存条目
    async fn add_item(&self, item: InventoryItem) -> Result<(), InventoryError> {
        let mut items = self.lock.lock().await;
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    // 获取库存条目
    async fn get_item(&self, id: &str) -> Result<InventoryItem, InventoryError> {
        let items = self.lock.lock().await;
        self.items
            .get(id)
            .cloned()
            .ok_or(InventoryError::ItemNotFound(id.to_string()))
    }

    // 更新库存条目
    async fn update_item(&self, id: &str, quantity: i32) -> Result<(), InventoryError> {
        let mut items = self.lock.lock().await;
        if let Some(item) = self.items.get_mut(id) {
            if quantity > 0 {
                item.quantity += quantity;
            } else {
                return Err(InventoryError::InsufficientStock(id.to_string()));
            }
        } else {
            return Err(InventoryError::ItemNotFound(id.to_string()));
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let inventory = Arc::new(Inventory::new());
    let (tx, mut rx) = mpsc::channel(100);

    // 添加库存条目
    let item1 = InventoryItem {
        id: "1".to_string(),
        name: "Apple".to_string(),
        quantity: 10,
    };
    inventory.add_item(item1).await.unwrap();

    // 获取库存条目
    let item = inventory.get_item("1").await.unwrap();
    println!("Item: \x22{}\x22, Quantity: \x22{}", item.name, item.quantity);

    // 更新库存条目
    inventory.update_item("1", -2).await.unwrap();
    let item = inventory.get_item("1").await.unwrap();
    println!("Updated Item: \x22{}\x22, Quantity: \x22{}", item.name, item.quantity);

    Ok(())
}