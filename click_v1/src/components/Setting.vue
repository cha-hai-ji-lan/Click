<template>
    <div v-show="comVisibility.setting['setting-open'] === true" class="setting-main"
        :class="{ 'setting-main-close': comVisibility.setting['setting-close'] }" ref="floatingWindowElement">
        <div class="setting-head" @mousedown="startDrag">
            <div class="setting-head-contain">
                <div class="titlebar-button tooltip" data-tooltip="导入配置 施工ing🔨">
                    <svg t="1767096453078" class="icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="13250" width="200" height="200">
                        <path
                            d="M111.6 148.8c7.9 0 16.9 17.9 21.6 32.2 25.9 88.7 143 182.6 248.6 194.2v-1.1c0-29.6 10.6-50.7 31.7-64.4 11.1-6.9 22.7-10.6 34.8-10.6 13.7 0 27.4 4.8 39.1 13.7L645.8 438c17.9 13.2 28.5 34.8 28.5 57.5 0 22.2-10.6 43.8-28 57L487.4 678.2c-1.6 1.1-2.6 2.1-4.2 2.6-7.4 4.2-19.5 9.5-34.3 9.5-12.1 0-23.7-3.2-34.3-10-26.4-16.9-30.6-43.8-32.2-55.4l-0.5-2.1c-0.5-1.6-0.5-3.7-0.5-5.3v-5.3c-162.6-26.9-314.1-192.6-314.1-352 0-29.6 4.2-59.6 12.7-88.1 4.2-13.8 18.4-26.5 31.6-23.3z m335.2 467.1c0.5 2.6 1.1 6.9 2.1 8.4l156.2-123.5c1.6-1.1 2.6-3.2 2.6-5.3 0-2.1-0.5-3.7-2.6-5.3L447.8 365.7c0 1.1-0.5 3.2-0.5 7.9v34.8c0 17.4-13.2 31.7-30.6 32.7-104.5 7.4-213.8-61.2-280.3-145.7 22.2 123 154.1 247.5 279.7 254.4 17.4 1.1 31.1 15.3 31.1 32.7v32.7l-0.4 0.7z m0.5-0.5"
                            p-id="13251"></path>
                        <path
                            d="M309 916.3c-67 0-121.4-54.9-121.4-121.4V593.2h69.1v201.6c0 29 23.8 52.3 52.3 52.3h527.3c29 0 52.3-23.8 52.3-52.3V226.4c0-29-23.8-52.3-52.3-52.3H399.8v-69.7h436.5c67 0 121.9 54.9 121.9 121.9v567.9c0 67-54.9 121.9-121.9 121.9H309v0.2z m0 0"
                            p-id="13252"></path>
                        <path
                            d="M841.6 626H729.1v-59.1h112.4V626z m0 154.1H545.5V721h296.1v59.1z m0-425.4h-237v-59.1h237v59.1z m0 140.9H729.1v-59.1h112.4v59.1z m0-59.1"
                            p-id="13253"></path>
                    </svg>
                </div>
                <div class="titlebar-button hide-setting tooltip" data-tooltip="关闭" @click="() => { collapse() }">
                    <svg t="1767095837641" class=" icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="11121" width="200" height="200">
                        <path
                            d="M40.048585 431.978671a39.998025 39.998025 0 0 0 39.998025-39.998025V79.99605h311.984596a39.998025 39.998025 0 0 0 0-79.99605H40.048585a39.998025 39.998025 0 0 0-39.998025 39.998025v351.982621a39.998025 39.998025 0 0 0 39.998025 39.998025zM984.001977 591.970771a39.998025 39.998025 0 0 0-39.998025 39.998026v311.984595H632.019357a39.998025 39.998025 0 0 0 0 79.99605h351.98262a39.998025 39.998025 0 0 0 39.998025-39.998025V631.968797a39.998025 39.998025 0 0 0-39.998025-39.998026zM788.571627 412.699623L728.014617 351.982621a15.99921 15.99921 0 0 0 1.279937-1.119945l283.026025-282.466053a39.998025 39.998025 0 0 0-56.637203-56.557208L672.897338 294.625453A8.719569 8.719569 0 0 0 672.017382 295.985386l-63.436868-63.436868A23.998815 23.998815 0 0 0 568.022516 246.547827l-26.078712 205.829837a23.998815 23.998815 0 0 0 26.798677 26.718681l205.829837-25.678732a23.998815 23.998815 0 0 0 13.999309-40.71799zM455.308082 544.933094l-205.829837 25.678732a23.998815 23.998815 0 0 0-13.999309 40.797986L296.035946 671.966822l-1.279937 1.039948-283.026026 282.626046a39.998025 39.998025 0 0 0 56.637204 56.637203l282.786037-282.866033A14.639277 14.639277 0 0 0 352.033181 727.964057l63.99684 63.516864a23.998815 23.998815 0 0 0 39.998025-13.919313l25.758728-205.909833a23.998815 23.998815 0 0 0-26.478692-26.718681z"
                            p-id="11122"></path>
                    </svg>
                </div>
            </div>

        </div>
        <div class="setting-contain">
            <div class="left-setting-contain">
                123
            </div>
            <div class="right-setting-contain">
                123
            </div>
        </div>

        <div v-for="(type, index) in dragType" :key="index" class="drag-btn" :class="type"
            @mousedown.stop="startMove($event, type)"></div>
    </div>
