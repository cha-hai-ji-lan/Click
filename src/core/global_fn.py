import platform
import subprocess
import os
import sys
import winreg


def get_resource_path(relative_path):
    """ 获取打包后资源的绝对路径 """
    if hasattr(sys, '_MEIPASS'):
        # 打包后的临时解压目录
        base_path = sys._MEIPASS
    else:
        # 开发时的当前目录
        base_path = os.path.abspath(r".\src")
        R"""
        注意点：
        relative_path 的开头有一个反斜杠 
        [](file://D:\Object_\Python_\Pyobject_\Click_pro\Cargo.lock)，
        这表示一个绝对路径。
        在 Windows 系统中，
        反斜杠 [](file://D:\Object_\Python_\Pyobject_\Click_pro\Cargo.lock) 
        是路径分隔符，因此 os.path.join 会将 \core\config.toml 
        视为一个绝对路径，而不是相对路径。
        因此，os.path.join 会忽略 base_path，
        直接返回 \core\config.toml，最终结果为 D:\core\config.toml
        """
    return os.path.join(base_path, relative_path)


def get_app_path():
    is_exe = False
    if getattr(sys, 'frozen', False):
        # 打包后的情况
        is_exe = True
        application_path = os.path.dirname(sys.executable)
    else:
        # 开发环境的情况
        application_path = os.path.dirname(os.path.abspath(__file__))
    return [application_path, is_exe]


def is_dark_theme_windows():
    try:
        # 打开注册表键
        reg_key = winreg.OpenKey(winreg.HKEY_CURRENT_USER,
                                 r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
        # 读取 AppsUseLightTheme 的值
        value, _ = winreg.QueryValueEx(reg_key, "AppsUseLightTheme")
        winreg.CloseKey(reg_key)
        # 如果值为 0，则表示深色主题
        return value != 0
    except FileNotFoundError:
        return True


def is_dark_theme_macos():
    try:
        # 使用 AppleScript 获取系统主题
        script = 'tell application "System Events" to tell appearance preferences to get dark mode'
        result = subprocess.run(["osascript", "-e", script], capture_output=True, text=True)
        # 如果返回 "true"，则表示深色主题
        return result.stdout.strip() != "true"
    except Exception as e:
        print(f"获取系统浅色主题: {e}")
        return True


def is_dark_theme_linux():
    try:
        # 读取 GTK 主题设置
        # 如果主题名称包含 "dark"，则认为是深色主题
        theme = os.popen("gsettings get org.gnome.desktop.interface gtk-theme").read().strip().lower()
        return "dark" not in theme
    except Exception as e:
        print(f"获取系统浅色主题: {e}")
        return True


def is_dark_theme():
    def get_os_type():
        system = platform.system().lower()
        if system == "windows":
            return is_dark_theme_windows()
        elif system == "darwin":
            return is_dark_theme_macos()
        elif system == "linux":
            return is_dark_theme_linux()
        else:
            return True  # 系统无法匹配时默认浅色主题

    return get_os_type()


if __name__ == '__main__':
    print(is_dark_theme_windows())
