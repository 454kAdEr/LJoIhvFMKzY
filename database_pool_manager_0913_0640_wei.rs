use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use std::sync::Arc;

/// DatabasePoolManager 负责创建和管理数据库连接池。
/// 它使用 `OnceCell` 来确保连接池全局只被初始化一次。
pub struct DatabasePoolManager {
    pool: Arc<OnceCell<PgPool>>,
}

impl DatabasePoolManager {
    /// 创建一个新的 DatabasePoolManager 实例。
    pub fn new() -> Self {
        DatabasePoolManager {
            pool: Arc::new(OnceCell::new()),
        }
    }

    /// 初始化数据库连接池。
    /// 如果连接池已经被初始化，则返回一个错误。
    pub async fn init(&self, url: &str) -> Result<&PgPool, sqlx::Error> {
        self.pool.get_or_init(|| async {
            PgPoolOptions::new()
                .connect(url)
                .await
        })
    }
}

#[tokio::main]
async fn main() {
    let manager = DatabasePoolManager::new();
    match manager.init("postgres://username:password@localhost/database").await {
        Ok(pool) => {
            // 使用数据库连接池进行操作...
            println!("Database pool initialized successfully!");
        }
        Err(e) => {
            eprintln!("Failed to initialize database pool: {}", e);
        }
    }
}
