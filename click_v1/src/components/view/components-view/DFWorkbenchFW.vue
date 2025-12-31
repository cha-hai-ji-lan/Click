<template>
    <div v-show="FloatingWindow['work-bench']" class="work-bench-mask"
        :class="{ 'work-bench-mask-close': FloatingWindow['work-bench-close'] }" @click="handleClickOutside">
        <div class="work-bench-main">
            <div data-tauri-drag-region class="drag-window-icon" ref="dragCtrBut">
                <div class=" move-icon-box tooltip" data-tooltip="拖动">
                    <svg t="1766338639348" class="small-icon move-icon" @mousedown="startDrag" viewBox="0 0 1024 1024"
                        version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="9788" width="200" height="200">
                        <path
                            d="M476.5 924V100c0-19.8 16.2-36 36-36s36 16.2 36 36v824c0 19.8-16.2 36-36 36s-36-16.2-36-36z"
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
            <div class="reset" @click="">
                <svg t="1767188658699" class="icon" @click="() => { resetIcon() }" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="5438" width="200" height="200">
                    <path
                        d="M54.016 460.8A461.0048 461.0048 0 0 1 460.8 54.016V51.2a51.2 51.2 0 0 1 102.4 0v2.816A461.0048 461.0048 0 0 1 969.984 460.8H972.8a51.2 51.2 0 0 1 0 102.4h-2.816A461.0048 461.0048 0 0 1 563.2 969.984V972.8a51.2 51.2 0 0 1-102.4 0v-2.816A461.0048 461.0048 0 0 1 54.016 563.2H51.2a51.2 51.2 0 0 1 0-102.4h2.816zM157.184 460.8H204.8a51.2 51.2 0 0 1 0 102.4h-47.616A358.656 358.656 0 0 0 460.8 866.816V819.2a51.2 51.2 0 0 1 102.4 0v47.616A358.656 358.656 0 0 0 866.816 563.2H819.2a51.2 51.2 0 0 1 0-102.4h47.616A358.656 358.656 0 0 0 563.2 157.184V204.8a51.2 51.2 0 0 1-102.4 0v-47.616A358.656 358.656 0 0 0 157.184 460.8zM512 665.6a153.6 153.6 0 1 1 0-307.2 153.6 153.6 0 0 1 0 307.2z"
                        p-id="5439"></path>
                </svg>
            </div>

        </div>
    </div>
</template>
<script setup lang="ts">
import { type FloatingWindowState } from '../../../class/PathIndex'
import { ref, reactive } from 'vue';
const dragCtrBut = ref<HTMLElement | null>(null);
// const moveIconBox = ref<HTMLElement | null>(null);
// const dragCtrBut = ref<HTMLElement | null>(null);
const isDragging = ref(false);  // 鼠标是否正在拖拽
const dragOffset = ref({ x: 0, y: 0 });  // 鼠标拖拽的偏移量


const props = defineProps<{
    FloatingWindow: FloatingWindowState,
    click: (whichOne: string, index?: number) => void,  // 接收函数类型

}>()


const handleClickOutside = (event: any) => {
    // 检查点击的元素是否是 .work-bench-main 或其子元素
    const workBenchMain = event.target.closest('.work-bench-main');
    if (!workBenchMain) {
        // 如果点击的不是 .work-bench-main 或其子元素，则触发事件
        props.click('work-bench');
    }
}


const startDrag = (event: MouseEvent) => {
    if (!dragCtrBut.value) return;

    isDragging.value = true;

    // 计算鼠标相对于悬浮窗左上角的偏移量
    const rect = dragCtrBut.value.getBoundingClientRect();
    dragOffset.value = {
        x: event.clientX - rect.left,
        y: event.clientY - rect.top
    };

    // 添加全局事件监听器
    document.addEventListener('mousemove', drag);
    document.addEventListener('mouseup', stopDrag);
};

// 拖拽过程
const drag = (event: MouseEvent) => {
    if (!isDragging.value || !dragCtrBut.value) return;

    // 计算新的位置
    const newX = event.clientX - dragOffset.value.x;
    const newY = event.clientY - dragOffset.value.y;

    // 应用新位置
    dragCtrBut.value.style.left = `${newX}px`;
    dragCtrBut.value.style.top = `${newY}px`;
};

