use tokio::io::{self, AsyncBufReadExt};
use tokio::process::Command;
use std::error::Error;

/// 启动交互式图表生成器程序
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 欢迎信息
    println!("Welcome to the interactive chart generator!");

    // 提示用户输入数据
    println!("Please enter chart type (e.g., 'line', 'bar', 'pie'): ");
    let mut chart_type = String::new();
    io::stdin().read_line(&mut chart_type).await?;
    chart_type = chart_type.trim().to_lowercase();

    // 检查输入是否有效
    if chart_type != "line" && chart_type != "bar" && chart_type != "pie" {
        println!("Invalid chart type. Please choose 'line', 'bar', or 'pie'.");
        return Ok(());
    }

    // 提示用户输入数据点数量
    println!("Please enter the number of data points: ");
    let mut data_points_count = String::new();
    io::stdin().read_line(&mut data_points_count).await?;
    let data_points_count: usize = data_points_count.trim().parse()?;

    // 提示用户输入数据点
    println!("Please enter the data points, separated by commas (e.g., '10,20,30'): ");
    let mut data_points = String::new();
    io::stdin().read_line(&mut data_points).await?;
    let data_points: Vec<f64> = data_points
        .trim()
        .split(',')
        .map(|s| s.parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()?;

    if data_points.len() != data_points_count {
        println!("The number of data points does not match the specified count.");
        return Ok(());
    }

    // 调用外部命令生成图表
    let output = Command::new("python3")
        .arg("-m")
        .arg("matplotlib")
        .arg("-")
        .arg(&chart_type)
        .arg(data_points.join(","))
        .output()
        .await?;

    if !output.status.success() {
        println!("Failed to generate chart.");
        return Ok(());
    }

    // 输出图表生成结果
    println!("Chart generated successfully!");

    Ok(())
}
