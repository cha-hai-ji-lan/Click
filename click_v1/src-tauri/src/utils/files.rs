use std::fs;
// 用于文件操作
use std::io;
use std::path::Path;
// 用于错误处理
use std::io::{Error, ErrorKind::ConnectionRefused};
use std::time; // 用于获取文件创建时间
use windows::{
    core::{Interface, HRESULT},
    Win32::{
        System::{
            Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL},
            Variant::VARIANT,
        },
        UI::Shell::{IShellWindows, IWebBrowser2, ShellWindows},
    },
};

pub struct ComGuard {
    initialized: bool,
}

impl ComGuard {
    pub fn new() -> HRESULT {
        unsafe { CoInitialize(None) }
    }
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

#[derive(Debug)]
pub struct FolderObject {
    pub name: String,
    pub path: String,
    pub modified_time: time::SystemTime,
    pub father_name: String,
}
///
/// ## `time::SystemTime`: 表示一个系统时间
/// > 实现了`debug trait`，因此可以在打印时使用
#[derive(Debug)]
pub struct FileObject {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified_time: time::SystemTime,
    pub extension_name: String,
    pub father_name: String,
}

#[derive(Debug)]
pub struct ExplorerOperate {
    pub path_stream: Vec<(String, String)>, // (path, name)
}

impl FolderObject {
    pub fn new(
        name: String,
        path: String,
        modified_time: time::SystemTime,
        father_name: String,
    ) -> Self {
        FolderObject {
            name,
            path,
            modified_time,
            father_name,
        }
    }
}

impl FileObject {
    pub fn new(
        name: String,
        path: String,
        size: u64,
        modified_time: time::SystemTime,
        extension_name: String,
        father_name: String,
    ) -> Self {
        FileObject {
            name,
            path,
            size,
            modified_time,
            extension_name,
            father_name,
        }
    }
    #[allow(dead_code)]
    pub fn rename_file(old_path: &str, new_path: &str) -> io::Result<()> {
        fs::rename(old_path, new_path)
    }
}

#[derive(Debug)]
pub enum FileSystemObject {
    #[allow(dead_code)]
    File(Vec<FileObject>),
    #[allow(dead_code)]
    Folder(Vec<FolderObject>),
    #[allow(dead_code)]
    Combined(Vec<FileObject>, Vec<FolderObject>),
}

/// # 按时间顺序读取目录下文件与目录
///
/// **默认**：时间从旧到新
#[allow(dead_code)]
pub fn list_dir_by_time_com(
    dir_path: Box<Path>,
    old_to_new: Option<bool>,
) -> Result<FileSystemObject, Error> {
    let old_to_new = old_to_new.unwrap_or(false);
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut files_with_time: Vec<FileObject> = Vec::new();
    let mut folders_with_time: Vec<FolderObject> = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => "NULL_NAME".to_string(),
        };
        let modified_time = metadata.modified()?;

        if path.is_file() {
            let size = metadata.len();
            let extension_name = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => "NULL_EXT".to_string(),
            };
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
    files_with_time.sort_by_key(|f| f.modified_time);
    folders_with_time.sort_by_key(|f| f.modified_time);
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

/// # 按时间顺序读取目录下文件
///
/// **默认**：时间从旧到新
#[allow(dead_code)]
pub fn list_dir_by_time_file(
    dir_path: Box<Path>,
    old_to_new: Option<bool>,
) -> Result<FileSystemObject, Error> {
    let old_to_new = old_to_new.unwrap_or(false);
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut files_with_time: Vec<FileObject> = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = entry.metadata()?;
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => "NULL_NAME".to_string(),
            };
            let modified_time = metadata.modified()?;
            let size = metadata.len();
            let extension_name = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => "NULL_EXT".to_string(),
            };
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
    files_with_time.sort_by_key(|f| f.modified_time);
    if old_to_new {
        Ok(FileSystemObject::File(files_with_time))
    } else {
        files_with_time.reverse();
        Ok(FileSystemObject::File(files_with_time))
    }
}

