use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

/// 一个简单的库存管理系统
struct InventoryManager {
    /// 存储库存项
    items: Arc<Mutex<HashMap<String, i32>>>,
}

impl InventoryManager {
    /// 创建一个新的库存管理系统
    pub fn new() -> Self {
        InventoryManager {
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 添加库存项
    pub async fn add_item(&self, item: String, quantity: i32) -> Result<(), String> {
        let mut items = self.items.lock().await;
        if quantity < 0 {
            return Err("Quantity cannot be negative".to_string());
        }
        items.entry(item).and_modify(|e| *e += quantity).or_insert(quantity);
        Ok(())
    }

    /// 减少库存项
    pub async fn remove_item(&self, item: String, quantity: i32) -> Result<(), String> {
        let mut items = self.items.lock().await;
        if quantity < 0 {
            return Err("Quantity cannot be negative".to_string());
        }
        if let Some(count) = items.get_mut(&item) {
            if *count < quantity {
                return Err("Insufficient quantity in inventory".to_string());
            }
            *count -= quantity;
            if *count == 0 {
                items.remove(&item);
            }
        } else {
            return Err("Item not found in inventory".to_string());
        }
        Ok(())
    }

    /// 获取库存项的详细信息
    pub async fn get_inventory(&self) -> HashMap<String, i32> {
        let items = self.items.lock().await;
        items.clone()
    }
}

#[tokio::main]
async fn main() {
    let manager = InventoryManager::new();

    // 添加一些库存项
    manager.add_item("Apple".to_string(), 100).await.unwrap();
    manager.add_item("Banana".to_string(), 50).await.unwrap();
    // 减少一些库存项
    manager.remove_item("Apple".to_string(), 20).await.unwrap();
    // 打印当前库存
    let inventory = manager.get_inventory().await;
    println!("Current inventory: {:?}