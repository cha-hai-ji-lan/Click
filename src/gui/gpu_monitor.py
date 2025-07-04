from PySide6.QtCharts import QChart, QValueAxis, QLineSeries, QChartView
from PySide6.QtCore import QRect, QSize, Qt, QPointF, QTimer
from PySide6.QtGui import QIcon, QScreen, QPainter
from PySide6.QtWidgets import QPushButton, QHBoxLayout, QWidget, QGridLayout, QVBoxLayout, QScrollArea, \
    QLabel

from core.core import monitor_resources_gpu
from src.gui import rely
from src.gui.root import MainWindowBase, ClickTitleBase

THEME = rely.program_theme()


class GPUMonitorTitle(ClickTitleBase):
    def __init__(self, parent=None):
        super().__init__(parent=parent)
        self.pin_key.clicked.connect(self.my_parent.pin_up)
        self.pin_key.setToolTipDuration(3000)
        self.pin_key.setToolTip("钉住窗口")
        self.set_title_layout()
        self.exe_style_son()

    def set_version(self):
        self.version = ''

    def exe_style_son(self):
        if THEME[1] is True:
            self.pin_key.setIcon(QIcon(":/assets/light_pin.png"))
        else:
            self.pin_key.setIcon(QIcon(":/assets/dark_pin.png"))
            self.pin_key.setIconSize(QSize(16, 16))
            self.pin_key.setStyleSheet(THEME[0].tool_tip_qss)

    def set_ico(self):
        """
        设置标题栏图标
        :return:
        """
        if THEME[1] is True:
            self.enlarge_button.setIcon(QIcon(":/assets/light_zoom_on.png"))
            self.minimize_button.setIcon(QIcon(":/assets/light_zoom_out.png"))
            self.close_button.setIcon(QIcon(":/assets/light_child_cancel.png"))
        else:
            self.enlarge_button.setIcon(QIcon(":/assets/dark_zoom_on.png"))
            self.minimize_button.setIcon(QIcon(":/assets/dark_zoom_out.png"))
            self.close_button.setIcon(QIcon(":/assets/dark_child_cancel.png"))
        self.enlarge_button.setIconSize(QSize(16, 16))
        self.minimize_button.setIconSize(QSize(16, 16))
        self.close_button.setIconSize(QSize(16, 16))

    def set_title_icon(self):
        """
        设置标题栏图标,与标题
        :return:
        """
        self.title_ico = QPushButton(" GPU --监控中")
        self.title_ico.setObjectName("ClickIcon")
        self.title_ico.setProperty("attribute", "mainWindowICON")
        self.title_ico.setIcon(QIcon(":/assets/Click.ico"))
        self.title_ico.setIconSize(QSize(24, 24))

    def set_title_layout(self):
        """
        设置标题栏布局
        :return: None
        """
        self.pin_key.setProperty("attribute", "mainWindow")
        gpu_title_layout = QHBoxLayout()
        gpu_title_layout.addWidget(self.title_ico, 0)
        gpu_title_layout.addWidget(self.pin_key, 0)
        self.grid_h_layout.addLayout(gpu_title_layout, 0, 0, alignment=Qt.AlignmentFlag.AlignLeft)


