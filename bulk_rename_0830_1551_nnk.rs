use std::fs;
use std::io;
# 扩展功能模块
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

/// Main function to perform batch file renaming
#[tokio::main]
async fn main() -> io::Result<()> {
    let source_dir = "./path/to/source"; // Replace with your source directory path
    let target_dir = "./path/to/target"; // Replace with your target directory path
    let regex = regex::Regex::new(r"(.*)\.[^.]*").unwrap(); // Regex to capture the file name without extension
# NOTE: 重要实现细节

    fs::read_dir(source_dir).await?
# NOTE: 重要实现细节
        .filter_map(|entry| async move { entry.ok().map(|e| e.path()) })
        .for_each(|file_path| async move {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let captures = regex.captures(file_name).unwrap();
            let new_name = format!("{}_renamded.{}", captures[1], file_path.extension().unwrap().to_str().unwrap());
            let new_file_path = target_dir.to_owned().join(&new_name);

            fs::copy(&file_path, &new_file_path).await?;
# FIXME: 处理边界情况
            fs::remove_file(&file_path).await?;
        })
        .await;

    Ok(())
}

/// This function is used to rename files from source directory to target directory
async fn batch_rename(source_dir: &str, target_dir: &str) -> io::Result<()> {
    let regex = regex::Regex::new(r"(.*)\.[^.]*").unwrap(); // Regex to capture the file name without extension

    fs::read_dir(source_dir).await?
        .filter_map(|entry| async move { entry.ok().map(|e| e.path()) })
# 添加错误处理
        .for_each(|file_path| async move {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let captures = regex.captures(file_name).unwrap();
            let new_name = format!("{}_renamded.{}", captures[1], file_path.extension().unwrap().to_str().unwrap());
            let new_file_path = target_dir.to_owned().join(&new_name);

            fs::copy(&file_path, &new_file_path).await?;
            fs::remove_file(&file_path).await?;
        })
        .await;

    Ok(())
}