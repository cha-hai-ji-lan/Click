from PySide6.QtGui import QIcon
from src.gui import InsideWindow
from PySide6.QtWidgets import QApplication
import sys

if __name__ == "__main__":
    app = QApplication(sys.argv)
    app.setWindowIcon(QIcon(":/assets/Click.ico"))
    window = InsideWindow(app)
    window.setWindowIcon(QIcon(":/assets/Click.ico"))
    sys.exit(app.exec())
