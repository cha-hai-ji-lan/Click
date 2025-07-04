# -*- 中心输入框逻辑处理 -*-
import platform
import re
import subprocess
import threading
import time
from typing import Any

import pythoncom
import pywintypes
import win32com.client
import click_rust_depends

from src.gui import rely

__config__ = rely.get_config()
keywords = list(__config__["WindowsConfig"]["key_word"].keys())
re_str__key_word = '|'.join(__config__["WindowsConfig"]["key_word"].keys())
_COMMAND: list = []
DYNAMIC_COMMAND: list = []
__CURRENT_PATH = []
time_thread = None  # 关机计时器线程控制器
explorer_thread = None  # 资源管理器监控线程控制器
cpu_thread = None  # CPU资源监控线程控制器
gpu_thread = None  # GPU资源监控线程控制器
brain_thread = None  # 指令执行线程控制器


class BrainCommand:

    def __init__(self, parent=None):
        self.my_parent = parent
        self.registry = 0

    def exe_shut_down(self, command_args: list, length: int, sign: bool = False):
        """
        关机倒计时
        :param command_args: 输入的时间参数
        :param length: 输入的参数长度
        :param sign: 是否为重启倒计时
        :return:  None
        """
        self.registry += 1
        rely.stretch_h_(self.my_parent.shut_down_info_button, 200, 0, 200)
        match length:
            case 1:
                hour = 0
                minute = 0
                second = command_args[0]
            case 2:
                hour = 0
                minute = command_args[0]
                second = command_args[1]
            case 3:
                hour = command_args[0]
                minute = command_args[1]
                second = command_args[2]
            case _:
                hour = 0
                minute = 0
                second = 0
        try:
            if command_args[0] < 0:
                hour = 0
        except ValueError:
            hour = 0
        try:
            if second < 0:
                second = 0
            elif second > 59:
                second_extr = second // 60
                minute += second_extr
                second = second - second_extr * 60
        except ValueError:
            second = 0
        try:
            if minute < 0:
                minute = 0
            elif minute > 59:
                min_extr = minute // 60
                hour += min_extr
                minute = minute - min_extr * 60
        except ValueError:
            minute = 0
        time_total = hour * 3600 + minute * 60 + second
        start_time = time.time()
        while self.my_parent.shut_down_running:  # 定期检查标志位
            if not sign:
                text = timer_event(hour, minute, second, start_time)
                time.sleep(1.0)
                end_time = time.time()
                self.my_parent.tray_icon.setToolTip(text)
                self.my_parent.shut_down_info_button.setText(F"重置[{text}]")
                if end_time - start_time >= time_total:
                    self.my_parent.shut_down_running = False
                    self.registry -= 1
                    shutdown()
                    break
            else:
                text = timer_event(hour, minute, second, start_time, True)
                time.sleep(1.0)
                end_time = time.time()
                self.my_parent.tray_icon.setToolTip(text)
                self.my_parent.shut_down_info_button.setText(F"重置[{text}]")
                if end_time - start_time >= time_total:
                    self.my_parent.shut_down_running = False
                    self.registry -= 1
                    restart_computer()
                    break
        else:
            self.my_parent.shut_down_running = False
            self.registry -= 1
        self.my_parent.tray_icon.setToolTip("...")


