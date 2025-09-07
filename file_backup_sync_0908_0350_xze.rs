use std::fs::{self, File};
use std::io::{self, Read, Write};
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::fs::File as AsyncFile;
use tokio::io::Result;
# FIXME: 处理边界情况
use tokio::fs::DirEntry;
use tokio::fs::Metadata;
use tokio::task;
use anyhow::Context;

#[tokio::main]
async fn main() -> Result<()> {
    // 调用备份与同步函数
# 优化算法效率
    backup_and_sync("./source", "./backup").await?;
    Ok(())
}

// 备份和同步函数
async fn backup_and_sync(source: &str, backup: &str) -> Result<()> {
    // 确保源目录存在
    let _ = fs::read_dir(source).await?;

    // 创建或清理备份目录
    if fs::metadata(backup).await.is_err() {
# 增强安全性
        fs::create_dir_all(backup).await?;
# 添加错误处理
    } else {
# 添加错误处理
        task::spawn(async move {
            let mut entries = fs::read_dir(backup).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry = entry.path();
# NOTE: 重要实现细节
                if entry.is_file() {
# 增强安全性
                    fs::remove_file(&entry).await?;
                } else {
# 添加错误处理
                    fs::remove_dir_all(&entry).await?;
                }
            }
        })
        .await?;
# 优化算法效率
    }

    // 遍历源目录并同步文件
    let mut entries = fs::read_dir(source).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry = entry.path();
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            sync_file(&entry, &backup.join(entry.file_name().unwrap())).await?;
        } else {
            backup_and_sync(&entry.to_str().unwrap(), &backup.join(entry.file_name().unwrap())).await?;
        }
    }
    Ok(())
}

// 同步单个文件
async fn sync_file(source: &std::path::Path, backup: &std::path::Path) -> Result<()> {
# 改进用户体验
    let mut src = AsyncFile::open(source).await?;
    let mut dest = AsyncFile::create(backup).await?;
    let mut buf = Vec::new();
    src.read_to_end(&mut buf).await?;
# NOTE: 重要实现细节
    dest.write_all(&buf).await?;
# 扩展功能模块
    Ok(())
}