/// # 按时间顺序读取目录下目录
///
/// **默认**：时间从旧到新
#[allow(dead_code)]
pub fn list_dir_by_time_folder(
    dir_path: Box<Path>,
    old_to_new: Option<bool>,
) -> Result<FileSystemObject, Error> {
    let old_to_new = old_to_new.unwrap_or(false);

    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut folders_with_time: Vec<FolderObject> = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

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
    folders_with_time.sort_by_key(|f| f.modified_time);
    if old_to_new {
        Ok(FileSystemObject::Folder(folders_with_time))
    } else {
        folders_with_time.reverse();
        Ok(FileSystemObject::Folder(folders_with_time))
    }
}

impl ExplorerOperate {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ExplorerOperate {
            path_stream: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn stream_from(path_stream: Vec<(String, String)>) -> Self {
        ExplorerOperate { path_stream }
    }

    pub(crate) fn show(&self) {
        for (path, name) in &self.path_stream {
            println!("{} {}", path, name);
        }
    }

    pub(crate) fn to_vec(&self) -> Vec<(String, String)> {
        self.path_stream.clone()
    }
}

pub fn get_active_explorer_path() -> Result<ExplorerOperate, Error> {
    let mut path_stream: Vec<(String, String)> = Vec::new();
    unsafe {
        // RAII 自动管理生命周期
        let _com_guard = ComGuard::new();

        let shell_windows: IShellWindows = match CoCreateInstance(&ShellWindows, None, CLSCTX_ALL) {
            Ok(shell) => shell,
            Err(_) => {
                CoUninitialize();
                return Err(Error::new(ConnectionRefused, "无法初始化 COM".to_string()));
            }
        };

        let count = match shell_windows.Count() {
            Ok(c) => c,
            Err(_) => {
                CoUninitialize();
                return Err(Error::new(
                    ConnectionRefused,
                    "无法获取 shell 窗口计数".to_string(),
                ));
            }
        };

        // 查找活动的Explorer窗口
        for i in 0..count {
            if let Ok(browser) = shell_windows
                .Item(&VARIANT::from(i))?
                .cast::<IWebBrowser2>()
            {
                // 获取类型信息计数
                // 获取当前URL/location
                match browser.LocationURL() {
                    Ok(url) => {
                        path_stream
                            .push((url.to_string().replace("file:///", ""), "".parse().unwrap()));
                        // LocationURL通常是file:///C:/path/to/folder格式 所以需要去掉file:///
                    }
                    Err(e) => {
                        println!("Failed to get LocationURL: {:?}", e);
                    }
                }

                // 或者获取LocationName（文件夹名称）
                match browser.LocationName() {
                    Ok(name) => {
                        if let Some(last_item) = path_stream.last_mut() {
                            last_item.1 = name.to_string();
                        }
                    }
                    Err(e) => {
                        println!("Failed to get LocationName: {:?}", e);
                    }
                }
            }
        }

        CoUninitialize();
        Ok(ExplorerOperate::stream_from(path_stream))
    }
}

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

pub fn change_name(path: Box<&Path>, new_name: String) -> Result<(), Error> {
    let old_path = *path;
    // 检查原路径是否存在
    if !old_path.exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "指定的文件或目录不存在",
        ));
    }

    // 获取父目录路径
    let parent_dir = match old_path.parent() {
        Some(parent) => parent,
        None => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "无法确定父目录",
            ));
        }
    };
    // 构建新的完整路径
    let new_path = parent_dir.join(&new_name);

    // 检查新路径是否已存在
    if new_path.exists() {
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "目标文件名已存在",
        ));
    }

    // 执行重命名操作
    fs::rename(old_path, &new_path)?;
    Ok(())
}

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
            "指定的文件或目录不存在",
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
    // 构建新的完整路径
    let new_path = parent_dir.join(&new_name);

    // 检查新路径是否已存在
    if new_path.exists() {
        return Err(Error::new(io::ErrorKind::AlreadyExists, "目标文件名已存在"));
    }

    // 执行重命名操作
    fs::rename(old_path, &new_path)?;
    Ok(())
}