class TimeShutdown:
    """
    倒计时类用于·控制电脑关机或重启时间
    """

    def __init__(self):

        self.stop = False
        self.not_fin = True

    def stop_now(self):
        self.stop = True

    def wait_me_moment(self, key_list: list, parent=None, length: int = 1):
        match length:
            case 1:
                hour = 0
                minute = 0
                second = key_list[0]
            case 2:
                hour = 0
                minute, second = key_list[0: 2]
            case 3:
                hour, minute, second = key_list[0: 4]
            case _:
                hour = 0
                minute = 0
                second = 0
        try:
            if key_list[0] < 0:
                hour = 0
        except ValueError:
            hour = 0
        try:
            if second < 0:
                second = 0
            elif second > 59:
                second_extr = second // 60
                minute += second_extr
                second = second - second_extr * 60
        except ValueError:
            second = 0
        try:
            if minute < 0:
                minute = 0
            elif minute > 59:
                min_extr = minute // 60
                hour += min_extr
                minute = minute - min_extr * 60
        except ValueError:
            minute = 0
        time_total = hour * 3600 + minute * 60 + second
        start_time = time.time()
        while parent.shut_down_running and not self.stop:  # 定期检查标志位
            text = timer_event(hour, minute, second, start_time)
            time.sleep(1.0)
            end_time = time.time()
            parent.tray_icon.setToolTip(text)
            parent.shut_down_info_button.setText(F"重置[{text}]")
            if end_time - start_time >= time_total:
                parent.shut_down_running = False
                shutdown()
                break
        else:
            parent.shut_down_running = False
            self.not_fin = False
        parent.tray_icon.setToolTip("...")
        self.not_fin = False


def brain_distribute(key_list: list, *, parent=None) -> None:
    global brain_thread
    key_list = [convert_number(x) for x in key_list]
    if not preprocessing(key_list[0], key_list, parent):
        lexer(key_list)
        brain_processing(parent)


def brain_processing(parent=None):
    global brain_thread
    try:
        if brain_thread.is_alive():
            pass
        else:
            brain_thread = threading.Thread(target=lambda: brain_execute(parent))
            brain_thread.start()
            time.sleep(0.1)
            try:
                if not parent.jump_widget.isVisible() and _COMMAND[0][0] in ["关机", "重启", "OFFICE", "监控"]:
                    parent.toggle_left_jump_dock()
                    match _COMMAND[0][0]:
                        case "关机":
                            rely.stretch_h_(parent.shut_down_info_button, 200, 0, 200)
                        case _:
                            pass
                else:
                    match _COMMAND[0][0]:
                        case "关机":
                            rely.stretch_h_(parent.shut_down_info_button, 200, 0, 200)
                        case "关于":
                            parent.open_about_page()
                            _COMMAND.pop(0)
                        case _:
                            pass
            except IndexError:
                parent.main_line_edit.ask_timer.stop()
    except AttributeError:  # 指令运行主线程并未启动
        brain_thread = threading.Thread(target=lambda: brain_execute(parent))
        brain_thread.start()
        time.sleep(0.1)
        try:
            if not parent.jump_widget.isVisible() and _COMMAND[0][0] in ["关机", "重启", "OFFICE", "监控"]:
                parent.toggle_left_jump_dock()
                match _COMMAND[0][0]:
                    case "关机":
                        rely.stretch_h_(parent.shut_down_info_button, 200, 0, 200)
                    case _:
                        pass
            else:
                match _COMMAND[0][0]:
                    case "关机":
                        rely.stretch_h_(parent.shut_down_info_button, 200, 0, 200)
                    case "关于":
                        parent.open_about_page()
                        _COMMAND.pop(0)
                    case _:
                        pass
        except IndexError:
            parent.main_line_edit.ask_timer.stop()


def brain_execute(parent=None):
    """
    子线程耗时任务·
    注意不要操控GUI如果要操控GUI让问询机制处理
    :param parent:
    :return:
    """
    global _COMMAND
    brain = BrainCommand(parent)
    try:
        match _COMMAND[0][0]:
            case "OFFICE":
                pass
            case "后":
                pass
            case "重启":
                if parent.shut_down_running:
                    parent.main_line_edit.setPlaceholderText("请重置后再设置新的重启时间")
                parent.shut_down_running = True
                brain.exe_shut_down(_COMMAND[0][1:], len(_COMMAND[0]) - 1, True)
                try:
                    _COMMAND.pop(0)
                except IndexError:
                    pass
            case "关机":
                if parent.shut_down_running:
                    parent.main_line_edit.setPlaceholderText("请重置后再设置新的关机时间")
                parent.shut_down_running = True
                brain.exe_shut_down(_COMMAND[0][1:], len(_COMMAND[0]) - 1)
                try:
                    _COMMAND.pop(0)
                except IndexError:
                    pass
            case _:
                pass
    except IndexError:
        pass


