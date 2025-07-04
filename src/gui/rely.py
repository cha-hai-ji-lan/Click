import os
from typing import Any
import toml
import sys
from PySide6.QtCore import QPropertyAnimation, QByteArray, QEasingCurve, Qt
from PySide6.QtWidgets import QPushButton, QVBoxLayout, QFrame, QDockWidget, QWidget, QFileDialog
from src.core.global_fn import is_dark_theme, get_app_path, get_resource_path
from src.gui.qss import dark_style as dark_sty_
from src.gui.qss import light_style as light_sty_

_CONFIG = None
exe_position = get_app_path()
if exe_position[1] and os.path.exists(os.path.join(exe_position[0], R"bin\config\config.toml")):  # 存在配置文件
    config_path = os.path.join(exe_position[0], R"bin\config\config.toml")
else:
    config_path = get_resource_path(r"bin\config\config.toml")
with open(config_path, "r", encoding="utf-8") as f:
    _CONFIG = toml.load(f)

IS_LIGHT = True
dark_style = dark_sty_
light_style = light_sty_
if is_dark_theme() and _CONFIG["WindowsConfig"]["global_theme"] == 0:
    from src.gui.qss import light_style as _sty_
elif _CONFIG["WindowsConfig"]["global_theme"] == 1:  # 1: 暗色调
    from src.gui.qss import dark_style as _sty_

    IS_LIGHT = False
elif _CONFIG["WindowsConfig"]["global_theme"] == 2:  # 2: 亮色调
    from src.gui.qss import light_style as _sty_
else:
    from src.gui.qss import dark_style as _sty_

    IS_LIGHT = False


def get_config():
    return _CONFIG


def program_theme(theme: int = 0):
    """
    程序主题样式选择器
    :param theme: 0: 跟随系统 1: 暗色 2: 亮色
    :return: [主题样式, 是否是浅色主题]
    """
    global IS_LIGHT
    if theme == 0:
        theme_style = _sty_
    elif theme == 1:
        theme_style = dark_style
        IS_LIGHT = False
    elif theme == 2:
        theme_style = light_style
    else:
        theme_style = _sty_
    return [theme_style, IS_LIGHT]


THEME = program_theme()


def fade_(parent, time_: int = 250, start_value: float = 0, end_value: float = 1, *, hide: bool = False):
    """
    淡入淡出动画
    默认淡入
    windowOpacity：用于控制全体控件
    opacity：用于控制控件本身
    """
    if start_value < end_value:
        parent.show()
    parent.animation = QPropertyAnimation(parent, QByteArray(b"windowOpacity"))
    parent.animation.setDuration(time_)  # 动画持续时间 1 秒
    parent.animation.setStartValue(start_value)  # 起始值：完全不透明
    parent.animation.setEndValue(end_value)  # 结束值：完全透明
    parent.animation.setEasingCurve(QEasingCurve.Type.InOutQuad)  # 缓动曲线
    parent.animation.start()
    if hide:
        parent.animation.finished.connect(parent.hide)
    if start_value > end_value and hide is False:
        parent.animation.finished.connect(parent.close)


def fade_units(parent, /, time_: int = 400, start_value: float = 1, end_value: float = 0):
    """
    配合型渐淡动画
    :param parent:
    :param time_:
    :param start_value:
    :param end_value:
    :return:
    """
    parent.animation = QPropertyAnimation(parent, QByteArray(b"windowOpacity"))
    parent.animation.setDuration(time_)  # 动画持续时间 1 秒
    parent.animation.setStartValue(start_value)  # 起始值：完全不透明
    parent.animation.setEndValue(end_value)  # 结束值：完全透明
    parent.animation.setEasingCurve(QEasingCurve.Type.InOutQuad)  # 缓动曲线
    return parent.animation.start


def stretch_h_(parent, /, time_: int = 400, start: int = 0, end: int = 200,
               animation_obj: Any | None = None, *ani_obj_args):
    """
    横向伸展窗口
    :param parent: 父类
    :param time_: 动画时间
    :param start: 起始值
    :param end: 结束值
    :param animation_obj: 其他的配合动画
    :param ani_obj_args: 配合动画参数 [PARENT, TIME, START, END]
    :return: None
    """

    parent.animation = QPropertyAnimation(parent, QByteArray(b"maximumWidth"))
    parent.animation.setDuration(time_)  # 动画持续时间（毫秒）
    parent.animation.setStartValue(start)
    parent.animation.setEndValue(end)
    parent.animation.setEasingCurve(QEasingCurve.Type.InQuad)  # 缓动曲线
    if animation_obj is not None and ani_obj_args:
        cooperate_animation = animation_obj(ani_obj_args[0], ani_obj_args[1], ani_obj_args[2], ani_obj_args[3])
        cooperate_animation()
    parent.animation.start()
    if end == 0:
        parent.animation.finished.connect(lambda: parent.hide())
    else:
        parent.show()


