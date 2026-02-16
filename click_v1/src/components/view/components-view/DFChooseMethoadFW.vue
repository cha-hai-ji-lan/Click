<!-- 用于 DealFile 选择文件处理功能 -->
<template>
    <div v-show="FloatingWindow['choose-function']" class="choose-function"
        :class="{ 'choose-function-close': FloatingWindow['choose-function-close'] }" ref="floatingWindowElement">
        <div class="static-head" @mousedown="startDrag">
            <div class="tip"><span>批处理操作</span>
                <span class="float-close-icon tooltip" data-tooltip="收起">
                    <svg t="1766338714320" class="small-icon" @click="() => { collapse() }" viewBox="0 0 1024 1024"
                        version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="10844" width="200" height="200">
                        <path
                            d="M199.36 572.768a31.904 31.904 0 0 0 22.624-9.376l294.144-294.144 285.728 285.728a31.968 31.968 0 1 0 45.248-45.248l-308.352-308.352a32 32 0 0 0-45.28 0l-316.768 316.768a31.968 31.968 0 0 0 22.656 54.624z"
                            p-id="10845"></path>
                        <path
                            d="M538.784 457.376a32 32 0 0 0-45.28 0l-316.768 316.768a31.968 31.968 0 1 0 45.248 45.248l294.144-294.144 285.728 285.728a31.968 31.968 0 1 0 45.248-45.248l-308.32-308.352z"
                            p-id="10846"></path>
                    </svg>

                </span>
            </div>
            <div class="button-small tooltip" data-tooltip="拖动">
                <svg t="1766338639348" class="small-icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="9788" width="200" height="200">
                    <path d="M476.5 924V100c0-19.8 16.2-36 36-36s36 16.2 36 36v824c0 19.8-16.2 36-36 36s-36-16.2-36-36z"
                        p-id="9789"></path>
                    <path
                        d="M100.5 476h824c19.8 0 36 16.2 36 36s-16.2 36-36 36h-824c-19.8 0-36-16.2-36-36s16.2-36 36-36zM690.1 797.6L538.4 949.3c-14.3 14.3-37.4 14.3-51.7 0L334.2 796.8c-14.3-14.3-14.6-37.9 0-52 14.1-13.5 36.5-13.4 50.4 0.5l117.6 117.6c5.8 5.8 15.1 5.8 20.8 0l115.6-115.6c14.3-14.3 37.9-14.6 52 0 13.5 14.1 13.4 36.5-0.5 50.3zM333.7 226.4L485.4 74.7c14.3-14.3 37.4-14.3 51.7 0l152.5 152.5c14.3 14.3 14.6 37.9 0 52-14.1 13.5-36.5 13.4-50.4-0.5L521.6 161.1c-5.8-5.8-15.1-5.8-20.8 0L385.1 276.7c-14.3 14.3-37.9 14.6-52 0-13.4-14.1-13.3-36.5 0.6-50.3z"
                        p-id="9790"></path>
                    <path
                        d="M226.7 690.1L75.1 538.4c-14.3-14.3-14.3-37.4 0-51.7l152.5-152.5c14.3-14.3 37.9-14.6 52 0 13.5 14.1 13.4 36.5-0.5 50.4L161.5 502.2c-5.8 5.8-5.8 15.1 0 20.8l115.6 115.6c14.3 14.3 14.6 37.9 0 52-14.1 13.5-36.5 13.4-50.4-0.5zM798.1 333.7l151.7 151.7c14.3 14.3 14.3 37.4 0 51.7L797.3 689.6c-14.3 14.3-37.9 14.6-52 0-13.5-14.1-13.4-36.5 0.5-50.4l117.6-117.6c5.8-5.8 5.8-15.1 0-20.8L747.8 385.1c-14.3-14.3-14.6-37.9 0-52 14.1-13.4 36.5-13.3 50.3 0.6z"
                        p-id="9791"></path>
                </svg>

            </div>
        </div>

        <div class="static-contain">
            <div draggable="true" class="item" :class="{ 'active-item': focus[0] }"
                @click="() => { changeFeedbackMethod('0') }">
                <span>修改名字字段</span>
            </div>
            <div draggable="true" class="item" :class="{ 'active-item': focus[1] }"
                @click="() => { changeFeedbackMethod('1') }">
                <span>改名排序</span>
            </div>
            <!-- <div draggable="true" class="item" :class="{ 'active-item': focus[2] }"
                @click="() => { changeFeedbackMethod('2') }">
                <span>搜集存放</span>
            </div> -->
            <div draggable="true" class="item" :class="{ 'active-item': focus[3] }"
                @click="() => { changeFeedbackMethod('3') }">
                <span>格式转化</span>
            </div>
            <!-- <div draggable="true" class="item" :class="{ 'active-item': focus[4] }"
                @click="() => { changeFeedbackMethod('1') }">
                <span>音视频处理</span>
            </div> -->
            <!-- <div draggable="true" class="item" :class="{ 'active-item': focus[5] }"
                @click="() => { changeFeedbackMethod('1') }">
                <span>文件解析</span>
            </div> -->
        </div>

        <div v-for="(type, index) in dragType" :key="index" class="drag-btn" :class="type"
            @mousedown.stop="startMove($event, type)"></div>
    </div>
