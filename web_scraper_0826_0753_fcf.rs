// web_scraper.rs
// A simple web content scraper using Rust and Tokio framework.

use reqwest; // For making HTTP requests
use tokio; // Async runtime
use std::error::Error; // For error handling
use std::fs::File; // For writing to a file
use std::io::Write; // For writing operations
use std::path::Path; // For file path operations

// Define a structure to hold scraped content
struct ScrapedContent {
    url: String,
    content: String,
}

async fn scrape_content(url: &str) -> Result<ScrapedContent, Box<dyn Error>> {
    // Make an HTTP GET request to the provided URL
    let response = reqwest::get(url).await?;
    
    // Check if the response was successful
    if response.status().is_success() {
        let content = response.text().await?;
        
        // Return the scraped content
        Ok(ScrapedContent {
            url: url.to_string(),
            content,
        })
    } else {
        // Handle non-successful responses
        Err(Box::new(reqwest::Error::from(reqwest::StatusCode::from_u16(
            response.status().as_u16())?
        )))
    }
}

async fn save_content(scrap: ScrapedContent, file_path: &Path) -> Result<(), Box<dyn Error>> {
    // Create a new file or truncate an existing one
    let mut file = File::create(file_path)?;
    
    // Write the scraped content to the file
    file.write_all(scrap.content.as_bytes())?;
    
    // Flush the file to ensure all data is written
    file.flush()?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // URL to scrape
    let url = "https://example.com";
    
    // File path to save the scraped content
    let file_path = Path::new("scraped_content.html");
    
    // Scrape the content from the URL
    let scrap = scrape_content(url).await?;
    
    // Save the scraped content to a file
    save_content(scrap, file_path).await?;
    
    println!("Scraping completed successfully!");
    Ok(())
}