def stretch_v_(parent, /, time_: int = 400, start: int = 0, end: int = 200,
               animation_obj: Any | None = None, *ani_obj_args):
    """
    纵向伸展窗口
    :param parent: 父类
    :param time_: 动画时间
    :param start: 起始值
    :param end: 结束值
    :param animation_obj: 其他的配合动画
    :param ani_obj_args: 配合动画参数 [PARENT, TIME, START, END]
    :return: None
    """
    # 初始化导航栏动画
    if start < end:
        parent.show()
    parent.animation = QPropertyAnimation(parent, QByteArray(b"maximumHeight"))
    parent.animation.setDuration(time_)  # 动画持续时间（毫秒）
    parent.animation.setStartValue(start)
    parent.animation.setEndValue(end)
    parent.animation.setEasingCurve(QEasingCurve.Type.InQuad)
    if animation_obj is not None and ani_obj_args:
        cooperate_animation = animation_obj(ani_obj_args[0], ani_obj_args[1], ani_obj_args[2], ani_obj_args[3])
        cooperate_animation()
    parent.animation.start()
    if end == 0:
        parent.animation.finished.connect(lambda: parent.show(), parent.hide())


def mouse_right_click_dis_menu(parent=None, name=None):
    match name:
        case "mainWindow":
            return main_menu(parent)
        case "mainWindowICON":
            return icon_menu(parent)
        case "aboutPageWindow":
            return normal_menu(parent)
        case "mainWindowSettingButton":
            return setup_button_menu(parent)
        case _:
            return normal_menu(parent)


def horizontal_line():
    line = QFrame()
    line.setFrameShape(QFrame.Shape.HLine)  # 设置为水平线
    # horizontal_line.setFrameShadow(QFrame.Sunken)  # 设置阴影效果（可选）
    line.setLineWidth(1)  # 线宽
    line.setMidLineWidth(0)  # 中间线宽
    return line


def vertical_line():
    line = QFrame()
    line.setFrameShape(QFrame.Shape.VLine)  # 设置为水平线
    # horizontal_line.setFrameShadow(QFrame.Sunken)  # 设置阴影效果（可选）
    line.setLineWidth(1)  # 线宽
    line.setMidLineWidth(0)  # 中间线宽
    return line


def main_menu(parent=None):
    """
    主窗口右击显示菜单
    :param parent: 父类
    :return: QMenu对象
    """
    # 注意需要将所有选项加到父类中否则会被GC清理无法显示
    parent.menu_ = ClickDockWidget(parent)
    parent.content_widget = QWidget()
    parent.content_widget.setObjectName("menu_widget")
    parent.menu_layout = QVBoxLayout()
    parent.action1 = QPushButton("首选项")
    parent.action1.clicked.connect(lambda: parent.open_setup_page())
    parent.action2 = QPushButton("关于")
    parent.action2.clicked.connect(lambda: parent.open_about_page())
    parent.action3 = QPushButton("退出")
    parent.action3.clicked.connect(lambda: closes(parent, fade_time=1, mode=1))
    parent.menu_layout.addWidget(parent.action1)
    parent.menu_layout.addWidget(parent.action2)
    parent.menu_layout.addWidget(horizontal_line())
    parent.menu_layout.addWidget(parent.action3)
    parent.content_widget.setLayout(parent.menu_layout)
    # 将 content_widget 设置为 QDockWidget 的主控件
    parent.menu_.setWidget(parent.content_widget)
    parent.menu_.setStyleSheet(THEME[0].menu_main_qss)
    return parent.menu_


def normal_menu(parent=None):
    """
    一般窗口右击显示菜单
    :param parent: 父类
    :return: QMenu对象
    """
    # 注意需要将所有选项加到父类中否则会被GC清理无法显示
    parent.menu_ = ClickDockWidget(parent)
    parent.content_widget = QWidget()
    parent.content_widget.setObjectName("menu_widget")
    parent.menu_layout = QVBoxLayout()
    parent.action1 = QPushButton("返回首页")
    parent.action1.clicked.connect(lambda: closes(parent, fade_time=1, menu=True))
    parent.action1.setObjectName("menu_attention_item1")
    parent.action2 = QPushButton("退出")
    parent.action2.clicked.connect(lambda: closes(parent, fade_time=1, mode=1, menu=True))
    parent.action2.setProperty("type", "menu_attention_item2")
    parent.menu_layout.addWidget(horizontal_line())
    parent.menu_layout.addWidget(parent.action1)
    parent.menu_layout.addWidget(parent.action2)
    parent.content_widget.setLayout(parent.menu_layout)
    parent.menu_.setWidget(parent.content_widget)
    parent.menu_.setStyleSheet(THEME[0].menu_main_qss)
    return parent.menu_


