<!-- 用于 DealFile 转换选定路径下的格式转换 -->
<template>
    <div class="format-conversion-container">
        <div v-if="active_path && active_path.length > 0" class="order-and-replace-name">
            <div class="item-title tooltip" data-tooltip="------示例 施工ing🔨">
                <IconInfo></IconInfo>
            </div>
            <div class="item-title tooltip" data-tooltip="配置 施工ing🔨" @click="() => { open_configure() }">
                <svg t="1766504502772" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="9296" width="200" height="200">
                    <path
                        d="M815.98 379.92c21.84-0.2 41.83-12.04 51.39-31.48l77.52-157.47c10.42-21.18 6.46-46.86-9.88-63.91L894.3 84.57c-18.3-19.09-46.97-22.94-69.62-9.4l-156.63 93.49c-16.94 10.12-27.47 28.67-27.47 48.41v70.13l-100.32 96.14c-30.02-32.81-56.01-61.27-69.8-76.49 10.82-91.02-6.52-157.53-51.59-197.79-63.69-56.86-158.93-40.02-196.93-30.12A37.72 37.72 0 0 0 194.92 106c-3.44 13.3 0.62 27.45 10.58 36.91l62.45 59.41c-1.18 16.28-8.15 31.38-19.89 42.97-10.64 10.5-24.11 17.02-38.66 18.86l-68.29-66.01c-10.3-9.94-25.07-13.21-38.63-8.54-13.49 4.66-23.08 16.34-25.05 30.45-6.29 45.12-13.59 156.76 54.53 218.73 40.81 37.14 99.25 48.39 173.87 33.53 2.06 2.16 4.34 4.57 6.79 7.14 13.14 13.81 29.93 31.45 43.35 44.9L114.99 765.29c-22.6 22.6-36.69 51.38-39.68 81.04-3.19 31.61 6.92 61.67 27.74 82.48 18.28 18.29 43.72 28.31 71.06 28.31 3.78 0 7.59-0.19 11.42-0.57 29.66-2.99 58.44-17.09 81.03-39.68l252.02-252.01c9.58 10.32 21.64 23.9 32.87 38.29-11.86 95.35 5.6 164.39 51.94 205.3 34.38 30.35 77.67 39.1 116.13 39.1 32.77 0 62.02-6.34 79.31-11.09 13.2-3.63 23.4-14.15 26.64-27.45 3.24-13.34-1.01-27.42-11.08-36.71l-60.43-55.97a61.251 61.251 0 0 1 18.28-42.72 61.258 61.258 0 0 1 38.8-17.45c17.16 17.36 45.84 46.04 61.23 59.6 9.95 8.77 23.99 11.7 36.59 7.56 12.69-4.12 22.38-14.75 25.28-27.72 21.94-97.55 10.13-167.95-35.08-209.25-58.66-53.6-151.71-37.85-196.52-25.9l-70.59-76.98 109.16-104.59 74.87 1.04zM358.8 435.47c-8.68-9.12-15.47-16.23-18.03-18.74-9.26-9.07-22.36-12.66-35.09-9.68-59.9 14.22-103.91 9.01-130.81-15.46-25.92-23.59-36.17-65.75-37.37-108.22l35.6 34.42a37.609 37.609 0 0 0 25.8 10.67c34.92 0.47 68.73-12.92 93.94-37.76 25.2-24.85 39.08-58.02 39.08-93.42v-5.95c0-10.33-4.3-20.34-11.75-27.39l-32.68-31.08c29.48-1.76 64.73 2.19 88.91 23.79 30.15 26.93 40.01 78.55 29.33 153.37-1.6 11.04 1.7 22.1 9.08 30.37 8.99 10.07 41.01 45.15 79.39 87.1l-12.5 11.98-20.44-20.44-60.21 60.21c-12.98-13.01-29.99-30.89-42.25-43.77zM221.47 871.78c-23.12 23.09-56.67 28.54-73.32 11.94-9.87-9.88-10.11-23.66-9.37-30.99 1.49-14.85 9.26-30.27 21.31-42.33l301.16-301.17 61.38 61.38-301.16 301.17z m445.07-256.21c9.97 10.83 25.43 14.91 39.29 10.37 10.73-3.44 106.26-32.64 150.22 7.51 23.92 21.85 27.62 61.79 23.68 101.23a3202.444 3202.444 0 0 1-30.7-30.87c-6.35-6.48-14.86-10.45-23.98-11.18-36.18-2.94-71.68 10.09-97.52 35.51a124.98 124.98 0 0 0-37.11 96.93l0.35 5.65c0.66 9.74 4.98 18.72 12.04 25.19l29.6 27.42c-28.91 1.85-63.07-1.73-86.8-22.68-38.51-34-37.34-105.29-29.57-159.16 1.4-9.82-1.17-20.05-7.11-28.12-15.2-20.54-32.45-39.95-45.22-53.62l49.13-49.13-35.15-35.15 8.2-7.86 80.65 87.96z m32.65-284.84l-166.6 159.62-5.77-5.77 160.11-153.44c11.09-10.59 17.44-25.46 17.44-40.78v-69.1l147.95-88.31 33.06 34.5-73.16 148.62-73.27-1.02c-15.21-0.32-29.05 5.38-39.76 15.68z"
                        p-id="9297"></path>
                </svg>
            </div>
            <div class="item-title tooltip" data-tooltip="图片格式" @click="() => { chooseMode('img') }">
                <svg t="1769356546322" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="5340" width="200" height="200">
                    <path
                        d="M938.666667 553.92V768c0 64.8-52.533333 117.333333-117.333334 117.333333H202.666667c-64.8 0-117.333333-52.533333-117.333334-117.333333V256c0-64.8 52.533333-117.333333 117.333334-117.333333h618.666666c64.8 0 117.333333 52.533333 117.333334 117.333333v297.92z m-64-74.624V256a53.333333 53.333333 0 0 0-53.333334-53.333333H202.666667a53.333333 53.333333 0 0 0-53.333334 53.333333v344.48A290.090667 290.090667 0 0 1 192 597.333333a286.88 286.88 0 0 1 183.296 65.845334C427.029333 528.384 556.906667 437.333333 704 437.333333c65.706667 0 126.997333 16.778667 170.666667 41.962667z m0 82.24c-5.333333-8.32-21.130667-21.653333-43.648-32.917333C796.768 511.488 753.045333 501.333333 704 501.333333c-121.770667 0-229.130667 76.266667-270.432 188.693334-2.730667 7.445333-7.402667 20.32-13.994667 38.581333-7.68 21.301333-34.453333 28.106667-51.370666 13.056-16.437333-14.634667-28.554667-25.066667-36.138667-31.146667A222.890667 222.890667 0 0 0 192 661.333333c-14.464 0-28.725333 1.365333-42.666667 4.053334V768a53.333333 53.333333 0 0 0 53.333334 53.333333h618.666666a53.333333 53.333333 0 0 0 53.333334-53.333333V561.525333zM320 480a96 96 0 1 1 0-192 96 96 0 0 1 0 192z m0-64a32 32 0 1 0 0-64 32 32 0 0 0 0 64z"
                        p-id="5341"></path>
                </svg>
            </div>
            <div class="item-title tooltip" data-tooltip="视频格式" @click="() => { chooseMode('vid') }">
                <svg t="1770809708808" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="5343" xmlns:xlink="http://www.w3.org/1999/xlink"
                    width="200" height="200">
                    <path
                        d="M387.398 325.052l308.427 178.843L387.398 684.32V325.05z m-1.707-72.642c-6.981-4.05-14.879-4.965-22.17-3.32-2.235-0.512-4.531-0.838-6.905-0.838-17.005 0-30.797 13.777-30.797 30.782v450.42c0 17.005 13.792 30.798 30.797 30.798 2.607 0 5.09-0.42 7.51-1.04a30.13 30.13 0 0 0 23.086-3.118l371.293-217.212c11.109-6.501 14.52-12.544 15.003-31.108 0.483-18.564 0.217-30.379-13.203-38.152L385.691 252.41zM527.39 960.764h-31.03C118.628 958.212 61.5 836.652 61.5 502.291c0-324.754 71.37-439.29 434.86-439.29h31.03c363.489 0 434.858 114.536 434.858 439.29 0 334.361-57.126 455.921-434.858 458.473M496.36 0C91.88 0-0.001 140.854-0.001 502.228c0 361.389 70.718 518.985 498.207 521.71h27.338c427.488-2.725 498.207-160.321 498.207-521.71C1023.751 140.854 931.87 0 527.39 0h-31.03z"
                        fill="#333333" p-id="5344"></path>
                </svg>
            </div>
            <div class="占位符"></div>
            <div class="submit-replace-name" @click="() => { subCov() }">转化
            </div>
        </div>
        <div v-else class="place-holder">
            <span>🔎 当前无可观察路径...</span>
        </div>
        <div v-if="formatMode === 'img'" class="conversion-item">
            <SelectorBar class="select-bar" v-model="selectedValueSon[0]" :options="img_sourceOption"
                placeholder="原始格式">
            </SelectorBar>
            <div class="text">转成</div>
            <SelectorBar class="select-bar" v-model="selectedValueSon[1]" :options="img_aimOption" placeholder="目标格式">
            </SelectorBar>
        </div>
        <div v-if="formatMode === 'vid'" class="conversion-item">
            <SelectorBar class="select-bar" v-model="selectedValueSon[0]" :options="vid_sourceOption"
                placeholder="原始格式">
            </SelectorBar>
            <div class="text">转成</div>
            <SelectorBar class="select-bar" v-model="selectedValueSon[1]" :options="vid_aimOption" placeholder="目标格式">
            </SelectorBar>
        </div>
        <div v-if="open_configure_interface" class="configer">
            <div class="item">
                <div class="name">
                    <div class="tooltip" data-tooltip="示例 施工ing🔨">
                        <IconInfo></IconInfo>

                    </div>
                    <div class="text">{{ args_item_title[0] }}</div>
                </div>
                <div v-if="args_mode[0] === 'nor-resize'" class="poer">
                    <input v-model="args_g2[0]" type="text" placeholder="宽 例如:800">
                    <input v-model="args_g2[1]" type="text" placeholder="高 例如:400">
                    <div class="inline-button" @click="() => { changStyle('resize') }">% 表示</div>
                </div>
                <div v-if="args_mode[0] === 'percent-resize'" class="poer">
                    <input v-model="args_g2[0]" type="text" placeholder="宽 例如:80%">
                    <div class="inline-button" @click="() => { changStyle('resize') }">像素表示</div>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref, watch, computed, } from 'vue';
