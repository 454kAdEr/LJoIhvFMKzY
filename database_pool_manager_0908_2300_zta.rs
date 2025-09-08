use async_trait::async_trait;
use tokio::sync::RwLock;
use tokio_postgres::{NoTls, Client, Error};
use std::sync::Arc;

/// Represents a connection pool that can manage multiple database connections.
#[derive(Clone)]
pub struct DatabasePool {
    pool: Arc<RwLock<Vec<Client>>>,
}

impl DatabasePool {
    /// Creates a new `DatabasePool` with the given number of connections.
    pub async fn new(url: &str, num_connections: usize) -> Result<Self, Error> {
        let mut pool = Vec::with_capacity(num_connections);
        for _ in 0..num_connections {
            let client = Client::connect(&url, NoTls).await?;
            pool.push(client);
        }

        Ok(DatabasePool {
            pool: Arc::new(RwLock::new(pool)),
        })
    }

    /// Acquires a connection from the pool.
    pub async fn acquire(&self) -> Result<Client, Error> {
        let pool = self.pool.read().await;
        if let Some(client) = pool.pop() {
            Ok(client)
        } else {
            Err(Error::ConnectionUnavailable)
        }
    }
}

/// A trait that defines database operations.
#[async_trait]
pub trait DatabaseOps {
    /// Executes a query and returns the result.
    async fn execute_query(&self, query: &str) -> Result<(), Error>;
}

/// An example struct that implements `DatabaseOps`.
pub struct MyDatabaseOps;

#[async_trait]
impl DatabaseOps for MyDatabaseOps {
    async fn execute_query(&self, query: &str) -> Result<(), Error> {
        let client = DatabasePool::acquire().await?;
        client.execute(query, &[]).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = DatabasePool::new("postgres://username:password@localhost/dbname", 5).await?;
    let ops = MyDatabaseOps;
    ops.execute_query("SELECT * FROM my_table").await?;
    Ok(())
}
