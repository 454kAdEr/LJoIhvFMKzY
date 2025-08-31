use std::collections::HashMap;
use tokio;
use anyhow::Result;

/// Represents a User with credentials.
#[derive(Debug, Clone)]
struct User {
    username: String,
    password: String,
}

/// Service that handles user authentication.
struct AuthService {
    users: HashMap<String, String>,
}

impl AuthService {
    /// Creates a new AuthService with a hashmap of users.
    fn new() -> AuthService {
        AuthService {
            users: HashMap::new(),
        }
    }

    /// Adds a new user to the AuthService.
    fn add_user(&mut self, username: String, password: String) {
        self.users.insert(username, password);
    }

    /// Authenticates a user with provided username and password.
    async fn authenticate(&self, username: &str, password: &str) -> Result<bool> {
        match self.users.get(username) {
            Some(actual_password) => {
                if actual_password == password {
                    Ok(true)
                } else {
                    Err(anyhow::anyhow!("Authentication failed!"))
                }
            },
            None => Err(anyhow::anyhow!("User not found!")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut service = AuthService::new();
    service.add_user("user1".to_string(), "password1".to_string());
    service.add_user("user2".to_string(), "password2".to_string());

    match service.authenticate("user1", "password1").await {
        Ok(is_authenticated) => {
            if is_authenticated {
                println!("User authenticated successfully!");
            } else {
                println!("User authentication failed!");
            }
        },
        Err(e) => {
            eprintln!("Authentication error: {}", e);
        },
    }

    Ok(())
}
