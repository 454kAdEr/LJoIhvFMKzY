use tokio::time::{sleep, Duration};
use std::time::{Instant, SystemTime};
use futures::stream;
use futures::stream::StreamExt;
use tokio::net::TcpListener;

/// 启动性能测试任务
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置测试参数
    let num_connections = 100; // 并发连接数
    let duration = Duration::from_secs(10); // 测试持续时间

    // 初始化监听器
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on port 8080");

    // 创建连接任务流
    let mut tasks = Vec::with_capacity(num_connections);

    for _ in 0..num_connections {
        // 为每个连接创建一个任务
        let task = tokio::spawn(async move {
            let start = Instant::now();
            let mut stream = stream::repeat("").take(1);
            while let Some(_) = stream.next().await {
                // 模拟数据传输
                tokio::time::sleep(Duration::from_millis(10)).await;
                if start.elapsed() > duration {
                    break;
                }
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        });
        tasks.push(task);
    }

    // 等待所有任务完成
    let results = futures::future::join_all(tasks).await;
    for result in results {
        if let Err(e) = result {
            eprintln!("Task error: {}", e);
        }
    }

    Ok(())
}
