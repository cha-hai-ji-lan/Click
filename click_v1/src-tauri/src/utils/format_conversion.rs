use std::process::Command;
use std::path::Path;

pub fn fm_cov(
    conversion_tool_path: String,
    args_g1: Option<Vec<String>>,
    input_file_path: Vec<String>,
    args_g2: Option<Vec<String>>,
    #[allow(unused_variables)]
    old_format: String,
    new_format: String,
) -> Result<(), Box<dyn std::error::Error>> {
    for file_path in input_file_path {
        // 构建输出文件路径
        let input_path = Path::new(&file_path);
        let file_stem = input_path.file_stem().ok_or("文件路径无效")?;
        let output_path = input_path.with_file_name(format!("{}.{}", file_stem.to_string_lossy(), new_format));

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
        println!("{:?}",cmd);
        // 执行命令
        let output = cmd.output()?;

        // 检查命令执行结果
        if !output.status.success() {
            return Err(format!(
                "转换失败 {}: {}",
                file_path,
                String::from_utf8_lossy(&output.stderr)
            ).into());
        }
    }

    Ok(())
}
