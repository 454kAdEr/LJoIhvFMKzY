use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;

/// FolderOrganizer is the main struct for organizing the folder structure.
struct FolderOrganizer {
    /// The root path where the organization process starts.
    root_path: PathBuf,
}

impl FolderOrganizer {
    /// Creates a new FolderOrganizer with the given root path.
    pub fn new(root_path: PathBuf) -> Self {
        FolderOrganizer { root_path }
    }

    /// Entry point to start the organization process.
    pub async fn organize(&self) -> io::Result<()> {
        self._traverse_and_organize(&self.root_path).await
    }

    /// Recursively traverses the directory and organizes files and subdirectories.
    async fn _traverse_and_organize(&self, path: &Path) -> io::Result<()> {
        let mut entries = fs::read_dir(path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            // Check if it's a directory or a file.
            if path.is_dir() {
                // If it's a directory, try to organize it.
                self._traverse_and_organize(&path).await?;
            } else {
                // If it's a file, move it to a designated 'files' folder.
                self._move_file_to_files_folder(&path).await?;
            }
        }
        Ok(())
    }

    /// Moves a file to the 'files' folder inside the current directory.
    async fn _move_file_to_files_folder(&self, file_path: &Path) -> io::Result<()> {
        let files_folder = self._create_or_get_files_folder(&file_path.parent().unwrap()).await?;
        let new_file_path = files_folder.join(file_path.file_name().unwrap());
        fs::rename(file_path, &new_file_path).await?;
        Ok(())
    }

    /// Creates a 'files' folder if it doesn't exist, or returns the path to it if it does.
    async fn _create_or_get_files_folder(&self, parent: &Path) -> io::Result<PathBuf> {
        let files_folder = parent.join("files");
        if !files_folder.exists() {
            fs::create_dir(&files_folder).await?;
        }
        Ok(files_folder)
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Example usage of FolderOrganizer
    let root_path = PathBuf::from("./path_to_your_directory");
    let organizer = FolderOrganizer::new(root_path);
    organizer.organize().await
}