class GPUMonitor(MainWindowBase):
    """
    GPU监控窗口
    """

    def __init__(self, parent=None, app=None):
        super().__init__(parent=parent, app=app)
        self.my_parent = parent
        self.pin_key = QPushButton("")  # 置顶键
        self.monitor_view = GPUMonitorListView(self)
        self.is_open = False
        self.is_topmost = False  # 是否置顶
        self.start_monit = False
        self.title_grid = QGridLayout()
        self.monitor_v_layout = QVBoxLayout()
        self.roll_widget = QScrollArea()  # 滚动窗口
        self.roll_widget.setWidgetResizable(True)
        self.monitor_chats: list = [None, None]
        self.gui_init()

    def gui_init(self):
        self.title_grid.addWidget(QLabel("GPU"), 0, 0, Qt.AlignmentFlag.AlignCenter)
        self.title_grid.addWidget(QLabel("GPU名称"), 0, 1, Qt.AlignmentFlag.AlignCenter)
        self.title_grid.addWidget(QLabel("GPU使用率"), 0, 2, Qt.AlignmentFlag.AlignCenter)
        self.title_grid.addWidget(QLabel("显存使用率"), 0, 3, Qt.AlignmentFlag.AlignCenter)
        self.roll_widget.setWidget(self.monitor_view)
        self.monitor_v_layout.addLayout(self.title_grid)
        self.monitor_v_layout.addWidget(self.roll_widget)
        for i in range(2):
            monitor_chart = GPUmonitorTable(self)
            self.monitor_chats[i] = monitor_chart
            self.monitor_v_layout.addWidget(self.monitor_chats[i], 3)
        self.grid_h_layout.addLayout(self.monitor_v_layout)

    def __start_monitor(self):
        # GPU使用率占用情况
        self.monitor_chats[0].set_title(F"{self.monitor_view.buttons[1].text()}GPU资源占用情况")
        self.monitor_chats[0].draw_gpu_start()
        # 显存
        self.monitor_chats[1].set_title(F"{self.monitor_view.buttons[1].text()}显存占用情况")
        self.monitor_chats[1].draw_gpu_memory_start()

    def open_myself(self):
        """
        打开窗口 并且伪点击一次置顶按钮
        为了 保持窗口置顶状态
        :return: None
        """
        rely.fade_(self)
        self.is_open = True
        self.title_bar.pin_key.click()
        monitor_resources_gpu(self)  # 只有GPU监视器显示的时候启动GPU监控

    def close_myself(self):
        """
        关闭窗口 并且刷新开启属性
        :return:
        """
        rely.fade_(self, 250, 1, 0)
        self.is_open = False
        self.monitor_chats[0].clean_chart()
        self.monitor_chats[1].clean_chart()

    def pin_up(self):
        """
        置顶控制 监控窗口默认置顶
        :return: None
        """
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

    def set_title(self):
        self.title_bar = GPUMonitorTitle(self)
        self.setMenuWidget(self.title_bar)

    def set_screen_size(self):
        """
        设置窗口大小和位置 GPU监控窗口位于右上角
        :return: None
        """
        self.resize(500, 400)
        # 获取主屏幕
        screen: QScreen = self.app.primaryScreen()
        screen_geometry = screen.geometry()  # 获取屏幕的几何信息
        # 窗口大小
        window_width = self.width()
        window_height = self.height()
        # 右上角坐标
        x = screen_geometry.width() - window_width
        y = 400  # 上边距为0
        # 设置窗口位置
        self.setGeometry(x, y, window_width, window_height)
        self.ori_screen_size = QRect(x, y, window_width, window_height)

    def close_main(self):
        super().close_main()
        for i in range(2):
            self.monitor_chats[i].clean_chart()
            self.monitor_chats[i].hide()

    def gpu_monitor_acquisition(self, value):
        """
        获取监控信息，并更新列表
        :param pos: 索引
        :param value: 新的监控项，格式：[pid, name, cpu_percent, mem_usage]
        :return:
        """
        if value[0] == self.monitor_view.buttons[0].text().strip():
            self.monitor_view.buttons[2].setText(F"{value[2]}%")
            self.monitor_view.buttons[3].setText(F"{value[3]}%")
        else:
            self.monitor_view.buttons[0].setText(value[0])
            self.monitor_view.buttons[1].setText(value[1])
            self.monitor_view.buttons[2].setText(F"{value[2]}%")
            self.monitor_view.buttons[3].setText(F"{value[3]}%")
        if not self.start_monit:
            self.__start_monitor()
            self.start_monit = True


