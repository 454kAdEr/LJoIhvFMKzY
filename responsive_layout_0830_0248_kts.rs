It is a simple example that listens for HTTP requests and returns a greeting message,
but can be expanded to serve dynamic content based on the request.
*/

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;

#[tokio::main]
async fn main() {
    // Bind the listener to a TCP socket
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind to address");
    println!("Server started on port 8080");

    // Accept connections and serve them
    loop {
        let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");
        tokio::spawn(handle_connection(socket));
    }
}

// Handles an individual connection
async fn handle_connection(mut socket: tokio::net::TcpStream) {
    // Read the request from the client
    let mut buf = [0; 1024];
    match socket.read(&mut buf).await {
        Ok(n) if n == 0 => return, // The client closed the connection
        Ok(_) => println!("Received request"),
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
            return;
        },
    }

    // TODO: Implement request parsing and routing logic here
    // For now, we just echo back a simple response
    let response = "HTTP/1.1 200 OK\r
Content-Type: text/html; charset=utf-8\r
\r
Hello, World!";
    match socket.write_all(response.as_bytes()).await {
        Ok(_) => println!("Sent response"),
        Err(e) => eprintln!("Failed to write to socket: {}", e),
    }

    // Close the socket
    match socket.shutdown().await {
        Ok(_) => println!("Socket closed"),
        Err(e) => eprintln!("Failed to close socket: {}", e),
    }
}
