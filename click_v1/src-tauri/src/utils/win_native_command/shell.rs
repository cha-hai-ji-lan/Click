use std::process::{Command, Stdio};
use std::io::{BufReader, Write};

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



pub struct CmdSession {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
    #[allow(dead_code)]
    stdout_reader: BufReader<std::process::ChildStdout>,
}

impl CmdSession {
    pub fn new() -> Result<Self, String> {
        let mut child = Command::new("cmd")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("启动cmd失败: {}", e))?;

        let stdin = child.stdin.take().ok_or("无法获取stdin".to_string())?;
        let stdout = child.stdout.take().ok_or("无法获取stdout".to_string())?;
        let stdout_reader = BufReader::new(stdout);

        Ok(CmdSession {
            child,
            stdin,
            stdout_reader,
        })
    }

    pub fn execute(&mut self, command: &str)  {
        // 发送命令
        writeln!(self.stdin, "{}", command)
            .map_err(|e| format!("写入命令失败: {}", e)).expect("TODO: panic message");
    }

    pub fn close(mut self) -> Result<(), String> {
        // 发送 exit 命令
        writeln!(self.stdin, "exit")
            .map_err(|e| format!("发送退出命令失败: {}", e))?;
        self.child.wait().map_err(|e| format!("等待进程结束失败: {}", e))?;
        Ok(())
    }
}