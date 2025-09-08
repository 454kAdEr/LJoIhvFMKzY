use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tokio::task;
use serde_json;
use plotters::prelude::*;

// 定义一个全局的图表数据结构
struct ChartData {
    data: HashMap<String, Vec<(f64, f64)>>,
}

// 异步处理客户端连接的任务
async fn handle_client(mut stream: tokio::net::TcpStream) -> io::Result<()> {
    let mut buf = vec![0; 1024];
    loop {
        let n = match stream.read(&mut buf).await {
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        
        let data = match serde_json::from_slice::<ChartData>(&buf[..n]) {
            Ok(data) => data,
            Err(_) => {
                println!("Failed to parse data");
                continue;
            },
        };
        
        generate_chart(&data).await;
    }
}

// 生成图表的函数
async fn generate_chart(data: &ChartData) {
    let root_area = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Interactive Chart", ("sans-serif", 50))
        .build_ranged(0..640, 0..480)?;
    
    for (key, values) in &data.data {
        chart.configure_series_labels()
            .name(key)
            .style(&RED_INK.into_label().into_value_label())
            .draw()?;
        
        for point in values {
            chart.draw_series(LineSeries::new(
                values.iter().map(|(x, y)| (x * 10, y * 10)),
                &RED_INK,
            ))?;
        }
    }
    
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK.filled())
        .draw()?;
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on port 8080");

    loop {
        let (stream, _) = listener.accept().await?;
        task::spawn(handle_client(stream));
    }
}
