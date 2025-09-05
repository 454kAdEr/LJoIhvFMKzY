 * This program demonstrates how to create a shopping cart service with basic operations:
 * adding items, removing items, and viewing the cart's contents.
 */

use std::collections::HashMap;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

// Define the structure for an item in the cart
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CartItem {
    id: u32, // Unique identifier for the item
    name: String, // Name of the item
    quantity: u32, // Quantity of the item
}

// Define the shopping cart structure
#[derive(Debug)]
struct ShoppingCart {
    items: HashMap<u32, CartItem>,
}

impl ShoppingCart {
    // Create a new shopping cart
    fn new() -> ShoppingCart {
        ShoppingCart {
            items: HashMap::new(),
        }
    }

    // Add an item to the cart
    fn add_item(&mut self, item: CartItem) {
        self.items.insert(item.id, item);
    }

    // Remove an item from the cart
    fn remove_item(&mut self, item_id: u32) {
        self.items.remove(&item_id);
    }

    // Get a copy of the cart's items
    fn get_items(&self) -> Vec<CartItem> {
        self.items.values().cloned().collect()
    }
}

// Define the shopping cart service with async operations
struct ShoppingCartService {
    cart: Mutex<ShoppingCart>,
}

impl ShoppingCartService {
    // Create a new shopping cart service
    fn new() -> ShoppingCartService {
        ShoppingCartService {
            cart: Mutex::new(ShoppingCart::new()),
        }
    }

    // Add an item to the shopping cart asynchronously
    async fn add_to_cart(&self, item: CartItem) -> Result<Value, String> {
        let mut cart = self.cart.lock().await;
        cart.add_item(item);
        Ok(json!({ "message": "Item added successfully" }))
    }

    // Remove an item from the shopping cart asynchronously
    async fn remove_from_cart(&self, item_id: u32) -> Result<Value, String> {
        let mut cart = self.cart.lock().await;
        cart.remove_item(item_id);
        Ok(json!({ "message": "Item removed successfully" }))
    }

    // Get the current shopping cart's items asynchronously
    async fn get_cart(&self) -> Result<Value, String> {
        let cart = self.cart.lock().await;
        let items = cart.get_items();
        Ok(serde_json::to_value(items).map_err(|e| e.to_string()))
    }
}

#[tokio::main]
async fn main() {
    // Create a new shopping cart service
    let cart_service = ShoppingCartService::new();

    // Add some items to the cart
    let item1 = CartItem { id: 1, name: "Apple".to_string(), quantity: 2 };
    let item2 = CartItem { id: 2, name: "Banana".to_string(), quantity: 3 };
    match cart_service.add_to_cart(item1).await {
        Ok(_) => println!("Item 1 added"),
        Err(e) => println!("Error adding item 1: {}", e),
    }
    match cart_service.add_to_cart(item2).await {
        Ok(_) => println!("Item 2 added"),
        Err(e) => println!("Error adding item 2: {}", e),
    }

    // Get the cart's items and display them
    match cart_service.get_cart().await {
        Ok(items) => println!("Cart items: {}", items),
        Err(e) => println!("Error getting cart items: {}", e),
    }
}
