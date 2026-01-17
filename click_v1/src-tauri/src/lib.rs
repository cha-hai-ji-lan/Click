extern crate core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod utils;
use open;

use serde_json::Value;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Instant;
// 引入 serde_json 库
use utils::{
    files::{
        file_object::DataProcessor,
        open_file_functions::{
            get_active_explorer_path, replace_name_by_modify_time, replace_name_by_modify_time_pool,
        },
        path_operations::{replace_name, traverse_directory_all},
    },
    timing::wait_with_timer,
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

/// 批量替换文件名
#[tauri::command]
fn change_file_name(
    rule: Vec<Value>,
    path: String,
    mode: i32,
    old_to_new: bool,
    order_mode: i32,
) -> Result<String, String> {
    let start = Instant::now();
    match replace_name_by_modify_time(
        rule,
        Box::new(Path::new(path.as_str())),
        mode,
        old_to_new,
        order_mode,
    ) {
        Ok(_) => {
            let duration = start.elapsed();
            Ok(format!("花费时间: {:?}", duration))
        }
        Err(e) => Err(format!("遍历目录失败: {}", e)),
    }
}
/// 批量替换路径池文件名
#[tauri::command]
fn change_pool_file_name(
    rule: Vec<Value>,
    path: Vec<String>,
    mode: i32,
    old_to_new: bool,
    order_mode: i32,
) -> Result<String, String> {
    let start = Instant::now();
    match replace_name_by_modify_time_pool(rule, path, mode, old_to_new, order_mode) {
        Ok(_) => {
            let duration = start.elapsed();
            Ok(format!("花费时间: {:?}", duration))
        }
        Err(e) => Err(format!("遍历目录失败: {}", e)),
    }
}
/// 运行 EXE 文件或 PowerShell 脚本
#[tauri::command]
fn run_exe(path: String) {
    // 检查文件扩展名来决定使用哪个命令
    let extension = Path::new(&path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("")
        .to_lowercase();

    thread::spawn(move || {
        let output = if extension == "ps1" {
            Command::new("powershell")
                .args(&["-WindowStyle", "Hidden", "-File", &path])
                .output()
                .expect("Failed to execute PowerShell command")
        } else {
            Command::new("cmd")
                .args(&["/C", "start", "", &path])
                .output()
                .expect("Failed to execute command")
        };

        if !output.status.success() {
            eprintln!("Failed to run file: {:?}", output);
        }
    });
}

#[tauri::command]
fn during_time_do_something(
    time: u64,
    target_time: u64,
    fn_mode: i32,
    mode: i32,
) -> Result<(), String> {
    match fn_mode {
        // 无执行函数， 执行函数作用范围大于 指定时间 所以指定时间也为 None
        0 => { // 允许 mode in [None, 0, 1]
            wait_with_timer(time, None, None, Option::from(mode));
        }
        1 => {}
        _ => {}
    }
    Ok(())
}
/// 打开 URL
#[tauri::command]
fn open_url(url: &str) -> Result<(), String> {
    match open::that(url) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("无法打开 URL: {e}")),
    }
}
/// 获取当前活动窗口的目录路径
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

/// 替换所有文件名字段
#[tauri::command]
fn replace_all_name(
    dir_path: String,
    old_name_sign: String,
    new_name_sign: String,
) -> Result<String, String> {
    let start = Instant::now();
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
            let duration = start.elapsed();
            Ok(format!("花费时间: {:?}", duration))
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
            run_exe,                  // 运行 EXE 文件
            open_url,                 // 打开 URL
            active_explorer_path,     // 获取当前活动窗口的目录路径
            replace_all_name,         // 替换所有文件名字段
            test_command,             // 测试命令
            change_file_name,         // 批量替换文件名
            change_pool_file_name,    // 批量替换路径池文件名
            during_time_do_something  // 运行指定时间后执行指定命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
