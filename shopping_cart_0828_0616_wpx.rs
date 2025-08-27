use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task;
use serde::{Serialize, Deserialize};

// 定义商品结构体
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

// 定义购物车项结构体
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CartItem {
    product_id: u32,
    quantity: u32,
}

// 购物车服务
struct ShoppingCart {
    items: HashMap<u32, CartItem>,
}

impl ShoppingCart {
    // 创建一个新的购物车
    fn new() -> Self {
        ShoppingCart {
            items: HashMap::new(),
        }
    }

    // 添加商品到购物车
    async fn add_product(&mut self, product_id: u32, quantity: u32) {
        let cart_item = self.items.entry(product_id).or_insert_with(|| CartItem {
            product_id,
            quantity: 0,
        });
        cart_item.quantity += quantity;
    }

    // 从购物车移除商品
    async fn remove_product(&mut self, product_id: u32, quantity: u32) -> Result<(), String> {
        if let Some(cart_item) = self.items.get_mut(&product_id) {
            if cart_item.quantity < quantity {
                return Err("Insufficient quantity in cart".to_string());
            }
            cart_item.quantity -= quantity;
            if cart_item.quantity == 0 {
                self.items.remove(&product_id);
            }
            Ok(())
        } else {
            Err("Product not found in cart".to_string())
        }
    }

    // 获取购物车中的商品总数
    async fn total_items(&self) -> u32 {
        self.items.values().map(|item| item.quantity).sum()
    }
}

// 主函数
#[tokio::main]
async fn main() {
    let mut cart = ShoppingCart::new();
    let product_id = 1;
    let quantity = 2;

    // 添加商品到购物车
    cart.add_product(product_id, quantity).await;

    // 尝试移除商品
    match cart.remove_product(product_id, quantity).await {
        Ok(_) => println!("Product removed from cart"),
        Err(e) => println!("Error: {}", e),
    }

    // 打印购物车中的商品总数
    println!("Total items in cart: {}", cart.total_items().await);
}
