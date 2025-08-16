// database_migration_tool.rs
//
// 这是一个使用RUST和TOKIO框架创建的数据库迁移工具。
// 它遵循RUST的最佳实践，具有清晰的代码结构和适当的错误处理。

use tokio::runtime::Runtime;
use diesel::prelude::*; // 引入diesel库的基本功能
use diesel::migration::MigrationManager; // 引入diesel的迁移管理功能
use std::path::Path; // 引入路径处理库
use std::env; // 引入环境变量处理库

// 数据库迁移结构
struct DatabaseMigrationTool {
    migration_manager: MigrationManager,
}

impl DatabaseMigrationTool {
    // 创建一个新的数据库迁移工具实例
    pub fn new() -> Self {
        let manager = MigrationManager::new(
            "migrations_directory", // 迁移文件存放目录
            "database_url", // 数据库连接URL
        ).expect("Failed to create migration manager.");
        DatabaseMigrationTool { migration_manager: manager }
    }

    // 执行迁移操作
    pub async fn migrate(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.migration_manager
            .run_pending_migrations(None)
            .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建数据库迁移工具实例
    let tool = DatabaseMigrationTool::new();

    // 执行迁移操作
    tool.migrate().await
}
