use std::{io};
use std::io::{BufRead, BufReader,Error, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// 处理一系列文档，每个文档由 `-start` 和 `-end` 分隔。
///
/// # 参数
/// * `doc_paths` - 要处理的文档路径列表
fn process_documents(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_file_path: Vec<String>,
    args_g2: Option<Vec<String>>,
    #[allow(dead_code)]
    old_format: String,
    new_format: String,
) -> Result<(), Error> {
    // 启动 doc.exe
    let mut child = Command::new(conversion_tool_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();  // 获取 stdin
    let stdout = child.stdout.take().unwrap();  // 获取 stdout
    let mut command1 = String::new();
    let mut command2 = String::new();

    // 使用字节流而不是 UTF-8 字符串
    let mut reader = BufReader::new(stdout);
    let mut buffer = Vec::new();

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
            println!("{}", line);
            if line.trim() == "-start" {
                found_start = true;
                break;
            }
        }
    }

    if !found_start {
        return Err(Error::new(
            io::ErrorKind::NotFound,
            "无法找到 -start 起始标志".to_string(),
        ));
    }

    for path in input_file_path {
        let mut new_path = PathBuf::from(&path);
        new_path.set_extension(&new_format);

        if let Some(ref args) = args_g1 {
            for i in args {
                command1 += &format!("{} ", i);
            }
        }
        if let Some(ref args) = args_g2 {
            for i in args {
                command2 += &format!("{} ", i);
            }
        }

        println!("正在处理文档：{:?}", path);
        println!("新文档路径：{:?}", new_path);
        // 发送文档路径
        if command2 == "".to_string(){
            writeln!(stdin, "{}{} {}", command1, path, new_path.display())?;

        } else{
            writeln!(stdin, "{}{} {}{} ", command1, path, command2, new_path.display())?;

        }

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
                println!("{}", line);
                if line.trim() == "-end" {
                    found_end = true;
                    break;
                }
            }
        }
        if !found_end {
            return Err(Error::new(
                io::ErrorKind::NotFound,
                "未收到 -end 信号".to_string(),
            ));
        }
    }

    // 发送退出命令
    println!("所有文档处理完毕，正在退出程序...");
    writeln!(stdin, "-exit")?;

    // 等待进程结束
    let _ = child.wait();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 定义要处理的文档列表（请根据实际情况修改路径）
    let documents = vec![
        r"C:\docs\report1.docx".to_string(),
        r"C:\docs\report2.docx".to_string(),
        r"C:\docs\report3.docx".to_string(),
    ];

    // 执行自动化交互
    // process_documents(r"D:\Object_\APP\Tauri\work\Click\click_v1\src-py\dist\main.exe".to_string(),"-docx2doc".to_string(), &documents, "xlsx".to_string())?;

    println!("所有文档已处理完毕。");
    Ok(())
}
