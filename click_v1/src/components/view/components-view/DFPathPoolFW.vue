<template>
    <!-- 顶部悬浮活动栏 -->
    <div v-show="FloatingWindow['choose-path-pool']" class="choose-path-pool"
        :class="{ 'choose-path-pool-close': FloatingWindow['choose-path-pool-close'] }" ref="floatingWindowElement">
        <div class="drag-head-contain" @mousedown="startDrag">
            <div class="tag-icon tooltip" data-tooltip="拖动">
                <svg t="1766338639348" class="icon " viewBox="0 0 1024 1024" version="1.1"
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
            <div class="tag-icon wire tooltip" data-tooltip="清除全部路径" @click="() => { click('close-path-item-all') }">
                <svg t="1766420001028" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="6984" width="200" height="200">
                    <path
                        d="M630.784 831.488c12.288 0 20.48-8.192 20.48-16.384l28.672-450.56c0-12.288-8.192-20.48-16.384-20.48-12.288 0-20.48 8.192-20.48 16.384l-28.672 450.56C614.4 823.296 622.592 831.488 630.784 831.488z"
                        p-id="6985"></path>
                    <path
                        d="M409.6 831.488c12.288 0 20.48-8.192 16.384-20.48l-28.672-450.56c0-12.288-8.192-20.48-20.48-16.384C368.64 344.064 360.448 352.256 360.448 360.448l28.672 450.56C389.12 823.296 397.312 831.488 409.6 831.488z"
                        p-id="6986"></path>
                    <path
                        d="M520.192 831.488c12.288 0 20.48-8.192 20.48-20.48l0-450.56c0-12.288-8.192-20.48-20.48-20.48-12.288 0-20.48 8.192-20.48 20.48l0 450.56C499.712 823.296 507.904 831.488 520.192 831.488z"
                        p-id="6987"></path>
                    <path
                        d="M839.68 229.376l-188.416 0L651.264 151.552c0-20.48-16.384-36.864-36.864-36.864l-188.416 0c-20.48 0-36.864 16.384-36.864 36.864l0 73.728L200.704 225.28C188.416 229.376 180.224 237.568 180.224 245.76c0 12.288 8.192 20.48 20.48 20.48l36.864 0 36.864 602.112c4.096 40.96 32.768 73.728 73.728 73.728l339.968 0c40.96 0 69.632-32.768 73.728-73.728l36.864-602.112 36.864 0C851.968 266.24 860.16 258.048 860.16 245.76 860.16 237.568 851.968 229.376 839.68 229.376zM425.984 151.552 614.4 151.552l0 73.728-188.416 0L425.984 151.552zM729.088 868.352c-4.096 20.48-16.384 36.864-36.864 36.864L352.256 905.216c-20.48 0-32.768-16.384-36.864-36.864L274.432 266.24l491.52 0L729.088 868.352z"
                        p-id="6988"></path>
                </svg>
            </div>
            <div class="tag-icon wire tooltip" data-tooltip="选择文件" @click="() => { openFileDialog() }">
                <svg t="1766902940874" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="11272" width="200" height="200">
                    <path
                        d="M682.624 693.312a32 32 0 0 1 0 64H341.312a32 32 0 0 1 0-64h341.312zM682.624 522.688a32 32 0 1 1 0 64H341.312a32 32 0 0 1 0-64h341.312zM426.624 352a32 32 0 0 1 0 64H341.312a32 32 0 0 1 0-64h85.312z"
                        p-id="11273"></path>
                    <path
                        d="M603.648 53.952a32.128 32.128 0 0 1 10.56 4.288l1.536 0.96a31.808 31.808 0 0 1 4.224 3.456l256 256a32.064 32.064 0 0 1 9.408 22.656v512A117.44 117.44 0 0 1 768 970.624H256a117.312 117.312 0 0 1-117.312-117.312V170.624A117.376 117.376 0 0 1 256 53.312h341.376l6.272 0.64zM256 117.312a53.376 53.376 0 0 0-53.312 53.312v682.688a53.312 53.312 0 0 0 53.312 53.312h512a53.44 53.44 0 0 0 53.376-53.312v-480H597.312a32 32 0 0 1-32-32v-224H256z m373.312 192h146.816L629.312 162.496v146.816z"
                        p-id="11274"></path>
                </svg>
            </div>
            <div class="tag-icon wire tooltip" data-tooltip="选择文件夹" @click="() => { openFolderDialog() }">
                <svg t="1766725619065" class="icon" viewBox="0 0 1329 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="10656" width="200" height="200">
                    <path
                        d="M1036.572 951.784h-780.721c-62.416 0-113.411-47.899-115.072-107.38l-78.125-495.818v-2.735c0-60.873 51.662-110.396 115.164-110.396h14.961v-63.036c0-60.873 51.651-110.396 115.13-110.396h234.199c42.067 0 105.428 34.714 140.178 74.338h302.237c48.717 0 161.228 35.661 166.943 104.891 45.468 14.762 78.311 56.069 78.311 104.599v2.747l-78.137 495.807c-1.659 59.493-52.643 107.38-115.072 107.38zM136.887 343.559l78.008 495.070v2.747c0 21.109 18.374 38.279 40.957 38.279h780.721c22.582 0 40.957-17.17 40.957-38.279v-2.747l78.008-495.070c-1.274-20.046-19.146-35.989-40.932-35.989h-37.075v-59.903c-1.823-3.846-12.027-14.026-34.48-24.113-23.096-10.357-46.52-15.078-58.536-15.078h-341.532l-10.87-17.147c-17.953-28.368-70.714-57.191-90-57.191h-234.199c-22.593 0-40.968 17.182-40.968 38.279v135.152h-89.135c-21.775 0-39.647 15.942-40.921 36z"
                        p-id="10657"></path>
                    <path d="M100.875 306.542h1086.428v69.779h-1086.428z" p-id="10658"></path>
                </svg>
            </div>

            <div class="tag-icon float-close-icon tooltip" data-tooltip="收起">
                <svg t="1766338714320" class="icon " @click="() => { click('choose-path-pool') }"
                    viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="10844" width="200"
                    height="200">
                    <path
                        d="M199.36 572.768a31.904 31.904 0 0 0 22.624-9.376l294.144-294.144 285.728 285.728a31.968 31.968 0 1 0 45.248-45.248l-308.352-308.352a32 32 0 0 0-45.28 0l-316.768 316.768a31.968 31.968 0 0 0 22.656 54.624z"
                        p-id="10845"></path>
                    <path
                        d="M538.784 457.376a32 32 0 0 0-45.28 0l-316.768 316.768a31.968 31.968 0 1 0 45.248 45.248l294.144-294.144 285.728 285.728a31.968 31.968 0 1 0 45.248-45.248l-308.32-308.352z"
                        p-id="10846"></path>
                </svg>
            </div>

        </div>
        <div class="drag-contain">
            <div v-if="userSelectedShortPath && userSelectedShortPath.length <= 0">
                <span>🔎 当前无注视路径</span>
            </div>
            <div v-else-if="userSelectedShortPath && userSelectedShortPath.length > 0"
                v-for="(path_item, index) in userSelectedShortPath" :key="index" class="user-choose-item">
                <span class="tooltip" data-tooltip="清除" @click="() => { click('close-path-item', index) }">
                    <svg t="1766420001028" class="small-icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="6984" width="200" height="200">
                        <path
                            d="M630.784 831.488c12.288 0 20.48-8.192 20.48-16.384l28.672-450.56c0-12.288-8.192-20.48-16.384-20.48-12.288 0-20.48 8.192-20.48 16.384l-28.672 450.56C614.4 823.296 622.592 831.488 630.784 831.488z"
                            p-id="6985"></path>
                        <path
                            d="M409.6 831.488c12.288 0 20.48-8.192 16.384-20.48l-28.672-450.56c0-12.288-8.192-20.48-20.48-16.384C368.64 344.064 360.448 352.256 360.448 360.448l28.672 450.56C389.12 823.296 397.312 831.488 409.6 831.488z"
                            p-id="6986"></path>
                        <path
                            d="M520.192 831.488c12.288 0 20.48-8.192 20.48-20.48l0-450.56c0-12.288-8.192-20.48-20.48-20.48-12.288 0-20.48 8.192-20.48 20.48l0 450.56C499.712 823.296 507.904 831.488 520.192 831.488z"
                            p-id="6987"></path>
                        <path
                            d="M839.68 229.376l-188.416 0L651.264 151.552c0-20.48-16.384-36.864-36.864-36.864l-188.416 0c-20.48 0-36.864 16.384-36.864 36.864l0 73.728L200.704 225.28C188.416 229.376 180.224 237.568 180.224 245.76c0 12.288 8.192 20.48 20.48 20.48l36.864 0 36.864 602.112c4.096 40.96 32.768 73.728 73.728 73.728l339.968 0c40.96 0 69.632-32.768 73.728-73.728l36.864-602.112 36.864 0C851.968 266.24 860.16 258.048 860.16 245.76 860.16 237.568 851.968 229.376 839.68 229.376zM425.984 151.552 614.4 151.552l0 73.728-188.416 0L425.984 151.552zM729.088 868.352c-4.096 20.48-16.384 36.864-36.864 36.864L352.256 905.216c-20.48 0-32.768-16.384-36.864-36.864L274.432 266.24l491.52 0L729.088 868.352z"
                            p-id="6988"></path>
                    </svg>

                </span>
                <span>
                    <svg v-if="selectFolder === true" t="1766153567513" class="small-icon" viewBox="0 0 1024 1024"
                        version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="7556" width="200" height="200">
                        <path
                            d="M910.096853 360.579647v-61.707521c0-52.999376-42.030493-112.28336-98.281728-112.28336H528.430288l-11.681508-22.048347c-11.685603-22.352441-34.588894-36.229158-59.807189-36.229158H154.570848c-50.436591 0-91.447294 42.168717-91.447294 94.010079v575.540275c0 51.814741 41.010703 93.969123 91.447294 93.969124h711.539892c50.436591 0 91.447294-42.154383 91.447294-93.969124V444.614011c-0.001024-30.633625-20.252452-63.054954-47.461181-84.034364z m-98.281728-117.546131c19.540851 0 41.838002 29.75206 41.838002 55.83861v39.040748c-0.956309-0.038908-1.913642-0.097269-2.865855-0.097269H608.55341l-50.21748-94.782089h253.479195z m89.297134 554.828099c0 20.684531-15.709473 37.524373-35.002543 37.524374h-711.538868c-19.292047 0-35.002544-16.839842-35.002544-37.524374V222.32134c0-20.712176 15.709473-37.565329 35.002544-37.565329h302.370743c4.134449 0 7.882893 2.274049 9.867182 6.07676l99.825748 188.420497a28.252068 28.252068 0 0 0 24.94287 15.007087h259.209881c20.229926 0 50.326011 30.12373 50.326011 50.353656v353.247604z"
                            p-id="7557">
                        </path>
                    </svg>
                    <svg v-else t="1766902940874" class="small-icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="11272" width="200" height="200">
                        <path
                            d="M682.624 693.312a32 32 0 0 1 0 64H341.312a32 32 0 0 1 0-64h341.312zM682.624 522.688a32 32 0 1 1 0 64H341.312a32 32 0 0 1 0-64h341.312zM426.624 352a32 32 0 0 1 0 64H341.312a32 32 0 0 1 0-64h85.312z"
                            p-id="11273"></path>
                        <path
                            d="M603.648 53.952a32.128 32.128 0 0 1 10.56 4.288l1.536 0.96a31.808 31.808 0 0 1 4.224 3.456l256 256a32.064 32.064 0 0 1 9.408 22.656v512A117.44 117.44 0 0 1 768 970.624H256a117.312 117.312 0 0 1-117.312-117.312V170.624A117.376 117.376 0 0 1 256 53.312h341.376l6.272 0.64zM256 117.312a53.376 53.376 0 0 0-53.312 53.312v682.688a53.312 53.312 0 0 0 53.312 53.312h512a53.44 53.44 0 0 0 53.376-53.312v-480H597.312a32 32 0 0 1-32-32v-224H256z m373.312 192h146.816L629.312 162.496v146.816z"
                            p-id="11274"></path>
                    </svg>
                </span>
                <span>{{ path_item }}</span>


            </div>
            <div v-else>
                <span>🔎 当前无注视路径</span>
            </div>
        </div>
        <div v-for="(type, index) in dragType" :key="index" class="drag-btn" :class="type"
            @mousedown.stop="startMove($event, type)"></div>
    </div>
