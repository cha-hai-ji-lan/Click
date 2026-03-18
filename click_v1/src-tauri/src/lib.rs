// extern crate core;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod utils;
use open;

use serde_json::Value;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Instant;
use tauri::{Emitter, Manager};
// 引入 serde_json 库
use utils::{
    files::{
        open_file_functions::{
            get_active_explorer_path, replace_name_by_modify_time, replace_name_by_modify_time_pool,
        },
        path_operations::{replace_name, traverse_directory_all},
    },
    format_conversion::{fm_cov, office_fm_cov},
    timing::{get_fmt_time, reset_timer_unnormal, timing_allocation},
};

#[tauri::command]
fn test_command(data: Vec<Value>) -> Result<String, String> {
   println!("{:?}", data);
    Ok("处理完成".to_string())
}

#[tauri::command]
async fn get_app_paths(app_handle: tauri::AppHandle) -> Result<String, String> {
    let resource_dir = app_handle
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?;
    let resource_path_str = resource_dir.display().to_string();
    // 修复：正确处理Option类型并返回Result
    let resources_path = resource_path_str
        .get(4..resource_path_str.len())
        .ok_or_else(|| "Failed to extract resource path".to_string())?
        .to_string();

    Ok(resources_path)
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
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now();
        let result = match replace_name_by_modify_time(
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
        };
        // 发送结果
        let _ = sender.send(result);
    });

    // 等待并返回结果
    receiver
        .recv()
        .unwrap_or_else(|_| Err("线程通信失败".to_string()))
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
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now();
        let result =
            match replace_name_by_modify_time_pool(rule, path, mode, old_to_new, order_mode) {
                Ok(_) => {
                    let duration = start.elapsed();
                    Ok(format!("花费时间: {:?}", duration))
                }
                Err(e) => Err(format!("遍历目录失败: {}", e)),
            };

        // 发送结果
        let _ = sender.send(result);
    });

    // 等待并返回结果
    receiver
        .recv()
        .unwrap_or_else(|_| Err("线程通信失败".to_string()))
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
    timing_allocation(time, target_time, fn_mode, mode); // 传参进入计时分配器 启动计时器
    Ok(())
}
#[tauri::command]
fn tc_get_fmt_time() -> String {
    get_fmt_time()
}
#[tauri::command]
fn tc_reset_timer_unnormal() {
    reset_timer_unnormal()
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
async fn replace_all_name(
    dir_path: String,
    old_name_sign: String,
    new_name_sign: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // 立即返回，不阻塞
    let handle = tokio::task::spawn_blocking(move || {
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
    });

    // 在后台执行，完成后通过事件通知前端
    tokio::spawn(async move {
        match handle.await {
            Ok(result) => {
                // 通过 Tauri 事件系统通知前端
                let _ = app_handle.emit("rename_complete", result);
            }
            Err(e) => {
                let _ = app_handle.emit("rename_error", format!("执行失败: {}", e));
            }
        }
    });

    // 立即返回
    Ok("操作已启动".to_string())
}

#[tauri::command]
fn replace_pool_all_name(
    main_path: String,
    old_name_sign: String,
    new_name_sign: String,
) -> Result<String, String> {
    // 使用通道在线程间传递结果
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now();
        let result = match replace_name(
            Box::new(Path::new(main_path.as_str())),
            &old_name_sign,
            &new_name_sign,
        ) {
            Ok(_) => {
                let duration = start.elapsed();
                Ok(format!("花费时间: {:?}", duration))
            }
            Err(e) => Err(format!("替换名称失败: {}", e)),
        };
        // 发送结果
        let _ = sender.send(result);
    });

    // 等待并返回结果
    receiver
        .recv()
        .unwrap_or_else(|_| Err("线程通信失败".to_string()))
}
#[tauri::command]
async fn format_conversion(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_dir_path: String,
    args_g2: Option<Vec<String>>,
    args_g3: Option<Vec<String>>,
    old_format: String,
    new_format: String,
) -> Result<String, String> {
    // 在单独的线程中执行格式转换，避免阻塞主线程
    let handle = tokio::task::spawn_blocking(move || {
        let mut input_file_path: Vec<String> = Vec::new(); // 初始化为空向量
        match traverse_directory_all(Box::new(Path::new(input_dir_path.as_str()))) {
            Ok(mut all_paths) => {
                all_paths.reverse(); // 倒序 保证不先修改父级目录的名称
                for path in all_paths {
                    // 获取路径的扩展名
                    let extension = Path::new(&path)
                        .extension()
                        .and_then(|ext| ext.to_str()) // 转换为字符串
                        .unwrap_or("") // 如果没有扩展名则返回空字符串
                        .to_lowercase(); // 统一转为小写以忽略大小写差异
                                         // 判断扩展名是否匹配 old_format
                    if extension == old_format.to_lowercase() {
                        // 扩展名匹配，执行后续操作
                        input_file_path.push(path);
                        // 在这里调用格式转换逻辑或其他处理
                    } else {
                        // 扩展名不匹配，跳过该文件
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("遍历目录失败{e}");
            }
        }
        let start = Instant::now();
        match fm_cov(
            conversion_tool_path,
            args_g1,
            input_file_path,
            args_g2,
            args_g3,
            old_format,
            new_format,
        ) {
            Ok(_) => {
                let duration = start.elapsed();
                Ok(format!("花费时间: {:?}", duration))
            }
            Err(e) => Err(format!("{}", e)),
        }
    });

    // 等待异步任务完成
    handle
        .await
        .unwrap_or_else(|e| Err(format!("异步任务执行失败: {}", e)))
}
#[tauri::command]
async fn pool_format_conversion(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_file_path: Vec<String>,
    args_g2: Option<Vec<String>>,
    args_g3: Option<Vec<String>>,
    old_format: String,
    new_format: String,
) -> Result<String, String> {
    // 在单独的线程中执行格式转换，避免阻塞主线程
    let handle = tokio::task::spawn_blocking(move || {
        let start = Instant::now();
        match fm_cov(
            conversion_tool_path,
            args_g1,
            input_file_path,
            args_g2,
            args_g3,
            old_format,
            new_format,
        ) {
            Ok(_) => {
                let duration = start.elapsed();
                Ok(format!("花费时间: {:?}", duration))
            }
            Err(e) => Err(format!("{}", e)),
        }
    });

    // 等待异步任务完成
    handle
        .await
        .unwrap_or_else(|e| Err(format!("异步任务执行失败: {}", e)))
}

#[tauri::command]
async fn office_format_conversion(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_dir_path: String,
    args_g2: Option<Vec<String>>,
    old_format: String,
    new_format: String,
) -> Result<String, String> {
    // 在单独的线程中执行格式转换，避免阻塞主线程
    let handle = tokio::task::spawn_blocking(move || {
        println!("conversion_tool_path:{:?}\ninput_dir_path{:?}", conversion_tool_path, input_dir_path);
        let mut input_file_path: Vec<String> = Vec::new(); // 初始化为空向量
        match traverse_directory_all(Box::new(Path::new(input_dir_path.as_str()))) {
            Ok(mut all_paths) => {
                all_paths.reverse(); // 倒序 保证不先修改父级目录的名称
                for path in all_paths {
                    // 获取路径的扩展名
                    let extension = Path::new(&path)
                        .extension()
                        .and_then(|ext| ext.to_str()) // 转换为字符串
                        .unwrap_or("") // 如果没有扩展名则返回空字符串
                        .to_lowercase(); // 统一转为小写以忽略大小写差异
                    // 判断扩展名是否匹配 old_format
                    if extension == old_format.to_lowercase() {
                        // 扩展名匹配，执行后续操作
                        input_file_path.push(path);
                        // 在这里调用格式转换逻辑或其他处理
                    } else {
                        // 扩展名不匹配，跳过该文件
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("遍历目录失败{e}");
            }
        }
        let start = Instant::now();
        match office_fm_cov(
            conversion_tool_path,
            args_g1,
            input_file_path,
            args_g2,
            old_format,
            new_format,
        ) {
            Ok(_) => {
                let duration = start.elapsed();
                Ok(format!("花费时间: {:?}", duration))
            }
            Err(e) => Err(format!("{}", e)),
        }
    });

    // 等待异步任务完成
    handle
        .await
        .unwrap_or_else(|e| Err(format!("异步任务执行失败: {}", e)))
}
#[tauri::command]
async fn office_pool_format_conversion(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_file_path: Vec<String>,
    args_g2: Option<Vec<String>>,
    old_format: String,
    new_format: String,
) -> Result<String, String> {
    // 在单独的线程中执行格式转换，避免阻塞主线程
    let handle = tokio::task::spawn_blocking(move || {
        let start = Instant::now();
        match office_fm_cov(
            conversion_tool_path,
            args_g1,
            input_file_path,
            args_g2,
            old_format,
            new_format,
        ) {
            Ok(_) => {
                let duration = start.elapsed();
                Ok(format!("花费时间: {:?}", duration))
            }
            Err(e) => Err(format!("{}", e)),
        }
    });

    // 等待异步任务完成
    handle
        .await
        .unwrap_or_else(|e| Err(format!("异步任务执行失败: {}", e)))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_app_paths,                  // 获取应用路径
            run_exe,                        // 运行 EXE 文件
            open_url,                       // 打开 URL
            active_explorer_path,           // 获取当前活动窗口的目录路径
            replace_all_name,               // 替换所有文件名字段
            replace_pool_all_name,          // 替换路径池所有文件名字段
            test_command,                   // 测试命令
            change_file_name,               // 批量替换文件名
            change_pool_file_name,          // 批量替换路径池文件名
            during_time_do_something,       // 运行指定时间后执行指定命令
            tc_get_fmt_time,                // 获取当前格式化好的字符串时间
            tc_reset_timer_unnormal,        // 重置计时器--非正常退出
            format_conversion,              // 格式转换
            pool_format_conversion,         // 批量替换路径池office文件格式
            office_format_conversion,       // 批量替换文件名
            office_pool_format_conversion,
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
