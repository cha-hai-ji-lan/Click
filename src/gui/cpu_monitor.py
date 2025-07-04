from PySide6.QtCharts import QChart, QLineSeries, QValueAxis, QChartView
from PySide6.QtCore import QRect, QSize, Qt, QPointF, QTimer
from PySide6.QtGui import QIcon, QScreen, QPainter
from PySide6.QtWidgets import QPushButton, QHBoxLayout, QVBoxLayout, \
    QScrollArea, QGridLayout, QCheckBox, QLabel, QWidget

from core.core import monitor_resources_cpu
from src.gui import rely
from src.gui.root import MainWindowBase, ClickTitleBase

normal_style_ = None


# 延时函数避免初始化导入时一部分内容在导入的同时又导入同模块其他内容循环导入
# 问题：延时函数有性能损耗
def get_monitor_style():
    from gui.qss import normal_style
    return normal_style


THEME = rely.program_theme()


class CPUMonitorTitle(ClickTitleBase):
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
        self.title_ico = QPushButton(" CPU --监控中")
        self.title_ico.setObjectName("ClickIcon")
        self.title_ico.setProperty("attribute", "mainWindowICON")
        self.title_ico.setIcon(QIcon(":/assets/Click.ico"))
        self.title_ico.setIconSize(QSize(24, 24))

    def set_title_layout(self):
        self.pin_key.setProperty("attribute", "mainWindow")
        cpu_title_layout = QHBoxLayout()
        cpu_title_layout.addWidget(self.title_ico, 0)
        cpu_title_layout.addWidget(self.pin_key, 0)
        self.grid_h_layout.addLayout(cpu_title_layout, 0, 0, alignment=Qt.AlignmentFlag.AlignLeft)


class CPUMonitor(MainWindowBase):
    def __init__(self, parent=None, app=None):
        super().__init__(parent=parent, app=app)
        self.pin_key = QPushButton("")  # 置顶键
        self.monitor_view = CPUMonitorListView(self)
        self.title_grid = QGridLayout()
        self.monitor_v_layout = QVBoxLayout()
        self.roll_widget = QScrollArea()  # 滚动窗口
        self.roll_widget.setWidgetResizable(True)
        self.monitor_mode = True  # 监控模式 True为CPU False为内存
        self.is_open = False
        self.is_topmost = False  # 是否置顶
        self.monitor_chats: list = [None for _ in range(10)]
        self.gui_init()

    def gui_init(self):
        self.title_grid.addWidget(QLabel("PID(进程ID)"), 0, 0, Qt.AlignmentFlag.AlignCenter)
        self.title_grid.addWidget(QLabel("进程名"), 0, 1, Qt.AlignmentFlag.AlignCenter)
        self.title_grid.addWidget(QLabel("CPU使用率"), 0, 2, Qt.AlignmentFlag.AlignCenter)
        self.title_grid.addWidget(QLabel("内存使用"), 0, 3, Qt.AlignmentFlag.AlignCenter)
        self.roll_widget.setWidget(self.monitor_view)
        self.monitor_v_layout.addLayout(self.title_grid)
        self.monitor_v_layout.addWidget(self.roll_widget)
        for i in range(10):
            monitor_chart = CPUmonitorTable(self)
            self.monitor_chats[i] = monitor_chart
            self.monitor_v_layout.addWidget(self.monitor_chats[i], 3)
        self.grid_h_layout.addLayout(self.monitor_v_layout)

    def open_myself(self, mode=True):
        rely.fade_(self)
        self.is_open = True
        self.title_bar.pin_key.click()
        if mode:  # 启动CPU监控
            self.monitor_mode = True
            monitor_resources_cpu(self, True)  # 只有CPU监视器显示的时候启动CPU监控
            self.title_bar.title_ico.setText(" CPU --监控中")
        else:  # 启动内存监控
            self.monitor_mode = False
            monitor_resources_cpu(self, False)  # 只有CPU监视器显示的时候启动内存监控
            self.title_bar.title_ico.setText(" 内存 --监控中")

    def close_myself(self):
        rely.fade_(self, 250, 1, 0)
        self.is_open = False

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

    def set_title(self):
        self.title_bar = CPUMonitorTitle(self)
        self.setMenuWidget(self.title_bar)

    def set_screen_size(self):
        self.resize(500, 400)
        # 获取主屏幕
        screen: QScreen = self.app.primaryScreen()
        screen_geometry = screen.geometry()  # 获取屏幕的几何信息
        # 窗口大小
        window_width = self.width()
        window_height = self.height()
        # 右上角坐标
        x = screen_geometry.width() - window_width
        y = 0
        # 设置窗口位置
        self.setGeometry(x, y, window_width, window_height)
        self.ori_screen_size = QRect(x, y, window_width, window_height)

    def close_main(self):
        super().close_main()
        for i in range(10):
            self.monitor_chats[i].clean_chart()
            self.monitor_chats[i].hide()

    def cpu_monitor_acquisition(self, pos, value):
        """
        获取监控信息，并更新列表
        :param pos: 索引
        :param value: 新的监控项，格式：[pid, name, cpu_percent, mem_usage]
        :return:
        """
        if value[0] == self.monitor_view.check_boxs[pos].text().strip():
            self.monitor_view.buttons[pos][1].setText(F"{value[2]}%")
            self.monitor_view.buttons[pos][2].setText(F"{value[3]}KB")
        else:
            self.monitor_view.check_boxs[pos].setChecked(False)  # 设置为未选中状态
            self.monitor_view.check_boxs[pos].setText(f"\t\t\t{value[0]}")
            self.monitor_view.buttons[pos][0].setText(value[1])
            self.monitor_view.buttons[pos][1].setText(F"{value[2]}%")
            self.monitor_view.buttons[pos][2].setText(F"{value[3]}KB")

    def memory_monitor_acquisition(self, pos, value):
        """
        获取监控信息，并更新列表
        :param pos: 索引
        :param value: 新的监控项，格式：[pid, name, cpu_percent, mem_usage]
        :return:
        """
        # print( value)
        if value[0] == self.monitor_view.check_boxs[pos].text().strip():
            self.monitor_view.buttons[pos][1].setText(F"{value[2]}%")
            self.monitor_view.buttons[pos][2].setText(F"{value[3]}B")
        else:
            self.monitor_view.check_boxs[pos].setChecked(False)  # 设置为未选中状态
            self.monitor_view.check_boxs[pos].setText(f"\t\t\t{value[0]}")
            self.monitor_view.buttons[pos][0].setText(value[1])
            self.monitor_view.buttons[pos][1].setText(F"{value[2]}%")
            self.monitor_view.buttons[pos][2].setText(F"{value[3]}B")


