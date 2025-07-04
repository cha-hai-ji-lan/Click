# 亮色调 主题配色
main_qss = \
    """
    * {
        color: rgb(49, 45, 37); /* 夜晚棕黑 */
        font-family: 楷体, 微软雅黑;
        font-size: 16px;
        font-weight: 500;
    }
    QPushButton{
        background-color: transparent; 
        border: 0px solid transparent;
        border-radius: 5px;
        padding: 0px;  /* 边框间距 */
    }
    QPushButton:hover{
        background-color: rgb(233, 230, 227); /* 明斯克灰 */
    }
    QPushButton:pressed{
        background-color: rgb(226, 228, 228); /* 纯净灰 */
        border: 1px solid rgb(204, 214, 220); /* 冰灰 */

    }

    QMainWindow[class="__MainWindow__"] QWidget[class="__MainWindowWidget__"] {
        background-color: rgb(240, 241, 237); /* 霜花白 */
        border: 3px solid rgb(199, 199, 198); /* 松灰 */
        /* border-radius: 20px;  圆角半径 */
        border-bottom-left-radius: 10px;
        border-bottom-right-radius: 10px;
    }
    QMainWindow[class="__MainTitleBar__"] QWidget[class="__MainTitleBarWidget__"] {
        font-weight: 700;
        background-color: rgb(221, 227, 224); /* 白桦 */
        border: 2px solid rgb(243, 240, 230); /* 菊蕊白 */
        /*border-radius: 10px 10px 0 0;  圆角半径 */
        border-top-left-radius: 10px;
        border-top-right-radius: 10px;
    }
    QWidget[class="normal_widget"] {
        background-color: rgb(233, 233, 225); /* 迷情白 */
        border: 1px solid rgb(243, 240, 245); /* 菱花白 */
    }
    QMainWindow[class="__MainTitleBar__"] QWidget[class="__MainTitleBarWidget__"] QPushButton#close_button:hover {
        background-color: rgb(196, 43, 28)
    }
    QMainWindow[class="__MainTitleBar__"] QWidget[class="__MainTitleBarWidget__"] QPushButton#close_button:pressed {
        background-color: rgb(199, 63, 48)
        border: 1px solid rgb(204, 214, 220); /* 冰灰 */
    }
    """
# 菜单主窗口
menu_main_qss = \
    """
    QDockWidget {
        min-width: 120px;
        font-size: 14px;
        background-color: transparent;
    }
    QWidget#menu_widget {
        background-color: rgb(221, 227, 224); /* 冰灰 */
        border: 2px solid rgb(221, 227, 224); /* 白桦 */
        border-radius: 15px; /* 圆角半径 */
        
    }
    QFrame {
        border: 2px solid rgb(231, 234, 229);
    } 
    QPushButton {
        border: 0px solid rgb(60, 63, 65);
        min-width: 100px;
        padding: 5px;          /* 菜单项内边距 */
    }
    QPushButton:hover {
        background-color: rgb(240, 240, 235); /* 冰灰 鼠标悬停时的背景色 */
        border-radius: 5px; /* 圆角半径 */
    }
    QPushButton:pressed {
        background-color: rgb(226, 228, 228); /* 纯净灰 点击时的背景色 */
        border: 2px solid rgb(221, 227, 224); /* 冰灰 */
    }   
    """
