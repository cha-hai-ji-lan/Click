use std::process::Command;

pub fn win_shutdown() {
    // 调用 Windows 的 shutdown 命令
    let output = Command::new("cmd")
        .args(&["/C", "shutdown /s /t 0"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("系统已启动关闭。");
    } else {
        eprintln!("未能启动关机: {}", String::from_utf8_lossy(&output.stderr));
    }
}