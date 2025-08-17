use std::fs;
use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageError};
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

/// 调整图片尺寸的函数
/// 
/// # 参数
/// * `source_path` - 源图片路径
/// * `target_path` - 目标图片路径（调整尺寸后的图片保存路径）
/// * `new_width` - 新的图片宽度
/// * `new_height` - 新的图片高度
///
/// # 返回值
/// `Result<(), ImageError>` - 成功时返回`Ok`，失败时返回`ImageError`
async fn resize_image(source_path: &Path, target_path: &Path, new_width: u32, new_height: u32) -> Result<(), ImageError> {
    // 读取源图片文件
    let mut file = File::open(source_path).await?;
    let mut image_bytes = Vec::new();
    file.read_to_end(&mut image_bytes).await?;

    // 从字节数据加载图片
    let image = image::load_from_memory(&image_bytes)?;

    // 调整图片尺寸
    let resized_image = image.resize(new_width, new_height, image::imageops::Nearest);

    // 将调整尺寸后的图片保存到目标路径
    let mut file = File::create(target_path).await?;
    resized_image.write_to(&mut file, image::ImageFormat::Png).await?;

    Ok(())
}

/// 批量调整文件夹中所有图片的尺寸
/// 
/// # 参数
/// * `source_folder_path` - 源文件夹路径
/// * `target_folder_path` - 目标文件夹路径
/// * `new_width` - 新的图片宽度
/// * `new_height` - 新的图片高度
///
/// # 返回值
/// `Result<(), io::Error>` - 成功时返回`Ok`，失败时返回`io::Error`
async fn batch_resize_images(source_folder_path: &Path, target_folder_path: &Path, new_width: u32, new_height: u32) -> Result<(), io::Error> {
    // 获取源文件夹中所有文件
    let files = fs::read_dir(source_folder_path)?;

    // 为每个文件创建一个任务
    let tasks = files.filter_map(|file| {
        let file = file.ok()?;
        let path = file.path();
        if path.is_file() {
            Some(async move {
                // 创建目标图片路径
                let target_path = target_folder_path.join(path.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::Other, 