# 中心对话框样式
inside_qss = \
    """
    QWidget#inside_weight {
        min-width: 600px;
        width: 600px;
        min-height: 75px;
        height: 175px;
        max-height: 450px;
        background-color: rgba(240, 240, 231, 96);  /* 流花白 */
        border: 2px solid rgba(161, 161, 160, 96);  /* 铝白色 */
        border-radius: 30px;

    }
    QWidget#inside_weight QTextEdit#main_line_edit {
        font-size: 24px;
        max-height: 40px;
        height:40px;
        background-color: rgba(200, 203, 196, 0);  /* 草纸白 */
        border: 2px solid rgba(60, 63, 65, 128);
        border-bottom-left-radius: 15px;
        border-bottom-right-radius: 5px;
        border-top-left-radius: 15px;
        border-top-right-radius: 5px;
    }
    QWidget#inside_weight QTextEdit#main_line_edit:focus {
        font-size: 24px;
        max-height: 40px;
        height:40px;
        background-color: rgba(200, 203, 196, 32);  /* 草纸白 */
        border: 2px solid rgba(60, 63, 65, 196);
        border-bottom-left-radius: 15px;
        border-bottom-right-radius: 5px;
        border-top-left-radius: 15px;
        border-top-right-radius: 5px;
    }
    QWidget#inside_weight QPushButton#main_pushbutton {
        width: 60px;
        height: 40px;
        background-color: rgba(223, 227, 222, 0);  /* 牛津白 */
        border: 2px solid rgba(60, 63, 65, 128); 
        border-bottom-left-radius: 5px;
        border-bottom-right-radius: 15px;
        border-top-left-radius: 5px;
        border-top-right-radius: 15px;

    }   
    QWidget#inside_weight QPushButton#main_pushbutton:pressed {
        width: 60px;
        height: 40px;
        background-color: rgba(223, 227, 222, 32);  /* 牛津白 */
        border: 2px solid rgba(60, 63, 65, 196); 
        border-bottom-left-radius: 5px;
        border-bottom-right-radius: 15px;
        border-top-left-radius: 5px;
        border-top-right-radius: 15px;

    }     
    """
# 左侧弹出窗口样式
left_jump_widget_qss = \
    """
    QWidget#jump_widget {
        min-width: 200px;
        width: 200px;
        background-color: rgba(43, 45, 48, 16);  /* 流花白 */
        border: 2px solid rgba(43, 45, 48, 64);  /* 铝白色 */
        border-bottom-left-radius: 5px;
        border-bottom-right-radius: 20px;
        border-top-left-radius: 5px;
        border-top-right-radius: 20px;
    }
    """
# 顶部滚动窗口样式
inside_top_qss = \
    """
    QLabel#roll_label {
        font-size:32px
    }
    """