</template>
<script setup lang="ts">
import { type FloatingWindowState } from '../../../class/PathIndex';
import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

const dragType = ['lt', 'lc', 'lb', 'ct', 'cb', 'rt', 'rc', 'rb']  // 存储可拖动标签
const floatingWindowElement = ref<HTMLElement | null>(null);
const isDragging = ref(false);  // 鼠标是否正在拖拽
const isMoving = ref(false);  // 鼠标是否正在缩放
const dragOffset = ref({ x: 0, y: 0 });  // 鼠标拖拽的偏移量
const moustPosX = ref(0)
const moustPosY = ref(0)
const resizeType = ref<string | null>(null)  // 当前拖动的边框
let minHeight = 60 // 最小高度
let minWidth = 50 // 最小宽度
let dragBoxX = 0 // 拖动框X位置
let dragBoxY = 0 // 拖动框Y位置
let maxWidth = 500 // 悬浮窗最大宽度
let maxHeight = 2000 // 悬浮窗最大高度
let dragBoxWidth = 0 // 拖动框宽度
let dragBoxHeight = 0 // 拖动框高度
let newWidth = 0 // 修改后的宽度
let newHeight = 0  // 修改后的高度


const focus = ref([false, false, false, false, false, false])
const props = defineProps<{
    FloatingWindow: FloatingWindowState,
    click: (whichOne: string, index?: number) => void,  // 接收函数类型
    changeMethod: (methodName: string) => void  // 接收函数类型

}>()

onMounted(() => {
    getsize()

})

onUnmounted(() => {
    stopDrag()
})
// 开始拖拽
const startDrag = (event: MouseEvent) => {
    if (!floatingWindowElement.value) return;

    isDragging.value = true;

    // 计算鼠标相对于悬浮窗左上角的偏移量
    const rect = floatingWindowElement.value.getBoundingClientRect();
    dragOffset.value = {
        x: event.clientX - rect.left,
        y: event.clientY - rect.top
    };

    // 添加全局事件监听器
    document.addEventListener('mousemove', drag);
    document.addEventListener('mouseup', stopDrag);
};

/**
 * 开始调整大小事件
 * @param {MouseEvent} event - 鼠标事件对象
 * @param {string} type - 调整大小的类型
 */
function startMove(event: any, type: any) {
    if (!floatingWindowElement.value) return;

    isMoving.value = true;
    // 计算鼠标相对于悬浮窗左上角的偏移量
    const rect = floatingWindowElement.value.getBoundingClientRect();  //  获取悬浮窗的矩形信息
    dragOffset.value = {
        x: event.clientX - rect.left,
        y: event.clientY - rect.top
    };
    moustPosX.value = event.clientX
    moustPosY.value = event.clientY  // 修正：应该是 event.clientY，不是 event.clientX
    dragBoxWidth = rect.width  // 拖动框宽度
    dragBoxHeight = rect.height  // 拖动框高度
    resizeType.value = type
    dragBoxX = rect.left  // 拖动框X位置
    dragBoxY = rect.top  // 拖动框Y位置

    document.addEventListener('mousemove', handleResize)
    document.addEventListener('mouseup', stopMove)
}

// 拖拽过程
const drag = (event: MouseEvent) => {
    if (!isDragging.value || !floatingWindowElement.value) return;

    // 计算新的位置
    const newX = event.clientX - dragOffset.value.x;
    const newY = event.clientY - dragOffset.value.y;

    // 应用新位置
    floatingWindowElement.value.style.left = `${newX}px`;
    floatingWindowElement.value.style.top = `${newY}px`;
};

/**
 * 处理调整大小事件
 * @param {MouseEvent} e - 鼠标事件对象
 */
