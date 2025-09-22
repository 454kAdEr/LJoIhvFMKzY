use tokio::process::Command;
use tokio::io::{self, AsyncBufReadExt};
use std::io::BufReader;
use anyhow::Result;

/// 运行数据库迁移命令
///
/// 这个函数使用`Command`结构体来执行系统命令，
/// 来运行数据库迁移工具，例如`migrate`命令。
#[tokio::main]
async fn main() -> Result<()> {
    // 打印开始迁移的信息
    println!("Starting database migration...");

    // 构建迁移命令
    let mut child = Command::new("migrate")
        .arg("up") // 运行迁移命令的up操作
        .stdout(io::piped()) // 获取标准输出
        .spawn()?;

    // 处理标准输出
    let output = child.stdout.take().unwrap();
    let reader = BufReader::new(output);
    let mut lines = reader.lines();

    // 逐行读取输出并打印
    while let Some(line) = await!(lines.next_line())? {
        println!("Migration output: {}", line);
    }

    // 等待命令执行结束
    let status = child.wait().await?;

    // 检查命令是否成功执行
    if !status.success() {
        anyhow::bail!("Migration failed with status: {}", status);
    } else {
        println!("Database migration completed successfully.");
    }

    Ok(())
}

/// 运行数据库迁移命令的异步版本
///
/// 这个函数提供了一个异步接口，允许在迁移过程中执行其他异步任务。
async fn run_migration() -> Result<()> {
    // 这里可以添加更多的逻辑，例如连接数据库、
    // 检查迁移状态等。
    await!(main()).map_err(|e| anyhow::Error::msg(e))
}
