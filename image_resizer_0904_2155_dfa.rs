 * Features:
 * - Efficient batch processing of images.
 * - Error handling for file operations and image processing.
 * - Extensive documentation and comments for maintainability and scalability.
 */

use image::{DynamicImage, GenericImageView, ImageError};
use std::fs;
use std::io::Write;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;

/// Main function to start the image resizing process.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the target dimensions for resizing.
    let target_width = 800;
    let target_height = 600;

    // Define the directory to look for images and to save resized images.
    let input_dir = "./input_images/";
    let output_dir = "./output_images/";

    // Ensure the output directory exists.
    fs::create_dir_all(output_dir)?;

    // List all image files in the input directory.
    let files = fs::read_dir(input_dir)?;
    for file in files {
        let file = file?;
        let path = file.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            tokio::spawn(async move {
                resize_image(&filename, input_dir, output_dir, target_width, target_height).await;
            });
        }
    }

    Ok(())
}

/// Function to resize a single image.
async fn resize_image(filename: &str, input_dir: &str, output_dir: &str, target_width: u32, target_height: u32) {
    // Read the image file.
    let input_path = Path::new(input_dir).join(filename);
    let reader = File::open(input_path).await.expect(&format!("Failed to open {}", filename));
    let mut decoder = image::codecs::jpeg::JpegDecoder::new(reader);
    let (_dynamic_img, _info) = match decoder.read_info().await {
        Ok(img_info) => img_info,
        Err(_) => return, // Skip if the image cannot be read.
    };

    // Resize the image.
    let dynamic_img = _dynamic_img.resize_exact(target_width, target_height, image::imageops::FilterType::Lanczos3);

    // Save the resized image.
    let output_path = Path::new(output_dir).join(&format!("resized_{}", filename));
    if let Err(e) = save_image(&dynamic_img, &output_path).await {
        eprintln!("Failed to save image: {}", e);
    }
}

/// Function to save an image to a file.
async fn save_image(image: &DynamicImage, output_path: &Path) -> Result<(), ImageError> {
    let mut output_file = File::create(output_path).await?;
    let encoder = image::codecs::jpeg::JpegEncoder::new();
    encoder.write_to(image, &mut output_file, image.width(), image.height()).await?;
    output_file.shutdown().await?;
    Ok(())
}
