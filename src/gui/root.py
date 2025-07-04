import re

from src.gui import rely
from PySide6.QtGui import QIcon, QPainter, QPen, QColor, QTextCursor, QTextCharFormat, QFont
from PySide6.QtWidgets import QMainWindow, QWidget, QHBoxLayout, \
    QPushButton, QLabel, QGridLayout, QApplication, QToolBar, QTextEdit, QSizePolicy
from PySide6.QtCore import Qt, QSize, Signal, QPoint, QPropertyAnimation, \
    QByteArray, QRect, QEvent, QTimer, QRegularExpression
import src.core as sc
from click_rust_depends import info, preload_txt
from src.core.core import brain_distribute, brain_processing, dynamic_checkup, clean_dynamic_commands
from src.gui.qss import titleQSS, normal_style

THEME = rely.program_theme()
__config__ = rely.get_config()


class ClickTitleBase(QMainWindow):
    closed = Signal()

    def __init__(self, parent=None):
        super().__init__(parent)
        self.my_parent = parent
        self.grid_h_layout = None  # 创建标题网格布局
        self.title_ico = None  # 标题栏图标
        self.version = None  # 版本号
        self.temp_widget = None  # 临时占位控件
        self.lock = 1  # 关屏锁
        self.widget = QWidget()
        self.grid_h_layout = QGridLayout(self)  # 创建标题网格布局
        self.setObjectName("ClickTitleBar_MAIN")
        self.setProperty("class", "__MainTitleBar__")
        self.setProperty("attribute", "mainWindow")
        self.widget.setObjectName("ClickTitleBarWidget_MAIN")
        self.widget.setProperty("class", "__MainTitleBarWidget__")
        self.widget.setProperty("attribute", "mainWindow")
        self.setFixedHeight(30)
        self.setWindowFlags(Qt.WindowType.FramelessWindowHint)

        self.pin_key = QPushButton("")  # 置顶键
        self.minimize_button = QPushButton("")  # 最小化窗口标题栏按钮
        self.minimize_button.setToolTip("最小化窗口")
        self.minimize_button.setToolTipDuration(3000)
        self.enlarge_button = QPushButton("")  # 最大化窗口标题栏按钮
        self.enlarge_button.setToolTip("最大化窗口")
        self.enlarge_button.setToolTipDuration(3000)
        self.close_button = QPushButton("")  # 关闭窗口标题栏按钮
        self.close_button.setToolTip("关闭窗口")
        self.close_button.setToolTipDuration(3000)
        self.set_title_icon()
        self.set_version()
        self.set_title_layout()
        self.gui_init()
        self.exe_style()
        self.set_ico()
        self.show()
        self.keyEvent_dict = {
            Qt.Key.Key_Escape: lambda: self.close(),
        }
        self.key_dict: dict = {
            "max_screen": 1,
        }

    def set_version(self):
        self.version = sc.__version__

    def gui_init(self):
        self.grid_h_layout.setContentsMargins(0, 0, 0, 0)  # 设置边距为0
        version_label = QLabel(F"{self.version}")
        version_label.setProperty("attribute", "mainWindow")
        self.grid_h_layout.addWidget(version_label, 0, 1, alignment=Qt.AlignmentFlag.AlignCenter)
        layout_normal = QHBoxLayout()

        self.enlarge_button.setObjectName("enlarge_button")
        self.enlarge_button.setProperty("attribute", "mainWindowEnlargeButton")

        self.enlarge_button.setFixedWidth(35)
        self.enlarge_button.setFixedHeight(35)
        self.enlarge_button.clicked.connect(lambda: self.zoom_in_event(self.enlarge_button))  # 最大（正常/小）化窗口

        self.minimize_button.setObjectName("minimize_button")
        self.minimize_button.setProperty("attribute", "mainWindowMinimizeButton")

        self.minimize_button.setFixedWidth(35)
        self.minimize_button.setFixedHeight(35)
        self.minimize_button.clicked.connect(lambda: self.zoom_in_event(self.enlarge_button, True))  # 最小化窗口

        self.close_button.setObjectName("close_button")
        self.close_button.setProperty("attribute", "mainWindowCloseButton")

        self.close_button.setFixedWidth(35)
        self.close_button.setFixedHeight(35)
        self.close_button.clicked.connect(lambda: self.my_parent.close_main())  # 关闭主窗口

        layout_normal.addWidget(self.minimize_button, 0)
        layout_normal.addWidget(self.enlarge_button, 0)
        layout_normal.addWidget(self.close_button, 0)
        self.grid_h_layout.addLayout(layout_normal, 0, 2, alignment=Qt.AlignmentFlag.AlignRight)
        self.widget.setLayout(self.grid_h_layout)
        self.setCentralWidget(self.widget)

    def exe_style(self):
        """
            样式渲染器
        继承要求，不允许重写，需要super()继承
        :return: None
        """
        self.setStyleSheet(titleQSS.main_title_qss)
        self.title_ico.setStyleSheet(titleQSS.title_button)
        self.minimize_button.setStyleSheet(THEME[0].only_tooltip_qss)
        self.enlarge_button.setStyleSheet(THEME[0].only_tooltip_qss)
        self.close_button.setStyleSheet(THEME[0].only_tooltip_qss)

    def set_ico(self):
        if THEME[1] is True:
            self.enlarge_button.setIcon(QIcon(":/assets/light_zoom_on.png"))
            self.minimize_button.setIcon(QIcon(":/assets/light_zoom_out.png"))
            self.close_button.setIcon(QIcon(":/assets/light_cancel.png"))
        else:
            self.enlarge_button.setIcon(QIcon(":/assets/dark_zoom_on.png"))
            self.minimize_button.setIcon(QIcon(":/assets/dark_zoom_out.png"))
            self.close_button.setIcon(QIcon(":/assets/dark_cancel.png"))
        self.enlarge_button.setIconSize(QSize(16, 16))
        self.minimize_button.setIconSize(QSize(16, 16))
        self.close_button.setIconSize(QSize(16, 16))

    def set_title_icon(self):
        self.title_ico = QPushButton(" 咔嗒")
        self.title_ico.setObjectName("ClickIcon")
        self.title_ico.setProperty("attribute", "mainWindowICON")
        self.title_ico.setIcon(QIcon(":/assets/Click.ico"))
        self.title_ico.setIconSize(QSize(24, 24))
        self.title_ico.clicked.connect(lambda: self.my_parent.open_office_cc())

    def set_title_layout(self):
        self.pin_key.setProperty("attribute", "mainWindow")
        title_layout = QHBoxLayout()
        title_layout.addWidget(self.title_ico, 0)
        title_layout.addWidget(self.pin_key, 0)
        self.grid_h_layout.addLayout(title_layout, 0, 0, alignment=Qt.AlignmentFlag.AlignLeft)

    def zoom_in_event(self, object_self, hide_flag=False):
        if hide_flag:
            if self.key_dict["max_screen"] == 0:  #
                self.my_parent.showNormal()  # 显示正常化
                if THEME[1] is True:
                    object_self.setIcon(QIcon(":/assets/light_zoom_on.png"))
                else:
                    object_self.setIcon(QIcon(":/assets/dark_zoom_on.png"))
                object_self.setToolTip("还原窗口")
                self.key_dict["max_screen"] = 1
            self.my_parent.mini_screen()
        else:
            if self.key_dict["max_screen"] == 0:  #
                self.my_parent.showNormal()  # 显示正常化
                if THEME[1] is True:
                    object_self.setIcon(QIcon(":/assets/light_zoom_on.png"))
                else:
                    object_self.setIcon(QIcon(":/assets/dark_zoom_on.png"))
                object_self.setToolTip("还原窗口")
                self.key_dict["max_screen"] = 1
            else:
                self.my_parent.showMaximized()  # 显示最大化
                if THEME[1] is True:
                    object_self.setIcon(QIcon(":/assets/light_zoom_in.png"))
                else:
                    object_self.setIcon(QIcon(":/assets/dark_zoom_in.png"))
                object_self.setToolTip("最大化窗口")
                self.key_dict["max_screen"] = 0
        if self.my_parent.context_dock is not None:
            self.my_parent.context_dock.hide()
            self.my_parent.context_dock.close()
            self.my_parent.context_dock = None

    def closeEvent(self, event, /):
        self.closed.emit()
        event.accept()


