from PySide6.QtCore import Qt, QSize
from PySide6.QtGui import QIcon, QAction, QStandardItemModel, QStandardItem
from PySide6.QtWidgets import QApplication, QWidget, QHBoxLayout, QPushButton, QVBoxLayout, QSystemTrayIcon, QMenu, \
    QSizePolicy, QListView, QLabel

from src.gui.cpu_monitor import CPUMonitor
from src.gui.gpu_monitor import GPUMonitor
from src.core.core import monitor_path_explorer, get_explorer_windows
from src.gui.office_cc import OfficeCCMainWindow
from src.gui.root import ClickTitleBase, MainWindowBase, BaseLineEdit, \
    AnimatedButton, ClickToolBar, TextAnimationLabel, \
    TransparentButton, ClickQPushButton
from src.gui.about_page import AboutPage
from gui.set_up import SetupPage
from src.gui.qss import normal_style
import sys
from src.gui import (
    resourcesAssets,
    rely as ani
)

THEME = ani.program_theme()


class InsideWindowTitle(ClickTitleBase):
    def __init__(self, parent=None):
        super().__init__(parent=parent)
        self.my_parent = parent
        self.exe_style_son()
        self.pin_key.clicked.connect(self.my_parent.pin_up)
        self.pin_key.setToolTipDuration(3000)
        self.pin_key.setToolTip("钉住窗口")

    def gui_init(self):
        super().gui_init()

    def exe_style_son(self):
        if THEME[1] is True:
            self.pin_key.setIcon(QIcon(":/assets/light_pin.png"))
        else:
            self.pin_key.setIcon(QIcon(":/assets/dark_pin.png"))
            self.pin_key.setIconSize(QSize(16, 16))
            self.pin_key.setStyleSheet(normal_style.transparent_button)


