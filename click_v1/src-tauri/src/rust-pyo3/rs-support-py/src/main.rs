use image::{ImageFormat, open, GenericImageView};
use std::path::Path;
use image::{imageops::FilterType};


fn create_multi_size_ico(
    input_path: &str,
    output_path: &str,
    sizes: &[u32]  // 多个尺寸的数组，如 &[16, 32, 48, 64, 128, 256]
) -> Result<(), Box<dyn std::error::Error>> {
    let img = open(input_path)?;
    let mut ico_images = Vec::new();

    // 为每个指定尺寸创建图片
    for &size in sizes {
        let resized_img = img.resize(size, size, FilterType::Lanczos3);
        ico_images.push(resized_img);
    }

    // 将第一个尺寸的图片保存为 ICO（image 库会自动包含多尺寸）
    if let Some(first_img) = ico_images.first() {
        first_img.save(output_path)?;
    }

    Ok(())
}

fn convert_image_with_resize(
    input_path: &str,
    output_path: &str,
    width: u32,
    height: u32,
    output_format: ImageFormat
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 打开并加载原始图片
    let img = open(input_path)?;

    // 2. 调整图片大小
    let resized_img = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    // 3. 保存为指定格式的图片
    resized_img.save_with_format(output_path, output_format)?;

    Ok(())
}




/// 按给定最大宽高比调整图片大小
fn resize_with_aspect_ratio(
    input_path: &str,
    output_path: &str,
    max_width: u32,
    max_height: u32
) -> Result<(), Box<dyn std::error::Error>> {
    let input= Path::new(input_path);
    let output= Path::new(output_path);
    let img = open(input)?;
    let (orig_width, orig_height) = img.dimensions();

    // 计算保持宽高比的目标尺寸
    let ratio = (max_width as f32 / orig_width as f32)
        .min(max_height as f32 / orig_height as f32);

    let new_width = (orig_width as f32 * ratio) as u32;
    let new_height = (orig_height as f32 * ratio) as u32;

    let resized_img = img.resize(new_width, new_height, FilterType::Lanczos3);
    resized_img.save(output)?;

    Ok(())
}

// 使用示例
fn main() -> Result<(), Box<dyn std::error::Error>> {
    convert_image_with_resize(
        r"D:\Downloads\1.avif",
        r"D:\Downloads\1.png",
        800,  // 目标宽度
        600,  // 目标高度
        ImageFormat::Png
    )?;

    println!("图片转换完成！");
    Ok(())
}
