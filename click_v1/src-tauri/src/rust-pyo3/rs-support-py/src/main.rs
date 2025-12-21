use image::{self, GenericImageView, ImageFormat};
use std::path::Path;

fn resize_png_image(input_path: &str, output_path: &str, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    // 打开图片
    let img = image::open(input_path)?;

    // 调整尺寸 - 使用不同的过滤算法
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    // 保存为PNG格式
    resized.save_with_format(output_path, ImageFormat::Png)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    resize_png_image("input.png", "output.png", 800, 600)?;
    println!("图片尺寸调整完成");
    Ok(())
}
