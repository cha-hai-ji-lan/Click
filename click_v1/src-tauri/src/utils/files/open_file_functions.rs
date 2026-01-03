use crate::utils::files::file_object::{ComGuard, ExplorerOperate};
use crate::utils::files::path_operations::{
    replace_name_by_modify_time_1, replace_name_by_modify_time_2, replace_name_by_modify_time_4,
    replace_name_by_modify_time_pool_1, replace_name_by_modify_time_pool_2,
    replace_name_by_modify_time_pool_4,
};
use percent_encoding::percent_decode_str;
use serde_json::Value;
use std::io::{Error, ErrorKind::ConnectionRefused};
use std::path::{Path, PathBuf};
use windows::{
    core::Interface,
    Win32::{
        System::{
            Com::{CoCreateInstance, CoUninitialize, CLSCTX_ALL},
            Variant::VARIANT,
        },
        UI::Shell::{IShellWindows, IWebBrowser2, ShellWindows},
    },
};

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
                        let url_string = url.to_string().replace("file:///", "");

                        // 使用 percent_encoding 库进行更彻底的解码
                        let decoded_path = percent_decode_str(&url_string)
                            .decode_utf8_lossy()
                            .to_string();

                        // 创建 PathBuf 对象
                        let path = PathBuf::from(decoded_path);

                        // 转换为字符串
                        let final_path = path.to_string_lossy().to_string();

                        path_stream.push((final_path.clone(), "".parse().unwrap()));
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

/// 根据修改时间来排序替换文件名
/// * `rule` - 排序规则
/// * `path` - 要修改的文件路径 父级路径
/// * `mode_option` - 排序模式
///
/// > 1 6(降序).  升序  时间由旧到新  从选中父级目录中排序    无递归排序   文件排序
/// > 2 7(降序).  升序  时间由旧到新  从选中父级目录中排序    无递归排序   文件夹排序
/// > 3 8(降序).  升序  时间由旧到新  从选中父级目录中排序    递归排序     文件夹排序 todo!(未完成)
/// > 4 9(降序).  升序  时间由旧到新  从选中父级目录中排序    无递归排序   混合排序
/// > 5 10(降序).  升序  时间由旧到新  从选中父级目录中排序    递归排序     混合排序 todo!(未完成)
/// * `返回值` - 空 | 失败

pub fn replace_name_by_modify_time(
    rule: Vec<Value>,
    path: Box<&Path>,
    mode_option: i32,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    match mode_option {
        1 => {
            replace_name_by_modify_time_1(rule, path, old_to_new, order_mode).expect("失败");
        }
        2 => {
            replace_name_by_modify_time_2(rule, path, old_to_new, order_mode).expect("失败");
        }
        3 => {}
        4 => {
            replace_name_by_modify_time_4(rule, path, old_to_new, order_mode).expect("失败");
        }
        5 => {}
        6 => {
            replace_name_by_modify_time_1(rule, path, old_to_new, order_mode).expect("失败");
        }
        7 => {}

        8 => {
            replace_name_by_modify_time_2(rule, path, old_to_new, order_mode).expect("失败");
        }
        9 => {}
        10 => {
            replace_name_by_modify_time_4(rule, path, old_to_new, order_mode).expect("失败");
        }
        _ => {
            replace_name_by_modify_time_1(rule, path, old_to_new, order_mode).expect("失败");
        }
    }
    Ok(())
}

/// 根据修改时间来排序替换文件名
/// * `rule` - 排序规则
/// * `path` - 要修改的文件路径
/// * `mode_option` - 排序模式
///
/// > 1 6. 升序  时间由旧到新  从路径池中排序         无递归排序  文件排序
/// > 2 7. 升序  时间由旧到新  从路径池中排序         无递归排序  文件夹排序
/// > 3 8. 升序  时间由旧到新  从路径池中排序         递归排序    文件夹排序todo!(未完成)
/// > 4 9. 升序  时间由旧到新  从路径池中排序         无递归排序   混合排序
/// > 5 10. 升序  时间由旧到新  从路径池中排序        递归排序    混合排序todo!(未完成)
/// * `返回值` - 空 | 失败
pub fn replace_name_by_modify_time_pool(
    rule: Vec<Value>,
    path: Vec<String>,
    mode_option: i32,
    old_to_new: bool,
    order_mode: i32,
) -> Result<(), Error> {
    match mode_option {
        1 => {
            replace_name_by_modify_time_pool_1(rule, path, old_to_new, order_mode).expect("失败");
        }
        2 => {
            replace_name_by_modify_time_pool_2(rule, path, old_to_new, order_mode).expect("失败");
        }
        3 => {}
        4 => {
            replace_name_by_modify_time_pool_4(rule, path, old_to_new, order_mode).expect("失败");
        }
        5 => {}
        6 => {
            replace_name_by_modify_time_pool_1(rule, path, old_to_new, order_mode).expect("失败");
        }
        7 => {
            replace_name_by_modify_time_pool_2(rule, path, old_to_new, order_mode).expect("失败");
        }
        8 => {}
        9 => {
            replace_name_by_modify_time_pool_4(rule, path, old_to_new, order_mode).expect("失败");
        }
        10 => {}
        _ => {}
    }
    Ok(())
}
