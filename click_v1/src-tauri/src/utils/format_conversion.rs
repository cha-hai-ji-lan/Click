use std::io;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn fm_cov(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_file_path: Vec<String>,
    args_g2: Option<Vec<String>>,
    args_g3: Option<Vec<String>>,
    #[allow(unused_variables)] old_format: String,
    new_format: String,
) -> Result<(), Box<dyn std::error::Error>> {
    for file_path in input_file_path {
        // 构建输出文件路径
        let input_path = Path::new(&file_path);
        let file_stem = input_path.file_stem().ok_or("文件路径无效")?;
        let output_path =
            input_path.with_file_name(format!("{}.{}", file_stem.to_string_lossy(), new_format));

        // 构建命令参数
        let mut cmd = Command::new(&conversion_tool_path);

        // 添加 args_g1 参数（如果存在）
        if let Some(ref args) = args_g1 {
            cmd.args(args);
        }

        // 添加输入文件路径
        cmd.arg(&file_path);

        // 添加 args_g2 参数（如果存在）
        if let Some(ref args) = args_g2 {
            cmd.args(args);
        }

        // 添加输出文件路径
        cmd.arg(output_path.to_string_lossy().as_ref());
        
        // 添加 args_g3 参数（如果存在）
        if let Some(ref args) = args_g3 {
            cmd.args(args);
        }
        println!("{:?}", cmd);
        // 执行命令
        let output = cmd.output()?;

        // 检查命令执行结果
        if !output.status.success() {
            return Err(format!(
                "转换失败 {}: {}",
                file_path,
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }
    }

    Ok(())
}

/// 处理office系列文档格式转化，每个文档由 `-start` 和 `-end` 分隔。
///
/// # 参数
/// * `doc_paths` - 要处理的文档路径列表
pub fn office_fm_cov(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_file_path: Vec<String>,
    args_g2: Option<Vec<String>>,
    #[allow(unused_variables)] old_format: String,
    #[allow(unused_variables)] new_format: String,
) -> Result<(), Error> {
    println!("正在处理文档...");
    // 启动 doc.exe
    let mut child = Command::new("cmd")
        .args(&["/c", "start", &conversion_tool_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap(); // 获取 stdin
    let stdout = child.stdout.take().unwrap(); // 获取 stdout
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
    for path in input_file_path {
        let mut new_path = PathBuf::from(&path);
        new_path.set_extension(&new_format);

        println!("正在处理文档：{:?}", path);
        println!("新文档路径：{:?}", new_path);
        // 发送文档路径
        if command2 == "".to_string() {
            println!("单参:{}{} {}", command1, path, new_path.display());
            writeln!(stdin, "{}{} {}", command1, path, new_path.display())?;
        } else {
            println!(
                "多参:{}{} {}{} ",
                command1,
                path,
                command2,
                new_path.display()
            );
            writeln!(
                stdin,
                "{}{} {}{} ",
                command1,
                path,
                command2,
                new_path.display()
            )?;
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
