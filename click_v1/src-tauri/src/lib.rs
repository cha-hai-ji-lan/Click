// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::process::Command;
use std::thread;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_exe, open_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
