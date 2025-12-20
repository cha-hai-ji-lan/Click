// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod utils;
use utils::files::{
    get_active_explorer_path,
    traverse_directory_all,  // 引入 traverse_directory_all 函数 广度优先遍历路径
    replace_name
};
use std::process::Command;
use std::thread;
use std::path::Path;

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
fn replace_all_name(dir_path: String, old_name_sign: String, new_name_sign: String) -> Result<(), String> {
    // 调用 traverse_directory_all 获取目录下所有路径
    match traverse_directory_all(Box::new(Path::new(dir_path.as_str()))) {
        Ok(all_paths) => {
            for path in all_paths {
                if let Err(e) = replace_name(Box::new(Path::new(path.as_str())), &old_name_sign, &new_name_sign) {
                    eprintln!("Failed to rename {:?}: {}", path, e);
                }
            }
            Ok(())
        },
        Err(e) => Err(format!("遍历目录失败: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_exe, open_url, active_explorer_path, replace_all_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
