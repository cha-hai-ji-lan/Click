from PySide6.QtCore import QSize, QRect
from PySide6.QtGui import QIcon
from PySide6.QtWidgets import QPushButton
from src.gui.root import MainWindowBase, ClickTitleBase
from src.gui import rely as ani


class OfficeCCTitleBar(ClickTitleBase):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.widget.setObjectName("OfficeCCTitleBar")
        self.widget.setProperty("attribute", "CCWindow")
        self.setProperty("attribute", "CCWindow")

    def set_version(self):
        self.version = "0.0.1"

    def set_title_icon(self):
        self.title_ico = QPushButton("Office CC")
        self.title_ico.setIcon(QIcon(":/assets/office_cc.ico"))
        self.title_ico.setProperty("attribute", "CCWindowICON")
        self.title_ico.setIconSize(QSize(32, 32))
        self.title_ico.clicked.connect(lambda: print("点击了CC"))

    def exe_style_son(self):
        pass


class OfficeCCMainWindow(MainWindowBase):
    def __init__(self, parent=None, app=None):
        super().__init__()
        self.app = app
        self.my_parent = parent
        self.setup_page = None  # 设置页面
        self.setObjectName("CCMainWindow")
        self.widget.setObjectName("CCMainWindowWidget")
        self.setProperty("attribute", "CCWindow")
        self.widget.setProperty("attribute", "CCWindow")
        ani.fade_(self)

    def set_screen_size(self):
        self.setGeometry(550, 200, 550, 400)
        self.ori_screen_size = QRect(550, 200, 550, 400)

    def set_title(self):
        self.title_bar = OfficeCCTitleBar(self)
        self.setMenuWidget(self.title_bar)

    def exe_style_son(self):
        pass