# 提示框样式
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
# 仅提示框样式 标题栏样式
only_tooltip_qss = \
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
    """
# Explorer 监视窗口样式
list_view_qss = \
    """
    QLabel {
        font-size: 14px;
        font-weight: 800;
        text-align: center;
        height: 30px;
        padding: 5px;
        border-radius: 5px;
        background-color: rgba(51, 127, 168, 196);
    }
     QListView {
        font-size: 14px;
        border-radius: 15px;
        font-weight: 800;
        outline: none;
        text-align: center;
     }
     QListView::indicator {
        width: 20px;
        height: 20px;
        border: 1px solid #5A5A5A;
        background-color: rgba(75, 76, 79, 64);
        border-radius: 4px;
    }
    QListView::indicator:checked {
        image: url(:/assets/light_next_todo_1.png); /* 替换为你自己的图标路径 */
        border: 1px solid rgb(255, 255, 255);
        background-color: rgba(53, 116, 240, 196);
    }
    QListView::indicator:unchecked:hover {
    background-color: rgba(67, 69, 74, 12);
    }
    QListView::item {
        padding: 2px;
        color: rgb(49, 45, 37); /* 夜晚棕黑 */
        border: none;
    }
    QListView::item:hover {
    color: rgb(75, 76, 79)
    background-color: rgba(255, 255, 255, 16); /* 浅白透明色 */
    border-radius: 6px;
    }
    /* 选中（点击后）的背景色 */
    QListView::item:selected {
        color: rgb(49, 45, 37)
        background-color: rgba(47, 72, 95, 0); /* 深蓝透明色 */
        border-radius: 6px;
        border: 0px solid #5A5A5A;
    }
    """
# 托盘菜单样式
tray_menu_qss = \
    """
    QMenu {
        background-color: rgb(49, 50, 56); /* 墨黑 */
        border: 2px solid rgb(45, 45, 45); /* 页岩黑 */
        border-radius: 5px; /* 圆角半径 */
        border-top-left-radius: 5px;
        border-top-right-radius: 5px;
        border-bottom-right-radius: 5px;
    }
    QMenu::item {
        min-width: 100px;
        padding: 5px;          /* 菜单项内边距
        background-color: rgb(49, 50, 56);
        border-radius: 5px;
    }
    QMenu::item:hover {
        background-color: rgb(233, 230, 227); /* 明斯克灰 鼠标悬停时的背景色 */
    }
    QMenu::item:selected {
        background-color: rgb(233, 230, 227); /* 明斯克灰 鼠标悬停时的背景色 */
    }
    QMenu::item:pressed {
        background-color: rgb(226, 228, 228); /* 纯净灰 */
        border: 1px solid rgb(204, 214, 220); /* 冰灰 */
    }
    """
# 左侧弹出栏弹出按钮样式
jump_button = \
    """
    QPushButton {
        max-height: 200px;
        weight: 20px;
        background-color: transparent;
        border: none;
        border-radius: 10px;
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
# 弹出栏响应按钮样式
attribute_button = \
    """
    QPushButton {
        height: 30px;
        max-weight: 200px;
        background-color: transparent;
        border: none;
        border-radius: 10px;
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
# 弹出栏内部响应按钮样式
attribute_inside_button = \
    """
    QPushButton {
        height: 30px;
        max-weight: 200px;
        background-color: rgba(228, 229, 226, 128);
        border: 1px solid rgb(243, 240, 128);
        border-radius: 10px;
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

setting_page_left_qss = \
    """
    QListWidget {
        font-size: 18px;
        font-weight: 900;
        text-align: center;
        max-weight: 50px;
        outline: none;
        background-color: transparent;
    }    
    QListWidget::item {
        border-radius: 4px;
        height: 40px;
        margin: 2px;
        padding: 5px;
        outline: none;
        border:none;
    }
    /* 悬停效果 */
    QListWidget::item:hover {
        color: rgb(49, 45, 37); /* 夜晚棕黑 */
        background-color: rgb(226, 227, 223);
    }

    /* 选中效果 (有焦点) */
    QListWidget::item:selected {
        color: rgb(49, 45, 37); /* 夜晚棕黑 */
        background-color: rgb(220, 220, 220);
        border: 2px solid rgba(68, 68, 71, 128);
    }

    /* 选中效果 (无焦点) */
    QListWidget::item:selected:!active {
        color: rgb(49, 45, 37); /* 夜晚棕黑 */
        background-color: rgb(220, 220, 220);
    }

    /* 垂直滚动条 */
    QListWidget::verticalScrollBar {
        color: rgb(49, 45, 37); /* 夜晚棕黑 */
        width: 10px;
        background: rgb(220, 220, 220);
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
        background-color: rgba(75, 76, 79, 64);
        border-radius: 4px;
    }
    QCheckBox::indicator:checked {
        image: url(:/assets/light_next_todo_1.png); /* 替换为你自己的图标路径 */
        border: 1px solid rgb(255, 255, 255);
        background-color: rgba(53, 116, 240, 196);
    }
    QCheckBox::indicator:unchecked:hover {
    background-color: rgba(67, 69, 74, 12);
    }
    QCheckBox::item {
        padding: 2px;
        color: rgb(49, 45, 37); /* 夜晚棕黑 */
        border: none;
    }
    QCheckBox::item:hover {
    color: rgb(75, 76, 79)
    background-color: rgba(255, 255, 255, 16); /* 浅白透明色 */
    border-radius: 6px;
    }
    /* 选中（点击后）的背景色 */
    QCheckBox::item:selected {
        color: rgb(49, 45, 37)
        background-color: rgba(47, 72, 95, 0); /* 深蓝透明色 */
        border-radius: 6px;
        border: 0px solid #5A5A5A;
    }
    """