class MainWindowBase(QMainWindow):
    closed = Signal()

    def __init__(self, parent=None, app=None):
        super().__init__()
        self.app = app
        self.my_parent = parent
        self.new_rect_count = 1  # 临时计数
        self.elapsed_timer = None  # 创建一个计时器
        self.context_dock = None  # 创建上下文菜单
        self.context_dock_geo = None  # 上下文菜单位置
        self.ori_screen_size = None  # 屏幕原始大小
        self.drag_start_position = None  # 鼠标左键按下的坐标
        self.last_mouse_position = None  # 鼠标最后位置
        self.right_drag_start_position = None  # 鼠标左键按下的坐标
        self.start_position_temp = None  # 鼠标按下的临时坐标
        self.show_min_height = None  # 屏幕显示最小高度
        self.show_min_width = None  # 屏幕显示最小宽度
        self.default_geometry = None  # 默认窗口大小
        self.dragging = False  # 是否正在拖动
        self.drag_edge = None  # 拖动边
        self.office_cc = None  # OfficeCC窗口
        self.title_bar = None  # 标题栏
        self.menu_main = None  # 菜单栏
        self.animation = None  # 动画对象
        self.animation1 = None  # 并行动画对象
        self.time_key = True  # 临时时间戳
        self.temp_delta = QPoint(0, 0)  # 临时偏移量
        self.last_resize_time = 0  # 记录上次调整时间
        self.resize_interval = 50  # 调整间隔（毫秒）
        self.grid_h_layout = QHBoxLayout(self)  # 窗口布局
        self.grid_h_layout.setObjectName("ClickMainWindowGridLayout_MAIN")
        self.grid_h_layout.setProperty("attribute", "mainWindow")
        self.lock = 1  # 关屏锁
        '''
                self.lock 关屏锁作用
                如果有主页面下设的子页面在自行关屏后 lock 从 1 修正为 0 说明 
                子页面回退的父级页面以及更上级页面将失去显示权力
                因为在子级页面出现前 父级页面所做的是隐藏自己而非关闭，
                若父级页面正真关闭，父级页面内存将会被GC清理，子级页面将无法通过my_parent 来调用操控父级API
                因为Qt 的页面可以二次显示，在关闭后 如果再次调用父级show()即会再次显示父级页面
                所以closes()逐层反向递归关闭页面后子级页面均会向父级页面的信号捕获插槽反馈显示信号，
                如果 lock被修正为 0 后父级页面将无法调用显示 API 
                并且closes 将反向递归将所有隐藏的 父级 API 关闭
                '''
        self.drag_position = QPoint()
        self.setMouseTracking(True)  # 鼠标追踪
        self.setObjectName("ClickMainWindow_MAIN")
        self.setProperty("class", "__MainWindow__")
        self.widget = QWidget()
        self.widget.setObjectName("ClickMainWindowWidget_MAIN")
        self.widget.setProperty("class", "__MainWindowWidget__")
        self.widget.setProperty("attribute", "mainWindow")
        self.widget.setLayout(self.grid_h_layout)
        self.setWindowFlags(Qt.WindowType.FramelessWindowHint)
        # 设置窗口背景透明（为圆角做准备）
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.set_screen_size()
        self.set_title()
        self.setCentralWidget(self.widget)
        self.exe_style()

    def gui_init(self):
        pass

    def exe_style(self):
        """
        样式渲染器
        继承要求，不允许重写，需要super()继承
        :return: None
        """
        self.setStyleSheet(THEME[0].main_qss)

    def set_title(self):
        self.title_bar = ClickTitleBase(self)
        self.setMenuWidget(self.title_bar)

    def set_screen_size(self):
        self.setGeometry(450, 150, 800, 600)
        self.ori_screen_size = QRect(450, 150, 800, 600)

    def mousePressEvent(self, event):  # 鼠标按下事件
        QApplication.instance().installEventFilter(self)
        if event.button() == Qt.MouseButton.LeftButton:
            self.drag_start_position = event.globalPosition().toPoint()  # 记录拖拽起始的全局位置
            self.drag_position = self.drag_start_position - self.frameGeometry().topLeft()  # 获取鼠标点击位置
            self.start_position_temp = event.globalPosition().toPoint()
            self.drag_edge = self.get_edge(event.position().toPoint())  # 获取当前拖拽的边缘
            if self.drag_edge:  # 如果是边缘拖拽模式
                self.dragging = True
                self.drag_start_position = event.globalPosition().toPoint()  # 记录拖拽起始的全局位置
                self.last_mouse_position = self.drag_start_position  # 新增：初始化上一次鼠标位置
            else:  # 如果是窗口整体拖拽
                self.drag_position = event.position().toPoint()
                event.accept()

        if event.button() == Qt.MouseButton.RightButton:
            self.right_drag_start_position = event.globalPosition().toPoint()
            widget = QApplication.widgetAt(self.right_drag_start_position)
            if widget and self.context_dock is None:
                self.__create_menu_dock(widget)
            elif widget and self.context_dock is not None:
                self.hide_menu_dock()
                self.__create_menu_dock(widget)
        super().mousePressEvent(event)

    def mouseMoveEvent(self, event):  # 鼠标移动事件
        if self.dragging and self.drag_edge:
            current_mouse_pos = event.globalPosition().toPoint()
            delta = current_mouse_pos - self.last_mouse_position  # 关键修正：计算相对于上一次的增量
            self.last_mouse_position = current_mouse_pos  # 更新上一次鼠标位置

            rect = self.geometry()
            self.show_min_width = 250  # 设置最小宽度
            self.show_min_height = 100  # 设置最小高度
            min_width = self.show_min_width
            min_height = self.show_min_height
            max_width = self.maximumWidth() if self.maximumWidth() != 16777215 else None
            max_height = self.maximumHeight() if self.maximumHeight() != 16777215 else None

            if self.drag_edge == 'left':
                new_left = rect.left() + delta.x()
                new_width = rect.right() - new_left
                if new_width >= min_width and (max_width is None or new_width <= max_width):
                    rect.setLeft(new_left)
            elif self.drag_edge == 'right':
                new_right = rect.right() + delta.x()
                new_width = new_right - rect.left()
                if new_width >= min_width and (max_width is None or new_width <= max_width):
                    rect.setRight(new_right)
            elif self.drag_edge == 'top':
                new_top = rect.top() + delta.y()
                new_height = new_top - rect.bottom()
                if new_height >= min_height and (max_height is None or new_height <= max_height):
                    rect.setTop(new_top)
            elif self.drag_edge == 'bottom':
                new_bottom = rect.bottom() + delta.y()
                new_height = rect.top() - new_bottom
                if new_height >= min_height and (max_height is None or new_height <= max_height):
                    rect.setBottom(new_bottom)
            elif self.drag_edge == 'top-left':
                new_left = rect.left() + delta.x()
                new_top = rect.top() + delta.y()
                if rect.right() - new_left >= self.show_min_width and rect.bottom() - new_top >= self.show_min_height:
                    rect.setLeft(new_left)
                    rect.setTop(new_top)
            elif self.drag_edge == 'top-right':
                new_right = rect.right() + delta.x()
                new_top = rect.top() + delta.y()
                if new_right - rect.left() >= self.show_min_width and rect.bottom() - new_top >= self.show_min_height:
                    rect.setRight(new_right)
                    rect.setTop(new_top)
            elif self.drag_edge == 'bottom-left':
                new_left = rect.left() + delta.x()
                new_bottom = rect.bottom() + delta.y()
                if rect.right() - new_left >= self.show_min_width and new_bottom - rect.top() >= self.show_min_height:
                    rect.setLeft(new_left)
                    rect.setBottom(new_bottom)
            elif self.drag_edge == 'bottom-right':
                new_right = rect.right() + delta.x()
                new_bottom = rect.bottom() + delta.y()
                if new_right - rect.left() >= self.show_min_width and new_bottom - rect.top() >= self.show_min_height:
                    rect.setRight(new_right)
                    rect.setBottom(new_bottom)
            self.setGeometry(rect)  # 设置新位置
            self.new_rect_count = 1
        elif event.buttons() == Qt.MouseButton.LeftButton:
            self.move(event.globalPosition().toPoint() - self.drag_position)
            event.accept()

        pos = event.position().toPoint()
        edge = self.get_edge(pos)
        if edge == 'left' or edge == 'right':
            self.setCursor(Qt.CursorShape.SizeHorCursor)
        elif edge == 'top' or edge == 'bottom':
            self.setCursor(Qt.CursorShape.SizeVerCursor)
        elif edge == 'top-left' or edge == 'bottom-right':
            self.setCursor(Qt.CursorShape.SizeFDiagCursor)
        elif edge == 'top-right' or edge == 'bottom-left':
            self.setCursor(Qt.CursorShape.SizeBDiagCursor)
        else:
            self.setCursor(Qt.CursorShape.ArrowCursor)

    def get_edge(self, pos):
        rect = self.rect()  # 获取窗口的矩形区域
        margin = 4  # 边缘检测的宽度
        diag_margin = 20

        left = pos.x() <= margin
        right = pos.x() >= rect.width() - margin
        top = pos.y() <= margin
        bottom = pos.y() >= rect.height() - margin
        if pos.x() <= diag_margin and pos.y() >= rect.height() - diag_margin:
            bottom = True,
            left = True,
        if pos.x() >= rect.width() - diag_margin and pos.y() >= rect.height() - diag_margin:
            bottom = True,
            right = True

        if left and top:
            return 'top-left'
        elif right and top:
            return 'top-right'
        elif left and bottom:
            return 'bottom-left'
        elif right and bottom:
            return 'bottom-right'
        elif left:
            return 'left'
        elif right:
            return 'right'
        elif top:
            return 'top'
        elif bottom:
            return 'bottom'

    def mini_screen(self):
        # 获取当前窗口的几何信息
        current_geometry = self.geometry()
        # 定义默认窗口大小
        self.default_geometry = QRect(
            self.geometry().x(), self.geometry().y(),
            self.ori_screen_size.width(), 100)

        # 创建动画对象
        self.animation = QPropertyAnimation(self, QByteArray(b"windowOpacity"))  # 窗口渐淡动画
        self.animation.setDuration(300)  # 设置动画持续时间（毫秒）
        self.animation.setStartValue(1)  # 设置动画起始值
        self.animation.setEndValue(0)  # 设置动画结束值（默认大小）
        self.animation1 = QPropertyAnimation(self, QByteArray(b"geometry"))  # 窗口百叶窗上收动画
        self.animation1.setDuration(200)  # 设置动画持续时间（毫秒）
        self.animation1.setStartValue(current_geometry)  # 设置动画起始值
        self.animation1.setEndValue(self.default_geometry)  # 设置动画结束值（默认大小）
        self.animation.start()  # 启动动画
        self.animation1.start()  # 启动动画
        self.animation.finished.connect(lambda: self.normal_setting())

    def normal_setting(self):
        self.showMinimized()  # 最小化窗口
        self.setWindowOpacity(1)  # 恢复窗口透明度
        self.setGeometry(self.ori_screen_size)

    def showMaximized(self):
        # 获取当前窗口的几何信息
        current_geometry = self.geometry()
        # 获取屏幕的几何信息
        screen_geometry = self.screen().availableGeometry()

        # 创建动画对象
        self.animation = QPropertyAnimation(self, QByteArray(b"geometry"))
        self.animation.setDuration(150)  # 设置动画持续时间（毫秒）
        self.animation.setStartValue(current_geometry)  # 设置动画起始值
        self.animation.setEndValue(screen_geometry)  # 设置动画结束值（全屏）
        self.animation.start()  # 启动动画

    def showNormal(self):
        # 获取当前窗口的几何信息
        current_geometry = self.geometry()
        # 定义默认窗口大小
        default_geometry = self.ori_screen_size

        # 创建动画对象
        self.animation = QPropertyAnimation(self, QByteArray(b"geometry"))
        self.animation.setDuration(150)  # 设置动画持续时间（毫秒）
        self.animation.setStartValue(current_geometry)  # 设置动画起始值
        self.animation.setEndValue(default_geometry)  # 设置动画结束值（默认大小）
        self.animation.start()  # 启动动画

    def mouseReleaseEvent(self, event):  # 鼠标释放事件
        self.setCursor(Qt.CursorShape.ArrowCursor)
        self.temp_delta = QPoint(0, 0)
        if event.button() == Qt.MouseButton.LeftButton:
            self.dragging = False
            self.drag_edge = None
            self.setCursor(Qt.CursorShape.ArrowCursor)

    def eventFilter(self, obj, event):
        """
        <eventFilter: QMainWindow 重写事件>\n
        <功能： 通过事件过滤器监控处理所有鼠标释放事件>\n
        接收到鼠标释放的信号判断浮动菜单栏是否正在显示且信号发出位置是否在DockWidget外，
        如果正在显示状态则关闭浮动菜单栏\n
        :param obj: 受事件过滤器监控的对象
        :type obj: QWidget
        :param event: 监控的事件信号集
        :type event: QEvent
        :return: __self__
        :rtype:bool
        """
        # 检测所有鼠标点击事件
        if event.type() == QEvent.Type.MouseButtonRelease:
            if self.context_dock and self.context_dock.isVisible():
                # 判断点击是否在DockWidget外
                if not self.context_dock.geometry().contains(self.start_position_temp):
                    self.hide_menu_dock()
                    # 移除事件过滤器避免持续监听
                    QApplication.instance().removeEventFilter(self)

        return super().eventFilter(obj, event)

    def contextMenuEvent(self, event):
        """
        <contextMenuEvent: QMainWindow 重写事件>\n
        <功能：忽略所有右击事件>\n
        通过忽略右击事件保证控件不显示原生菜单栏，从而保证自制菜单的显示
        :param event: 所有右击菜单事件集
        :type event: QContextMenuEvent
        :return: None
        """
        event.ignore()

    def closeEvent(self, event):
        """
        <closeEvent: QMainWindow 重写事件>\n
        <功能：接收窗口关闭时上层的回传信号>\n
        主要用于多窗口打开时关闭主窗口其他窗口同时进行关闭的信号传递\n
        以避免信号中断导致主窗口关闭其他窗口依然存在，并失去有信号处理能力的主窗口，
        导致其余窗口无法进行任何操作，导致BUG\n
        :param event: 关闭窗口的回传信号事件
        :type event: QCloseEvent
        :return:
        """
        self.closed.emit()  # 发送关闭信号
        event.accept()  # 处理关闭时上层回传信号事件

    def close_main(self):
        """
        <close_main: MainWindowBase 公共属性>\n
        <功能: 渐淡隐藏关闭当前显示窗口>
        所有继承自MainWindowBase的窗口控件在退出时均会调用此方法来渐淡关闭
        :return: None
        """
        rely.fade_(self, 200, 1, 0)

    def hide_menu_dock(self):
        """
        <hide_menu_dock: MainWindowBase 公共属性>\n
        <功能: 关闭隐藏当前浮动显示的菜单栏>\n
        尝试隐藏当前浮动菜单栏后关闭菜单栏并结束对菜单栏浮动窗口的对象引用
        便于GC清理已无用的DockWidget
        :return: 关闭状态
        :rtype: int
        """
        try:
            self.context_dock.hide()
            self.context_dock.close()
            self.context_dock = None
            return 0
        except Exception as e:
            print(e)
            return -1

    def __create_menu_dock(self, widget):
        """
        < __create_menu_dock：MainWindowBase 私有属性> \n
        < 功能: 创建一个菜单浮动窗口 >\n
        可以创建非原生浮动菜单栏的对象均继承自MainWindowBase或ClickTitleBase\n
        因为ClickTitleBase对于MainWindowBase是作为子控件所以在标题栏处访问菜单浮动窗口\n
        信号会直接传递给ClickTitleBase的父类控件而父类控件均继承自MainWindowBase其内部实现了这一方法\n
        通过右击控件获得"attribute"属性名以此匹配分配对应的菜单对象并检查浮动窗口控件是否已显示
        如果没有显示将在右击鼠标处显示
        :param widget: 受检查属性的控件
        :type widget: QWidget
        :return: None
        """
        self.context_dock = rely.mouse_right_click_dis_menu(self, widget.property("attribute"))
        # 将窗口移动到鼠标位置
        if not self.context_dock.isVisible():  # 如果菜单没有显示
            self.context_dock.move(self.right_drag_start_position)
            self.context_dock_geo = self.context_dock.geometry()
            self.start_position_temp = self.right_drag_start_position
            self.context_dock.show()


