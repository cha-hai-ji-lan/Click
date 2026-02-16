use crate::utils::files::file_object::{FileObject, FileSystemObject, FolderObject};
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

// 定义公共 trait
trait NamedObject {
    fn name(&self) -> &str;
}

// 为 FileObject 实现 NamedObject
impl NamedObject for FileObject {
    fn name(&self) -> &str {
        &self.name
    }
}

// 为 FolderObject 实现 NamedObject
impl NamedObject for FolderObject {
    fn name(&self) -> &str {
        &self.name
    }
}

// 抽象的排序函数
fn sort_by_name<T: NamedObject>(objects: &mut Vec<T>) {
    objects.sort_by(|a, b| {
        let a_name = a.name();
        let b_name = b.name();

        // 判断是否为纯数字字符串
        let a_is_numeric = a_name.chars().all(|c| c.is_ascii_digit());
        let b_is_numeric = b_name.chars().all(|c| c.is_ascii_digit());

        match (a_is_numeric, b_is_numeric) {
            // 两个都是数字：按数值大小排序
            (true, true) => {
                match (a_name.parse::<i64>(), b_name.parse::<i64>()) {
                    (Ok(num_a), Ok(num_b)) => num_a.cmp(&num_b),
                    _ => a_name.cmp(b_name), // 如果解析失败则按字符串排序
                }
            }
            // 只有a是数字：数字排在后面
            (true, false) => std::cmp::Ordering::Greater,
            // 只有b是数字：数字排在后面
            (false, true) => std::cmp::Ordering::Less,
            // 两个都不是数字：按字符串排序
            (false, false) => a_name.cmp(b_name),
        }
    });
}

fn match_file_sort_mode(mode: i32, objects: &mut Vec<FileObject>) {
    match mode {
        // 文件排序
        // 按修改时间排序（从旧到新）
        1 => {
            objects.sort_by_key(|f| f.modified_time);
        }
        // 按名称排序
        2 => sort_by_name(objects),
        // 按文件大小
        3 => {
            objects.sort_by_key(|f| f.size); // 按文件从小到大排序
        }
        _ => {
            objects.sort_by_key(|f| f.modified_time);
        }
    }
}
fn match_folder_sort_mode(mode: i32, objects: &mut Vec<FolderObject>) {
    match mode {
        // 文件夹排序
        1 => {
            objects.sort_by_key(|f| f.modified_time);
        }
        2 => {
            sort_by_name(objects);
        }
        // 按文件大小 对于文件夹是极耗时操作
        3 => {
            analyze_folder_size(objects);
            objects.sort_by_key(|f| f.size); // 按文件从小到大排序
        }
        _ => {
            objects.sort_by_key(|f| f.modified_time);
        }
    }
}

///  # 分析父级文件夹下的所有文件夹与文件获取文件流对象
/// ‘dir_path` - 要分析的目录路径
///
/// `old_to_new` - 是否按旧到新排序
///  >
///  > **默认** ；时间从旧到新
///  >
/// `mode` - 排序模式
///  >
///  > 1 - 按修改时间排序（从旧到新）
///  >
///  > 2 - 按名称排序
///  >
/// `返回值` - 排序后的文件流对象 或分析失败报错
#[allow(dead_code)]
pub fn list_dir_com(
    dir_path: Box<&Path>,
    old_to_new: bool,
    mode: i32,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut files_with_time: Vec<FileObject> = Vec::new();
    let mut folders_with_time: Vec<FolderObject> = Vec::new();

    for entry in fs::read_dir(*dir_path)? {
        let entry = entry?;
        let path = entry.path().to_string_lossy().replace("/", "\\");
        let path = PathBuf::from(path);

        let metadata = entry.metadata()?;
        let mut name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string().replace(".*", ""),
            None => "NULL_NAME".to_string(),
        };
        let modified_time = metadata.modified()?;

        if path.is_file() {
            let size = metadata.len();
            let extension_name = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => "NULL_EXT".to_string(),
            };
            name = name.replace(&format!(".{}", extension_name), "");
            files_with_time.push(FileObject::new(
                name,
                path.to_string_lossy().to_string(),
                size,
                modified_time,
                extension_name,
                parent_name.clone(),
            ));
        } else {
            folders_with_time.push(FolderObject::new(
                name,
                path.to_string_lossy().to_string(),
                modified_time,
                parent_name.clone(),
            ));
        }
    }

    match_file_sort_mode(mode, &mut files_with_time);
    match_folder_sort_mode(mode, &mut folders_with_time);
    if old_to_new {
        Ok(FileSystemObject::Combined(
            files_with_time,
            folders_with_time,
        ))
    } else {
        files_with_time.reverse();
        folders_with_time.reverse();
        Ok(FileSystemObject::Combined(
            files_with_time,
            folders_with_time,
        ))
    }
}