def convert_number(item):
    """
    将切片命令条目尝试转换成数字
    :param item: 待转换的条目
    :return: None
    """
    if item.find(".") != -1:
        try:
            return float(item)
        except ValueError:
            try:
                return item.upper()
            except AttributeError:
                return item
    else:
        try:
            return int(item)
        except ValueError:
            try:
                return item.upper()
            except AttributeError:
                return item


def lexer(key_list: list) -> None:
    """
    词法分析
    接受输入命令切片根据关键词参数定义进行词法分析
    进行命令切片
    :param key_list: 关键词列表
    :return: None
    """
    global _COMMAND
    key_index = 1
    for key in keywords:
        try:
            key_index += key_list.index(key)
            match key:
                case "OFFICE":
                    pass
                case "后":
                    pass
                case "关于":
                    item = ["关于"]
                    _COMMAND.append(item)
                case "重启":
                    item = ["重启"]
                    for i in key_list[key_index: key_index + 3]:
                        if isinstance(i, int | float):
                            item.append(i)
                        else:
                            break
                    if len(item) == 1:
                        item.extend(__config__["WindowsConfig"]["shutdown"]["default_reboot_time"])
                    _COMMAND.append(item)
                case "关机":
                    item = ["关机"]
                    for i in key_list[key_index: key_index + 3]:
                        if isinstance(i, int | float):
                            item.append(i)
                        else:
                            break
                    if len(item) == 1:
                        item.extend(__config__["WindowsConfig"]["shutdown"]["default_shutdown_time"])
                    _COMMAND.append(item)

        except ValueError:
            pass


def preprocessing(text: Any, key_list: list, parent=None) -> bool:
    """
    默认命令预处理
    :param text: 输入的第一段
    :param key_list: 输入的参数
    :param parent: 父窗口
    :return: bool 是否启用默认程序
    """
    global time_thread
    shut_down = TimeShutdown()
    if isinstance(text, int):  # 输入的第一段是数字则启动默认关机
        time_cache = [text]
        try:
            time_cache = [int(i) for i in key_list[0:3]]  # 输入的数字为3位
        except ValueError:
            try:
                time_cache = [int(i) for i in key_list[0:2]]  # 输入的数字为2位
            except ValueError:
                pass
        finally:
            time_thread = threading.Thread(
                target=lambda: shut_down.wait_me_moment(key_list, parent, length=len(time_cache)))
            if parent.shut_down_running:
                parent.main_line_edit.setPlaceholderText("请重置后再设置新的关机时间")
                return True
            else:
                start_timer(parent, time_thread)
                return True
    elif text == __config__["keys"]["global_setting"][0]:
        time_thread = threading.Thread(
            target=lambda: shut_down.wait_me_moment(__config__["WindowsConfig"]["shutdown"]["default_shutdown_time"],
                                                    parent, length=3))
        start_timer(parent, time_thread)
        return True
    else:
        return False


def start_timer(parent=None, timer_thread=None):
    """
    默认命令启动器
    :param parent: 父窗口
    :param timer_thread: 默认命令执行线程
    :return:
    """
    parent.shut_down_running = True
    timer_thread.start()
    rely.stretch_h_(parent.shut_down_info_button, 200, 0, 200)
    if parent.jump_widget.isVisible():
        pass
    else:
        parent.toggle_left_jump_dock()


def timer_event(hour, minute, second, start_time, signal=False):
    """
    计时序列化
    :param hour:
    :param minute:
    :param second:
    :param start_time:
    :param signal:
    :return:
    """
    end_time = time.time()
    now_time = start_time - end_time
    if now_time is not None:
        current_time = time.time()
        wait = hour * 3600 + minute * 60 + second
        wait -= current_time - start_time
        hour_ = wait // 3600
        min_ = (wait - hour_ * 3600) // 60
        sec_ = wait - hour_ * 3600 - min_ * 60
        if not signal:
            return f"{int(hour_)}:{int(min_)}:{int(sec_)}后关机"
        else:
            return f"{int(hour_)}:{int(min_)}:{int(sec_)}后重启"
    else:
        return "..."


