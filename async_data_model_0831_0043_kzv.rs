use tokio::sync::RwLock;

/// 定义一个异步的数据模型
/// 用于在异步环境中安全地读写数据
#[derive(Debug, Clone)]
struct AsyncDataModel {
    /// 保护数据模型数据的读写锁
    data: RwLock<String>,
}

impl AsyncDataModel {
    /// 创建一个新的 AsyncDataModel 实例
    pub fn new(data: String) -> Self {
        AsyncDataModel {
            data: RwLock::new(data),
        }
    }

    /// 异步读取数据模型中的数据
    pub async fn read_data(&self) -> Result<String, tokio::sync::TryLockError<'_>> {
        let data = self.data.read().await?;
        Ok(data.clone())
    }

    /// 异步写入数据模型中的数据
    pub async fn write_data(&self, new_data: String) -> Result<(), tokio::sync::TryLockError<'_>> {
        let mut data = self.data.write().await?;
        *data = new_data;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // 创建 AsyncDataModel 实例
    let data_model = AsyncDataModel::new("Initial data".to_string());

    // 异步读取数据
    match data_model.read_data().await {
        Ok(data) => println!("Read data: \"{}\"", data),
        Err(e) => println!("Failed to read data: {:?}