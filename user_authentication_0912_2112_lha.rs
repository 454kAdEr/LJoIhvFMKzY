use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use warp::Filter;

// Define a user struct with username and password
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // Simulated in-memory database
    let users: HashMap<String, User> = vec![
        ("user1".to_string(), User { username: "user1".to_string(), password: "password1".to_string() }),
    ]
    .into_iter()
    .collect();

    // Share the in-memory database across threads
    let users = Arc::new(Mutex::new(users));

    // Define the authentication filter
    let auth_filter = warp::path("auth")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users(users))
        .and_then(authenticate);

    // Start the server
    warp::serve(auth_filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Helper function to extract users from the shared Arc<Mutex<>>
async fn with_users(users: Arc<Mutex<HashMap<String, User>>>) -> Arc<Mutex<HashMap<String, User>>> {
    users
}

// Authentication function
async fn authenticate(user: User, users: Arc<Mutex<HashMap<String, User>>>) -> JsonResult<String> {
    // Lock the mutex to access the shared data
    let mut users_guard = users.lock().await;

    // Check if the user exists in the database
    if let Some(stored_user) = users_guard.get(&user.username) {
        // Check if the passwords match
        if stored_user.password == user.password {
            Ok("Authenticated successfully".to_string())
        } else {
            Err(serde_json::Error::custom("Invalid password"))
        }
    } else {
        Err(serde_json::Error::custom("User not found"))
    }
}