class GPUMonitorListView(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.my_parent = parent
        self.h_layout = QHBoxLayout(self)
        # 创建一个二维列表保存按钮
        self.buttons: list = [None for _ in range(4)]
        self.setLayout(self.h_layout)
        self.gui_init()
        self.set_style()

    def gui_init(self):
        # 构建按钮矩阵
        for col in range(4):
            button = QPushButton(" ")
            self.h_layout.addWidget(button)
            self.buttons[col] = button  # 存入列表中
            button.clicked.connect(lambda checked, c=col: self.on_button_click(c))

    def set_style(self):
        self.setStyleSheet(THEME[0].monitor_qss)

    def on_button_click(self, col):
        pass


class GPUmonitorTable(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.my_parent = parent
        self.chart = QChart()  # 创建图表
        self.series = QLineSeries()  # 创建折线序列
        self.axisX = QValueAxis()  # 创建X轴
        self.axisY = QValueAxis()  # 创建Y轴
        self.chart_view = QChartView(self.chart)  # 创建图表视图
        self.data_points = []  # 数据点列表
        self.x_counter = 0  # X轴计数器
        self.max_points = 50  # X轴最大点数
        self.table_h_layout = QHBoxLayout(self)
        self.__title_name: str = "<__NULL__>"
        self.timer = QTimer()  # 绘图计时器
        self.gui_init()
        self.setFixedHeight(210)

    def gui_init(self):
        self.chart.addSeries(self.series)  # 添加序列到图表
        self.chart.addAxis(self.axisX, Qt.AlignmentFlag.AlignBottom)
        self.chart.addAxis(self.axisY, Qt.AlignmentFlag.AlignLeft)

        self.series.attachAxis(self.axisX)  # 将序列与X轴关联
        self.series.attachAxis(self.axisY)  # 将序列与Y轴关联

        self.chart.setTitle(self.__title_name)  # 设置图表标题
        self.chart.setAnimationOptions(QChart.AnimationOption.SeriesAnimations)  # 设置动画选项
        self.chart.legend().hide()  # 隐藏图例

        self.chart_view.setRenderHint(QPainter.RenderHint.Antialiasing)  # 设置渲染提示
        self.table_h_layout.addWidget(self.chart_view)
        self.setLayout(self.table_h_layout)

    def set_title(self, title_name: str):
        self.__title_name = title_name
        self.chart.setTitle(self.__title_name)  # 重设标题

    def update_data(self, new_value: float):
        # 生成新数据点 (这里使用随机数模拟)
        self.x_counter += 1

        # 添加新点
        self.data_points.append(QPointF(self.x_counter, new_value))

        # 移除旧点 (保持固定数量)
        if len(self.data_points) > self.max_points:
            self.data_points.pop(0)

        # 更新折线序列
        self.series.replace(self.data_points)

        # 动态调整X轴范围
        min_x = max(0, self.x_counter - self.max_points)
        max_x = self.x_counter
        self.axisX.setRange(min_x, max_x)

        # 动态调整Y轴范围 (可选)
        y_values = [point.y() for point in self.data_points]
        min_y = min(y_values) * 0.9999 if y_values else 0
        max_y = max(y_values) * 1.0001 if y_values else 100
        self.axisY.setRange(min_y, max_y)

    def update_memory_data(self, new_value: float):

        print(new_value,  "GPU memory")
        # 生成新数据点 (这里使用随机数模拟)
        self.x_counter += 1

        # 添加新点
        self.data_points.append(QPointF(self.x_counter, new_value))

        # 移除旧点 (保持固定数量)
        if len(self.data_points) > self.max_points:
            self.data_points.pop(0)

        # 更新折线序列
        self.series.replace(self.data_points)

        # 动态调整X轴范围
        min_x = max(0, self.x_counter - self.max_points)
        max_x = self.x_counter
        self.axisX.setRange(min_x, max_x)

        # 动态调整Y轴范围 (可选)
        y_values = [point.y() for point in self.data_points]
        min_y = min(y_values) * 0.999 if y_values else 0
        max_y = max(y_values) * 1.001 if y_values else 100
        self.axisY.setRange(min_y, max_y)

    def draw_gpu_start(self):

        self.timer.setInterval(1000)  # 更新间隔(毫秒)
        self.timer.timeout.connect(
            lambda: self.update_data(float(self
                                           .my_parent
                                           .monitor_view.buttons[2]
                                           .text()
                                           .strip()[0:-1]
                                           )))
        self.timer.start()

    def draw_gpu_memory_start(self):
        self.timer.setInterval(1000)  # 更新间隔(毫秒)
        self.timer.timeout.connect(
            lambda: self.update_memory_data(float(self
                                                  .my_parent
                                                  .monitor_view.buttons[3]
                                                  .text()
                                                  .strip()[0:-1]
                                                  )))
        self.timer.start()

    def clean_chart(self):
        self.timer.stop()  # 停止计时器
        self.data_points = []
        self.hide()
        self.series.replace(self.data_points)
