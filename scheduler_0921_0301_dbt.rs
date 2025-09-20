use tokio::time::{self, Duration};
use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 设置定时任务调度器
    let mut interval = time::interval(Duration::from_secs(5));
    let start = Instant::now();

    loop {
        interval.tick().await;

        if start.elapsed().as_secs() >= 30 {
            println!("定时任务执行超过30秒，程序退出。");
            break;
        } else {
            println!("定时任务执行中... 已运行 {} 秒", start.elapsed().as_secs());
        }
    }

    Ok(())
}
