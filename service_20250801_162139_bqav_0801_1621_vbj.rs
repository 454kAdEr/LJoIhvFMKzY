// folder_structure_organizer.rs
// 一个使用RUST和TOKIO框架的文件夹结构整理器

use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io;
use std::io::ErrorKind;

// 整理器配置
#[derive(Debug)]
struct OrganizerConfig {
    pub root_path: PathBuf,
    pub max_depth: usize, // 最大深度
    pub ignore_dirs: Vec<String>, // 忽略的目录列表
}

// 文件夹结构整理器
async fn organize_folder(config: &OrganizerConfig) -> io::Result<()> {
    let mut stack: Vec<_> = vec![config.root_path.as_path()];
    while let Some(current_dir) = stack.pop() {
        let entries = fs::read_dir(current_dir).await?;
        for entry in entries {
            let entry = entry.await?;
            if let Ok(metadata) = entry.metadata().await {
                if metadata.is_dir() {
                    // 忽略配置中指定的目录
                    if config.ignore_dirs.contains(&entry.file_name().to_str().unwrap().to_string()) {
                        continue;
                    }
                    // 如果超过最大深度，则跳过
                    if current_dir.components().count() - config.root_path.components().count() > config.max_depth {
                        continue;
                    }
                    stack.push(entry.path());
                } else {
                    // 这里是对文件的处理逻辑，可根据需要添加
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 配置示例
    let config = OrganizerConfig {
        root_path: PathBuf::from("./example"),
        max_depth: 3,
        ignore_dirs: vec![".git".to_string()], // 忽略 .git 目录
    };

    // 调用整理函数
    organize_folder(&config).await
}