class BaseLineEdit(QTextEdit):
    def __init__(self, parent=None, brother=None):
        super().__init__(parent)
        self.my_parent = parent
        self.my_brother = brother
        self.ask_timer = QTimer(self)  # 命令问询计时器
        self.check_timer = QTimer(self)  # 动态响应问询计时器
        self.cursor = self.textCursor()  # 获取当前光标对象
        self.cursor_pos = None  # 光标的位置
        self.color_list = [91, 71, 10, 255]  # 最丑的颜色作为警示染色错误
        self.order_object = None  # 当前关键字对象
        self.communicate: dict = {"will": True}

        self.main_text = ""  # 存储输入的文本
        self.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Fixed)
        self.setFocusPolicy(Qt.FocusPolicy.StrongFocus)  # 设置焦点策略为StrongFocus，即只有当按钮被点击时才会获取焦点
        self.key_str: str = '|'.join(__config__["WindowsConfig"]["key_word"].keys())
        if THEME[1]:
            self.ori_color = QColor(49, 45, 37)
        else:
            self.ori_color = QColor(235, 238, 236)
        self.default_format = QTextCharFormat()  # 创建默认文本格式
        self.highlight_format = QTextCharFormat()  # 创建高亮文本格式
        self.initialize_highlighting()

    def initialize_highlighting(self):
        # 连接文本变化信号
        self.textChanged.connect(self.highlight_keywords)
        self.default_format.setForeground(self.ori_color)

    def get_current_text_color(self, ) -> QColor:
        # 获取当前文本光标
        cursor = self.textCursor()
        # 获取当前字符格式
        char_format = cursor.charFormat()
        # 获取前景色（字体颜色）
        color = char_format.foreground().color()
        return color

    def highlight_keywords(self):
        # 阻止递归触发
        self.blockSignals(True)

        # 获取当前文本和光标
        text = self.toPlainText().upper()  # 转换成大写 英文指令大小写不敏感
        if text and not self.my_parent.clear_text_edit_button.isVisible():
            if not self.my_parent.clear_text_edit_button.isVisible():
                rely.stretch_h_(self.my_parent.clear_text_edit_button, 200, 0, 100)
            if not self.my_parent.complete_all_tasks_button.isVisible():
                rely.stretch_h_(self.my_parent.complete_all_tasks_button, 200, 0, 100)
        if not text and self.my_parent.clear_text_edit_button.isVisible():
            rely.stretch_h_(self.my_parent.clear_text_edit_button, 200, 100, 0)
        self.cursor_pos = self.cursor.position()

        # 重置所有文本格式
        self.cursor.select(QTextCursor.SelectionType.Document)
        self.cursor.setCharFormat(self.default_format)

        # 使用正则表达式查找关键词
        regex = QRegularExpression(F"{self.key_str}")  # 例如： 关机
        matches = regex.globalMatch(text)  # 创建匹配器
        self.order_object = None

        # 应用高亮格式
        while matches.hasNext():
            match = matches.next()  # 获取下一个匹配项
            self.order_object = match.captured(0)  # 将英文指令转化成大写
            self.color_list = __config__["WindowsConfig"]["key_word"][self.order_object]  # 获取对应命令的颜色列表
            self.highlight_format.setForeground(QColor(self.color_list[0], self.color_list[1],
                                                       self.color_list[2], self.color_list[3]))
            self.highlight_format.setFontWeight(QFont.Weight.Bold)
            start = match.capturedStart()  # 获取匹配项的起始位置和结束位置
            end = match.capturedEnd()  # 获取匹配项的结束位置
            self.cursor.setPosition(start)
            self.cursor.movePosition(QTextCursor.MoveOperation.Right, QTextCursor.MoveMode.KeepAnchor, end - start)
            self.cursor.setCharFormat(self.highlight_format)

        # 恢复光标位置
        self.cursor.setPosition(self.cursor_pos)
        self.setTextCursor(self.cursor)

        # 恢复信号连接
        self.blockSignals(False)

    def ask_after(self):
        """
        问询命令执行是否有子线程在执行
        如果问询获知无命令子线程执行，则继续执行命令
        :return: None
        """
        self.ask_timer.timeout.connect(lambda: brain_processing(self.my_parent))
        self.ask_timer.start(500)  # 每 500ms 问询一次

    def check_after(self):
        """
        输入命令动态响应问询被动命令
        为了节省开销 动态问询频率较低
        :return: None
        """
        self.check_timer.timeout.connect(lambda: dynamic_checkup(self.my_parent, self))
        self.check_timer.start(1000)

    @staticmethod
    def clean_dynamic_command():
        """
        清除动态命令
        :return: None
        """
        clean_dynamic_commands()

    def file_monitoring(self, mode):
        """
        监控当前资源管理器操作的所有路径
        :return: None
        """
        self.my_parent.worker_explorer_start(mode)

    def enter_deal(self):
        """
        接收TextEdit输入的命令，并拆分成单个命令与参数
        :return:
        """
        if self.toPlainText().strip():
            self.main_text = self.toPlainText().strip()

            result = re.findall(
                R"((?=.*[a-zA-Z\u4e00-\u9fff])"
                R"[\w\u4e00-\u9fff.-]+|"
                R"#?[\u4e00-\u9fff]+|"
                R"[a-zA-Z]+|"
                R"[-+]?(?:\d+\.\d*|\.\d+|\d+))", self.main_text)
            self.clear()
            brain_distribute(result, parent=self.my_parent)
        else:
            brain_distribute(["<__SHUTDOWN_NOW__>"], parent=self.my_parent)

    def contextMenuEvent(self, event):
        # 忽略右键菜单事件
        event.ignore()

    def keyPressEvent(self, event):
        if event.key() == Qt.Key.Key_Return or event.key() == Qt.Key.Key_Enter:
            self.enter_deal()
            self.ask_after()
            event.accept()
        else:
            super().keyPressEvent(event)

    def focusInEvent(self, event):
        if not self.check_timer.isActive():
            self.check_after()
        self.my_brother.show_button()
        if not self.toPlainText().strip():  # 如果输入框为空
            self.setPlaceholderText("默认直接关机")

        super().focusInEvent(event)

    def focusOutEvent(self, event):
        if not self.toPlainText().strip():  # 如果输入框没有内容
            self.my_brother.hide_button()
            self.setPlaceholderText("我能为你效劳什么？")

        super().focusOutEvent(event)


