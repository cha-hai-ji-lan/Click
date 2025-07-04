from PySide6.QtCore import QRect, QSize
from PySide6.QtGui import QIcon
from PySide6.QtWidgets import QPushButton, QLabel, QVBoxLayout
from click_rust_depends import click_about
from src.gui.root import MainWindowBase, ClickTitleBase
from src.gui import rely as ani


class AboutTitlePage(ClickTitleBase):

    def __init__(self, parent=None):
        super().__init__(parent)
        self.widget.setObjectName("AboutPageTitle")
        self.widget.setProperty("attribute", "aboutPageWindow")
        self.setProperty("attribute", "aboutPageWindow")

    def set_title_icon(self):
        self.title_ico = QPushButton("关于咔嗒")
        self.title_ico.setObjectName("ClickIcon")
        self.title_ico.setProperty("attribute", "mainWindowICON")
        self.title_ico.setIcon(QIcon(":/assets/Click.ico"))
        self.title_ico.setIconSize(QSize(32, 32))

    def set_version(self):
        self.version = ""

    def exe_style_son(self):
        pass


class AboutPage(MainWindowBase):
    def __init__(self, parent=None, app=None):
        super().__init__()
        self.my_parent = parent
        self.app = app
        self.about_v_layout = QVBoxLayout()
        self.setObjectName("AboutPageWindow")
        self.widget.setObjectName("AboutPageWindowWidget")
        self.setProperty("attribute", "aboutPageWindow")
        self.widget.setProperty("attribute", "aboutPageWindow")
        self.grid_h_layout.setProperty("attribute", "aboutPageWindow")
        self.about_v_layout.setProperty("attribute", "aboutPageWindow")
        self.gui_init()
        ani.fade_(self)

    def gui_init(self):
        for i in range(14):
            about_label = QLabel(click_about(i))
            about_label.setProperty("attribute", "aboutPageWindow")
            self.about_v_layout.addWidget(about_label)
        self.grid_h_layout.addLayout(self.about_v_layout)

    def exe_style_son(self):
        pass

    def set_screen_size(self):
        self.setGeometry(550, 200, 550, 400)
        self.ori_screen_size = QRect(550, 200, 550, 400)

    def set_title(self):
        self.title_bar = AboutTitlePage(self)
        self.setMenuWidget(self.title_bar)
