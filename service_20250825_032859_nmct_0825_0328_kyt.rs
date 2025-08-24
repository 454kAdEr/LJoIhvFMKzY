 * It includes error handling and is designed for maintainability and scalability.
 */

use std::path::Path;
use tokio::fs;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::process;

// Constants for file extensions to be categorized
const IMAGES: &str = "images";
const DOCUMENTS: &str = "documents";
const AUDIO: &str = "audio";
const VIDEO: &str = "video";
const OTHERS: &str = "others";

// Function to determine the category based on file extension
fn get_category(file_path: &Path) -> &'static str {
    match file_path.extension().and_then(|e| e.to_str()) {
        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") => IMAGES,
        Some("pdf") | Some("doc") | Some("docx\) | Some("txt") => DOCUMENTS,
        Some("mp3\) | Some("wav") => AUDIO,
        Some("mp4") | Some("avi\) => VIDEO,
        _ => OTHERS,
    }
}

// Asynchronous function to organize files into categories
async fn organize_files(dir_path: &Path) -> io::Result<()> {
    let mut dirs = vec![IMAGES.into(), DOCUMENTS.into(), AUDIO.into(), VIDEO.into(), OTHERS.into()];

    // Create directories for categories
    for dir in dirs.iter() {
        let path = dir_path.join(dir);
        fs::create_dir_all(&path).await?;
    }

    // Read directory entries
    let mut entries = fs::read_dir(dir_path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry = entry.path();

        // Check if it's a file and not a directory
        if entry.is_file() {
            let category = get_category(&entry);
            let target_dir = dir_path.join(category);
            let target_path = target_dir.join(entry.file_name().unwrap());

            // Move file to the appropriate category directory
            fs::rename(&entry, &target_path).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // Define the directory path to be organized
    let dir_path = Path::new("./directory_to_organize");

    // Call the function to organize files
    if let Err(e) = organize_files(dir_path).await {
        eprintln!("Error organizing files: {}", e);
    }
}
