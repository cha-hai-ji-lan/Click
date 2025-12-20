use windows::Win32::System::Variant::VARIANT;
use windows::{
    core::*,
    Win32::{Foundation::*, System::Com::*, UI::Shell::*},
};

fn get_active_explorer_path() -> Result<Option<String>> {
    unsafe {
        let _ = CoInitialize(None);

        let shell_windows: IShellWindows = match CoCreateInstance(&ShellWindows, None, CLSCTX_ALL) {
            Ok(shell) => shell,
            Err(_) => {
                CoUninitialize();
                return Ok(None);
            }
        };

        let count = match shell_windows.Count() {
            Ok(c) => c,
            Err(_) => {
                CoUninitialize();
                return Ok(None);
            }
        };

        // 查找活动的Explorer窗口
        for i in 0..count {
            if let Ok(window) = shell_windows.Item(&VARIANT::from(i)) {
                if let Ok(folder_view) = window.cast::<IShellFolderViewDual>() {
                    // 修改为 cast
                    if let Ok(folder) = folder_view.Folder() {
                        if let Ok(item) = folder.Items() {
                            if let Ok(path) = item.Item(&VARIANT::from(0)) {
                                CoUninitialize();
                                return Ok(Some(path.Path()?.to_string()));
                            }
                        }
                    }
                }
            }
        }

        CoUninitialize();
        Ok(None)
    }
}