class InsideWindow(MainWindowBase):
    def __init__(self, app):
        super().__init__(app=app)
        self.app = app
        self.is_topmost = False  # 是否置顶
        self.tool_setting_button = None  # 设置按钮
        self.about_page = None  # 关于页面
        self.setup_page = None  # 设置页面
        self.times_num = 0  # 待办次数

        # 资源管理器监控栏
        self.list_view = ClickListView(self)
        # CPU资源监控器
        self.cpu_monitor_view = CPUMonitor(self, self.app)
        # GPU资源监控器
        self.gpu_monitor_view = GPUMonitor(self, self.app)
        # 中心对话栏
        self.main_pushbutton = AnimatedButton("")
        # 设置交互内容按钮
        self.main_line_edit = BaseLineEdit(self, brother=self.main_pushbutton)
        # 中心问题按钮
        self.question_button = TransparentButton("  ")
        # 伸缩弹出框唤醒按钮。
        self.jump_out_button = QPushButton()
        # 伸缩弹出框关闭按钮。
        self.jump_inside_button = QPushButton()
        # 左侧弹出框响应按钮
        self.shut_down_info_button = QPushButton("重置")  # 关机重置按钮
        self.worker_explorer_monitoring_stop_button = QPushButton("停止监控资源管理器")  # 资源管理器监控器停止按钮
        # 轮滚提示词标签
        self.roll_label = TextAnimationLabel(self)

        # 选择文件按钮
        self.select_file_button = QPushButton("选择目标文件")
        self.clear_text_edit_button = QPushButton("清空输入行")
        self.complete_all_tasks_button = QPushButton("结束全部任务")

        # 主界面窗口布局设置
        self.main_v_layout = QVBoxLayout(self)  # 主界面中心布局
        self.main_h_layout = QHBoxLayout(self)  # 主界面中心控件布局
        self.main_h_layout_line1 = QHBoxLayout(self)  # 问题按钮布局位置

        self.inside_v_left_layout = QVBoxLayout(self)  # 主界面左侧布局
        self.inside_v_mid_layout = QVBoxLayout(self)  # 主界面中间布局
        self.inside_v_right_layout = QVBoxLayout(self)  # 主界面右侧布局

        # 创建内部分布窗口
        self.jump_widget = QWidget()  # 左侧弹出窗口
        self.inside_weight = QWidget()  # 居中窗口
        self.inside_left_weight = QWidget()  # 左侧窗口
        self.inside_right_weight = QWidget()  # 右侧窗口
        self.inside_top_weight = QWidget()  # 上部窗口
        self.inside_bottom_weight = QWidget()  # 下部窗口
        self.inside_left_top_weight = QWidget()  # 左上窗口
        self.inside_right_top_weight = QWidget()  # 右上窗口
        self.inside_left_bottom_weight = QWidget()  # 左下窗口
        self.inside_right_bottom_weight = QWidget()  # 右下窗口

        # 创建工具栏
        self.tool_bar = ClickToolBar(self)

        # 创建系统托盘图标
        self.tray_icon = QSystemTrayIcon(QIcon(":/assets/Click.ico"), self.app)

        # 创建菜单
        self.menu = QMenu()
        self.action_show = QAction("关于")
        self.action_exit = QAction("退出")

        # 多线程任务
        self.worker_explorer_running = False  # 资源管理器监控器逻辑运行状态
        self.shut_down_running = False  # 关机逻辑运行状态

        # 预载入
        self.set_main_windows_name()  # 设置主界面各组件名称与属性
        self.__set_layout()  # 设置主界面总体布局
        self.gui_init()  # 设置主界面各组件布局
        self.tool_bar_set()  # 设置工具栏
        self.set_menu()  # 设置工具栏
        self.exe_style_son()  # 设置主界面各组件样式
        self.binding_interaction()  # 设置主界面各组件交互
        ani.fade_(self)  # 启动动画

    # 预载入内容
    def gui_init(self):
        # 设置主界面中心布局
        self.main_h_layout.addWidget(self.main_line_edit, 1)
        self.main_h_layout.addWidget(self.main_pushbutton, 0, Qt.AlignmentFlag.AlignLeft)
        self.main_h_layout_line1.addWidget(self.clear_text_edit_button, 0, Qt.AlignmentFlag.AlignLeft)
        self.main_h_layout_line1.addWidget(self.complete_all_tasks_button, 0, Qt.AlignmentFlag.AlignLeft)
        self.main_h_layout_line1.addWidget(self.select_file_button, 0, Qt.AlignmentFlag.AlignLeft)
        self.main_h_layout_line1.addWidget(self.question_button, 0, Qt.AlignmentFlag.AlignRight)
        self.main_v_layout.addLayout(self.main_h_layout, 1)
        self.main_v_layout.addLayout(self.main_h_layout_line1, 1)
        self.select_file_button.hide()
        self.clear_text_edit_button.hide()
        self.complete_all_tasks_button.hide()

        # 设置顶端监控栏
        jump_top_v_lay = QVBoxLayout()
        jump_top_v_lay.addWidget(self.list_view)
        self.list_view.hide()

        # 设置主界面轮播
        roll_lab_v = QVBoxLayout()
        roll_lab_v.addLayout(jump_top_v_lay, Qt.AlignmentFlag.AlignTop)
        roll_lab_v.addWidget(self.roll_label, Qt.AlignmentFlag.AlignBottom)
        roll_lab_v.setProperty("attribute", "mainWindow")
        self.inside_top_weight.setLayout(roll_lab_v)
        self.inside_weight.setLayout(self.main_v_layout)

        # 设置左侧弹出栏按钮
        jump_left_h_lay = QHBoxLayout()
        jump_left_h_lay.addWidget(self.jump_out_button, 1, Qt.AlignmentFlag.AlignLeft)
        self.inside_left_weight.setLayout(jump_left_h_lay)
        self.jump_left_widget()

    def set_main_windows_name(self):
        # 设置滚动提示词属性与名称
        self.roll_label.setObjectName("roll_label")  # 轮滚提示词标签
        self.roll_label.setProperty("attribute", "mainWindow")
        # 设置文件监控列表属性与名称
        self.list_view.setObjectName("list_view")
        self.list_view.setProperty("attribute", "mainWindow")

        # 设置按钮属性
        self.main_pushbutton.setProperty("attribute", "mainWindow")
        self.question_button.setProperty("attribute", "mainWindow")
        self.jump_out_button.setProperty("attribute", "mainWindow")
        self.jump_inside_button.setProperty("attribute", "mainWindow")
        self.shut_down_info_button.setProperty("attribute", "mainWindow")
        self.worker_explorer_monitoring_stop_button.setProperty("attribute", "mainWindow")
        self.select_file_button.setProperty("attribute", "mainWindow")
        self.clear_text_edit_button.setProperty("attribute", "mainWindow")
        self.complete_all_tasks_button.setProperty("attribute", "mainWindow")

        # 设置工具栏属性名称
        self.tool_bar.setObjectName("tool_bar")
        self.tool_bar.setProperty("class", "normal_widget")
        self.tool_bar.setProperty("attribute", "mainWindow")

        # 设置主要组件名称
        self.main_line_edit.setObjectName("main_line_edit")
        self.main_pushbutton.setObjectName("main_pushbutton")
        self.main_h_layout.setObjectName("inside_h_layout")
        self.main_v_layout.setObjectName("main_layout")

        # 设置主要组件属性
        self.main_line_edit.setProperty("attribute", "mainWindow")
        self.main_pushbutton.setProperty("attribute", "mainWindow")
        self.main_line_edit.setProperty("attribute", "mainWindow")

        # 设置窗口名称
        self.jump_widget.setObjectName("jump_widget")
        self.inside_weight.setObjectName("inside_weight")
        self.inside_left_weight.setObjectName("inside_left_weight")
        self.inside_right_weight.setObjectName("inside_right_weight")
        self.inside_top_weight.setObjectName("inside_top_weight")
        self.inside_bottom_weight.setObjectName("inside_bottom_weight")
        self.inside_left_top_weight.setObjectName("inside_left_top_weight")
        self.inside_right_top_weight.setObjectName("inside_right_top_weight")
        self.inside_left_bottom_weight.setObjectName("inside_left_bottom_weight")
        self.inside_right_bottom_weight.setObjectName("inside_right_bottom_weight")

        # 设置组件属性
        self.jump_widget.setProperty("attribute", "mainWindow")
        self.inside_weight.setProperty("attribute", "mainWindow")
        self.inside_left_weight.setProperty("attribute", "mainWindow")
        self.inside_right_weight.setProperty("attribute", "mainWindow")
        self.inside_top_weight.setProperty("attribute", "mainWindow")
        self.inside_bottom_weight.setProperty("attribute", "mainWindow")
        self.inside_left_top_weight.setProperty("attribute", "mainWindow")
        self.inside_right_top_weight.setProperty("attribute", "mainWindow")
        self.inside_left_bottom_weight.setProperty("attribute", "mainWindow")
        self.inside_right_bottom_weight.setProperty("attribute", "mainWindow")

    def set_menu(self):
        # 设置菜单
        self.tray_icon.setContextMenu(self.menu)
        # 显示系统托盘图标
        self.tray_icon.show()
        # 添加托盘图标点击事件
        self.tray_icon.activated.connect(self.tray_icon_clicked)

    def jump_left_widget(self):
        # 隐藏左侧弹出栏
        self.jump_widget.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
        self.jump_widget.adjustSize()
        self.jump_widget.setMinimumWidth(100)
        self.jump_widget.hide()
        jump_left_inside_h_lay = QHBoxLayout()
        jump_left_inside_v_lay = QVBoxLayout()
        jump_left_inside_v_lay.setSpacing(2)  # 按钮间距设为 2 像素
        jump_left_inside_v_lay.setContentsMargins(10, 10, 10, 10)  # 布局外边缘留 10 像素边距
        jump_left_inside_v_lay.addWidget(self.shut_down_info_button, 1, Qt.AlignmentFlag.AlignTop)
        jump_left_inside_v_lay.addWidget(self.worker_explorer_monitoring_stop_button, 1, Qt.AlignmentFlag.AlignTop)
        jump_left_inside_h_lay.addLayout(jump_left_inside_v_lay, 1)
        jump_left_inside_h_lay.addWidget(self.jump_inside_button, 0, Qt.AlignmentFlag.AlignRight)

        self.jump_widget.setLayout(jump_left_inside_h_lay)
        self.jump_inside_button.hide()
        self.shut_down_info_button.hide()
        self.worker_explorer_monitoring_stop_button.hide()

    def exe_style_son(self):
        self.inside_weight.setStyleSheet(THEME[0].inside_qss)
        self.jump_widget.setStyleSheet(THEME[0].left_jump_widget_qss)
        self.inside_top_weight.setStyleSheet(THEME[0].inside_top_qss)
        self.jump_out_button.setStyleSheet(THEME[0].jump_button)
        self.jump_inside_button.setStyleSheet(THEME[0].jump_button)
        self.shut_down_info_button.setStyleSheet(THEME[0].attribute_button)
        self.worker_explorer_monitoring_stop_button.setStyleSheet(THEME[0].attribute_button)
        self.select_file_button.setStyleSheet(THEME[0].attribute_inside_button)
        self.clear_text_edit_button.setStyleSheet(THEME[0].attribute_inside_button)
        self.complete_all_tasks_button.setStyleSheet(THEME[0].attribute_inside_button)
        self.tool_setting_button.setStyleSheet(THEME[0].tool_tip_qss)
        self.menu.setStyleSheet(THEME[0].tray_menu_qss)
        self.title_bar.pin_key.setStyleSheet(THEME[0].tool_tip_qss)
        self.list_view.setStyleSheet(THEME[0].list_view_qss)

        if THEME[1]:
            self.question_button.setIcon(QIcon(":/assets/light_question.png"))
            self.main_pushbutton.setIcon(QIcon(":/assets/light_next_redo_1.png"))
            self.jump_out_button.setIcon(QIcon(":/assets/light_todo_1.png"))
            self.jump_inside_button.setIcon(QIcon(":/assets/light_redo_1.png"))
            self.tool_setting_button.setIcon(QIcon(":/assets/light_setting.png"))

        else:
            self.question_button.setIcon(QIcon(":/assets/dark_question.png"))
            self.main_pushbutton.setIcon(QIcon(":/assets/dark_next_redo_1.png"))
            self.jump_out_button.setIcon(QIcon(":/assets/dark_todo_1.png"))
            self.jump_inside_button.setIcon(QIcon(":/assets/dark_redo_1.png"))
            self.tool_setting_button.setIcon(QIcon(":/assets/dark_setting.png"))

        self.main_pushbutton.setIconSize(QSize(24, 24))
        self.question_button.setIconSize(QSize(24, 24))
        self.tool_setting_button.setIconSize(QSize(24, 24))
        self.jump_out_button.setIconSize(QSize(16, 16))
        self.jump_inside_button.setIconSize(QSize(16, 16))

    def set_title(self):
        self.title_bar = InsideWindowTitle(self)
        self.setMenuWidget(self.title_bar)

    def pin_up(self):
        global THEME
        self.is_topmost = not self.is_topmost
        if self.is_topmost:
            self.setWindowFlags(Qt.WindowType.FramelessWindowHint | Qt.WindowType.WindowStaysOnTopHint)
            if THEME[1]:
                self.title_bar.pin_key.setIcon(QIcon(":/assets/light_pin_on.png"))
            else:
                self.title_bar.pin_key.setIcon(QIcon(":/assets/dark_pin_on.png"))
            self.title_bar.pin_key.setToolTip("拔掉钉子")
        else:
            self.setWindowFlags(Qt.WindowType.FramelessWindowHint)
            if THEME[1]:
                self.title_bar.pin_key.setIcon(QIcon(":/assets/light_pin.png"))
            else:
                self.title_bar.pin_key.setIcon(QIcon(":/assets/dark_pin.png"))
            self.title_bar.pin_key.setToolTip("钉住窗口")
        # 必须重新显示窗口才能使标志生效
        self.show()

    def binding_interaction(self):
        """
        绑定主界面程序交互逻辑
        :return:
        """
        # 中心对话输入栏
        self.main_line_edit.setPlaceholderText("我能为您效劳什么？")
        # 中心对话输入栏确认按钮交互绑定
        self.main_pushbutton.clicked.connect(self.main_line_edit.enter_deal)
        # 中心问题按钮交互绑定
        self.question_button.clicked.connect(lambda: print("中心问题按钮被点击了"))
        # 伸缩弹出框唤醒按钮交互绑定
        self.jump_out_button.clicked.connect(lambda: self.toggle_left_jump_dock())
        # 伸缩弹出框关闭按钮交互绑定
        self.jump_inside_button.clicked.connect(lambda: self.toggle_left_jump_dock())

        # 关机重置按钮交互绑定
        self.shut_down_info_button.clicked.connect(lambda: self.shut_down_restart())
        self.worker_explorer_monitoring_stop_button.clicked.connect(lambda: self.worker_explorer_stop())
        # 关机重置按钮伸缩约束
        self.shut_down_info_button.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Fixed)
        self.worker_explorer_monitoring_stop_button.setSizePolicy(
            QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Fixed)
        # 选择文件按钮交互绑定
        self.select_file_button.clicked.connect(lambda: self.set_text_edit_text())
        self.clear_text_edit_button.clicked.connect(lambda: self.clear_edit())
        self.complete_all_tasks_button.clicked.connect(lambda: self.complete_all_tasks())

        # 工具栏约束
        self.tool_bar.setContextMenuPolicy(Qt.ContextMenuPolicy.NoContextMenu)  # 禁用右键菜单

        # 托盘菜单 关于 交互绑定
        self.action_show.triggered.connect(lambda: self.open_about_page())
        self.menu.addAction(self.action_show)
        # 托盘菜单 关闭 交互绑定
        self.action_exit.triggered.connect(lambda: self.close_main())
        self.menu.addAction(self.action_exit)

    # 多线程处理
    def worker_explorer_start(self, mode):
        if self.worker_explorer_running is True:
            pass
        else:
            match mode:
                case 0:
                    if not self.list_view.isVisible():  # 列表显示时
                        ani.stretch_h_(self.list_view, 400, 0, 1000)
                        self.worker_explorer_running = True
                        monitor_path_explorer(self)
                        ani.stretch_h_(self.worker_explorer_monitoring_stop_button, 200, 0, 200)
                        if not self.select_file_button.isVisible():
                            ani.stretch_h_(self.select_file_button, 200, 0, 100)
                        if not self.jump_widget.isVisible():
                            self.toggle_left_jump_dock()
                case 1:
                    if not self.list_view.isVisible():  # 列表显示时
                        ani.stretch_h_(self.list_view, 400, 0, 1000)
                        self.worker_explorer_running = True
                        monitor_path_explorer(self)
                        if not self.worker_explorer_monitoring_stop_button.isVisible():
                            ani.stretch_h_(self.worker_explorer_monitoring_stop_button, 200, 0, 200)
                        if not self.jump_widget.isVisible():
                            self.toggle_left_jump_dock()
                case 2:
                    flag = False
                    if self.list_view.model.rowCount() == 0:
                        flag = True
                    else:
                        index = self.list_view.model.index(0, 0)
                        flags = self.list_view.model.flags(index)
                        if flags & Qt.ItemFlag.ItemIsUserCheckable:
                            flag = True
                    if self.list_view.isVisible() and flag:
                        ani.stretch_h_(self.list_view, 400, 1000, 0)
                    if not self.list_view.isVisible():  # 列表显示时
                        self.list_view.model.clear()
                        self.list_view.set_now_item(get_explorer_windows(True))
                        ani.stretch_h_(self.list_view, 400, 0, 1000)
                    if not self.select_file_button.isVisible():
                        ani.stretch_h_(self.select_file_button, 200, 0, 100)

    def worker_explorer_stop(self):
        if self.list_view.isVisible():  # 列表显示时
            ani.stretch_h_(self.list_view, 400, 1000, 0)
        self.worker_explorer_running = False
        if self.worker_explorer_monitoring_stop_button.isVisible():
            ani.stretch_h_(self.worker_explorer_monitoring_stop_button, 200, 200, 0)
        if self.main_line_edit.order_object == "监控":
            self.main_line_edit.order_object = None

    def explorer_monitor_acquisition(self, value):
        """
        获取文件监控信息, 并更新监控列表
        :param value: 监控信息
        :return:
        """
        appended_list_view = self.list_view.get_item_list()
        for item in value:
            if item in appended_list_view:
                pass
            else:
                self.list_view.add_check_item(item)
        for item in appended_list_view:
            if item not in value:
                self.list_view.remove_item(item)

    def shut_down_restart(self):
        """
        重置 关机
        :return:
        """
        self.shut_down_running = False
        self.main_line_edit.setPlaceholderText("我能为您效劳什么？")
        self.shut_down_info_button.setText("重置")
        if self.shut_down_info_button.isVisible():
            ani.stretch_h_(self.shut_down_info_button, 200, 200, 0)
        if self.select_file_button.isVisible():
            ani.stretch_h_(self.select_file_button, 200, 100, 0)
        if self.jump_widget.isVisible():
            self.toggle_left_jump_dock()

    def set_text_edit_text(self):
        """
        选择好的文件 写入输入框
        :return:
        """
        input_text = ' '.join(ani.open_file_dialogue(self))
        self.main_line_edit.setText(self.main_line_edit.toPlainText().strip() + ' ' + input_text)

    def complete_all_tasks(self):
        """
        结束所有正在运行的指令行任务 包括动态问询任务
        :return: None
        """
        self.clear_edit()
        self.shut_down_restart()
        self.worker_explorer_stop()
        self.worker_explorer_running = False
        self.shut_down_running = False
        self.cpu_monitor_view.close_myself()
        self.gpu_monitor_view.close_myself()
        self.main_line_edit.ask_timer.stop()
        self.main_line_edit.check_timer.stop()
        self.main_line_edit.clean_dynamic_command()
        ani.stretch_h_(self.complete_all_tasks_button, 200, 100, 0)

    def clear_edit(self):
        self.main_line_edit.clear()
        ani.stretch_h_(self.clear_text_edit_button, 200, 100, 0)

    def toggle_left_jump_dock(self):
        """
        显示/隐藏 左侧跳转栏
        :return:
        """
        if self.jump_widget.isVisible():
            ani.stretch_v_(self.jump_inside_button, 200, 200, 0)
            ani.stretch_v_(self.jump_out_button, 200, 0, 200)
            ani.stretch_h_(self.jump_widget, 400, 300, 0)
        else:
            ani.stretch_v_(self.jump_out_button, 200, 200, 0)
            ani.stretch_v_(self.jump_inside_button, 200, 0, 200)
            ani.stretch_h_(self.jump_widget, 400, 0, 300)

    def tool_bar_set(self):
        self.tool_setting_button = ClickQPushButton("")
        self.tool_setting_button.setProperty("attribute", "mainWindowSettingButton")
        self.tool_setting_button.setToolTipDuration(3000)
        self.tool_setting_button.setToolTip("首选项")
        self.tool_setting_button.clicked.connect(lambda: self.open_setup_page())

        self.tool_bar.addWidget(self.tool_setting_button)
        self.tool_bar.addSeparator()
        self.tool_bar.setIconSize(QSize(24, 24))
        self.addToolBar(self.tool_bar)

    def open_office_cc(self):
        if self.context_dock is not None:
            self.hide_menu_dock()
        ani.fade_(self, 250, 1, 0)
        self.office_cc = OfficeCCMainWindow(self, self.app)
        self.office_cc.closed.connect(lambda: self.close_office_cc())

    def close_office_cc(self):
        if self.lock:
            ani.fade_(self, 400, 0, 1)
        if self.office_cc.context_dock is not None:
            self.office_cc.hide_menu_dock()
        self.office_cc = None

    def open_about_page(self):
        if self.context_dock is not None:
            self.hide_menu_dock()
        ani.fade_(self, 250, 1, 0)
        self.about_page = AboutPage(self, self.app)
        self.about_page.closed.connect(lambda: self.close_about_page())

    def close_about_page(self):
        if self.lock:
            ani.fade_(self, 400, 0, 1)
        if self.about_page.context_dock is not None:
            self.about_page.hide_menu_dock()
        self.about_page = None

    def open_setup_page(self):
        if self.context_dock is not None:
            self.hide_menu_dock()
        ani.fade_(self, 250, 1, 0)
        self.setup_page = SetupPage(self, self.app)
        self.setup_page.closed.connect(lambda: self.close_setup_page())

    def close_setup_page(self):
        if self.lock:
            ani.fade_(self, 400, 0, 1)
        if self.setup_page.context_dock is not None:
            self.setup_page.hide_menu_dock()
        self.setup_page = None

    def close_main(self):
        super().close_main()
        self.complete_all_tasks()

    def tray_icon_clicked(self, reason):
        # 双击或单击托盘图标时恢复显示主窗口
        if reason == QSystemTrayIcon.ActivationReason.DoubleClick or reason == QSystemTrayIcon.ActivationReason.Trigger:
            self.showNormal()

    def __set_layout(self):
        # 设置主界面控件布局
        # 左侧布局
        self.inside_v_left_layout.addWidget(self.inside_left_top_weight)
        self.inside_v_left_layout.addWidget(self.inside_left_weight)
        self.inside_v_left_layout.addWidget(self.inside_left_bottom_weight)
        # 中间布局
        self.inside_v_mid_layout.addWidget(self.inside_top_weight, Qt.AlignmentFlag.AlignCenter)
        self.inside_v_mid_layout.addWidget(self.inside_weight, 2, Qt.AlignmentFlag.AlignTop)
        self.inside_v_mid_layout.addWidget(self.inside_bottom_weight, Qt.AlignmentFlag.AlignTop)
        # 右侧布局
        self.inside_v_right_layout.addWidget(self.inside_right_top_weight)
        self.inside_v_right_layout.addWidget(self.inside_right_weight)
        self.inside_v_right_layout.addWidget(self.inside_right_bottom_weight)

        self.grid_h_layout.addWidget(self.jump_widget, 1)
        self.grid_h_layout.addLayout(self.inside_v_left_layout, 1)
        self.grid_h_layout.addLayout(self.inside_v_mid_layout, 2)
        self.grid_h_layout.addLayout(self.inside_v_right_layout, 1)


