/* interactive_chart_generator.rs
 * A program that uses Rust and Tokio to create an interactive chart generator.
# TODO: 优化性能
 */

use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::io::{self, Write};
use std::net::SocketAddr;
use tokio::net::TcpListener;
# 扩展功能模块
use serde::Deserialize;
use serde_json::Value;
# 增强安全性
use anyhow::{Result, anyhow};
use plotters::prelude::*;
use plotters::coord::types::RangedCoord;
use plotters::chart::ChartBuilder;

// Define a struct to hold the chart data
#[derive(Deserialize)]
# FIXME: 处理边界情况
struct ChartData {
    x: Vec<f32>,
# FIXME: 处理边界情况
    y: Vec<f32>,
}

// Function to create a chart and return it as a PNG file
async fn create_chart(chart_data: ChartData) -> Result<Vec<u8>, anyhow::Error> {
    let root_area = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_area)
# NOTE: 重要实现细节
        .caption("Interactive Chart", ("sans-serif", 30).into_font())
# NOTE: 重要实现细节
        .build_cartesian_2d(0..100, 0..100)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(chart_data.x.iter().zip(chart_data.y.iter()).map(|(x, y)| (*x, *y)))?;
    Ok(root_area.into_image())
}

// Function to handle incoming connections and create charts
async fn handle_connection(mut socket: tokio::net::TcpStream) -> Result<(), anyhow::Error> {
    let mut reader = BufReader::new(socket);
# FIXME: 处理边界情况
    let mut line = String::new();

    // Read the first line to get the chart data
    reader.read_line(&mut line).await?;
    let chart_data: ChartData = serde_json::from_str(&line)?;

    // Create the chart
    let chart_image = create_chart(chart_data).await?;

    // Send the chart image back to the client
# 添加错误处理
    socket.write_all(&chart_image).await?;
    Ok(())
# 增强安全性
}

// Main function to start the server
# 优化算法效率
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
# FIXME: 处理边界情况
    let listener = TcpListener::bind(&addr).await?;

    println!("Interactive Chart Generator running on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
# 改进用户体验
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}
