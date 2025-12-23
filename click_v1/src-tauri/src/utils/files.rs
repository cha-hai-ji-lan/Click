use std::fs;
use std::fs::File;
// 用于文件操作
use serde_json::Value;
use std::io;
use std::path::Path; // 引入 serde_json 库
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

/// 更名迭代器
pub struct DataProcessor {
    pub data: Vec<Value>,
    pub counter: i32,
}

impl DataProcessor {
    pub fn new(data: Vec<Value>, counter: i32) -> Self {
        DataProcessor {
            data,
            counter,
        }
    }

    pub fn next(&mut self) -> String {
        let result: String = self.data.iter().map(|part| {
            match part {
                Value::String(s) => s.clone(),
                Value::Array(n) => {
                    n.iter().map(|item| {
                        match item {
                            Value::String(s) => {
                                let mut ori_str = s.clone();
                                ori_str.pop(); // 去除末尾1为了加上自增过的字符

                                let c = match s.chars().last() {
                                    Some(ch) => ch,
                                    None => return "A".to_string(), // 处理空字符串情况
                                };

                                // 添加额外的'z'字符（基于全局计数器）
                                let extra_z_count = self.counter / 52;
                                for _ in 0..extra_z_count {
                                    ori_str.push('z');
                                }
                                
                                let up_num = self.counter % 52;

                                // 计算基于字符位置的增量，而不是全局计数器
                                let base_ascii = c as u8;
                                let new_ascii;

                                // 处理跨越大小写边界的逻辑
                                if base_ascii < 90 && base_ascii + up_num as u8 > 90 { // 大写字母 A-Z
                                    new_ascii = 97 + (base_ascii + up_num as u8 - 91);
                                    // 读冲出去了 ！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！
                                } else if base_ascii < 122 && base_ascii + up_num as u8 > 122 { // 小写字母 a-z
                                    new_ascii = 65 + (base_ascii + up_num as u8 - 123);
                                    // 读冲出去了 ！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！

                                    ori_str.push('z');
                                } else {
                                    // 非字母字符的处理
                                    new_ascii = (base_ascii as i32 + up_num) as u8;
                                }
                                


                                ori_str.push(new_ascii as char);

                                ori_str
                            }

                            Value::Number(n) => {
                                let number = n.to_string().parse::<i32>().unwrap();
                                let result = (number + self.counter).to_string();
                                result
                            },
                            _ => "".to_string(),
                        }
                    }).collect()
                }
                _ => "".to_string(),
            }
        }).collect();
        self.counter += 1;  // 递增计数器 全局加 1
        result
    }
}


/// 管理 COM 对象
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

/// # 按时间顺序读取目录下文件与目录
///
/// **默认**：时间从旧到新
#[allow(dead_code)]
pub fn list_dir_by_time_com(
    dir_path: Box<&Path>,
    old_to_new: bool,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut files_with_time: Vec<FileObject> = Vec::new();
    let mut folders_with_time: Vec<FolderObject> = Vec::new();

    for entry in fs::read_dir(*dir_path)? {
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
    dir_path: Box<&Path>,
    old_to_new: bool,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut files_with_time: Vec<FileObject> = Vec::new();

    for entry in fs::read_dir(*dir_path)? {
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
    dir_path: Box<&Path>,
    old_to_new: bool,
) -> Result<FileSystemObject, Error> {
    let parent_name = match dir_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from("NULL_NAME"),
    };

    let mut folders_with_time: Vec<FolderObject> = Vec::new();

    for entry in fs::read_dir(*dir_path)? {
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

/// 修改文件名
///   * `path` - 要修改的文件路径
///   * `new_name` - 新的文件名
///   * `返回值` - 修改结果
pub fn change_name(path: Box<&Path>, new_name: String) -> Result<(), Error> {
    let old_path = *path;
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

/// 替换文件名
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

    if new_name == old_path_name {
        return Ok(());
    }
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

/// 根据修改时间来排序替换文件名
/// * `rule` - 排序规则
/// * `path` - 要修改的文件路径
/// * `mode_option` - 排序模式
/// * `返回值` - 排序结果
///
/// `mode_option`
/// > 1101. 升序  时间由旧到新 从选中父级目录中排序    无递归排序  文件排序
/// > 1102. 升序  时间由旧到新 从选中父级目录中排序    无递归排序  文件夹排序
/// > 1103. 升序  时间由旧到新 从选中父级目录中排序    无递归排序  混合排序
/// > 1201. 升序  时间由旧到新 从路径池中排序         无递归排序  文件排序
/// > 1202. 升序  时间由旧到新 从路径池中排序         无递归排序  文件夹排序
/// > 1203. 升序  时间由旧到新 从路径池中排序         无递归排序  混合排序
/// > 1211. 升序  时间由旧到新 从路径池中排序         递归排序    文件排序
/// > 1212. 升序  时间由旧到新 从路径池中排序         递归排序    文件夹排序
/// > 1213. 升序  时间由旧到新 从路径池中排序         递归排序    混合排序
/// > 2101. 降序 时间由新到旧  从选中父级目录中排序    无递归排序   文件排序
/// > 2102. 降序 时间由新到旧  从选中父级目录中排序    无递归排序   文件夹排序
/// > 2103. 降序 时间由新到旧  从选中父级目录中排序    无递归排序   混合排序
/// > 2201. 降序 时间由新到旧  从路径池中排序         无递归排序   文件排序
/// > 2202. 降序 时间由新到旧  从路径池中排序         无递归排序   文件夹排序
/// > 2203. 降序 时间由新到旧  从路径池中排序         无递归排序   混合排序
/// > 2211. 降序 时间由新到旧  从路径池中排序         递归排序     文件排序
/// > 2212. 降序 时间由新到旧  从路径池中排序         递归排序     文件夹排序
/// > 2213. 降序 时间由新到旧  从路径池中排序         递归排序     混合排序

pub fn replace_name_by_modify_time(
    rule: Vec<Value>,
    path: Box<Path>,
    mode_option: i32,
) -> Result<(), Error> {
    match mode_option {
        1101 => {}
        1102 => {}
        1103 => {}
        1201 => {}
        1202 => {}
        1203 => {}
        1211 => {}
        1212 => {}
        1213 => {}
        2101 => {}
        2102 => {}
        2103 => {}
        2201 => {}
        2202 => {}
        2203 => {}
        2211 => {}
        2212 => {}
        2213 => {}
        _ => {}
    }
    Ok(())
}
fn replace_name_by_modify_time_1101(rule: Vec<Value>, path: Box<&Path>) -> Result<(), Error> {
    let mut name = "".to_string();
    for part in rule.clone() {
        name += &part.to_string();
    }
    if let Ok(dir_list) = list_dir_by_time_file(path, true) {
        match dir_list {
            FileSystemObject::File(files) => {
                for a_file in files {
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
            FileSystemObject::Combined(files, folders) => {
                println!("当前操作只处理文件，但获取到了文件夹 与 文件");
            }
        }
    };
    Ok(())
}
