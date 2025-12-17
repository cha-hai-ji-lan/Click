import re
import pywintypes
import win32com.client as win32


class ComExplorer:
    """
    创建Windows Explorer COM接口实例
    """

    def __init__(self):
        """
        初始化窗口实例
        """
        self.obj = win32.Dispatch("Shell.Application").Windows()
        # 调用COM接口，返回一个Shell对象实例，用于后续操作Windows资源管理器相关功能
        # .Windows() 方法返回该Shell应用的所有窗口集合
        self.__CURRENT_PATH__ = []
        self.__PATH__ = []

    def get_explorer_operate_path(self):
        """
        获取Windows资源管理器当前操作的路径
        """

        for path in self.obj.Document.Folder.Self.Path:
            try:
                self.__CURRENT_PATH__.append(path)
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
                self.__PATH__.append(path)
            except AttributeError or pywintypes.com_error:
                continue
        return self.__PATH__

    def get_explorer_windows(self, path=False):
        windows = {}
        hwnd = None
        for window in self.obj:
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


class WinCOM:
    """
    创建对于Windows的COM接口实例操作
    """
