use windows::{
    core::*,
    Win32::{
        System::Com::*,
        UI::Shell::*,
    },
};
use windows::Win32::System::Variant::VARIANT;

fn get_active_explorer_path() -> Result<String> {
    unsafe {
        let _ = CoInitialize(None);

        let shell_windows: IShellWindows = match CoCreateInstance(&ShellWindows, None, CLSCTX_ALL) {
            Ok(shell) => shell,
            Err(_) => {
                CoUninitialize();
                return Ok("None1".to_string());
            }
        };

        let count = match shell_windows.Count() {
            Ok(c) => c,
            Err(_) => {
                CoUninitialize();
                return Ok("None2".to_string());
            }
        };


        // 查找活动的Explorer窗口
        for i in 0..count {
            println!("1");
            if let Ok(window) = shell_windows.Item(&VARIANT::from(i)) {
                println!("2");
                // 获取类型信息计数
                let hr = window.GetTypeInfoCount();
                println!("hr: {:?}", hr);
                let hr = window.GetTypeInfo(0, 0x409,); // 0x409 is LCID for English-US
                println!("{:?}",  hr);
                if let Ok(browser) = window.cast::<IWebBrowser2>() {
                    println!("Cast to IWebBrowser2 successful");

                    // 获取当前URL/location
                    match browser.LocationURL() {
                        Ok(url) => {
                            println!("Location URL: {}", url);
                            // LocationURL通常是file:///C:/path/to/folder格式

                        }
                        Err(e) => {
                            println!("Failed to get LocationURL: {:?}", e);
                        }
                    }

                    // 或者获取LocationName（文件夹名称）
                    match browser.LocationName() {
                        Ok(name) => {
                            println!("Location Name: {}", name);
                        }
                        Err(e) => {
                            println!("Failed to get LocationName: {:?}", e);
                        }
                    }
                }
            }
        }
        CoUninitialize();
        Ok("None3".to_string())
    }
}
fn main() {
    match get_active_explorer_path() {
        Ok(path) => println!("{}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}