</template>
<script setup lang="ts">
import { ComVisibility } from "../class/PathIndex"
import { ref, onUnmounted } from 'vue';
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
const props = defineProps<{
    comVisibility: ComVisibility,
    openWindow: (openName: string) => void,

}>()


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
    getsize()
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
    if (floatingWindowElement.value) {
        floatingWindowElement.value.style.left = `${newX}px`;
        floatingWindowElement.value.style.top = `${newY}px`;
    }
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

const getsize = async () => {
    let size = getCurrentWindow().innerSize();
    // 悬浮窗尺寸只能和主窗口一样大
    maxWidth = (await size).width
    maxHeight = (await size).height
}

const collapse = () => {
    props.openWindow('setting')
    if (!floatingWindowElement.value || newHeight <= 0) return
    floatingWindowElement.value.style.height = `${newHeight}px`;  // 移除 important
}
</script>
<style scoped>
.setting-main {
    display: flex;
    justify-content: start;
    align-content: center;
    flex-direction: column;
    position: fixed;
    top: 10%;
    left: 10%;
    width: 80%;
    min-width: fit-content;
    height: 80%;
    border-radius: 2vmin;
    padding: 10px;
    border: 2px solid var(--main-border);
    box-shadow: 0 4px 12px var(--font-color);
    white-space: nowrap;
    /* - 防止文本换行 */
    overflow: hidden;
    /* - 隐藏超出容器的文本 */
    background: linear-gradient(to right, var(--title-bar-lg-1), var(--title-bar-lg-2));
    animation: show-setting 0.5s ease-in-out forwards;
}

.setting-main-close {
    animation: hide-setting 0.5s ease-in-out forwards;

}

.setting-head {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    border-radius: 2vmin;
    height: 10%;
    min-height: 20px;
    max-height: 60px;
    padding: 2px;
    box-shadow: -2px 0.5vmin  20px -2px var(--positive-show-color);
    background: conic-gradient(from 0deg,
            var(--normal-attention-color),
            var(--title-min-icon-hover-shadow),
            var(--title-min-icon-active-shadow),

            var(--title-min-icon-shadow),
            var(--active-attention-color),

            var(--main-border),
            var(--normal-attention-color));
}

.setting-head-contain {
    display: flex;
    flex: 1;
    justify-content: start;
    align-items: center;
    flex-direction: row;
    border-radius: 2vmin;
    height: 98%;
    background: linear-gradient(to right, var(--title-bar-lg-1), var(--title-bar-lg-2));
}

.titlebar-button {
    display: flex;
    height: 4vh;
    width: 4vh;
    min-height: 15px;
    max-height: 30px;
    border-radius: 1vh;
    margin-right: 1vw;
    justify-content: center;
    align-items: center;
    will-change: filter;
    transition: 0.5s;

}

.setting-contain {
    display: flex;
    flex: 1;
    justify-content: center;
    align-content: center;
    flex-direction: row;
    width: 100%;
    padding: 1vmin;
    border-radius: 1vmin;
    box-sizing: border-box;
}

.left-setting-contain {
    width: 30%;
    min-width: fit-content;
    height: 100%;
    min-height: fit-content;

    position: relative;
    /* background-color: #fff; */

}

.left-setting-contain:after {
    position: absolute;
    content: "";
    top: 15%;
    right: 0;
    width: 2px;
    height: 70%;
    background: radial-gradient(circle at center, var(--icon-active-shadow), var(--icon-hover-shadow), var(--title-bar-lg-1), var(--title-bar-lg-2));

}

.right-setting-contain {
    flex: 1;
    min-width: fit-content;
    height: 100%;
    min-height: fit-content;

    /* background-color: #e00303; */

}

.hide-setting {
    margin-left: auto;
    margin-right: 2vmin;
}

.hide-setting:hover {
    animation: zoom-in-icon 0.75s forwards;
}

/* -------------------------------------------动画区------------------------------------------- */

@keyframes show-setting {
    0% {
        opacity: 0;
        transform: scale(0);
    }

    100% {
        opacity: 1;
        transform: scale(1);

    }

}

@keyframes hide-setting {
    0% {
        opacity: 1;
        transform: scale(1);
    }

    100% {
        opacity: 0;
        transform: scale(0);

    }

}


@keyframes zoom-in-icon {
    0% {
        transform: scale(0.9);
    }

    25% {
        transform: scale(0.75);
    }

    50% {
        transform: scale(0.9);
    }

    75% {
        transform: scale(0.75);
    }

    100% {
        transform: scale(1);
    }
}
</style>