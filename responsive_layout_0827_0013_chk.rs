use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// Represents a client connection
struct Client {
    stream: tokio::net::TcpStream,
    address: SocketAddr,
}

/// Handles client messages
async fn handle_client(mut client: Client) -> io::Result<()> {
    let mut buf = [0; 1024];
    
    loop {
        let n = client.stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(()); // Connection closed
        }
        
        let response = format!("Received: {}", String::from_utf8_lossy(&buf[..n]));
        client.stream.write_all(response.as_bytes()).await?;
    }
}

/// Main server logic
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut incoming = listener.incoming();
    let clients = Arc::new(Mutex::new(HashMap::new()));
    
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let client = Client {
            stream,
            address: stream.peer_addr()?,
        };
        
        let clients = Arc::clone(&clients);
        tokio::spawn(async move {
            let _ = handle_client(client).await;
            // Remove client from the map after the connection is closed
            let mut clients_map = clients.lock().await;
            clients_map.remove(&client.address);
        });
    }
    
    Ok(())
}

/// This is a simple responsive layout server program using Tokio.
/// It listens for incoming TCP connections and echoes back any received messages.
/// The server maintains a map of active connections, allowing for potential
/// responsive layout features to be implemented based on connected clients.
