use tokio_postgres::{Client, NoTls, Error};

// 异步函数，用于执行查询并防止SQL注入
async fn execute_query(client: &mut Client, query: &str, params: &[&(dyn std::fmt::Display + Sync)]) -> Result<(), Error> {
    client.execute(query, params).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // 初始化数据库连接
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("连接数据库失败: {}