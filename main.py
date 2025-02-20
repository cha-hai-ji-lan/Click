import os
from os.path import dirname as opd
import sys
import platform
import threading
import time

from PySide6.QtGui import QIcon, QAction
from PySide6.QtWidgets import (QApplication, QWidget,
                               QHBoxLayout, QVBoxLayout,
                               QLineEdit, QLabel,
                               QPushButton, QStatusBar, QMainWindow,
                               QMessageBox, QMenu, QSystemTrayIcon)


class Click(QMainWindow):
    def __init__(self, app_):
        super().__init__()
        self.start_time = None
        self.end_time = None
        self.running = True
        self.app = app_
        self.resize(350, 200)
        self.setWindowIcon(QIcon(opd(__file__) + r"\assets\icon.ico"))
        self.setWindowTitle("咔嗒")
        self.setStatusTip("咔嗒.咔嗒..咔嗒...时间到了该关机了")
        widget = QWidget()
        txt_hour = QLabel("时")
        txt_hour.setStatusTip("设置关机延迟多少小时")
        txt_min = QLabel("分")
        txt_min.setStatusTip("设置关机延迟多少分钟")
        txt_sec = QLabel("秒")
        txt_sec.setStatusTip("设置关机延迟多少秒")
        txt_shut = QLabel("后关机")
        self.edit_hour = QLineEdit()
        self.edit_hour.setStatusTip("设置关机延迟多少小时")
        self.edit_hour.setPlaceholderText("0 ~ ...")
        self.edit_min = QLineEdit()
        self.edit_min.setStatusTip("设置关机延迟多少分钟")
        self.edit_min.setPlaceholderText("0 ~ 59   输多了也是59")

        self.edit_sec = QLineEdit()
        self.edit_sec.setStatusTip("设置关机延迟多少秒")
        self.edit_sec.setPlaceholderText("0 ~ 59   输多了也是59")
        btn_shut = QPushButton("关机")
        btn_shut.setStatusTip("按动按钮电脑进入睡觉的倒计时")
        btn_shut.clicked.connect(self.start_timer)
        btn_init = QPushButton("重置")
        btn_init.setStatusTip("一键清空输入的时间，不知道能不能阻止进入关机倒计时的电脑")
        btn_init.clicked.connect(self.clear)

        lay_h1 = QHBoxLayout()
        lay_h1.addWidget(self.edit_hour)
        lay_h1.addWidget(txt_hour)
        lay_h1.addWidget(self.edit_min)
        lay_h1.addWidget(txt_min)
        lay_h1.addWidget(self.edit_sec)
        lay_h1.addWidget(txt_sec)
        lay_h1.addWidget(txt_shut)

        lay_h2 = QHBoxLayout()
        lay_h2.addWidget(btn_shut)
        lay_h2.addWidget(btn_init)

        lay_v1 = QVBoxLayout()
        lay_v1.addLayout(lay_h1)
        lay_v1.addLayout(lay_h2)

        statusBar = QStatusBar()

        widget.setLayout(lay_v1)

        self.setCentralWidget(widget)
        self.setStatusBar(statusBar)

        self.time_h = 0
        self.time_m = 0
        self.time_s = 0
        # 创建系统托盘图标
        self.tray_icon = QSystemTrayIcon(QIcon(opd(__file__) + r"\assets\icon.ico"), self.app)

        # 创建菜单
        self.menu = QMenu()
        self.action_show = QAction("显示信息")
        self.action_show.triggered.connect(self.show_message)
        self.menu.addAction(self.action_show)

        self.action_exit = QAction("退出")
        self.action_exit.triggered.connect(self.exit_app)
        self.menu.addAction(self.action_exit)

        # 设置菜单
        self.tray_icon.setContextMenu(self.menu)
        # 显示系统托盘图标
        self.tray_icon.show()
        # 添加托盘图标点击事件
        self.tray_icon.activated.connect(self.tray_icon_clicked)

    @staticmethod
    def show_message():
        QMessageBox.information(QWidget(), "消息框", "点了没用还没想到做点啥",
                                QMessageBox.StandardButton.Ok)

    def closeEvent(self, event):
        # 隐藏主窗口
        self.hide()
        # 显示系统托盘图标
        self.tray_icon.show()
        # 阻止默认关闭行为
        event.ignore()

    def tray_icon_clicked(self, reason):
        # 双击或单击托盘图标时恢复显示主窗口
        if reason == QSystemTrayIcon.ActivationReason.DoubleClick or reason == QSystemTrayIcon.ActivationReason.Trigger:
            self.showNormal()

    def exit_app(self):
        self.running = False  # 停止计时器
        self.tray_icon.hide()
        sys.exit()

    def hourChanged(self):
        try:
            if int(self.edit_hour.text()) < 0:
                self.time_h = 0
            else:
                self.time_h = int(self.edit_hour.text())
        except ValueError:
            self.time_h = 0

    def minChanged(self):
        try:
            if int(self.edit_min.text()) < 0:
                self.time_m = 0
            elif int(self.edit_min.text()) > 59:
                self.time_m = 59
            else:
                self.time_m = int(self.edit_min.text())
        except ValueError:
            self.time_m = 0

    def secChanged(self):
        try:
            if int(self.edit_sec.text()) < 0:
                self.time_s = 0
            elif int(self.edit_sec.text()) > 59:
                self.time_s = 59
            else:
                self.time_s = int(self.edit_sec.text())
        except ValueError:
            self.time_s = 0

    def wait_me_moment(self):
        self.hourChanged()
        self.minChanged()
        self.secChanged()
        time_total = self.time_h * 3600 + self.time_m * 60 + self.time_s
        self.start_time = time.time()
        while self.running:  # 定期检查标志位
            time.sleep(1)
            self.end_time = time.time()
            self.tray_icon.setToolTip(self.timer_event())
            if self.end_time - self.start_time >= time_total:
                self.shutdown()
                break
            else:
                self.setStatusTip(self.timer_event())
        self.tray_icon.setToolTip("...")

    def clear(self):
        self.running = False
        self.edit_hour.setText("")
        self.edit_min.setText("")
        self.edit_sec.setText("")
        self.tray_icon.setToolTip("...")

    def timer_event(self):
        self.end_time = time.time()
        now_time = self.start_time - self.end_time
        if now_time is not None:
            current_time = time.time()
            wait = self.time_h * 3600 + self.time_m * 60 + self.time_s
            wait -= current_time - self.start_time
            hour_ = wait // 3600
            min_ = (wait - hour_ * 3600) // 60
            sec_ = wait - hour_ * 3600 - min_ * 60
            return f"{int(hour_)}:{int(min_)}:{int(sec_)}后关机"
        else:
            return "..."

    # 多线程非阻塞计时
    def start_timer(self):
        self.running = True
        timer_thread = threading.Thread(target=self.wait_me_moment)
        timer_thread.start()

    @staticmethod
    def shutdown():
        system_name = platform.system()
        if system_name == "Windows":
            os.system("shutdown /s /t 1")
        elif system_name == "Linux" or system_name == "Darwin":  # Darwin 适用于 macOS
            os.system("shutdown -h now")


if __name__ == '__main__':
    app = QApplication(sys.argv)
    click = Click(app)
    click.show()
    sys.exit(app.exec())
