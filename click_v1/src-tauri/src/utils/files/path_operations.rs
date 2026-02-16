use crate::utils::files::analyze_the_path::{list_dir_com, list_dir_folder, list_path_folder};
use crate::utils::files::{
    analyze_the_path::{list_dir_file, list_path_file},
    file_object::{DataProcessor, FileSystemObject},
};
use serde_json::Value;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

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
    let mut queue: std::collections::VecDeque<PathBuf> = std::collections::VecDeque::new(); // 使用VecDeque作为队列

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
                paths.push(path_str.to_string().replace("/", "\\"));
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

/// 修改文件名（保留扩展名）- 性能优化版
///   * `path` - 要修改的文件路径
///   * `new_name` - 新的文件名（不包含扩展名）
///   * `返回值` - 修改结果
pub fn change_name(path: Box<&Path>, new_name: String) -> Result<(), Error> {
    let old_path = path.to_path_buf();

    // 一次性获取所有需要的信息，减少系统调用
    let metadata = match fs::metadata(&old_path) {
        Ok(meta) => meta,
        Err(e) => {
            return Err(Error::new(
                io::ErrorKind::NotFound,
                format!("无法访问文件: {:?}, 错误: {}", old_path, e),
            ));
        }
    };

    // 获取父目录路径
    let parent_dir = match old_path.parent() {
        Some(parent) => parent,
        None => {
            return Err(Error::new(io::ErrorKind::InvalidInput, "无法确定父目录"));
        }
    };

    // 构建新路径（合并文件和目录的处理逻辑）
    let new_path = if metadata.is_file() {
        // 提取扩展名并构建新文件名
        let new_name_with_ext = match old_path.extension() {
            Some(ext) => format!("{}.{}", new_name, ext.to_string_lossy()),
            None => new_name,
        };
        parent_dir.join(&new_name_with_ext)
    } else {
        parent_dir.join(&new_name)
    };

    // 检查新路径是否已存在（提前检查，避免重命名失败）
    if new_path.exists() {
        return Err(Error::new(
            io::ErrorKind::AlreadyExists,
            format!("目标名称已存在: {:?}", new_path),
        ));
    }

    // 直接执行重命名操作
    fs::rename(&old_path, &new_path).map_err(|e| {
        // 错误处理保持原有逻辑
        println!("✗ 文件重命名失败:");
        println!("  错误类型: {:?}", e.kind());
        println!("  错误信息: {}", e);

        match e.kind() {
            io::ErrorKind::PermissionDenied => {
                println!("  🔧 解决建议:");
                println!("     1. 检查文件权限设置");
                println!("     2. 确保程序有足够的管理员权限");
                println!("     3. 关闭可能正在使用该文件的程序");
            }
            io::ErrorKind::AlreadyExists => {
                println!("  🔧 解决建议:");
                println!("     1. 目标文件名 '{}' 已存在", new_path.display());
                println!("     2. 请使用不同的文件名");
            }
            io::ErrorKind::NotFound => {
                println!("  🔧 解决建议:");
                println!("     1. 源文件可能已被移动、删除或重命名");
                println!("     2. 请确认文件路径是否正确");
            }
            io::ErrorKind::Interrupted => {
                println!("  🔧 解决建议:");
                println!("     1. 操作被系统中断");
                println!("     2. 请稍后重试");
            }
            _ => {
                println!("  🔧 解决建议:");
                println!("     1. 文件可能正在被其他程序使用");
                println!("     2. 请关闭音频播放器、编辑器等相关程序");
                println!("     3. 重启程序后再次尝试");
            }
        }
        e
    })
}


