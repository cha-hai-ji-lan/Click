import sys
import json
from pathlib import Path
from PySide6.QtCore import QObject, Signal, Slot, Property, QUrl
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine

class Task(QObject):
    def __init__(self, title, completed=False, parent=None):
        super().__init__(parent)
        self._title = title
        self._completed = completed

    # 定义属性（供 QML 绑定）
    titleChanged = Signal(str)
    @Property(str, notify=titleChanged)
    def title(self):
        return self._title

    completedChanged = Signal(bool)
    @Property(bool, notify=completedChanged)
    def completed(self):
        return self._completed

    @completed.setter
    def completed(self, value):
        if self._completed != value:
            self._completed = value
            self.completedChanged.emit(value)

class TaskManager(QObject):
    def __init__(self):
        super().__init__()
        self.tasks = []
        self.load_tasks()

    # 暴露任务列表给 QML
    tasksChanged = Signal()
    @Property(list, notify=tasksChanged)
    def tasks(self):
        return self._tasks

    @tasks.setter
    def tasks(self, value):
        self._tasks = value
        self.tasksChanged.emit()

    # 加载任务（从 JSON 文件）
    def load_tasks(self):
        try:
            with open("tasks.json", "r") as f:
                data = json.load(f)
                self.tasks = [Task(task["title"], task["completed"]) for task in data]
        except FileNotFoundError:
            self.tasks = []

    # 保存任务（到 JSON 文件）
    def save_tasks(self):
        data = [{"title": task.title, "completed": task.completed} for task in self.tasks]
        with open("tasks.json", "w") as f:
            json.dump(data, f)

    # 添加任务
    @Slot(str)
    def add_task(self, title):
        new_task = Task(title)
        self.tasks.append(new_task)
        self.tasks = self.tasks  # 触发更新
        self.save_tasks()

    # 删除任务
    @Slot(QObject)
    def remove_task(self, task):
        if task in self.tasks:
            self.tasks.remove(task)
            self.tasks = self.tasks  # 触发更新
            self.save_tasks()

if __name__ == "__main__":
    app = QGuiApplication(sys.argv)
    engine = QQmlApplicationEngine()

    # 将 TaskManager 暴露给 QML
    task_manager = TaskManager()
    engine.rootContext().setContextProperty("taskManager", task_manager)

    # 加载 QML 文件
    engine.load("main.qml")

    if not engine.rootObjects():
        sys.exit(-1)

    sys.exit(app.exec())