def setup_button_menu(parent=None):
    """
    一般窗口右击显示菜单
    :param parent: 父类
    :return: QMenu对象
    """
    # 注意需要将所有选项加到父类中否则会被GC清理无法显示
    parent.menu_ = ClickDockWidget(parent)
    parent.content_widget = QWidget()
    parent.content_widget.setObjectName("menu_widget")
    parent.menu_layout = QVBoxLayout()
    parent.action1 = QPushButton("关于")
    parent.action1.clicked.connect(lambda: parent.open_about_page())
    parent.action2 = QPushButton("退出")
    parent.action2.clicked.connect(lambda: closes(parent, fade_time=1, mode=1, menu=True))
    parent.action2.setProperty("type", "menu_attention_item2")
    parent.menu_layout.addWidget(parent.action1)
    parent.menu_layout.addWidget(horizontal_line())
    parent.menu_layout.addWidget(parent.action2)
    parent.content_widget.setLayout(parent.menu_layout)
    parent.menu_.setWidget(parent.content_widget)
    parent.menu_.setStyleSheet(THEME[0].menu_main_qss)
    return parent.menu_


def icon_menu(parent=None):
    # 注意需要将所有选项加到父类中否则会被GC清理无法显示
    parent.menu_ = ClickDockWidget(parent)
    parent.content_widget = QWidget()
    parent.content_widget.setObjectName("menu_widget")
    parent.menu_layout = QVBoxLayout()
    if parent.title_bar.key_dict["max_screen"] == 0:
        parent.action1 = QPushButton("还原")
    else:
        parent.action1 = QPushButton("最大化")
    parent.action1.clicked.connect(lambda: parent.title_bar.zoom_in_event(parent.title_bar.enlarge_button))
    parent.action2 = QPushButton("最小化")
    parent.action2.clicked.connect(lambda: parent.title_bar.zoom_in_event(parent.title_bar.enlarge_button, True))
    parent.action3 = QPushButton("退出")
    parent.action3.clicked.connect(lambda: closes(parent, fade_time=1, mode=1, menu=True))
    parent.menu_layout.addWidget(parent.action1)
    parent.menu_layout.addWidget(parent.action2)
    parent.menu_layout.addWidget(horizontal_line())
    parent.menu_layout.addWidget(parent.action3)
    parent.menu_.setLayout(parent.menu_layout)
    parent.content_widget.setLayout(parent.menu_layout)
    parent.menu_.setWidget(parent.content_widget)
    parent.menu_.setStyleSheet(THEME[0].menu_main_qss)
    return parent.menu_


def closes(sun_object=None, fade_time=0, mode=0, menu=False):
    if menu:
        sun_object.context_dock.hide()
        sun_object.context_dock.close()
        sun_object.context_dock = None
    parent = None
    match mode:
        case 0:
            if sun_object.objectName().find("_MAIN") == -1:
                try:
                    parent = sun_object.my_parent
                except TypeError:
                    print("已在首页")
                    return
                if fade_time:
                    sun_object.lock = 0
                    fade_(sun_object, 200, 1, 0)
                else:
                    sun_object.lock = 0
                    sun_object.close()
                fade_time -= 1 if fade_time != 0 else 0
                closes(parent, fade_time)
            else:
                pass
        case 1:
            try:
                parent = sun_object.my_parent
            except AttributeError:
                pass
            if fade_time:
                sun_object.lock = 0
                fade_(sun_object, 200, 1, 0)
            else:
                sun_object.lock = 0
                sun_object.close()
                sun_object.closed.connect(lambda: sys.exit())
            fade_time -= 1 if fade_time != 0 else 0
            if parent is None:
                sun_object.close()
            else:
                closes(parent, fade_time, mode=1)


def open_file_dialogue(parent):
    files = []
    file_paths, _ = QFileDialog.getOpenFileNames(
        caption="选择多个文件",
        dir=".",  # 初始目录
        filter="所有文件 (*.*);;文本文件 (*.txt);;文本文件 (*.toml);;文本文件 (*.json);;文本文件 (*.click)"  # 可选过滤器
    )
    for file_path in file_paths:
        files.append(os.path.basename(file_path))
    stretch_h_(parent.select_file_button, 200, 100, 0)
    return files


class ClickDockWidget(QDockWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.my_parent = parent
        self.setWindowFlags(Qt.WindowType.FramelessWindowHint)  # 设置无边框
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)  # 设置窗口背景透明（为圆角做准备）
        self.setTitleBarWidget(QWidget())
        self.setFloating(True)  # 设置为浮动窗口