// pub fn change_name(path: Box<&Path>, new_name: String) -> Result<(), Error> {
//     let old_path = path.to_path_buf();
//     let new_path;
//     // 检查原路径是否存在
//     if !old_path.exists() {
//         return Err(Error::new(
//             io::ErrorKind::NotFound,
//             "指定的文件或目录不存在",
//         ));
//     }
//
//     // 获取父目录路径
//     let parent_dir = match old_path.parent() {
//         Some(parent) => parent,
//         None => {
//             return Err(Error::new(io::ErrorKind::InvalidInput, "无法确定父目录"));
//         }
//     };
//     if old_path.is_file() {
//         // 提取扩展名
//         let extension = old_path.extension();
//         let new_name_with_ext = match extension {
//             Some(ext) => {
//                 let ext_str = ext.to_string_lossy();
//                 format!("{}.{}", new_name, ext_str)
//             }
//             None => new_name,
//         };
//
//         // 构建新的完整路径
//         new_path = parent_dir.join(&new_name_with_ext);
//         // 检查新路径是否已存在
//         if new_path.exists() {
//             return Err(Error::new(io::ErrorKind::AlreadyExists, "目标文件名已存在"));
//         }
//     } else {
//         // 构建新的完整路径
//         new_path = parent_dir.join(&new_name);
//         // 检查新路径是否已存在
//         if new_path.exists() {
//             return Err(Error::new(io::ErrorKind::AlreadyExists, "目标文件名已存在"));
//         }
//     }
//
//     // 执行重命名操作
//     // let old_path = PathBuf::from( old_path.to_string_lossy().replace("/", "\\"));
//     // let new_path = PathBuf::from( new_path.to_string_lossy().replace("/", "\\"));
//     println!("旧路径:{:?}\n新路径:{:?}", old_path, &new_path);
//     fs::rename(old_path.as_path(), new_path.as_path())?;
//     println!("文件名已修改");
//     Ok(())
// }

/// 替换文件名
///   * `path` - 要修改的文件路径
///   * `old_name_sign` - 要替换的旧文件名标识符
///   * `new_name_sign` - 要替换的新文件名标识符
///   * `返回值` - 替换结果
/// 替换文件名（增强版 - 添加详细错误处理）
///   * `path` - 要修改的文件路径
///   * `old_name_sign` - 要替换的旧文件名标识符
///   * `new_name_sign` - 要替换的新文件名标识符
///   * `返回值` - 替换结果
pub fn replace_name(
    path: Box<&Path>,
    old_name_sign: &str,
    new_name_sign: &str,
) -> Result<(), Error> {
    let old_path = *path;
    let old_path_name = old_path.file_name().unwrap().to_str().unwrap().to_string();

    // 检查原路径是否存在
    if !old_path.exists() {
        return Err(Error::new(
            io::ErrorKind::NotFound,
            format!("指定的文件或目录不存在: {:?}", old_path),
        ));
    }

    // 获取父目录路径
    let parent_dir = match old_path.parent() {
        Some(parent) => parent,
        None => {
            return Err(Error::new(io::ErrorKind::InvalidInput, "无法确定父目录"));
        }
    };

    let new_name = old_path_name.replace(old_name_sign, new_name_sign);

    if new_name == old_path_name {
        return Ok(());
    }

    // 构建新的完整路径
    let new_path = parent_dir.join(&new_name);

    // 检查新路径是否已存在
    if new_path.exists() {
        return Err(Error::new(
            io::ErrorKind::AlreadyExists,
            format!("目标文件名已存在: {:?}", new_path),
        ));
    }

    // 检查文件是否被占用
    match fs::metadata(&old_path) {
        Ok(_) => {}
        Err(e) => {
            println!("警告: 无法获取文件元数据: {}", e);
        }
    }
    // 执行重命名操作
    match fs::rename(&old_path, &new_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("✗ 文件重命名失败:");
            println!("  错误类型: {:?}", e.kind());
            println!("  错误信息: {}", e);

            // 提供更详细的错误建议
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    println!("  建议: 检查文件权限，确保有足够的权限进行重命名操作");
                }
                io::ErrorKind::AlreadyExists => {
                    println!("  建议: 目标文件名已存在，请选择不同的名称");
                }
                io::ErrorKind::NotFound => {
                    println!("  建议: 源文件可能已被移动或删除");
                }
                _ => {
                    println!("  建议: 文件可能正在被其他程序使用，请关闭相关程序后重试");
                }
            }
            Err(e)
        }
    }
}

