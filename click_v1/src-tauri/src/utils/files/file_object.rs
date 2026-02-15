use std::fs;
// 用于文件操作
use serde_json::Value;
use std::io;
// 用于错误处理
use chrono::Local;
use std::time;
// 用于获取文件创建时间
use windows::{
    core::HRESULT,
    Win32::System::Com::{CoInitialize, CoUninitialize},
};
pub struct DataProcessor {
    pub data: Vec<Value>,
    pub counter: i32,
}

impl DataProcessor {
    pub fn new(data: Vec<Value>, counter: i32) -> Self {
        DataProcessor { data, counter }
    }

    pub fn next(&mut self) -> String {
        let result: String = self
            .data
            .iter()
            .map(|part| {
                match part {
                    Value::String(s) => s.clone(),
                    Value::Array(n) => {
                        n.iter()
                            .map(|item| {
                                match item {
                                    Value::String(s) => {
                                        let mut ori_str = s.clone();

                                        let c = match ori_str.pop() {
                                            Some(ch) => ch,
                                            None => return "A".to_string(), // 处理空字符串情况
                                        };
                                        if !('a' <= c && c <= 'z' || 'A' <= c && c <= 'Z') {
                                            // 获取当前系统时间
                                            let local_time = Local::now();
                                            ori_str =
                                                format!("{}", local_time.format("%Y-%m-%d-%6f"));
                                            return ori_str;
                                        }
                                        // 计算基于字符位置的增量，而不是全局计数器
                                        let new_ascii;

                                        // 添加额外的'z'字符（基于全局计数器）
                                        // let extra_z_count = self.counter / 52;  // A- Z a - z
                                        let extra_z_count = self.counter / 26;
                                        if c >= 'A' && c <= 'Z' {
                                            for _ in 0..extra_z_count {
                                                ori_str.push('Z');
                                            }
                                            // let up_num = self.counter % 52;  // A- Z a - z
                                            let up_num = self.counter % 26;

                                            if c < 'Z' && c as u8 + up_num as u8 > 'Z' as u8 {
                                                ori_str.push('Z');
                                                new_ascii = 64 + (c as i32 + up_num - 90) as u8;
                                            } else {
                                                new_ascii = c as u8 + up_num as u8;
                                            }
                                            ori_str.push(new_ascii as char);
                                        } else if c >= 'a' && c <= 'z' {
                                            for _ in 0..extra_z_count {
                                                ori_str.push('z');
                                            }

                                            // let up_num = self.counter % 52;  // A- Z a - z
                                            let up_num = self.counter % 26;

                                            if c < 'z' && c as u8 + up_num as u8 > 'z' as u8 {
                                                ori_str.push('z');
                                                new_ascii = 96 + (c as i32 + up_num - 122) as u8;
                                            } else {
                                                new_ascii = c as u8 + up_num as u8;
                                            }
                                            ori_str.push(new_ascii as char);
                                        }

                                        ori_str
                                    }

                                    Value::Number(n) => {
                                        let number = n.to_string().parse::<i32>().unwrap();
                                        let result = (number + self.counter).to_string();
                                        result
                                    }
                                    _ => "".to_string(),
                                }
                            })
                            .collect()
                    }
                    _ => "".to_string(),
                }
            })
            .collect();
        self.counter += 1; // 递增计数器 全局加 1
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
    pub size: u64,
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
            size: 0,  // 默认为0 保证性能
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


