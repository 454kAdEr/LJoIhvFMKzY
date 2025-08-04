use std::fs;
use std::path::Path;
use tokio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
# NOTE: 重要实现细节

/// 批量文件重命名工具
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取目标文件夹路径
    let target_folder = std::env::args().nth(1).ok_or("No target folder provided")?;
    // 获取新文件的前缀
    let new_prefix = std::env::args().nth(2).ok_or("No new prefix provided")?;

    // 检查目标文件夹是否存在
    if !Path::new(&target_folder).is_dir() {
        return Err("Target folder does not exist".into());
    }

    // 读取目标文件夹中的所有文件
    let files = fs::read_dir(&target_folder).await?;
    for file in files {
        let file = file?;
        let path = file.path();
        if path.is_file() {
            let new_name = format!("{}{}_{}", new_prefix, path.display().to_string().split('.').nth(1).unwrap_or_default(), path.extension().unwrap_or_default().to_str().unwrap_or_default());
            let new_path = path.with_file_name(new_name);
# FIXME: 处理边界情况

            // 重命名文件
            fs::rename(&path, &new_path).await?;
# FIXME: 处理边界情况
        }
    }

    Ok(())
}

/// 错误处理和日志记录可以进一步增强，以提供更详细的信息和更好的用户体验。
/// 该程序目前使用命令行参数接收目标文件夹和新文件前缀，可以根据需要进行修改。
/// 程序异步运行，以提高文件操作性能。