function handleResize(e: any) {
    if (!isMoving.value || !floatingWindowElement.value) return

    let deltaLX = e.clientX - dragBoxX
    let deltaTY = e.clientY - dragBoxY
    let deltaRX = e.clientX - moustPosX.value
    let deltaBY = e.clientY - moustPosY.value  // 修正：应该是减去 moustPosY.value，不是 dragOffset.value.x
    newWidth = dragBoxWidth - 24  // 减掉24去除 padding 和 border的误差
    newHeight = dragBoxHeight - 24
    switch (resizeType.value) {

        case 'lt':
            // 鼠标水平移动距离就是宽度变化量（反向）
            newWidth -= deltaLX
            newHeight -= deltaTY
            if (newWidth < maxWidth && newWidth > minWidth) {
                floatingWindowElement.value.style.width = `${newWidth}px`;
                floatingWindowElement.value.style.left = `${dragBoxX + deltaLX}px`;
            }
            if (newHeight < maxHeight && newHeight > minHeight) {
                floatingWindowElement.value.style.height = `${newHeight}px`;
                floatingWindowElement.value.style.cssText += `height: ${newHeight}px !important`;  // 高度最开始是动画设置的所以需要修改就得用important 并且为样式添加height（即将height样式从 动画中移除）
                floatingWindowElement.value.style.top = `${dragBoxY + deltaTY}px`;
            }
            break;

        case 'lc':
            // 鼠标水平移动距离就是宽度变化量（反向）
            newWidth -= deltaLX
            if (newWidth < maxWidth && newWidth > minWidth) {
                floatingWindowElement.value.style.width = `${newWidth}px`;
                floatingWindowElement.value.style.left = `${dragBoxX + deltaLX}px`;
            }
            break;

        case 'lb':
            // 鼠标水平移动距离就是宽度变化量（反向）
            newWidth -= deltaLX
            newHeight += deltaBY

            if (newWidth < maxWidth && newWidth > minWidth) {
                floatingWindowElement.value.style.width = `${newWidth}px`;
                floatingWindowElement.value.style.left = `${dragBoxX + deltaLX}px`;
            }
            if (newHeight < maxHeight && newHeight > minHeight) {
                floatingWindowElement.value.style.cssText += `height: ${newHeight}px !important`;  // 高度最开始是动画设置的所以需要修改就得用important 并且为样式添加height（即将height样式从 动画中移除）
                floatingWindowElement.value.style.top = `${dragBoxY}px`;

            }
            break;
        case 'ct':
            newHeight -= deltaTY

            if (newHeight < maxHeight && newHeight > minHeight) {
                floatingWindowElement.value.style.height = `${newHeight}px`;
                floatingWindowElement.value.style.cssText += `height: ${newHeight}px !important`;  // 高度最开始是动画设置的所以需要修改就得用important 并且为样式添加height（即将height样式从 动画中移除）
                floatingWindowElement.value.style.top = `${dragBoxY + deltaTY}px`;
            }
            break;
        case 'cb':
            newHeight += deltaBY
            if (newHeight < maxHeight && newHeight > minHeight) {
                floatingWindowElement.value.style.cssText += `height: ${newHeight}px !important`;  // 高度最开始是动画设置的所以需要修改就得用important 并且为样式添加height（即将height样式从 动画中移除）
                floatingWindowElement.value.style.top = `${dragBoxY}px`;

            }
            break;
        case 'rc':
            newWidth += deltaRX
            if (newWidth < maxWidth && newWidth > minWidth) {
                floatingWindowElement.value.style.width = `${newWidth}px`;
            }
            break;
        case 'rt':
            newWidth += deltaRX
            newHeight -= deltaTY

            if (newWidth < maxWidth && newWidth > minWidth) {
                floatingWindowElement.value.style.width = `${newWidth}px`;
            }
            if (newHeight < maxHeight && newHeight > minHeight) {
                floatingWindowElement.value.style.height = `${newHeight}px`;
                floatingWindowElement.value.style.cssText += `height: ${newHeight}px !important`;  // 高度最开始是动画设置的所以需要修改就得用important 并且为样式添加height（即将height样式从 动画中移除）
                floatingWindowElement.value.style.top = `${dragBoxY + deltaTY}px`;
            }
            break;
        case 'rb':
            newWidth += deltaRX
            newHeight += deltaBY

            if (newWidth < maxWidth && newWidth > minWidth) {
                floatingWindowElement.value.style.width = `${newWidth}px`;
            }
            if (newHeight < maxHeight && newHeight > minHeight) {
                floatingWindowElement.value.style.cssText += `height: ${newHeight}px !important`;  // 高度最开始是动画设置的所以需要修改就得用important 并且为样式添加height（即将height样式从 动画中移除）
                floatingWindowElement.value.style.top = `${dragBoxY}px`;

            }
            break;

    }
}

// 停止拖拽
const stopDrag = () => {
    isDragging.value = false;
    isMoving.value = false;



    // 移除全局事件监听器
    document.removeEventListener('mousemove', drag);  // 移除拖拽调节位置事件
    document.removeEventListener('mouseup', stopDrag);
};
const stopMove = () => {
    if (!floatingWindowElement.value) return
    isDragging.value = false;
    isMoving.value = false;
    document.documentElement.style.setProperty("--float-window2-height", `${newHeight}px`)  // 修改浮动窗口2起始高度



    // 移除全局事件监听器
    document.removeEventListener('mousemove', handleResize) // 移除拖拽调节大小事件
    document.removeEventListener('mouseup', stopMove);
};