def shutdown():
    system_name = platform.system()
    if system_name == "Windows":
        subprocess.run(["shutdown", "/s", "/t", "0"])
    elif system_name == "Linux" or system_name == "Darwin":  # Darwin 适用于 macOS
        subprocess.run(["sudo", "shutdown", "-h", "now"], check=True)
    else:
        print("此操作系统不支持")


def restart_computer():
    system_name = platform.system()
    if system_name == "Windows":
        subprocess.run(["shutdown", "/r", "/t", "0"])
    elif system_name in ["Linux", "Darwin"]:  # Darwin 是 macOS
        subprocess.run(["sudo", "reboot"])
    else:
        print("此操作系统不支持")


#  获取所有Explorer窗口的目录
def get_explorer_paths() -> list[str]:
    """

    :return: 当前打开explorer正在操作的窗口
    :rtype: list[str]
    """
    global __CURRENT_PATH
    shell = win32com.client.Dispatch("Shell.Application")
    paths = []
    __CURRENT_PATH = []
    for window in shell.Windows():
        try:
            path = window.Document.Folder.Self.Path
            __CURRENT_PATH.append(path)
            if path.find("::{645FF040-5081-101B-9F08-00AA002F954E}") != -1:
                path = path.replace("::{645FF040-5081-101B-9F08-00AA002F954E}", "回收站:\\")
            elif path.find("::{F874310E-B6B7-47DC-BC84-B9E6B38F5903}") != -1:
                path = path.replace("::{F874310E-B6B7-47DC-BC84-B9E6B38F5903}", "主文件夹:\\")
            elif path.find("::{20D04FE0-3AEA-1069-A2D8-08002B30309D}") != -1:
                path = path.replace("::{20D04FE0-3AEA-1069-A2D8-08002B30309D}", "此电脑:\\")
            elif path.find("::{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}") != -1:
                path = path.replace("::{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}", "网络:\\")
            elif path.find("::{031E4825-7B94-4DC3-B131-E946B44C8DD5}") != -1:
                path = path.replace("::{031E4825-7B94-4DC3-B131-E946B44C8DD5}", "库:\\")
            elif path.find("::{E88865EA-0E1C-4E20-9AA6-EDCD0212C87C}") != -1:
                path = path.replace("::{E88865EA-0E1C-4E20-9AA6-EDCD0212C87C}", "图库:\\")
            elif path.find("::{26EE0668-A00A-44D7-9371-BEB064C98683}") != -1:
                path = path.replace("::{26EE0668-A00A-44D7-9371-BEB064C98683}", "控制面板:\\")
                if path.find("\\0") != -1:
                    path = path.replace("\\0", "\\所有控制面板项")
                if path.find("\\1") != -1:
                    path = path.replace("\\1", "\\外观和个性化")
                    if path.find("::{7B81BE6A-CE2B-4676-A29E-EB907A5126C5}") != -1:
                        path = path.replace("::{7B81BE6A-CE2B-4676-A29E-EB907A5126C5}", "\\轻松使用设置中心")
                    elif path.find("::{93412589-74D4-4E4E-AD0E-E0CB621440FD}") != -1:
                        path = path.replace("::{93412589-74D4-4E4E-AD0E-E0CB621440FD}", "\\字体设置")
                if path.find("\\2") != -1:
                    path = path.replace("\\2", "\\硬件和声音")
                if path.find("\\3") != -1:
                    path = path.replace("\\3", "\\网络和 Internet")
                # if path.find("\\4") != -1:
                #     path = path.replace("\\4", "\\系统和安全") 非常奇怪没有 4
                if path.find("\\5") != -1:
                    path = path.replace("\\5", "\\系统和安全")
                    if path.find("::{B98A2BEA-7D42-4558-8BD1-832F41BAC6FD}") != -1:
                        path = path.replace("::{B98A2BEA-7D42-4558-8BD1-832F41BAC6FD}", "\\备份和还原(Windows 7)")
                    elif path.find("::{BB64F8A7-BEE7-4E1A-AB8D-7D8273F7FDB6}") != -1:
                        path = path.replace("::{BB64F8A7-BEE7-4E1A-AB8D-7D8273F7FDB6}", "\\安全和维护")
                    elif path.find("::{4026492F-2F69-46B8-B9BF-5654FC07E423}") != -1:
                        path = path.replace("::{4026492F-2F69-46B8-B9BF-5654FC07E423}", "\\Windows Defender 防火墙")
                    elif path.find("::{025A5937-A6BE-4686-A844-36FE4BEC8B6D}") != -1:
                        path = path.replace("::{025A5937-A6BE-4686-A844-36FE4BEC8B6D}", "\\电源选项")
                    elif path.find("::{F6B6E965-E9B2-444B-9286-10C9152EDBC5}") != -1:
                        path = path.replace("::{F6B6E965-E9B2-444B-9286-10C9152EDBC5}", "\\文件历史记录")
                    elif path.find("::{F942C606-0914-47AB-BE56-1321B8035096}") != -1:
                        path = path.replace("::{F942C606-0914-47AB-BE56-1321B8035096}", "\\存储空间")
                    elif path.find("::{ECDB0924-4208-451E-8EE0-373C0956DE16}") != -1:
                        path = path.replace("::{ECDB0924-4208-451E-8EE0-373C0956DE16}", "\\工作文件夹")
                    elif path.find("::{D20EA4E1-3957-11D2-A40B-0C5020524153}") != -1:
                        path = path.replace("::{D20EA4E1-3957-11D2-A40B-0C5020524153}", "\\Windows 工具")
                if path.find("\\6") != -1:
                    path = path.replace("\\6", "\\时钟和区域")
                if path.find("\\7") != -1:
                    path = path.replace("\\7", "\\轻松使用")
                if path.find("\\8") != -1:
                    path = path.replace("\\8", "\\程序")
                if path.find("\\9") != -1:
                    path = path.replace("\\9", "\\用户帐户")
            paths.append(path)
        except AttributeError or pywintypes.com_error:
            continue
    return paths


