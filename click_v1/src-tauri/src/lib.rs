extern crate core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod utils;
use open;

use serde_json::Value;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
// 引入 serde_json 库
use utils::files::{
    get_active_explorer_path,  // 获取当前活动窗口的目录
    replace_name,  // 修改文件名字
    replace_name_by_modify_time, // 按修改时间修改文件名
    replace_name_by_modify_time_pool, // 按修改时间修改文件名 修改路径池中的路径
    traverse_directory_all,      // 引入 traverse_directory_all 函数 广度优先遍历路径
    DataProcessor,
};
#[tauri::command]
fn test_command(data: Vec<Value>) -> Result<String, String> {
    let mut name_obj = DataProcessor::new(data, 0);
    for _ in 0..100 {
        let name_parts = name_obj.next();

        println!("{name_parts}")
    }
    Ok("处理完成".to_string())
}

#[tauri::command]
fn change_file_name(rule: Vec<Value>, path: String, mode: i32, old_to_new: bool, order_mode:i32) -> Result<(), String> {
    replace_name_by_modify_time(rule, Box::new(Path::new(path.as_str())), mode, old_to_new, order_mode)
        .map_err(|e| e.to_string())
}
#[tauri::command]
fn change_pool_file_name(rule: Vec<Value>, path: Vec<String>, mode: i32, old_to_new: bool, order_mode:i32) -> Result<(), String> {
    replace_name_by_modify_time_pool(rule, path, mode, old_to_new, order_mode)
        .map_err(|e| e.to_string())
}
#[tauri::command]
fn run_exe(path: String) {
    // 使用系统命令启动 EXE 文件，并在新线程中执行
    thread::spawn(move || {
        let output = Command::new("cmd")
            .args(&["/C", "start", "", &path])
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Failed to run EXE: {:?}", output);
        }
    });
}
// 打开 URL
#[tauri::command]
fn open_url(url: &str) -> Result<(), String> {
    match open::that(url) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("无法打开 URL: {e}")),
    }
}

#[tauri::command]
fn active_explorer_path() -> Vec<(String, String)> {
    match get_active_explorer_path() {
        Ok(paths) => paths.to_vec(),
        Err(e) => {
            eprintln!("Error: {}", e);
            vec![]
        }
    }
}

#[tauri::command]
fn replace_all_name(
    dir_path: String,
    old_name_sign: String,
    new_name_sign: String,
) -> Result<(), String> {
    // 调用 traverse_directory_all 获取目录下所有路径
    match traverse_directory_all(Box::new(Path::new(dir_path.as_str()))) {
        Ok(mut all_paths) => {
            all_paths.reverse(); // 倒序 保证不先修改父级目录的名称
            for path in all_paths {
                match replace_name(
                    Box::new(Path::new(path.as_str())),
                    &old_name_sign,
                    &new_name_sign,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("遍历目录失败: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            run_exe,
            open_url,
            active_explorer_path,
            replace_all_name,
            test_command,
            change_file_name,
            change_pool_file_name,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
