// interactive_chart_generator.rs
//
// 这是一个使用RUST和TOKIO框架的交互式图表生成器。
// 它允许用户输入数据并生成图表。

use tokio::io;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 欢迎信息和说明
    println!("Welcome to the interactive chart generator!");
    println!("Please enter your data in the format: label,value");
    println!("Type 'done' when you're finished entering data.");

    // 创建一个哈希映射来存储图表数据
    let mut chart_data: HashMap<String, f64> = HashMap::new();

    // 获取输入
    let mut input = String::new();
    loop {
        input.clear();
        println!("Enter data (label,value) or 'done' to finish: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input).await?;

        // 检查输入是否为'done'
        if input.trim().eq_ignore_ascii_case("done") {
            break;
        }

        // 解析输入数据
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 2 {
            println!("Invalid input format. Please use label,value.");
            continue;
        }

        let label = parts[0].trim().to_string();
        let value = parts[1].trim().parse::<f64>().map_err(|e| e.to_string())?;
        chart_data.insert(label, value);
    }

    // 生成图表（这里只是一个简单示例，实际应用中您可能需要使用图形库）
    println!("Generating chart...");
    for (label, value) in &chart_data {
        println!("Label: {}, Value: {}", label, value);
    }

    Ok(())
}
