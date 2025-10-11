// 安全扫描工具
// 使用 Rust 和 Tokio 框架实现

use tokio::fs;
use tokio::io;
use tokio::process::Command;
use std::path::Path;
use std::str;
use std::time::Duration;

/// 定义错误类型
#[derive(Debug)]
enum ScanError {
    Io(io::Error),
    CommandFailed(String),
}

impl From<io::Error> for ScanError {
    fn from(err: io::Error) -> Self {
        ScanError::Io(err)
    }
}

/// 安全扫描函数
/// 扫描给定目录，执行安全检查命令
async fn scan_directory(path: &Path) -> Result<(), ScanError> {
    // 读取目录内容
    let entries = fs::read_dir(path).await?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // 检查路径是否为文件
        if path.is_file() {
            // 执行安全检查命令
            let output = Command::new("clamscan")
                .arg("--infected")
                .arg(path)
                .output()
                .await?;
            
            // 检查命令是否成功执行
            if !output.status.success() {
                return Err(ScanError::CommandFailed(str::from_utf8(&output.stderr)?.to_string()));
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // 定义要扫描的目录路径
    let path = Path::new("/path/to/directory");
    
    // 执行安全扫描
    match scan_directory(path).await {
        Ok(_) => println!("安全扫描完成，未发现异常。"),
        Err(e) => match e {
            ScanError::Io(err) => eprintln!("IO 错误: {}", err),
            ScanError::CommandFailed(msg) => eprintln!("命令执行失败: {}", msg),
        },
    }
}