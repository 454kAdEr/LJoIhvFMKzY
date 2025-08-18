// math_toolkit.rs
// Rust program that provides a set of mathematical calculation tools using Tokio framework.

use std::error::Error;
use tokio::task;

// Define a struct for the MathToolkit to hold any necessary state.
struct MathToolkit;

impl MathToolkit {
    // Add method to calculate the sum of two numbers.
    async fn add(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
        Ok(a + b)
    }

    // Subtract method to calculate the difference of two numbers.
    async fn subtract(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
        Ok(a - b)
    }

    // Multiply method to calculate the product of two numbers.
    async fn multiply(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
        Ok(a * b)
    }

    // Divide method to calculate the quotient of two numbers.
    async fn divide(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
        if b == 0 {
            Err("Cannot divide by zero".to_string().into())
        } else {
            Ok(a / b)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let toolkit = MathToolkit;

    // Perform calculations using the toolkit.
    let sum = task::spawn_blocking(move || toolkit.add(10, 5).await).await??;
    println!("Sum: {}", sum);

    let difference = task::spawn_blocking(move || toolkit.subtract(10, 5).await).await??;
    println!("Difference: {}", difference);

    let product = task::spawn_blocking(move || toolkit.multiply(10, 5).await).await??;
    println!("Product: {}", product);

    let quotient = task::spawn_blocking(move || toolkit.divide(10, 5).await).await??;
    println!("Quotient: {}", quotient);

    Ok(())
}