///  # 分析父级文件夹下的所有文件获取文件流对象
/// ‘dir_path` - 要分析的目录路径
///
/// `old_to_new` - 是否按旧到新排序
///  >
///  > **默认** ；时间从旧到新
///  >
/// `mode` - 排序模式
///  >
///  > 1 - 按修改时间排序（从旧到新）
///  >
///  > 2 - 按名称排序
///  >
/// `返回值` - 排序后的文件流对象 或分析失败报错
#[allow(dead_code)]
pub fn list_dir_file(
    dir_path: Box<&Path>,
    old_to_new: bool,
    mode: i32,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut files_with_time: Vec<FileObject> = Vec::new();

    for entry in fs::read_dir(*dir_path)? {
        let entry = entry?;
        let path = entry.path().to_string_lossy().replace("/", "\\");
        let path = PathBuf::from(path);

        if path.is_file() {
            let metadata = entry.metadata()?;
            let mut name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => "NULL_NAME".to_string(),
            };
            let modified_time = metadata.modified()?;
            let size = metadata.len();
            let extension_name = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => "NULL_EXT".to_string(),
            };
            name = name.replace(&format!(".{}", extension_name), "");
            files_with_time.push(FileObject::new(
                name,
                path.to_string_lossy().to_string(),
                size,
                modified_time,
                extension_name,
                parent_name.clone(),
            ));
        }
    }

    // 按修改时间排序（从旧到新）
    match_file_sort_mode(mode, &mut files_with_time);
    if old_to_new {
        Ok(FileSystemObject::File(files_with_time))
    } else {
        files_with_time.reverse();
        Ok(FileSystemObject::File(files_with_time))
    }
}

///  # 分析父级文件夹下的所有文件夹获取文件流对象
/// ‘dir_path` - 要分析的目录路径
///
/// `old_to_new` - 是否按旧到新排序
///  >
///  > **默认** ；时间从旧到新
///  >
/// `mode` - 排序模式
///  >
///  > 1 - 按修改时间排序（从旧到新）
///  >
///  > 2 - 按名称排序
///  >
/// `返回值` - 排序后的文件流对象 或分析失败报错
#[allow(dead_code)]
pub fn list_dir_folder(
    dir_path: Box<&Path>,
    old_to_new: bool,
    mode: i32,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut folders_with_time: Vec<FolderObject> = Vec::new();

    for entry in fs::read_dir(*dir_path)? {
        let entry = entry?;
        let path = entry.path().to_string_lossy().replace("/", "\\");
        let path = PathBuf::from(path);

        if !path.is_file() {
            let metadata = entry.metadata()?;
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => "NULL_NAME".to_string(),
            };
            let modified_time = metadata.modified()?;

            folders_with_time.push(FolderObject::new(
                name,
                path.to_string_lossy().to_string(),
                modified_time,
                parent_name.clone(),
            ));
        } else {
        }
    }
    match_folder_sort_mode(mode, &mut folders_with_time);
    if old_to_new {
        Ok(FileSystemObject::Folder(folders_with_time))
    } else {
        folders_with_time.reverse();
        Ok(FileSystemObject::Folder(folders_with_time))
    }
}

///  # 分析输入全部文件与文件夹路径获取文件流对象
/// ‘dir_path` - 要分析的路径表
///
/// `old_to_new` - 是否按旧到新排序
///  >
///  > **默认** ；时间从旧到新
///  >
/// `mode` - 排序模式
///  >
///  > 1 - 按修改时间排序（从旧到新）
///  >
///  > 2 - 按名称排序
///  >
/// `返回值` - 排序后的文件流对象 或分析失败报错
#[allow(dead_code)]
pub fn list_path_com(
    dir_path: Vec<&Path>,
    old_to_new: bool,
    mode: i32,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path[0].file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };
    let mut files_with_time: Vec<FileObject> = Vec::new();
    let mut folders_with_time: Vec<FolderObject> = Vec::new();

    for entry_path in dir_path {
        let path = entry_path.to_path_buf(); // 修复：使用 to_path_buf() 而不是 into_path_buf()
        let metadata = path.metadata()?;
        let mut name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string().replace(".*", ""),
            None => "NULL_NAME".to_string(),
        };
        let modified_time = metadata.modified()?;

        if path.is_file() {
            let size = metadata.len();
            let extension_name = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => "NULL_EXT".to_string(),
            };
            name = name.replace(&format!(".{}", extension_name), "");
            files_with_time.push(FileObject::new(
                name,
                path.to_string_lossy().to_string(),
                size,
                modified_time,
                extension_name,
                parent_name.clone(),
            ));
        } else {
            folders_with_time.push(FolderObject::new(
                name,
                path.to_string_lossy().to_string(),
                modified_time,
                parent_name.clone(),
            ));
        }
    }

    // 按修改时间排序（从旧到新）
    match_file_sort_mode(mode, &mut files_with_time);
    match_folder_sort_mode(mode, &mut folders_with_time);
    if old_to_new {
        Ok(FileSystemObject::Combined(
            files_with_time,
            folders_with_time,
        ))
    } else {
        files_with_time.reverse();
        folders_with_time.reverse();
        Ok(FileSystemObject::Combined(
            files_with_time,
            folders_with_time,
        ))
    }
}