// pub fn replace_name(
//     path: Box<&Path>,
//     old_name_sign: &str,
//     new_name_sign: &str,
// ) -> Result<(), Error> {
//     let old_path = *path;
//     let old_path_name = old_path.file_name().unwrap().to_str().unwrap().to_string();
//
//     // 检查原路径是否存在
//     if !old_path.exists() {
//         return Err(Error::new(
//             io::ErrorKind::NotFound,
//             "指定的文件或目录不存在",
//         ));
//     }
//
//     // 获取父目录路径
//     let parent_dir = match old_path.parent() {
//         Some(parent) => parent,
//         None => {
//             return Err(Error::new(io::ErrorKind::InvalidInput, "无法确定父目录"));
//         }
//     };
//     let new_name = old_path_name.replace(old_name_sign, new_name_sign);
//
//     if new_name == old_path_name {
//         return Ok(());
//     }
//     // 构建新的完整路径
//     let new_path = parent_dir.join(&new_name);
//
//     // 检查新路径是否已存在
//     if new_path.exists() {
//         return Err(Error::new(io::ErrorKind::AlreadyExists, "目标文件名已存在"));
//     }
//     let old_path = PathBuf::from(old_path.to_string_lossy().replace("\\", "/"));
//     let new_path = PathBuf::from(new_path.to_string_lossy().replace("\\", "/"));
//     // 执行重命名操作
//     fs::rename(old_path.as_path(), new_path.as_path())?;
//     Ok(())
// }

