use std::fs;
use std::fs::{DirEntry, Metadata};
// 用于文件操作
use std::io;
use std::path::{Path, PathBuf};
// 用于错误处理
use std::time; // 用于获取文件创建时间

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
    File(Vec<FileObject>),
    Folder(Vec<FolderObject>),
    Combined(Vec<FileObject>, Vec<FolderObject>),
}

/// # 按时间顺序读取目录下文件与目录
///
/// **默认**：时间从旧到新
#[allow(dead_code)]
pub fn list_dir_by_time_com(dir_path: Box<Path>, old_to_new:Option<bool>) -> Result<FileSystemObject, io::Error> {
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
        Ok(FileSystemObject::Combined(files_with_time, folders_with_time))

    } else {
        files_with_time.reverse();
        folders_with_time.reverse();
        Ok(FileSystemObject::Combined(files_with_time, folders_with_time))
    }

}

/// # 按时间顺序读取目录下文件
///
/// **默认**：时间从旧到新
#[allow(dead_code)]
pub fn list_dir_by_time_file(dir_path: Box<Path>, old_to_new:Option<bool>) -> Result<FileSystemObject, io::Error> {
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
pub fn list_dir_by_time_folder(dir_path: Box<Path>, old_to_new:Option<bool>) -> Result<FileSystemObject, io::Error> {
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