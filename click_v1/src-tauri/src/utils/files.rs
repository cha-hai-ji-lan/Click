use std::fs; // 用于文件操作
use std::io; // 用于错误处理
use std::time; // 用于获取文件创建时间

#[derive(Debug)]
pub struct FolderObject {
    pub name: String,
    pub path: String,
    pub created_time: time::SystemTime,
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
    pub created_time: time::SystemTime,
    pub extension_name: String,
    pub father_name: String,
}

impl FolderObject {
    pub fn new(
        name: String,
        path: String,
        created_time: time::SystemTime,
        father_name: String,
    ) -> Self {
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
        created_time: time::SystemTime,
        extension_name: String,
        father_name: String,
    ) -> Self {
        FileObject {
            name,
            path,
            size,
            created_time,
            extension_name,
            father_name,
        }
    }
    pub fn rename_file(old_path: &str, new_path: &str) -> io::Result<()> {
        fs::rename(old_path, new_path)
    }
}

#[derive(Debug)]
pub enum FileSystemObject {
    File(Vec<FileObject>),
    Folder(Vec<FolderObject>),
    Combined(Vec<FileObject>, Vec<FolderObject>),
}

fn list_files_by_time(
    dir_path: &str,
    mod_option: Option<i32>,
) -> Result<FileSystemObject, io::Error> {
    let mut files_with_time: Vec<FileObject> = Vec::new();
    let mut folder_with_time: Vec<FolderObject> = Vec::new();
    let param = mod_option.unwrap_or(1);
    match param {
        1 => {
            // 解析文件条目
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path(); // 获取文件路径
                // 只处理文件（可选：也可以包含目录）
                if path.is_file() {
                    let file_name = path.file_stem().unwrap().to_string_lossy().to_string(); // 获取文件名
                    let file_path = path.to_string_lossy().to_string(); // 获取文件路径
                    let metadata = fs::metadata(&path)?; // 获取文件元数据 文件大小、文件类型、创建时间、
                    let file_size = metadata.len(); // 获取修改时间
                    let file_modified_time = metadata.modified()?; // 获取修改时间
                    let file_extension_name =
                        path.extension().unwrap().to_string_lossy().to_string();
                    let father_dir_name = path
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();

                    files_with_time.push(FileObject::new(
                        file_name,
                        file_path,
                        file_size,
                        file_modified_time,
                        file_extension_name,
                        father_dir_name,
                    ));
                }
            }
            // 按修改时间排序（从旧到新）
            files_with_time.sort_by_key(|file| file.created_time);
            Ok(FileSystemObject::File(files_with_time))
        }
        2 => {
            // 解析目录条目
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path(); // 获取文件路径
                // 只处理文件（可选：也可以包含目录）
                if !path.is_file() {
                    let dir_path = path.to_string_lossy().to_string(); // 获取目录路径
                    let dir_name = path.file_name().unwrap().to_string_lossy().to_string(); // 获取目录名
                    let metadata = fs::metadata(&path)?; // 获取目录元数据
                    let dir_modified_time = metadata.modified()?; // 获取修改时间
                    let father_dir_name = path
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();

                    folder_with_time.push(FolderObject::new(
                        dir_name,
                        dir_path,
                        dir_modified_time,
                        father_dir_name,
                    ));
                }
            }

            // 按修改时间排序（从旧到新）
            folder_with_time.sort_by_key(|folder| folder.created_time);
            Ok(FileSystemObject::Folder(folder_with_time))
        }
        _ => {
            // 解析文件和目录条目
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path(); // 获取文件路径
                if path.is_file() {
                    let file_name = path.file_stem().unwrap().to_string_lossy().to_string(); // 获取文件名
                    let file_path = path.to_string_lossy().to_string(); // 获取文件路径
                    let metadata = fs::metadata(&path)?; // 获取文件元数据 文件大小、文件类型、创建时间、
                    let file_size = metadata.len(); // 获取修改时间
                    let file_modified_time = metadata.modified()?; // 获取修改时间
                    let file_extension_name =
                        path.extension().unwrap().to_string_lossy().to_string();
                    let father_dir_name = path
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();

                    files_with_time.push(FileObject::new(
                        file_name,
                        file_path,
                        file_size,
                        file_modified_time,
                        file_extension_name,
                        father_dir_name, ));
                } else {
                    // 处理目录
                    let dir_path = path.to_string_lossy().to_string(); // 获取目录路径
                    let dir_name = path.file_name().unwrap().to_string_lossy().to_string(); // 获取目录名
                    let metadata = fs::metadata(&path)?; // 获取目录元数据
                    let dir_modified_time = metadata.modified()?; // 获取修改时间
                    let father_dir_name = path
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();

                    folder_with_time.push(FolderObject::new(
                        dir_name,
                        dir_path,
                        dir_modified_time,
                        father_dir_name,
                    ));
                }
            }

            // 按修改时间排序（从旧到新）
            files_with_time.sort_by_key(|file| file.created_time);
            folder_with_time.sort_by_key(|folder| folder.created_time);
            Ok(FileSystemObject::Combined(
                files_with_time,
                folder_with_time,
            ))
        }
    }
}
