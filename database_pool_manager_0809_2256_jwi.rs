 * comments for understanding, and follows Rust best practices for maintainability and scalability.
 */

use tokio::sync::Mutex;
use deadpool_postgres::{Manager, Pool};
use postgres::{Client, NoTls};
use std::error::Error;
use std::sync::Arc;

// Database configuration structure
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

// Database pool manager
pub struct DatabasePoolManager {
    pool: Arc<Mutex<Pool>>,
}

impl DatabasePoolManager {
    // Create a new database pool manager with the given configuration
    pub async fn new(config: DatabaseConfig) -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new(|url| {
            // Establish a connection to the database
            let (client, connection) = postgres::connect(url, NoTls)?;
            // The connection object is returned to the pool once it's dropped
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Connection error: {}", e);
                }
            });
            Ok(client)
        }).await?;

        // Create a new pool with the manager
        let pool = Pool::new(manager, 16); // 16 connections in the pool

        Ok(DatabasePoolManager {
            pool: Arc::new(Mutex::new(pool)),
        })
    }

    // Acquire a client from the pool
    pub async fn get_client(&self) -> Result<Client, Box<dyn Error>> {
        let client = self.pool.lock().await.get().await?;
        Ok(client)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define database configuration
    let config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        user: "user".to_string(),
        password: "password".to_string(),
        database: "database".to_string(),
    };

    // Create a database pool manager
    let db_pool_manager = DatabasePoolManager::new(config).await?;

    // Get a client from the pool
    let client = db_pool_manager.get_client().await?;

    // Use the client to perform database operations
    // For example, to execute a query:
    let rows = client.query("SELECT * FROM your_table", &[]).await?;
    for row in rows {
        // Process each row
    }

    Ok(())
}
