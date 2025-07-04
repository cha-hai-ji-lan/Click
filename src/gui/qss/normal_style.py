text_edit_qss = \
    """
    QTextEdit {
    font-size : 18px
    }
    """
transparent_button = \
    """
    QPushButton {
        background-color: transparent;
        border: none;
    }
    QPushButton:hover {
        background-color: transparent;
        border: none;
    }
    QPushButton:pressed {
        background-color: transparent;
        border: none;
    }
    """
tool_tip_qss = \
    """
    QToolTip {
        background-color: rgb(238, 238, 238);
        border: 2px solid rgba(199, 199, 198, 128);
        border-top-left-radius: 8px;
        border-top-right-radius: 4px;
        border-bottom-left-radius: 4px;
        font-weight: 700;
        padding: 5px;
    }
    QPushButton:hover {
        background-color: transparent;
        border: 2px solid rgb(68, 68, 71);
    }
    QPushButton:pressed {
        background-color: transparent;
        border: 2px solid rgba(68, 68, 71, 128);
    }
    """

label_qss = \
    """
    QLabel {
        font-size: 14px;
        font-weight: 700;
        height: 20px
        background-color: rgba(51, 127, 168, 196)
    }
    """
monitor_qss = \
    """
    QPushButton {
        min-height: 20px;
        background-color: transparent;
        border-radius: 8px;
        border: none;
    }
    QPushButton:hover {
        background-color: transparent;
        border: 1px solid rgba(119, 242, 255, 128);
    }
    QPushButton:pressed {
        background-color: transparent;
        border: 2px solid rgb(119, 242, 255);
        
    }
    QCheckBox {
    min-height: 20px;
    }
    QCheckBox::indicator {
        width: 20px;
        height: 20px;
        border: 1px solid #5A5A5A;
        background-color: #333333;
        border-radius: 4px;
    }
    QCheckBox::indicator:checked {
        image: url(:/assets/dark_next_todo_1.png); /* 图标路径 */
        border: 1px solid rgb(0, 0, 0);
        background-color: rgba(53, 116, 240, 196);
    }
    QCheckBox::indicator:unchecked:hover {
    background-color: rgba(67, 69, 74, 32);
    }
    
    """
