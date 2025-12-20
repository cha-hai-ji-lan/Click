use std::fs;
use std::io::Error;
use std::path::Path;
#[allow(dead_code)]
pub fn traverse_directory_all(dir_path: Box<&Path>) -> Result<Vec<String>, Error> {
    let mut paths: Vec<String> = Vec::new();
    let mut queue: std::collections::VecDeque<std::path::PathBuf> =
        std::collections::VecDeque::new();

    // 初始化队列
    queue.push_back(dir_path.to_path_buf());

    // 广度优先遍历，避免深层递归导致栈溢出
    while let Some(current_path) = queue.pop_front() {
        let read_dir = fs::read_dir(&current_path)?;

        for entry in read_dir {
            let entry = entry?;
            let path = entry.path();

            // 将路径添加到结果列表
            if let Some(path_str) = path.to_str() {
                paths.push(path_str.to_string().replace("\\", "/"));
            }

            // 如果是目录，则加入队列以便后续遍历
            if path.is_dir() {
                queue.push_back(path);
            }
        }
    }

    Ok(paths)
}
fn main() {
    let dir_path = Box::new(Path::new(r"D:\Desktop\实验程序\实验程序"));
    let paths = traverse_directory_all(dir_path);
    println!("{:?}", paths);
}