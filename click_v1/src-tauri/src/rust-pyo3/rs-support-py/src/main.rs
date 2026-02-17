use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

/// 处理一系列文档，每个文档由 `-start` 和 `-end` 分隔。
///
/// # 参数
/// * `doc_paths` - 要处理的文档路径列表
fn process_documents(doc_paths: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    // 启动 doc.exe
    let mut child = Command::new(r"D:\Object_\APP\Tauri\work\Click\click_v1\src-py\dist\main.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    // 使用字节流而不是 UTF-8 字符串
    let mut reader = BufReader::new(stdout);
    let mut buffer = Vec::new();

    // 创建超时机制的线程
    let timeout_handle = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(10));
        // 超时后可以考虑终止进程
    });

    for path in doc_paths {
        println!("正在处理文档：{}", path);

        // 等待 "-start" 输出
        let mut found_start = false;
        loop {
            buffer.clear();
            let bytes_read = reader.read_until(b'\n', &mut buffer)?;
            if bytes_read == 0 {
                break; // EOF
            }

            // 尝试转换为字符串，忽略无效UTF-8
            if let Ok(line) = String::from_utf8(buffer.clone()) {
                if line.trim() == "-start" {
                    found_start = true;
                    break;
                }
            }
        }

        if !found_start {
            return Err("未收到 -start 信号".into());
        }

        // 发送文档路径
        writeln!(stdin, "{}", path)?;

        // 等待 "-end" 输出
        let mut found_end = false;
        loop {
            buffer.clear();
            let bytes_read = reader.read_until(b'\n', &mut buffer)?;
            if bytes_read == 0 {
                break; // EOF
            }

            // 尝试转换为字符串，忽略无效UTF-8
            if let Ok(line) = String::from_utf8(buffer.clone()) {
                if line.trim() == "-end" {
                    found_end = true;
                    break;
                }
            }
        }

        if !found_end {
            return Err("未收到 -end 信号".into());
        }
    }

    // 发送退出命令
    println!("所有文档处理完毕，正在退出程序...");
    writeln!(stdin, "exit")?;

    // 等待进程结束
    let _ = child.wait();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 定义要处理的文档列表（请根据实际情况修改路径）
    let documents = vec![
        r"C:\docs\report1.txt",
        r"C:\docs\report2.txt",
        r"C:\docs\report3.txt",
    ];

    // 执行自动化交互
    process_documents(&documents)?;

    println!("所有文档已处理完毕。");
    Ok(())
}