class CPUMonitorListView(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.my_parent = parent
        self.layout = QGridLayout(self)
        # 创建一个二维列表保存按钮
        self.buttons: list = [[None for _ in range(3)] for _ in range(10)]
        self.check_boxs: list = [None for _ in range(10)]
        self.setLayout(self.layout)
        self.gui_init()
        self.set_style()

    def gui_init(self):
        # 构建按钮矩阵
        for row in range(10):
            checkbox = QCheckBox(" ")
            checkbox.clicked.connect(lambda checked, r=row: self.click_check_box(r))
            self.layout.addWidget(checkbox, row, 0)
            self.check_boxs[row] = checkbox
            for col in range(3):
                button = QPushButton(" ")
                self.layout.addWidget(button, row, col + 1)
                self.buttons[row][col] = button  # 存入列表中
                button.clicked.connect(lambda checked, r=row: self.on_button_click(r))

    def set_style(self):
        self.setStyleSheet(THEME[0].monitor_qss)

    def on_button_click(self, row):
        if self.check_boxs[row].isChecked():  # 如果选中，则取消选择
            self.check_boxs[row].setChecked(False)  # 设置为未选中状态
        else:
            self.check_boxs[row].setChecked(True)  # 设置为选中状态

    def click_check_box(self, row):
        if self.check_boxs[row].isChecked():
            if not self.my_parent.monitor_chats[row].isVisible():
                if self.my_parent.monitor_mode:  # CPU模式
                    self.my_parent.monitor_chats[row].set_title(F"{self.buttons[row][0].text()}CPU资源占用情况")
                    self.my_parent.monitor_chats[row].draw_start(row)
                else:  # 内存模式
                    self.my_parent.monitor_chats[row].set_title(F"{self.buttons[row][0].text()}内存占用情况(MB)")
                    self.my_parent.monitor_chats[row].draw_memory_start(row)
                self.my_parent.monitor_chats[row].show()
            else:
                self.my_parent.monitor_chats[row].clean_chart()
                if self.my_parent.monitor_mode:  # CPU模式
                    self.my_parent.monitor_chats[row].set_title(F"{self.buttons[row][0].text()}CPU资源占用情况")
                    self.my_parent.monitor_chats[row].draw_start(row)
                else:  # 内存模式
                    self.my_parent.monitor_chats[row].set_title(F"{self.buttons[row][0].text()}内存占用情况(MB)")
                    self.my_parent.monitor_chats[row].draw_memory_start(row)
                self.my_parent.monitor_chats[row].show()
        else:
            self.my_parent.monitor_chats[row].clean_chart()
            self.my_parent.monitor_chats[row].hide()
            for i in range(10):
                if self.my_parent.monitor_chats[i].isVisible():
                    break
                if i == 9:
                    self.my_parent.set_screen_size()


class CPUmonitorTable(QWidget):
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
        self.hide()

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

    def draw_start(self, row: int):
        self.timer.setInterval(1000)  # 更新间隔(毫秒)
        self.timer.timeout.connect(
            lambda: self.update_data(float(self
                                           .my_parent
                                           .monitor_view.buttons[row][1]
                                           .text()
                                           .strip()[0:-2]
                                           )))
        self.timer.start()

    def draw_memory_start(self, row: int):
        self.timer.setInterval(1000)  # 更新间隔(毫秒)
        self.timer.timeout.connect(
            lambda: self.update_memory_data(float(self
                                                  .my_parent
                                                  .monitor_view.buttons[row][-1]
                                                  .text()
                                                  .strip()[0:-2]
                                                  ) / 1_048_576.0))
        self.timer.start()

    def clean_chart(self):
        self.timer.stop()  # 停止计时器
        self.data_points = []
        self.hide()
        self.series.replace(self.data_points)
