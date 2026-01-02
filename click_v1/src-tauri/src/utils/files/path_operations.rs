
use std::fs;
use std::path::{Path, PathBuf}; 
use std::io::{Error};
use std::time;
/// 递归遍历目录下所有路径（广度优先版本）
///
/// 对于较深的目录结构可能存在性能问题，但适用于大多数用途
///
/// # 参数
/// * `dir_path` - 要遍历的目录路径
///
/// # 返回值
/// * `Ok(Vec<String>)` - 包含所有文件和目录路径的向量
/// * `Err(Error)` - 遇到错误时返回错误信息
#[allow(dead_code)]
pub fn traverse_directory_all(dir_path: Box<&Path>) -> Result<Vec<String>, Error> {
    let mut paths: Vec<String> = Vec::new();
    let mut queue: std::collections::VecDeque<PathBuf> =
        std::collections::VecDeque::new(); // 使用VecDeque作为队列

    // 初始化队列
    queue.push_back(dir_path.to_path_buf()); // 将目录路径加入队列

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

/// 递归遍历目录下所有路径（深度优先版本）
///
/// 对于较深的目录结构可能有栈溢出风险，但对于一般用途足够高效
///
/// # 参数
/// * `dir_path` - 要遍历的目录路径
///
/// # 返回值
/// * `Ok(Vec<String>)` - 包含所有文件和目录路径的向量
/// * `Err(Error)` - 遇到错误时返回错误信息
#[allow(dead_code)]
pub fn traverse_directory_all_dfs(dir_path: Box<Path>) -> Result<Vec<String>, Error> {
    let mut paths: Vec<String> = Vec::new();

    fn traverse_recursive(path: &Path, paths: &mut Vec<String>) -> Result<(), Error> {
        let read_dir = fs::read_dir(path)?;

        for entry in read_dir {
            let entry = entry?;
            let entry_path = entry.path();

            // 将路径添加到结果列表
            if let Some(path_str) = entry_path.to_str() {
                paths.push(path_str.to_string());
            }

            // 如果是目录，则递归遍历
            if entry_path.is_dir() {
                traverse_recursive(&entry_path, paths)?;
            }
        }

        Ok(())
    }

    traverse_recursive(&dir_path, &mut paths)?;
    Ok(paths)
}