class ClickListView(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.my_parent = parent
        self.layout = QVBoxLayout(self)
        self.model = QStandardItemModel()  # 创建一个数据模型
        self.view = QListView()
        self.title = QLabel("当前操作的目录")
        self.title.setProperty("attribute", "mainWindow")
        self.set_style()
        self.set_list_view()

    def set_style(self):
        """
        设置监控列表样式
        :return: None
        """
        self.title.setStyleSheet(normal_style.label_qss)

    def set_list_view(self):
        """
        设置列表控件
        :return: None
        """
        self.view.setModel(self.model)
        self.layout.addWidget(self.title)
        self.layout.addWidget(self.view)
        self.setLayout(self.layout)

    def add_check_item(self, item_text):
        """
        设置独立项目配置
        :param item_text: 项目名称
        :return:
        """
        item = QStandardItem(item_text)  # 创建一个标准项
        item.setCheckable(True)  # 设置为可勾选
        item.setEditable(False)  # 可选：禁止编辑文本
        self.model.appendRow(item)

    def get_item_list(self, my_index=QStandardItem().index()):
        """
        获取所有项目
        :param my_index: 索引
        :return:
        """
        list_text = []
        for row in range(self.model.rowCount(my_index)):
            for column in range(self.model.columnCount(my_index)):
                index = self.model.index(row, column, my_index)
                item = self.model.itemFromIndex(index)
                list_text.append(item.text())
        return list_text

    def set_now_item(self, item_text):
        """
        设置 当前项目 命令 的条目
        :param item_text:
        :return:
        """
        item = QStandardItem(item_text)  # 创建一个标准项
        item.setCheckable(False)  # 设置为可勾选
        item.setEditable(False)  # 可选：禁止编辑文本
        self.model.appendRow(item)

    def get_checked_items(self):
        """
        获取所有选中项目
        :return:
        """
        checked_items = []
        for row in range(self.model.rowCount()):
            item = self.model.item(row)
            if item.checkState() == Qt.CheckState.Checked:
                checked_items.append(item.text())
        return checked_items

    def remove_item(self, item_name):
        """
        删除项目
        :param item_name: 项目名称
        :return:
        """
        items = self.model.findItems(item_name, Qt.MatchFlag.MatchExactly)  # 匹配文本为 "Apple" 的项
        for item in items:
            row = item.row()
            self.model.removeRow(row)


if __name__ == "__main__":
    app_ = QApplication(sys.argv)
    window = InsideWindow(app_)
    window.show()
    sys.exit(app_.exec())
