// shopping_cart.rs
// This module provides a basic shopping cart implementation using Rust and Tokio.

use std::collections::HashMap;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use serde_json;

// Define the structure of a product.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

// Define the shopping cart.
struct ShoppingCart {
    items: HashMap<u32, (Product, u32)>, // (Product ID, (Product, quantity))
}

impl ShoppingCart {
    // Create a new shopping cart.
    pub fn new() -> Self {
        ShoppingCart {
            items: HashMap::new(),
        }
    }

    // Add a product to the cart.
    pub fn add_product(&mut self, product: Product, quantity: u32) {
        let entry = self.items.entry(product.id).or_insert((product, 0));
        entry.1 .1 += quantity;
    }

    // Remove a product from the cart.
    pub fn remove_product(&mut self, product_id: u32, quantity: u32) -> Result<(), String> {
        if let Some((_, count)) = self.items.get_mut(&product_id) {
            if *count >= quantity {
                *count -= quantity;
                if *count == 0 {
                    self.items.remove(&product_id);
                }
                Ok(())
            } else {
                Err("Not enough quantity in cart".to_string())
            }
        } else {
            Err("Product not found in cart".to_string())
        }
    }

    // Get the total price of the cart.
    pub fn total_price(&self) -> f64 {
        self.items.iter().map(|(_, (product, quantity))| {
            product.price * (*quantity) as f64
        }).sum()
    }
}

#[tokio::main]
async fn main() {
    // Create a new shopping cart.
    let mut cart = ShoppingCart::new();

    // Add products to the cart.
    cart.add_product(Product { id: 1, name: "Apple".to_string(), price: 0.5 }, 2);
    cart.add_product(Product { id: 2, name: "Banana".to_string(), price: 0.3 }, 3);

    // Print the total price of the cart.
    println!("Total price: {:.2}", cart.total_price());

    // Remove a product from the cart.
    if let Err(e) = cart.remove_product(2, 2) {
        eprintln!("Error: {}", e);
    }

    // Print the total price after removal.
    println!("Total price after removal: {:.2}", cart.total_price());
}