import SelectorBar from '../../components/SelectorBar.vue'
import IconInfo from '../../../icon/icons/IconInfo.vue'
import { imgExpendArgs2 } from '../../../util/FormatCov'
import { type PathItem } from '@/class/PathIndex'
const props = defineProps<{
    active_path: PathItem[] | null,
    formatSelectedValue: string[],
    inputRefSortName: string,
    SubmitRepluceName: (toolMethoad: string) => void
}
>()

const open_configure_interface = ref(false)

const cov_type = ref("format-conversion")
const selectedValueSon = ref(props.formatSelectedValue)
const formatMode = ref('')
const img_sourceOption = [
    { value: "AUTO", label: "自行推导" },
    { value: "jpg", label: "jpg|jpeg" },
    { value: "png", label: "png" },
    { value: "helf", label: "helf" },
    { value: "ico", label: "ico" },
    { value: "webp", label: "webp" },
    { value: "gif", label: "gif" },
    { value: "webm", label: "webm" },
    { value: "avif", label: "avif" },
    { value: "apng", label: "apng" },
    { value: "bmp", label: "bmp" },
    { value: "tif", label: "tif" },
    { value: "tiff", label: "tiff" },
    { value: "svg", label: "svg" },
    { value: "pdf", label: "pdf" },
    { value: "psd", label: "psd" },
    { value: "ai", label: "ai" },
    { value: "pbm", label: "pbm" },
    { value: "ppm", label: "ppm" },
    { value: "pgm", label: "pgm" },
    { value: "pnm", label: "pnm" },
    { value: "tag", label: "tag" },
    { value: "arw", label: "arw(摄影)" },
    { value: "nef", label: "nef(摄影)" },
    { value: "cr2", label: "cr2(摄影)" },
    { value: "eps", label: "eps" },
    { value: "ps", label: "ps" },
]
const img_aimOption = [
    { value: "jpg", label: "jpg|jpeg" },
    { value: "png", label: "png" },
    { value: "helf", label: "helf" },
    { value: "ico", label: "ico" },
    { value: "webp", label: "webp" },
    { value: "gif", label: "gif" },
    { value: "webm", label: "webm" },
    { value: "avif", label: "avif" },
    { value: "apng", label: "apng" },
    { value: "bmp", label: "bmp" },
    { value: "tif", label: "tif" },
    { value: "tiff", label: "tiff" },
    { value: "svg", label: "svg" },
    { value: "pdf", label: "pdf" },
    { value: "psd", label: "psd" },
    { value: "ai", label: "ai" },
    { value: "pbm", label: "pbm" },
    { value: "ppm", label: "ppm" },
    { value: "pgm", label: "pgm" },
    { value: "pnm", label: "pnm" },
    { value: "tag", label: "tag" },
    { value: "eps", label: "eps" },
    { value: "ps", label: "ps" },
]
// const args_g1 :string[] = ["", ""]
/**
 * @constant args_g2 参数索引对应表
 * img-----------------------------------------------
 * 0: 照片宽度 || 1: 照片高度 ||
 * ==================================================
 */