const changeFeedbackMethod = (methoad: string) => {
    props.changeMethod(methoad);
    focus.value.fill(false);
    switch (methoad) {
        case '0':
            focus.value[0] = true;
            break;
        case '1':
            focus.value[1] = true;
            break;
        case '2':
            focus.value[2] = true;
            break;
        case '3':
            focus.value[3] = true;
            break;
        case '4':
            focus.value[4] = true;
            break;
        case '5':
            focus.value[5] = true;
            break;

        default:
            console.error("changeFeedbackMethod：方法未定义")
            break;
    }

}
const getsize = async () => {
    let size = getCurrentWindow().innerSize();
    // 悬浮窗尺寸只能和主窗口一样大
    maxWidth = (await size).width
    maxHeight = (await size).height
}

const collapse = () => {
    props.click('choose-function')
    if (!floatingWindowElement.value) return
    floatingWindowElement.value.style.height = `${newHeight}px`;  // 移除 important
}

</script>

<style scoped>
.choose-function {
    position: fixed;
    display: flex;
    justify-self: center;
    align-items: center;
    flex-direction: column;
    font-size: 2.5vmin;
    top: 5vh;
    left: 0vw;
    height: 90vh;
    width: 30vmin;
    min-width: fit-content;
    min-height: 60px;
    transform: translateZ(0);
    will-change: transform;
    user-select: none;
    overflow: auto;
    z-index: 1;
    /* 添加以下样式使元素更像悬浮窗 */
    border: 1px solid var(--unite-but-color);
    background: var(--title-bar-lg-2);
    /* 画格子模拟纸张 */
    background-image:
        linear-gradient(to right, var(--button-color) 1px, transparent 1px),
        linear-gradient(to bottom, var(--button-color) 1px, transparent 1px);
    background-size: var(--grid-size) var(--grid-size);
    overflow: auto;

    border-radius: 2vmin;
    box-shadow: 0 4px 12px var(--font-color);
    border: 2px solid var(--positive-show-color);
    padding: 10px;
    overflow-x: hidden;
    animation: show-choose-function 0.5s ease-in-out forwards;

}

.choose-function-close {
    animation: hide-choose-function 0.5s ease-in-out forwards;
}

.static-head {
    display: flex;
    justify-self: start;
    align-items: start;
    flex-direction: column;
    height: 10%;
    min-height: 60px;

    width: 100%;
    position: relative;
}

.static-head::after {
    content: '';
    position: absolute;
    bottom: 2px;
    left: 0;
    width: 100%;
    height: 2px;
    background: linear-gradient(to right, var(--main-border) 5%, var(--font-color) 10%, var(--positive-show-color) 15%, var(--font-color) 85%, var(--main-border) 95%);
}

.tip {
    display: flex;
    justify-self: start;
    align-items: center;
    flex-direction: row;
    width: 100%;
    border: 2px dashed var(--main-border);
}

.button-small {
    margin-top: 1vmin;
}


.static-contain {
    width: 100%;
    overflow-y: auto;
    overflow-x: hidden;
}

.float-close-icon {
    margin-left: auto;
    margin-right: 2vmin;
}

.item {
    letter-spacing: var(--letter-spacing);
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: row;
    width: 90%;
    height: 5vmin;

    border-bottom-right-radius: 1vmin;
    box-shadow: 2px 4px 4px var(--tool-bar-color);

    border-top: var(--title-min-icon-shadow) solid 0.3vmin;
    border-bottom: var(--main-border) solid 0.3vmin;
    background: transparent;
    margin-top: 2vmin;
    min-height: 20px;
    max-height: 100px;
    /* height: 5vh;
    width: 80%; */
    transition: 0.1s;
    white-space: nowrap;
    /* - 防止文本换行 */
    overflow: hidden;
    /* - 隐藏超出容器的文本 */
}

.item:active {
    animation: active-icon 0.25s forwards;
    animation-timing-function: linear;
}

.active-item {
    box-shadow: 2px 4px 4px linear-gradient(45deg, var(--active-attention-color), var(--normal-attention-color));
    border-radius: 2vmin;
}



/* ------------------------------------------------动画区------------------------------------------------------------*/

@keyframes show-choose-function {
    0% {
        height: 0vh;
        opacity: 0;
    }

    100% {
        height: var(--float-window2-height);
        opacity: 1;

    }

}

@keyframes hide-choose-function {
    0% {
        height: var(--float-window2-height);
        opacity: 1;

    }

    100% {
        height: 0vh;
        opacity: 0;

    }


}

</style>