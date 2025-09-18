use image::{DynamicImage, GenericImageView, ImageFormat};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use tokio::task;

/// 调整图片尺寸的函数
///
/// 接受源图片路径和目标尺寸，返回调整后的图片
async fn resize_image(input_path: &Path, output_path: &Path, new_width: u32, new_height: u32) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    let resized_img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);
    resized_img.save(output_path)?;
    Ok(())
}

/// 批量调整图片尺寸
///
/// 接受包含图片路径的文件夹路径和目标尺寸，处理该文件夹下所有图片
async fn batch_resize_images(folder_path: &Path, target_size: (u32, u32)) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(folder_path).await?;
    for entry in entries {
        let entry = entry.await?;
        let path = entry.path();
        if path.is_file() {
            let output_path = path.with_extension("resized");
            task::spawn(async move {
                if let Err(e) = resize_image(&path, &output_path, target_size.0, target_size.1).await {
                    eprintln!("Error resizing image: {}, error: {}", path.display(), e);
                }
            })
            .await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let folder_path = Path::new("./images");
    let target_size = (800, 600); // 目标尺寸
    match batch_resize_images(folder_path, target_size).await {
        Ok(_) => println!("All images resized successfully."),
        Err(e) => eprintln!("Error resizing images: {}", e),
    }
}