///  # 分析输入全部文件路径获取文件流对象
/// ‘dir_path` - 要分析的路径表
///
/// `old_to_new` - 是否按旧到新排序
///  >
///  > **默认** ；时间从旧到新
///  >
/// `mode` - 排序模式
///  >
///  > 1 - 按修改时间排序（从旧到新）
///  >
///  > 2 - 按名称排序
///  >
/// `返回值` - 排序后的文件流对象 或分析失败报错
#[allow(dead_code)]
pub fn list_path_file(
    dir_path: Vec<&Path>,
    old_to_new: bool,
    mode: i32,
) -> Result<FileSystemObject, Error> {
    println!("{:?}",dir_path);
    let parent_name = match dir_path[0].file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };
    let mut files_with_time: Vec<FileObject> = Vec::new();

    for entry_path in dir_path {
        let path = entry_path.to_path_buf(); // 修复：使用 to_path_buf() 而不是 into_path_buf()
        let metadata = path.metadata()?;
        let mut name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string().replace(".*", ""),
            None => "NULL_NAME".to_string(),
        };
        let modified_time = metadata.modified()?;
        let size = metadata.len();
        let extension_name = match path.extension() {
            Some(ext) => ext.to_string_lossy().to_string(),
            None => "NULL_EXT".to_string(),
        };
        name = name.replace(&format!(".{}", extension_name), "");
        files_with_time.push(FileObject::new(
            name,
            path.to_string_lossy().to_string(),
            size,
            modified_time,
            extension_name,
            parent_name.clone(),
        ));
    }

    // 按修改时间排序（从旧到新）
    match_file_sort_mode(mode, &mut files_with_time);
    if old_to_new {
        Ok(FileSystemObject::File(files_with_time))
    } else {
        files_with_time.reverse();
        Ok(FileSystemObject::File(files_with_time))
    }
}

///  # 分析输入全部文件夹路径获取文件流对象
/// ‘dir_path` - 要分析的路径表
///
/// `old_to_new` - 是否按旧到新排序
///  >
///  > **默认** ；时间从旧到新
///  >
/// `mode` - 排序模式
///  >
///  > 1 - 按修改时间排序（从旧到新）
///  >
///  > 2 - 按名称排序
///  >
/// `返回值` - 排序后的文件流对象 或分析失败报错
#[allow(dead_code)]
pub fn list_path_folder(
    dir_path: Vec<&Path>,
    old_to_new: bool,
    mode: i32,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path[0].file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };
    let mut folders_with_time: Vec<FolderObject> = Vec::new();
    for entry_path in dir_path {
        let path = entry_path.to_path_buf();
        let metadata = path.metadata()?;
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => "NULL_NAME".to_string(),
        };
        let modified_time = metadata.modified()?;
        folders_with_time.push(FolderObject::new(
            name,
            path.to_string_lossy().to_string(),
            modified_time,
            parent_name.clone(),
        ))
    }
    match_folder_sort_mode(mode, &mut folders_with_time);
    folders_with_time.sort_by_key(|f| f.modified_time);
    if old_to_new {
        Ok(FileSystemObject::Folder(folders_with_time))
    } else {
        folders_with_time.reverse();
        Ok(FileSystemObject::Folder(folders_with_time))
    }
}

///  # 获取文件夹大小
/// ‘folder_obj` - 要分析的文件夹对象
///
/// `返回值` - 无
///  >
///  > **默认** ；时间从旧到新
///  >
/// `返回值` - 无
#[allow(dead_code)]
#[allow(dead_code)]
pub fn analyze_folder_size(folder_obj: &mut Vec<FolderObject>) {
    for obj in folder_obj {
        let path = Path::new(&obj.path);
        // 计算文件夹大小（递归计算所有文件）
        let size = calculate_folder_size(path);
        obj.size = size.expect("REASON");
    }
}

///  # 获取文件夹大小
fn calculate_folder_size(path: &Path) -> Result<u64, Error> {
    let mut total_size = 0;
    let mut queue: std::collections::VecDeque<PathBuf> = std::collections::VecDeque::new();
    let mut visited: std::collections::HashSet<PathBuf> = std::collections::HashSet::new(); // 防止循环引用

    queue.push_back(path.to_path_buf());
    visited.insert(path.to_path_buf());

    while let Some(current_path) = queue.pop_front() {
        let read_dir = match fs::read_dir(&current_path) {
            Ok(dir) => dir,
            Err(_) => continue, // 跳过无法读取的目录
        };

        for entry in read_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue, // 跳过无法访问的条目
            };

            let path = entry.path();

            // 检查是否为符号链接
            if path.is_symlink() {
                let canonical_path = match fs::canonicalize(&path) {
                    Ok(cp) => cp,
                    Err(_) => continue,
                };
                if !visited.insert(canonical_path) {
                    continue; // 防止循环引用
                }
            } else if path.is_dir() {
                if !visited.insert(path.clone()) {
                    continue; // 防止重复访问
                }
                queue.push_back(path);
            } else {
                total_size += entry.metadata()?.len();
            }
        }
    }
    Ok(total_size)
}
