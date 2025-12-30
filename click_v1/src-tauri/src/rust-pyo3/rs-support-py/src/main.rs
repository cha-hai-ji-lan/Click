use std::fs::File;
use std::io::{Read, Write};

/// Android的 从JPG文件中提取MP4
fn extract_mp4_from_jpg(jpg_path: &str, mp4_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 读取JPG文件
    let mut file = File::open(jpg_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // 查找 b'ftyp'
    let ftyp_pos = find_subsequence(&data, b"ftyp");

    if ftyp_pos.is_none() {
        eprintln!("Error: 'ftyp' not found");
        return Ok(());
    }

    let offset = ftyp_pos.unwrap();
    println!("Found 'ftyp' at offset {} (0x{:08X})", offset, offset);

    let start_pos = if offset >= 4 { offset - 4 } else { 0 };

    // 写入MP4文件
    let mut output_file = File::create(mp4_path)?;
    output_file.write_all(&data[start_pos..])?;

    println!("Saved MP4 to {}", mp4_path);
    Ok(())
}

// 辅助函数：查找字节序列
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn main() {
    let jpg_path = r"D:\Desktop\mmexport1766572393302.jpg";
    let mp4_path = r"D:\Desktop\ardu\ios2_output.mp4";

    if let Err(e) = extract_mp4_from_jpg(jpg_path, mp4_path) {
        eprintln!("Error: {}", e);
    }
}