# 获取当前操作目录
def get_explorer_windows(path=False):
    """获取所有Explorer窗口的句柄和路径"""
    shell = win32com.client.Dispatch("Shell.Application")
    windows = {}
    hwnd = None
    for window in shell.Windows():
        try:
            # 获取窗口句柄和路径
            hwnd = window.HWND
            path = window.Document.Folder.Self.Path
            windows[hwnd] = path
        except AttributeError:
            # 忽略无法获取路径的窗口（如“此电脑”）
            continue
    if path:
        if re.search(r'^.*?::', windows[hwnd]):
            return "explorer:\\"
        else:
            return windows[hwnd]
    return windows


def monitor_explorer(interval=1):
    """监控Explorer窗口路径变化"""
    previous_windows = {}

    while True:
        current_windows = get_explorer_windows()

        # 检测新打开的窗口或路径变化
        for hwnd, path in current_windows.items():
            if hwnd not in previous_windows:
                print(f"[新窗口] 路径: {path}")
            elif previous_windows[hwnd] != path:
                print(f"[路径变化] 窗口 {hwnd}: {previous_windows[hwnd]} → {path}")

        # 检测关闭的窗口
        for hwnd in previous_windows:
            if hwnd not in current_windows:
                print(f"[窗口关闭] 原路径: {previous_windows[hwnd]}")

        previous_windows = current_windows.copy()
        time.sleep(interval)


