use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;

/// InventoryItem represents an item in the inventory with its quantity.
#[derive(Clone, Debug)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

/// InventoryManager manages the inventory items.
#[derive(Clone)]
struct InventoryManager {
    items: Arc<Mutex<HashMap<u32, InventoryItem>>>,
}

impl InventoryManager {
    /// Creates a new InventoryManager.
    pub fn new() -> Self {
        InventoryManager {
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a new item to the inventory.
    pub async fn add_item(&self, item: InventoryItem) -> Result<()> {
        let mut items = self.items.lock().await;
        items.insert(item.id, item);
        Ok(())
    }

    /// Removes an item from the inventory.
    pub async fn remove_item(&self, item_id: u32) -> Result<()> {
        let mut items = self.items.lock().await;
        if items.remove(&item_id).is_some() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Item with id {} not found", item_id))
        }
    }

    /// Updates the quantity of an existing item in the inventory.
    pub async fn update_quantity(&self, item_id: u32, quantity: u32) -> Result<()> {
        let mut items = self.items.lock().await;
        if let Some(item) = items.get_mut(&item_id) {
            item.quantity = quantity;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Item with id {} not found", item_id))
        }
    }

    /// Retrieves an item from the inventory by its ID.
    pub async fn get_item(&self, item_id: u32) -> Result<InventoryItem> {
        let items = self.items.lock().await;
        items.get(&item_id).cloned().ok_or_else(|| anyhow::anyhow!("Item with id {} not found", item_id))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let manager = InventoryManager::new();
    
    // Adding items to the inventory.
    manager.add_item(InventoryItem { id: 1, name: "Apple".to_string(), quantity: 10 }).await?;
    manager.add_item(InventoryItem { id: 2, name: "Banana".to_string(), quantity: 20 }).await?;
    
    // Updating an item's quantity.
    manager.update_quantity(1, 15).await?;
    
    // Retrieving an item.
    let item = manager.get_item(1).await?;
    println!("Retrieved item: {:?}", item);
    
    // Removing an item.
    manager.remove_item(2).await?;
    
    Ok(())
}