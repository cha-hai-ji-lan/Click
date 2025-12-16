import platform
import re
import subprocess
import threading
import time
from typing import Any

import pythoncom
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



class WinCOM:
    """
    创建对于Windows的COM接口实例操作
    """