def get_selected_files_in_explorer():
    """
    获取当前Explorer窗口中选中的文件路径
    动态问询将会访问
    :return:
    """
    pythoncom.CoInitialize()  # 初始化 COM 库（多线程环境需要）

    shell = win32com.client.Dispatch("Shell.Application")
    selected_files = []

    for window in shell.Windows():
        try:
            # 获取浏览器窗口的文档对象
            doc = window.Document
            if hasattr(doc, "SelectedItems"):
                items = doc.SelectedItems()
                for item in items:
                    selected_files.append(item.Path)
        except Exception as e:
            print(f"读取窗口失败: {e}")

    pythoncom.CoUninitialize()  # 清理 COM
    return selected_files


def dynamic_checkup(grand_parent, parent):
    """
    动态窗口检查问询处理器
    <功能： 通过问询器问询此函数达到动态输入命令时会有相关功能动态响应>\n
    输入指令时，有些指令附有伴随响应功能或只有伴随响应功能，\n
    问询处理器的功能就是检测当前命令框输入关键字有无含有伴随功能\n
    如果发现有伴随功能则立刻响应展开功能便于用户操作
    :param grand_parent: 主窗口
    :param parent: 待动态检查窗口
    :return: None
    """
    global DYNAMIC_COMMAND
    current_text = parent.toPlainText().strip()  # 获取文本框内容
    # 获取当前输入的命令
    DYNAMIC_COMMAND = re.findall(re_str__key_word, current_text.upper()) or [""]
    if grand_parent.list_view.isVisible() and not any(
            item in ["监控", "当前目录", "OFFICE"] for item in DYNAMIC_COMMAND):
        # explorer 监视器存在 并且无监控发出逻辑关键词
        grand_parent.worker_explorer_stop()  # 没有文件操作预发出命令监控停止维持
    elif grand_parent.cpu_monitor_view.is_open and not any(item in ["CPU", "内存"] for item in DYNAMIC_COMMAND):
        grand_parent.cpu_monitor_view.close_myself()
    elif grand_parent.gpu_monitor_view.is_open and "GPU" not in DYNAMIC_COMMAND:
        grand_parent.gpu_monitor_view.close_myself()
    if any(item in ["监控", "当前目录", "OFFICE"] for item in DYNAMIC_COMMAND) and not any(
            item in ["CPU", "GPU"] for item in DYNAMIC_COMMAND):
        match parent.order_object:
            case "OFFICE":
                parent.file_monitoring(0)
            case "监控":
                parent.file_monitoring(1)
            case "当前目录":
                parent.file_monitoring(2)
            case _:
                pass
        selected_file = get_selected_files_in_explorer()
        if DYNAMIC_COMMAND[-1] and len(DYNAMIC_COMMAND) > 1:  # 检测当前输入的命令是否足够进行进一步判断
            # 检测检测根命令是否为文件操作命令
            # 并且 当且explorer是否处于文件以选择状态状态
            # 并且 检测从属命令是否以加入到输入行中
            # 并且 是否已经执行当前的 “将” 命令
            if (DYNAMIC_COMMAND[-2] in ["当前目录", "OFFICE"] and selected_file
                    and "#选择的文件" in DYNAMIC_COMMAND[-1] and parent.communicate["will"]):
                parent.communicate["will"] = False
                grand_parent.main_line_edit.setText(current_text + ' #选择的文件')
        # 如果当前没有选择的文件就刷新 “将” 命令 附加从属命令的权限
        if not selected_file:
            parent.communicate["will"] = True
            if parent.order_object == "#选择的文件":
                grand_parent.main_line_edit.setText(current_text[:-7])
    if DYNAMIC_COMMAND[-1] == "CPU" and DYNAMIC_COMMAND[-2] in ["监控", "和"] and "监控" in DYNAMIC_COMMAND:
        if DYNAMIC_COMMAND[DYNAMIC_COMMAND.index("监控") + 1] != "和" and grand_parent.list_view.isVisible():
            # 如果 关键字 “监控” 后面没有 “和” 关键字 就停止Explorer.exe的监控并只开启CPU监控
            grand_parent.worker_explorer_stop()
        if not grand_parent.cpu_monitor_view.is_open and "内存" not in DYNAMIC_COMMAND:
            grand_parent.cpu_monitor_view.open_myself()
    elif DYNAMIC_COMMAND[-1] == "内存" and DYNAMIC_COMMAND[-2] in ["监控", "和"] and "内存" in DYNAMIC_COMMAND:
        if DYNAMIC_COMMAND[DYNAMIC_COMMAND.index("监控") + 1] != "和" and grand_parent.list_view.isVisible():
            # 如果 关键字 “监控” 后面没有 “和” 关键字 就停止Explorer.exe的监控并只开启内存监控
            grand_parent.worker_explorer_stop()
        if not grand_parent.cpu_monitor_view.is_open and "CPU" not in DYNAMIC_COMMAND:  # 检测CPU监控器是否开启
            grand_parent.cpu_monitor_view.open_myself(False)
    if DYNAMIC_COMMAND[-1] == "GPU" and DYNAMIC_COMMAND[-2] in ["监控", "和"] and "监控" in DYNAMIC_COMMAND:
        if DYNAMIC_COMMAND[DYNAMIC_COMMAND.index("监控") + 1] != "和" and grand_parent.list_view.isVisible():
            # 如果 关键字 “监控” 后面没有 “和” 关键字 就停止Explorer.exe的监控并只开启GPU监控
            grand_parent.worker_explorer_stop()
        if not grand_parent.gpu_monitor_view.is_open:
            grand_parent.gpu_monitor_view.open_myself()
    else:
        grand_parent.gpu_monitor_view.is_open = False