const args_g2: string[] = ["", ""]
const args_mode = ref<string[]>(["nor-resize", ""])
const args_item_title = ref<string[]>(["(横纵比🔒)调整图像尺寸(像素)", ""])
const vid_sourceOption = [
    { value: "AUTO", label: "自行推导" },
    { value: "avi", label: "avi" },
    { value: "mp4", label: "mp4" },
]
const vid_aimOption = [
    { value: "avi", label: "avi" },
    { value: "mp4", label: "mp4" },
]



const emit = defineEmits<{
    (e: 'update:formatSelectedValue', value: string[]): void,
}>()
watch(selectedValueSon, (newValue) => {
    console.log(newValue)
})
const combinedState = computed(() => ({
    selectedValue: selectedValueSon.value,
}));

// 监听组合状态的变化
watch(
    combinedState,
    (newState, oldState) => {
        // 检查具体是哪个值发生了变化
        if (newState.selectedValue !== oldState.selectedValue) {
            emit('update:formatSelectedValue', newState.selectedValue);

        }
    }
);

const subCov = () => {
    imgExpendArgs2(args_g2);
    console.log(cov_type.value);
    props.SubmitRepluceName(cov_type.value);
}
const chooseMode = (input_formate: string) => {
    switch (input_formate) {
        case 'img':
            cov_type.value = "img-format-conversion";
            break;
        case "vid":
            cov_type.value = "vid-format-conversion";
            break;
        default:
            break;
    }
    formatMode.value = input_formate;
}