</template>
<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { type FloatingWindowState } from '../../../class/PathIndex';
import { ref, onMounted, onUnmounted } from "vue";
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
    userSelectedShortPath: string[] | null,
    selectFolder: boolean,
    FloatingWindow: FloatingWindowState,
    click: (whichOne: string, index?: number) => void,  // 接收函数类型
    openFileDialog: () => void,
    openFolderDialog: () => void,

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
    if (floatingWindowElement.value) {
        floatingWindowElement.value.style.left = `${newX}px`;
        floatingWindowElement.value.style.top = `${newY}px`;
    }
};

/**
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

const getsize = async () => {
    let size = getCurrentWindow().innerSize();
    // 悬浮窗尺寸只能和主窗口一样大
    maxWidth = (await size).width
    maxHeight = (await size).height
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
</script>
<style scoped>
.choose-path-pool {
    position: fixed;
    display: flex;
    justify-self: center;
    align-items: center;
    flex-direction: column;
    top: 1vh;
    left: 1vw;
    height: 80vmin;
    width: 45vmin;
    min-width: fit-content;
    min-height: fit-content;
    transform: translateZ(0);
    will-change: transform;
    user-select: none;
    overflow: auto;
    z-index: 100;

    /* 添加以下样式使元素更像悬浮窗 */

    border: 1px solid var(--unite-but-color);
    background: var(--title-bar-lg-2);
    /* 画格子模拟纸张 */
    background-image:
        linear-gradient(to right, var(--button-color) 1px, transparent 1px),
        linear-gradient(to bottom, var(--button-color) 1px, transparent 1px);
    background-size: var(--grid-size) var(--grid-size);
    overflow: auto;
    font-size: 2.5vmin;
    border-radius: 8px;
    box-shadow: 0 4px 12px var(--font-color);
    border: 1px solid #e0e0e0;
    padding: 1vmin;
    overflow-x: hidden;
    animation: show-choose-pool 0.5s ease-in-out forwards;

}