// 停止拖拽
const stopDrag = () => {
    isDragging.value = false;

    // 移除全局事件监听器
    document.removeEventListener('mousemove', drag);  // 移除拖拽调节位置事件
    document.removeEventListener('mouseup', stopDrag);
};

const resetIcon = () => {
    if (dragCtrBut.value) {
        dragCtrBut.value.style.top = '6%';
        dragCtrBut.value.style.left = '90%';
    }
}

</script>
<style scoped>
.work-bench-mask {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 15;
    background-color: rgba(0, 0, 0, 0);
    backdrop-filter: blur(8px);
    animation: show-work-bench 0.5s ease-in-out forwards;
}

.work-bench-mask-close {
    animation: hide-work-bench 0.5s ease-in-out forwards;

}

.work-bench-main {
    position: fixed;
    top: 5%;
    left: 5%;
    width: 90%;
    min-width: fit-content;
    height: 90%;
    min-height: fit-content;

    border-radius: 2vmin;
    border: 2px dashed var(--main-border);
    background: var(--title-bar-lg-2);
    box-shadow: 0px 0px 10px 2px var(--positive-show-color);
}

.drag-window-icon {
    position: fixed;
    display: flex;
    justify-content: start;
    align-items: end;
    top: 6%;
    left: 90%;
    width: 6vmin;
    height: 6vmin;
    z-index: 16;
    background: var(--title-bar-lg-1);
    border: 1px dashed var(--main-border);
    border-radius: 1vmin;
    box-shadow: 0px 0px 8px 1px var(--positive-show-color);
    cursor: grab;


}

.drag-window-icon:active {
    cursor: grabbing;

}

.move-icon-box {
    position: absolute;
    cursor: grab;
}

.move-icon-box:active {
    cursor: grabbing;
}

.small-icon {
    height: 2.5vmin;
    width: 2.5vmin;
    min-height: 2px;
    max-height: 15px;
    min-width: 2px;
    max-width: 15px;
    fill: var(--icon-color)
}

.small-icon:active {
    animation: active-icon 0.25s forwards;
    animation-timing-function: linear;
}

.move-icon {
    cursor: grab;
}

.move-icon:active {
    cursor: grabbing;
}

.reset {
    position: absolute;
    left: 1%;
    top: 1%;
}

.icon {
    height: 3vmin;
    width: 3vmin;
    min-height: 10px;
    max-height: 25px;
    min-width: 15px;
    max-width: 30px;
    fill: var(--icon-color)
}

.icon:active {
    animation: active-icon 0.25s forwards;
    animation-timing-function: linear;
}

/* --------------------------------------------------------气泡区提示----------------------------------------------------- */

.tooltip {
    position: relative;
    cursor: pointer;

}

.tooltip:hover::after {
    content: attr(data-tooltip);
    position: absolute;
    top: 103%;
    left: 50%;
    transform: translateX(-50%);
    background-color: var(--button-color);
    color: var(--font-color);
    text-align: center;
    padding: 6px 10px;
    border-radius: 4px;
    white-space: nowrap;
    z-index: 1000;
    font-size: 1.5vmin;
    opacity: 1;
    margin-bottom: 5px;

}


/* 为气泡添加过渡效果 */
.tooltip::after {
    animation: show-bubbles 0.5s forwards;
    animation-timing-function: ease-in-out;
}

/* --------------------------------------------------------动画区----------------------------------------------------- */


@keyframes show-work-bench {
    0% {
        opacity: 0;
        transform: scale(0);
    }

    100% {
        opacity: 1;
        transform: scale(1);

    }

}

@keyframes hide-work-bench {
    0% {
        opacity: 1;
        transform: scale(1);
    }

    100% {
        opacity: 0;
        transform: scale(0);

    }

}

@keyframes active-icon {
    0% {
        transform: scale(1);
    }

    50% {
        transform: scale(0.85);
    }

    100% {
        transform: scale(1);
    }

}
</style>