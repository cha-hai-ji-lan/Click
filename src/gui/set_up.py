from PySide6.QtCore import QRect, QSize, Qt
from PySide6.QtGui import QIcon
from PySide6.QtWidgets import QPushButton, QHBoxLayout, QScrollArea, QWidget, QVBoxLayout, QStackedWidget, QListWidget, \
    QSplitter
from src.gui.root import MainWindowBase, ClickTitleBase
from src.gui import rely as ani

THEME = ani.program_theme()


class SetupTitlePage(ClickTitleBase):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setObjectName("SetupTitlePage")
        self.widget.setObjectName("SetupTitlePageWidget")
        self.widget.setProperty("attribute", "setupPageWindow")
        self.setProperty("attribute", "setupPageWindow")
        self.exe_style_son()

    def set_title_icon(self):
        self.title_ico = QPushButton("首选项")
        self.title_ico.setObjectName("ClickSettingIcon")
        self.title_ico.setProperty("attribute", "mainWindowICON")

    def set_version(self):
        self.version = ""

    def exe_style_son(self):
        if THEME[1]:
            self.title_ico.setIcon(QIcon(":/assets/light_click_setting.png"))
        else:
            self.title_ico.setIcon(QIcon(":/assets/dark_click_setting.png"))
        self.title_ico.setIconSize(QSize(32, 32))


class SetupPage(MainWindowBase):

    def __init__(self, parent=None, app=None):
        super().__init__()
        self.my_parent = parent
        self.app = app
        self.setup_h_layout = QHBoxLayout(self)
        self.left_h_layout = QHBoxLayout(self)
        self.splitter = QSplitter(Qt.Orientation.Horizontal)

        self.roll_right_widget = QScrollArea()
        self.left_widget = QListWidget()  # 左侧选择栏按钮
        self.right_widget = QStackedWidget()

        self.gui_init()
        self.exe_style_son()
        self.set_attribute()
        self.set_event()
        ani.fade_(self)

    def gui_init(self):
        #  左侧选择栏按钮布局

        self.left_widget.setFixedWidth(150)
        self.left_widget.addItem("常规设置")
        self.left_widget.addItem("外观设置")
        self.left_widget.addItem("高级设置")
        self.left_h_layout.addWidget(self.left_widget)
        # 右侧设置栏布局
        self.right_widget.addWidget(OnePage(self))
        self.right_widget.addWidget(TwoPage(self))
        self.right_widget.addWidget(OtherPage(self))

        # 设置界面布局
        self.roll_right_widget.setWidget(self.right_widget)
        self.roll_right_widget.setWidgetResizable(True)
        self.splitter.addWidget(self.left_widget)
        self.splitter.addWidget(self.roll_right_widget)
        self.grid_h_layout.addWidget(self.splitter)

    def exe_style_son(self):
        self.left_widget.setStyleSheet(THEME[0].setting_page_left_qss)

    def set_event(self):
        # 连接点击事件
        self.left_widget.currentRowChanged.connect(self.right_widget.setCurrentIndex)

    def set_screen_size(self):
        self.setGeometry(550, 200, 600, 500)
        self.ori_screen_size = QRect(550, 200, 600, 500)

    def set_attribute(self):
        self.setObjectName("SetupPage")
        self.widget.setObjectName("SetupPageWidget")
        self.setProperty("attribute", "setupPageWindow")
        self.widget.setProperty("attribute", "setupPageWindow")
        self.setup_h_layout.setProperty("attribute", "setupPageWindow")
        self.roll_right_widget.setProperty("attribute", "setupPageWindow")
        self.left_widget.setProperty("attribute", "setupPageWindow")
        self.right_widget.setProperty("attribute", "setupPageWindow")

    def set_title(self):
        self.title_bar = SetupTitlePage(self)
        self.setMenuWidget(self.title_bar)


class OnePage(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.other_h_layout = QHBoxLayout(self)
        self.other_v_layout = QVBoxLayout(self)
        self.other_widget = QWidget()
        self.other_button = QPushButton("页面一")
        self.gui_init()

    def gui_init(self):
        self.other_h_layout.addWidget(self.other_button)
        self.other_widget.setLayout(self.other_h_layout)
        self.other_v_layout.addWidget(self.other_widget)
        self.setLayout(self.other_v_layout)


class TwoPage(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.other_h_layout = QHBoxLayout(self)
        self.other_v_layout = QVBoxLayout(self)
        self.other_widget = QWidget()
        self.other_button = QPushButton("页面二")
        self.gui_init()

    def gui_init(self):
        self.other_h_layout.addWidget(self.other_button)
        self.other_widget.setLayout(self.other_h_layout)
        self.other_v_layout.addWidget(self.other_widget)
        self.setLayout(self.other_v_layout)


class OtherPage(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.other_h_layout = QHBoxLayout(self)
        self.other_v_layout = QVBoxLayout(self)
        self.other_widget = QWidget()
        self.other_button = QPushButton("其他")
        self.gui_init()

    def gui_init(self):
        self.other_h_layout.addWidget(self.other_button)
        self.other_widget.setLayout(self.other_h_layout)
        self.other_v_layout.addWidget(self.other_widget)
        self.setLayout(self.other_v_layout)
