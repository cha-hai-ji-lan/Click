use std::fs; // 用于文件操作
use std::io; // 用于错误处理
use std::path::Path; // 用于路径操作

pub struct FolderObject {
    pub name: String,
    pub path: String,
    pub created_time: String,
    pub father_name: String,
}
pub struct FileObject {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub created_time: String,
    pub extension_name: String,
    pub father_folder_name: String,
    pub father_name: String,
}

impl FolderObject {
    pub fn new(name: String, path: String, created_time: String, father_name: String) -> Self {
        FolderObject {
            name,
            path,
            created_time,
            father_name,
        }
    }
}

impl FileObject {
    pub fn new(
        name: String,
        path: String,
        size: u64,
        created_time: String,
        extension_name: String,
        father_folder_name: String,
        father_name: String,
    ) -> Self {
        FileObject {
            name,
            path,
            size,
            created_time,
            extension_name,
            father_folder_name,
            father_name,
        }
    }
    pub fn rename_file(old_path: &str, new_path: &str) -> io::Result<()> {
        fs::rename(old_path, new_path)
    }
}

#[derive(Debug)]
pub enum FileSystemObject {
    File(Vec<(String, String, u64, std::time::SystemTime)>),
    Folder(Vec<(String, String, std::time::SystemTime)>),
    Combined(
        Vec<(String, String, u64, std::time::SystemTime)>,
        Vec<(String, String, std::time::SystemTime)>,
    ),
}

fn list_files_by_time(
    dir_path: &str,
    mod_option: Option<i32>,
) -> Result<FileSystemObject, io::Error> {
    let mut files_with_time: Vec<(String, String, u64, std::time::SystemTime)> = Vec::new();
    let mut folder_with_time: Vec<(String, String, std::time::SystemTime)> = Vec::new();

    // 读取目录条目
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path(); // 获取文件路径
                                 // 只处理文件（可选：也可以包含目录）
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string(); // 获取文件名
            let file_path = path.to_string_lossy().to_string(); // 获取文件路径
            let metadata = fs::metadata(&path)?; // 获取文件元数据 文件大小、文件类型、创建时间、
            let file_size = metadata.len(); // 获取修改时间
            let file_modified_time = metadata.modified()?; // 获取修改时间

            files_with_time.push((file_name, file_path, file_size, file_modified_time));
        } else {
            // 处理目录
            let dir_path = path.to_string_lossy().to_string(); // 获取目录路径
            let dir_name = path.file_name().unwrap().to_string_lossy().to_string(); // 获取目录名
            let metadata = fs::metadata(&path)?; // 获取目录元数据
            let dir_modified_time = metadata.modified()?; // 获取修改时间
            folder_with_time.push((dir_name, dir_path, dir_modified_time));
        }
    }

    // 按修改时间排序（从旧到新）
    files_with_time.sort_by_key(|(_, _, _, time)| *time);
    folder_with_time.sort_by_key(|(_, _, time)| *time);
    let param = mod_option.unwrap_or(1);
    match param {
        1 => {
            Ok(FileSystemObject::File(files_with_time))
        }
        2 => {
            Ok(FileSystemObject::Folder(folder_with_time))
        }
        _ => {
            Ok(FileSystemObject::Combined(files_with_time, folder_with_time))
        }
    }
}