class AnimatedButton(QPushButton):
    def __init__(self, text, parent=None):
        super().__init__(text, parent)
        self.my_parent = parent
        self._default_width = 0  # 按钮的默认宽度
        self._expanded_width = 60  # 按钮展开后的宽度
        self._animation = None
        self.setFixedWidth(self._default_width)
        self.stretch()

    def stretch(self):
        self._animation = QPropertyAnimation(self, QByteArray(b"minimumWidth"))  # 控制按钮的最小宽度
        self._animation.setDuration(120)  # 动画时长 300 毫秒
        self.setMinimumWidth(self._default_width)  # 设置最小宽度

    def show_button(self):
        """鼠标进入时触发"""
        self._animation.setStartValue(self.width())  # 从当前宽度开始
        self._animation.setEndValue(self._expanded_width)  # 展开到目标宽度
        self._animation.start()

    def hide_button(self):
        """鼠标离开时触发"""
        self._animation.setStartValue(self.width())  # 从当前宽度开始
        self._animation.setEndValue(self._default_width)  # 收缩到默认宽度
        self._animation.start()


class TransparentButton(QPushButton):
    def __init__(self, text, parent=None):
        super().__init__(text, parent)
        self.setStyleSheet(normal_style.transparent_button)


class ClickQPushButton(QPushButton):
    def __init__(self, text, parent=None):
        super().__init__(text, parent)
        self.setWindowFlags(Qt.WindowType.NoDropShadowWindowHint)  # 忽略窗口阴影
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)


