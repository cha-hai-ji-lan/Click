import pythoncom


class OfficeCheck:
    """Office检查器检查是否可用docto"""

    def __init__(self):
        # 初始化COM库
        pythoncom.CoInitialize()
        self.word_app = None
        self.excel_app = None
        self.power_point_app = None

    def __enter__(self):
        return self

    def __exit__(self):
        self.close()

    def close(self):
        """关闭所有Office应用程序"""
        if self.word_app:
            self.word_app.Quit()
            self.word_app = None
        if self.excel_app:
            self.excel_app.Quit()
            self.excel_app = None
        if self.power_point_app:
            self.power_point_app.Quit()
            self.power_point_app = None
        pythoncom.CoUninitialize()