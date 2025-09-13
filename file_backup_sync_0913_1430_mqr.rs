use tokio::fs::{self, File, OpenOptions};
# 优化算法效率
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
# TODO: 优化性能
use std::sync::Arc;
# 增强安全性
use tokio::time::{sleep, Duration};

// 定义一个结构体，用于同步文件
struct FileSyncer {
    src_path: PathBuf,
    dst_path: PathBuf,
    last_sync_time: RwLock<HashMap<PathBuf, u64>>,
}

impl FileSyncer {
    // 构造函数
# 添加错误处理
    fn new(src: PathBuf, dst: PathBuf) -> Self {
        FileSyncer {
            src_path: src,
            dst_path: dst,
# 扩展功能模块
            last_sync_time: RwLock::new(HashMap::new()),
        }
    }

    // 同步单个文件
    async fn sync_file(&self, file_path: &Path) -> io::Result<()> {
        let mut src_file = File::open(file_path).await?;
        let mut dst_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.dst_path.join(file_path.strip_prefix(&self.src_path).unwrap_or(file_path)))
# 扩展功能模块
            .await?;

        let mut buffer = Vec::new();
        src_file.read_to_end(&mut buffer).await?;
        dst_file.write_all(&buffer).await?;
        dst_file.sync_all().await?;

        // 更新最后同步时间
        let mut last_sync = self.last_sync_time.write().await;
        let file_key = file_path.to_path_buf();
        last_sync.insert(file_key.clone(), file_path.metadata().await.unwrap().modified().unwrap().unix_time_nanos() as u64);
        Ok(())
    }

    // 同步整个目录
# 添加错误处理
    async fn sync_directory(&self) -> io::Result<()> {
        let mut stack: Vec<PathBuf> = vec![self.src_path.clone()];
        while let Some(current_path) = stack.pop() {
            let mut entries = fs::read_dir(&current_path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    stack.push(entry_path.clone());
# 增强安全性
                } else {
                    let last_sync_time = {
                        let last_sync = self.last_sync_time.read().await;
                        last_sync.get(&entry_path).copied()
                    };
                    if last_sync_time != entry_path.metadata().await?.modified()?.unix_time_nanos() as u64 {
# 增强安全性
                        self.sync_file(&entry_path).await?;
# 增强安全性
                    }
# 增强安全性
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 设置源目录和目标目录
# 扩展功能模块
    let src = PathBuf::from("./src");
    let dst = PathBuf::from("./dst");

    // 创建FileSyncer实例
    let file_syncer = FileSyncer::new(src, dst);

    // 同步目录
    file_syncer.sync_directory().await?;
# 增强安全性

    Ok(())
}
