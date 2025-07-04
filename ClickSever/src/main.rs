use winreg::enums::*;
use winreg::RegKey;
use std::env;
use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use toml;

fn main() {
    let current_path: Vec<String> = get_my_father_path();
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    my_path: PathBuf,
    my_father_path: PathBuf,
    initiate: InitiateSection,
}

#[derive(Deserialize, Serialize, Debug)]
struct InitiateSection {
    object_path: Vec<PathBuf>,
}

fn get_my_father_path() -> Vec<String> {
    let exe_path = env::current_exe().expect("无法获取可执行文件路径");
    let parent_path = exe_path.parent()
        .expect("无法获取父路径");
    vec![exe_path.to_string_lossy().to_string(), parent_path.to_string_lossy().to_string()]
}
fn read_sever_config() -> Result<(), Box<dyn Error>> {
    // 从文件读取 TOML 数据
    let toml_content = fs::read_to_string("D:\\Object_\\Python_\\Pyobject_\\Click_pro\\src\\bin\\config\\clicksever.toml")?;

    // 解析为结构体
    let config: Config = toml::from_str(&toml_content)?;

    println!("Name: {:?}", config);

    Ok(())
}
///  写入配置文件
fn write_config_to_toml(current_path: Vec<String>) -> Result<(), Box<dyn Error>> {

    let config = Config {
        my_path: current_path[0].parse().unwrap(),
        my_father_path: current_path[1].parse().unwrap(),
        initiate: InitiateSection {
            object_path: vec![
                "C:\\Users\\Administrator\\Desktop\\ClickSever\\ClickSever.exe".parse().unwrap(),
                "C:\\Users\\Administrator\\Desktop\\ClickSever\\ClickSever.exe".parse().unwrap(),
            ]
        },
    };

    // 序列化为 TOML 字符串
    let toml_str = toml::to_string(&config)?;

    // 写入文件
    fs::write("config.toml", toml_str)?;

    println!("配置已写入 config.toml");
    Ok(())
}
/// 启用 Windows 注册表中 ClickSever 的自动启动功能
fn enable_autostart_windows() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;  // 获取当前可执行文件的路径
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let (key, _) = hkcu.create_subkey(path)?;
    key.set_value("ClickSever", &exe_path.to_string_lossy().to_string())?;
    Ok(())
}

/// 禁用 Windows 注册表中 ClickSever 的自动启动功能
fn disable_autostart_windows() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let (key, _) = hkcu.create_subkey(path)?;
    key.delete_value("ClickSever")?;
    Ok(())
}
#[test]
fn test() {
    if cfg!(target_os = "windows") {
        match disable_autostart_windows() {
            Ok(_) => println!("已成功启用 Autostart。"),
            Err(e) => eprintln!("无法启用自动启动: {}", e),
        }
    } else {
        println!("此平台不支持 Autostart。");
    }
}