const open_configure = () => {
    if (open_configure_interface.value === false) {
        open_configure_interface.value = true;
    } else {
        open_configure_interface.value = false;
    }
    console.log(open_configure_interface.value);

}

const changStyle = (mode: string) => {
    switch (mode) {
        case "resize":
            if (args_mode.value[0] === "nor-resize") {
                args_mode.value[0] = "percent-resize"
                args_item_title.value[0] = "(横纵比🔒)调整图像尺寸(%)"
            } else if (args_mode.value[0] === "percent-resize") {
                args_mode.value[0] = "nor-resize"
                args_item_title.value[0] = "(横纵比🔒)调整图像尺寸(像素)"
            }
            break;

        default:
            break;
    }

}
</script>
<style scoped>
.format-conversion-container {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: start;
    flex-direction: column;
}

.order-and-replace-name {
    font-size: 2vmin;
    display: grid;
    width: 100%;
    height: 4vmin;
    grid-template-columns: 5% 5% 5% 5% 65% 15%;
    border-bottom: 1px dashed var(--unite-but-color);
}


.item-title {
    height: 4vmin;
    display: flex;
    justify-content: center;
    align-items: center;
    user-select: none;
    /* 用户无法选择 */
    -webkit-user-select: none;
    /* Safari兼容性 */
    -moz-user-select: none;
    /* Firefox兼容性 */
    -ms-user-select: none;
    /* IE兼容性 */
    border-bottom: 1px dashed var(--unite-but-color);
}