.choose-path-pool-close {
    animation: hide-choose-pool 0.5s ease-in-out forwards;
}

.drag-head-contain {
    display: flex;
    justify-self: start;
    align-items: start;
    flex-direction: row;
    height: 10%;
    min-height: 20px;
    width: 100%;
    min-width: 30px;
}

.tag-icon {
    width: 4vmin;
    height: 4vmin;
    display: flex;
    position: relative;
    justify-content: start;
    align-items: center;
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

.small-icon {
    margin: 0 0.5vmin;
    height: 2.5vmin;
    width: 2.5vmin;
    min-height: 2px;
    max-height: 15px;
    min-width: 2px;
    max-width: 15px;
    fill: var(--icon-color)
}

.float-close-icon {
    margin-left: auto;
    margin-right: 2vmin;
}

.wire::before {
    content: "";
    position: absolute;
    /* top: 50%; */
    left: -0.5vmin;
    width: 0.2vmin;
    border-radius: 0.1vmin;
    max-width: 2px;
    height: 2vmin;
    max-height: 20px;
    background-color: var(--main-border);
    /* transform: translateY(50%); */
}



.drag-contain {
    flex: 1;
    width: 100%;
    border: 1px dashed var(--unite-but-color);
    border-radius: 1vmin;
}

.user-choose-item {
    display: flex;
    justify-self: center;
    height: 2.5vmin;
    font-size: 2vmin;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    -webkit-box-orient: vertical;
    border-bottom: 1px dashed var(--unite-but-color);


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


.lt {
    position: absolute;
    top: 0;
    left: 0;
    height: 2.5%;
    width: 2.5%;
    cursor: nw-resize;
}

.lc {
    position: absolute;
    top: 2.5%;
    left: 0;
    height: 95%;
    width: 2.5%;
    cursor: e-resize;
}

.lb {
    position: absolute;
    bottom: 0;
    left: 0;
    height: 2.5%;
    width: 2.5%;
    cursor: sw-resize;


}

.ct {
    position: absolute;
    top: 0;
    left: 2.5%;
    width: 95%;
    height: 2.5%;
    cursor: n-resize;


}

.cb {
    position: absolute;
    bottom: 0;
    left: 2.5%;
    width: 95%;
    height: 2.5%;
    cursor: s-resize;
}

.rt {
    position: absolute;
    top: 0;
    right: 0;
    height: 2.5%;
    width: 2.5%;
    cursor: ne-resize;

}

.rc {
    position: absolute;
    top: 2.5%;
    right: 0;
    height: 95%;
    width: 2.5%;
    cursor: e-resize;

}

.rb {
    position: absolute;
    bottom: 0;
    right: 0;
    height: 2.5%;
    width: 2.5%;
    cursor: se-resize;

}

/* ----------------------------------------动画区----------------------------------------------- */


@keyframes show-choose-pool {
    0% {
        height: 0vmin;
        opacity: 0;
    }

    100% {
        height: 80vmin;
        opacity: 1;

    }

}

@keyframes hide-choose-pool {
    0% {
        height: 80vmin;
        opacity: 1;

    }

    100% {
        height: 0vmin;
        opacity: 0;

    }
}

@keyframes show-bubbles {
    0% {
        opacity: 0.25;
    }

    100% {
        opacity: 1;
    }

}
</style>