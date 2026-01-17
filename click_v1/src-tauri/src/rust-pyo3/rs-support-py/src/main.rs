// use voice_toolkit::{audio, transcribe_file_unified};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let model_path = r"D:\Desktop\DW\ggml-medium-q8_0.bin"; // 下载的 Whisper 模型
    // let audio_path = r"D:\Desktop\DW\1.wav"; // 你的音频文件
    // // audio::convert_audio_for_whisper(r"D:\Desktop\DW\1.mp3", r"D:\Desktop\DW\1.wav").await?;
    // let result = transcribe_file_unified(model_path, audio_path).await?;
    // println!("转录结果: {}", result.text);
    Ok(())
}




// use std::fs::File;
// use std::io::{Read, Write};
//
// /// Android的 从JPG文件中提取MP4
// fn extract_mp4_from_jpg(jpg_path: &str, mp4_path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     // 读取JPG文件
//     let mut file = File::open(jpg_path)?;
//     let mut data = Vec::new();
//     file.read_to_end(&mut data)?;
//
//     // 查找 b'ftyp'
//     let ftyp_pos = find_subsequence(&data, b"ftyp");
//
//     if ftyp_pos.is_none() {
//         eprintln!("Error: 'ftyp' not found");
//         return Ok(());
//     }
//
//     let offset = ftyp_pos.unwrap();
//     println!("Found 'ftyp' at offset {} (0x{:08X})", offset, offset);
//
//     let start_pos = if offset >= 4 { offset - 4 } else { 0 };
//
//     // 写入MP4文件
//     let mut output_file = File::create(mp4_path)?;
//     output_file.write_all(&data[start_pos..])?;
//
//     println!("Saved MP4 to {}", mp4_path);
//     Ok(())
// }
//
// // 辅助函数：查找字节序列
// fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
//     haystack
//         .windows(needle.len())
//         .position(|window| window == needle)
// }
//
// fn main() {
//     let jpg_path = r"D:\Desktop\mmexport1766572393302.jpg";
//     let mp4_path = r"D:\Desktop\ardu\ios2_output.mp4";
//
//     if let Err(e) = extract_mp4_from_jpg(jpg_path, mp4_path) {
//         eprintln!("Error: {}", e);
//     }
// }