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

pub fn win_calc_md5(file_path: String) -> Result<String, String> {
    // 调用 Windows 的 certutil 命令计算文件的 MD5 值
    let output = Command::new("cmd")
        .args(&["/C", &format!("certutil -hashfile {file_path} MD5")])
        .output()
        .map_err(|e| format!("执行命令失败: {}", e))?;

    if output.status.success() {
        // 将输出转换为字符串并提取 MD5 值
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        // MD5 值通常在第二行
        if lines.len() >= 2 {
            Ok(lines[1].trim().to_string())
        } else {
            Err("无法解析 MD5 输出".to_string())
        }
    } else {
        Err(format!("计算 MD5 失败: {}", String::from_utf8_lossy(&output.stderr)))
    }
}pub fn win_calc_sha256(file_path: String) -> Result<String, String> {
    // 调用 Windows 的 certutil 命令计算文件的 MD5 值
    let output = Command::new("cmd")
        .args(&["/C", &format!("certutil -hashfile {file_path} SHA256")])
        .output()
        .map_err(|e| format!("执行命令失败: {}", e))?;

    if output.status.success() {
        // 将输出转换为字符串并提取 MD5 值
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        // MD5 值通常在第二行
        if lines.len() >= 2 {
            Ok(lines[1].trim().to_string())
        } else {
            Err("无法解析 MD5 输出".to_string())
        }
    } else {
        Err(format!("计算 MD5 失败: {}", String::from_utf8_lossy(&output.stderr)))
    }
}