class ClickToolBar(QToolBar):

    def contextMenuEvent(self, event):
        # 忽略右键菜单事件
        event.ignore()


class ClickTextEdit(QTextEdit):
    """
    <ClickTextEdit: QTextEdit 封装类>
    """

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setWindowFlags(Qt.WindowType.WindowTransparentForInput)  # 忽略输入
        self.setWindowFlags(Qt.WindowType.NoDropShadowWindowHint)  # 忽略窗口阴影

    def contextMenuEvent(self, event):
        # 忽略右键菜单事件
        event.ignore()


class TextAnimationLabel(QLabel):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.delete_timer = None  # 定时器对象
        self.write_timer = None  # 文字显示定时器
        self.current_text = ""  # 当前显示的文本
        self.target_text = "你好，欢迎回来"  # 目标文本
        self.is_deleting = False  # 是否处于删除模式
        self.cursor_visible = True  # 光标可见状态
        self.cursor_timer = None  # 光标闪烁定时器
        self.waiting = True  # 等待器
        self.lay_key = None  #
        self.first_wait = 3  #
        self.init_ui()
        self.init_animation()
        self.control_draw()

    def init_ui(self):
        self.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.cursor_visible = True  # 光标可见状态
        self.current_text = ""
        self.is_deleting = False

    def is_first_time(self):
        if __config__["keys"]["open_for_first_time"]:
            self.target_text = preload_txt(self.first_wait)
            self.first_wait -= 1
            self.waiting = True
            self.lay_key = True
            __config__["keys"]["open_for_first_time"] = False
        elif self.first_wait != -1 and self.lay_key:
            self.target_text = preload_txt(self.first_wait)
            self.first_wait -= 1
            self.waiting = True
        else:
            self.lay_key = False
            if self.first_wait == -1:
                self.waiting = False

    def control_draw(self):

        if self.is_deleting:
            self.delete_ctr()
        else:
            self.write_ctr()

    def init_animation(self):
        # 光标闪烁定时器
        self.cursor_timer = QTimer(self)
        self.cursor_timer.timeout.connect(self.toggle_cursor)  # 切换光标可见状态
        self.cursor_timer.start(500)  # 500ms 切换一次

    def delete_ctr(self):
        # 文字删除定时器
        self.delete_timer = QTimer(self)
        self.delete_timer.timeout.connect(self.delete_text)
        self.delete_timer.start(1200)  # 0.5秒后开始删除

    def write_ctr(self):
        # 文字显示定时器
        if self.first_wait != -1:
            self.is_first_time()
        if self.waiting:
            self.waiting = False
        else:
            self.target_text = info()
        self.write_timer = QTimer(self)
        self.write_timer.timeout.connect(self.write_text)
        self.write_timer.start(250)  # 每 250ms 显示一个字

    def toggle_cursor(self):
        """切换光标可见状态"""
        self.cursor_visible = not self.cursor_visible
        self.update()  # 触发重绘

    def delete_text(self):
        """逐字符删除文本"""
        if len(self.current_text) > 0:
            self.current_text = self.current_text[:-1]  # 删除最后一个字符
            self.update()
        else:
            # 删除完成后重置
            self.current_text = ""
            self.is_deleting = False
            self.delete_timer.stop()
            QTimer.singleShot(3000, self.control_draw)  # 1000ms 后开始写入

    def write_text(self):
        """逐字符显示文本"""
        if len(self.current_text) < len(self.target_text):
            self.current_text = self.target_text[:len(self.current_text) + 1]  # 增加一个字符
            self.update()
        else:
            # 显示完成后停止定时器
            self.is_deleting = True
            self.write_timer.stop()
            QTimer.singleShot(3000, self.control_draw)  # 3000ms 后开始删除

    def paintEvent(self, event):
        """自定义绘制文本和光标"""
        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        # 绘制文本
        painter.drawText(self.rect(), self.alignment(), self.current_text)

        # 绘制闪烁光标（仅在写入或删除模式下显示）
        if self.cursor_visible:
            # 计算光标位置
            text_width = self.fontMetrics().horizontalAdvance(self.current_text)  # 文本宽度
            cursor_x = self.rect().center().x() + text_width // 2  # 微调位置
            cursor_y_center = self.rect().center().y()  # 光标中心位置

            pen = QPen(QColor("#5cc9fd"))  # 设置线条颜色
            pen.setWidth(2)  # 设置线条宽度为 3 像素
            # 绘制竖线
            painter.setPen(pen)
            painter.drawLine(
                cursor_x + 1,
                cursor_y_center - 25,  # 光标高度
                cursor_x + 1,
                cursor_y_center + 25  # 光标高度
            )
