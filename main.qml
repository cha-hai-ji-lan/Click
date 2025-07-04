import QtQuick 6.0
import QtQuick.Controls 6.0
import QtQuick.Layouts 6.0
import QtQuick.Window 6.0

Window {
    width: 400
    height: 400
    visible: true
    title: "Task Manager"

    // 背景
    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop { position: 0; color: "#f0f0f0" }
            GradientStop { position: 1; color: "#d0d0d0" }
        }
    }

    // 主布局
    ColumnLayout {
        anchors.fill: parent
        spacing: 10

        // 输入框和添加按钮
        RowLayout {
            Layout.margins: 10
            TextField {
                id: taskInput
                placeholderText: "Enter a new task..."
                Layout.fillWidth: true
                onAccepted: addButton.clicked()
            }
            Button {
                id: addButton
                text: "Add"
                onClicked: {
                    if (taskInput.text.trim() !== "") {
                        taskManager.add_task(taskInput.text)
                        taskInput.clear()
                    }
                }
            }
        }

        // 任务列表
        ListView {
            id: listView
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: taskManager.tasks
            spacing: 5
            clip: true

            delegate: Rectangle {
                id: taskDelegate
                width: listView.width
                height: 50
                color: completed ? "#d0ffd0" : "#ffe0e0"
                radius: 5

                // 滑动删除动画
                Behavior on x { NumberAnimation { duration: 200 } }

                // 任务内容
                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 10
                    CheckBox {
                        checked: completed
                        onCheckedChanged: completed = checked
                    }
                    Text {
                        text: title
                        Layout.fillWidth: true
                        font.pixelSize: 16
                        elide: Text.ElideRight
                        color: completed ? "gray" : "black"
                        opacity: completed ? 0.5 : 1
                    }
                    Button {
                        text: "×"
                        onClicked: taskManager.remove_task(modelData)
                        background: Rectangle { color: "transparent" }
                    }
                }

                // 鼠标区域（滑动删除）
                MouseArea {
                    anchors.fill: parent
                    drag.target: parent
                    drag.axis: Drag.XAxis
                    drag.minimumX: -parent.width
                    drag.maximumX: 0
                    onReleased: {
                        if (parent.x < -parent.width * 0.3) {
                            parent.x = -parent.width
                            taskManager.remove_task(modelData)
                        } else {
                            parent.x = 0
                        }
                    }
                }
            }
        }
    }
}