.select-bar {
    height: 4vmin;

}

.text {
    display: flex;
    justify-content: center;
    align-items: center;
}

.conversion-item {
    display: grid;
    width: 100%;
    height: 4vmin;
    grid-template-columns: 45% 10% 45%;
    height: 4vmin;
    width: 100%;
}

.input-box {
    font-family: "楷体", 'Courier New', Courier, monospace;
    font-size: 2vmin;
    text-align: center;
    height: 4vmin;
    padding: 0px 0px;
    margin: auto 0;
    background: transparent;
    border-top: 0px dashed var(--unite-but-color);
    border-bottom: 2px dashed var(--unite-but-color);
    border-left: 0px dashed var(--unite-but-color);
    border-right: 0px dashed var(--unite-but-color);
    transition: all 0.3s ease;
}

.input-box:hover {
    border-color: var(--positive-show-color);
}

.input-box:focus {
    outline: none;
    /* 取消默认的焦点轮廓 */
    /* border: 1px solid var(--active-attention-color); */
    border-bottom: 2px dashed var(--active-attention-color);
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);

}

.submit-replace-name {
    border-radius: 1vmin;
    height: 4vmin;
    border-left: 2px solid var(--active-attention-color);
    border-right: 2px solid var(--active-attention-color);
    display: flex;
    justify-content: center;
    align-items: center;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    transition: 0.25s;
}

.submit-replace-name:hover {
    box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
    filter: drop-shadow(0 0 1em var(--title-min-icon-hover-shadow));

}

.submit-replace-name:active {
    background: var(--title-min-icon-shadow);
    box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
    animation: active-icon 0.25s forwards;
    animation-timing-function: linear;
}

.place-holder {
    width: 100%;
    height: 4vmin;
    font-size: medium;
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--font-color);

}

.configer {
    flex: 1;
    width: 100%;
    border: 2px dashed var(--unite-but-color);
    border-radius: 1vmin;
    overflow-y: auto;

}

.item {
    flex: 1;
    width: 100%;
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
}

.name {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 4vmin;
    border-bottom: 1px dashed var(--unite-but-color);



}

.poer {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 4vmin;
    border-bottom: 1px dashed var(--unite-but-color);

    input {
        flex: 1;
        font-family: "楷体", 'Courier New', Courier, monospace;
        font-size: 2vmin;
        margin-left: auto;
        height: 4vmin;
        text-align: center;
        background-color: transparent;
        border: 0px dashed var(--unite-but-color);
        border-bottom: 2px dashed var(--unite-but-color);

        &:hover {
            border-color: var(--positive-show-color);
        }

        &:focus {
            outline: none;
            /* 取消默认的焦点轮廓 */
            /* border: 1px solid var(--active-attention-color); */
            border-bottom: 2px dashed var(--active-attention-color);
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);

        }
    }
}

.inline-button {
    width: 10%;
    margin-right: 2vmin;
    margin-left: auto;
    padding: 0 0.5vmin;
    border: 0.2vmin solid var(--normal-attention-color);
    background-color: var(--main-back-ground);
    border-radius: 1vmin;
    transition: box-shadow 0.5s ease-in-out;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;

    &:hover {
        border: 0.15vmin solid var(--active-attention-color);
        box-shadow: 0 0 2vmin var(--normal-attention-color);
    }

    &:active {
        background-color: var(--normal-attention-color);
        border: 0.15vmin solid var(--active-attention-color);
        box-shadow: 0 0 2vmin var(--normal-attention-color);
        animation: active-icon 0.25s forwards;
        animation-timing-function: linear;
    }
}
</style>