def clean_dynamic_commands():
    global DYNAMIC_COMMAND
    DYNAMIC_COMMAND = []


# 资源管理器监控器
def monitor_path_explorer(parent=None):
    """
    监控资源管理器当前操作目录监控器
    :param parent:
    :return:
    """
    global explorer_thread
    explorer_thread = threading.Thread(target=lambda: explorer_update(parent))
    explorer_thread.start()


def monitor_resources_cpu(parent=None, mode=True):
    """
    监控CPU资源线程启动器
    :param parent: 父窗口对象
    :param mode:  模式
    :return: None
    """
    global cpu_thread
    if mode:
        cpu_thread = threading.Thread(target=lambda: cpu_resources_update(parent))
        cpu_thread.start()
    else:
        cpu_thread = threading.Thread(target=lambda: cpu_memory_update(parent))
        cpu_thread.start()


def monitor_resources_gpu(parent=None):
    """
    监控GPU资源线程启动器
    :param parent:
    :return:
    """
    global gpu_thread
    gpu_thread = threading.Thread(target=lambda: gpu_resources_update(parent))
    gpu_thread.start()


def explorer_update(parent=None):
    """
    监控资源管理器当前操作目录更新·
    :param parent: 主窗口对象
    :return: None
    """
    while parent.worker_explorer_running:
        parent.explorer_monitor_acquisition(get_explorer_paths())
        time.sleep(1)


def cpu_resources_update(parent=None):
    """
    CPU监控器更新·
    :param parent: 主窗口对象
    :return: None
    """
    while parent.is_open:
        monitor_obj = click_rust_depends.cpu_info()
        for index, data in enumerate(monitor_obj[:10]):
            parent.cpu_monitor_acquisition(index, data)  # pos (pid data)
            time.sleep(0.15)
        time.sleep(2)


def cpu_memory_update(parent=None):
    """
    内存更新
    :param parent: 主窗口对象
    :return: None
    """
    while parent.is_open:
        monitor_obj = click_rust_depends.cpu_memory_info()
        for index, data in enumerate(monitor_obj[:10]):
            parent.memory_monitor_acquisition(index, data)  # pos (pid data)
            time.sleep(0.15)
        time.sleep(2)


def gpu_resources_update(parent=None):
    """
    GPU监控器更新
    :param parent:
    :return:
    """
    while parent.is_open:
        parent.gpu_monitor_acquisition(click_rust_depends.gpu_info())  # pos (pid data)
        time.sleep(0.5)