/// > 1.  升序  时间由旧到新  从选中父级目录中排序    无递归排序   文件排序
pub fn replace_name_by_modify_time_1(
    rule: Vec<Value>,
    path: Box<&Path>,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    println!("replace_name_by_modify_time_1");
    let mut name_obj = DataProcessor::new(rule, 0);
    if let Ok(dir_list) = list_dir_file(path, old_to_new, order_mode) {
        match dir_list {
            FileSystemObject::File(files) => {
                for a_file in files {
                    let mut name = name_obj.next();
                    name = name
                        .replace("{}", &a_file.name)
                        .replace("{原名}", &a_file.name);
                    match change_name(Box::new(Path::new(&a_file.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件{}修改失败: {}", a_file.name, e);
                        }
                    };
                }
            }
            FileSystemObject::Folder(_) => {
                println!("当前操作只处理文件，但获取到了文件夹");
            }
            FileSystemObject::Combined(_, _) => {
                println!("当前操作只处理文件，但获取到了文件夹 与 文件");
            }
        }
    };
    Ok(())
}

/// > 2.  升序  时间由旧到新  从选中父级目录中排序    无递归排序   文件夹排序
pub fn replace_name_by_modify_time_2(
    rule: Vec<Value>,
    path: Box<&Path>,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    let mut name_obj = DataProcessor::new(rule, 0);
    if let Ok(dir_list) = list_dir_folder(path, old_to_new, order_mode) {
        match dir_list {
            FileSystemObject::File(_) => {
                println!("当前操作只处理文件夹，但获取到了文件");
            }
            FileSystemObject::Folder(folder) => {
                for a_folder in folder {
                    let mut name = name_obj.next();
                    name = name
                        .replace("{}", &a_folder.name)
                        .replace("{原名}", &a_folder.name);
                    match change_name(Box::new(Path::new(&a_folder.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件夹{}修改失败: {}", a_folder.name, e);
                        }
                    };
                }
            }
            FileSystemObject::Combined(_, _) => {
                println!("当前操作只处理文件，但获取到了文件夹 与 文件");
            }
        }
    };
    Ok(())
}

/// > 4.  升序  时间由旧到新  从选中父级目录中排序    无递归排序   混合排序
pub fn replace_name_by_modify_time_4(
    rule: Vec<Value>,
    path: Box<&Path>,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    let mut name_obj1 = DataProcessor::new(rule.clone(), 0);
    let mut name_obj2 = DataProcessor::new(rule, 0);
    if let Ok(dir_list) = list_dir_com(path, old_to_new, order_mode) {
        match dir_list {
            FileSystemObject::File(_) => {
                println!("当前操作只处理文件 与 文件夹，但只获取到了文件");
            }
            FileSystemObject::Folder(_) => {
                println!("当前操作只处理文件 与 文件夹，但只获取到了文件夹");
            }
            FileSystemObject::Combined(files, folders) => {
                for a_file in files {
                    let mut name = name_obj1.next();
                    name = name
                        .replace("{}", &a_file.name)
                        .replace("{原名}", &a_file.name);
                    match change_name(Box::new(Path::new(&a_file.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件{}修改失败: {}", a_file.name, e);
                        }
                    };
                }
                for a_folder in folders {
                    let mut name = name_obj2.next();
                    name = name
                        .replace("{}", &a_folder.name)
                        .replace("{原名}", &a_folder.name);
                    match change_name(Box::new(Path::new(&a_folder.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件夹{}修改失败: {}", a_folder.name, e);
                        }
                    };
                }
            }
        }
    };
    Ok(())
}

/// > 1 6. 升序  时间由旧到新  从路径池中排序         无递归排序  文件排序
pub fn replace_name_by_modify_time_pool_1(
    rule: Vec<Value>,
    path: Vec<String>,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    let mut name_obj = DataProcessor::new(rule, 0);
    let path_obj = path.iter().map(|p| Path::new(p)).collect::<Vec<&Path>>();
    if let Ok(path_list) = list_path_file(path_obj, old_to_new, order_mode) {
        match path_list {
            FileSystemObject::File(files) => {
                for a_file in files {
                    let mut name = name_obj.next();
                    name = name
                        .replace("{}", &a_file.name)
                        .replace("{原名}", &a_file.name);
                    match change_name(Box::new(Path::new(&a_file.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件{}修改失败: {}", a_file.name, e)
                        }
                    }
                }
            }
            FileSystemObject::Folder(_) => {
                println!("当前操作只处理文件，但获取到了文件夹");
            }
            FileSystemObject::Combined(_, _) => {
                println!("当前操作只处理文件，但获取到了文件夹 与 文件")
            }
        }
    };

    Ok(())
}

/// > 2 7. 升序  时间由旧到新  从路径池中排序         无递归排序  文件夹排序
pub fn replace_name_by_modify_time_pool_2(
    rule: Vec<Value>,
    path: Vec<String>,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    let mut name_obj = DataProcessor::new(rule, 0);
    let path_obj = path.iter().map(|p| Path::new(p)).collect::<Vec<&Path>>();
    if let Ok(path_list) = list_path_folder(path_obj, old_to_new, order_mode) {
        match path_list {
            FileSystemObject::File(_) => {
                println!("当前操作只处理文件夹，但获取到了文件");
            }
            FileSystemObject::Folder(folders) => {
                for a_folder in folders {
                    let mut name = name_obj.next();
                    name = name
                        .replace("{}", &a_folder.name)
                        .replace("{原名}", &a_folder.name);
                    match change_name(Box::new(Path::new(&a_folder.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件{}修改失败: {}", a_folder.name, e)
                        }
                    }
                }
            }
            FileSystemObject::Combined(_, _) => {
                println!("当前操作只处理文件，但获取到了文件夹 与 文件")
            }
        }
    };

    Ok(())
}

/// > 4 9. 升序  时间由旧到新  从路径池中排序         无递归排序   混合排序
pub fn replace_name_by_modify_time_pool_4(
    rule: Vec<Value>,
    path: Vec<String>,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    let mut name_obj1 = DataProcessor::new(rule.clone(), 0);
    let mut name_obj2 = DataProcessor::new(rule, 0);
    let path_obj = path.iter().map(|p| Path::new(p)).collect::<Vec<&Path>>();
    if let Ok(path_list) = list_path_folder(path_obj, old_to_new, order_mode) {
        match path_list {
            FileSystemObject::File(_) => {
                println!("当前操作只处理文件 与 文件夹，但只获取到了文件");
            }
            FileSystemObject::Folder(_) => {
                println!("当前操作只处理文件 与 文件夹，但只获取到了文件夹");
            }
            FileSystemObject::Combined(files, folders) => {
                for a_file in files {
                    let mut name = name_obj1.next();
                    name = name
                        .replace("{}", &a_file.name)
                        .replace("{原名}", &a_file.name);
                    match change_name(Box::new(Path::new(&a_file.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件{}修改失败: {}", a_file.name, e)
                        }
                    }
                }
                for a_folder in folders {
                    let mut name = name_obj2.next();
                    name = name
                        .replace("{}", &a_folder.name)
                        .replace("{原名}", &a_folder.name);
                    match change_name(Box::new(Path::new(&a_folder.path)), name.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("文件{}修改失败: {}", a_folder.name, e)
                        }
                    }
                }
            }
        }
    };

    Ok(())
}
