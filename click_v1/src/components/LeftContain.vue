<template>
    <div v-show="comVisibility.LeftContain['LeftContain-open']" class="main-left">
        <div class="sub-left-contain">
            <div v-for="(item) in comVisibility.LeftContain['LeftContain-data']['item']" :key="item['index']"
                class="single-item">
                <RouterLink :to="item.router" class="single-contain">
                        <div class="icon menu-icon">
                            <component :is="getIconComponent(item.icon)" :mainColor="mainColor" />
                        </div>
                        <div class="name">{{ item["name"] }}</div>
                </RouterLink>



            </div>
        </div>

    </div>
</template>
<script setup lang="ts">
import { RouterLink } from "vue-router"
import { iconComponents } from "../util/PluginObjects"
import { IconComponentsType } from "../class/IconIndex"
import { watch, ref, onMounted, onUnmounted } from "vue"
const props = defineProps({
    comVisibility: {
        type: Object,
        default: {}
    },
    leftContainer: {
        type: [HTMLElement, null],
        default: null
    },
    open_sidebar_left: { //  暂未使用
        type: Function,
        default: () => { console.error("未获得父组件 侧栏控制权") }
    },
    mainColor: {
        type: Object,
        default: () => ({})
    }
});

// 使用一个响应式引用来存储宽度值
const containerWidth = ref(0);

// 创建一个ResizeObserver来监听元素尺寸变化
let resizeObserver: ResizeObserver | null = null;


onMounted(() => {
    setTimeout(
        () => {
            if (props.leftContainer) {
                // 初始化宽度
                containerWidth.value = props.leftContainer.getBoundingClientRect().width;

                // 创建ResizeObserver实例
                resizeObserver = new ResizeObserver(entries => {
                    for (let entry of entries) {
                        containerWidth.value = entry.contentRect.width;
                    }
                });

                // 开始观察元素
                resizeObserver.observe(props.leftContainer);
                console.log("开始观察元素")
            }
        }, 300
    )

});

watch(containerWidth, (newValue) => {
    // console.log(newValue)
    if (newValue <= 75) {
        // 当宽度小于等于75时，根据宽度动态调整blur和letter-spacing
        let blurValue = Math.max(0, Math.min(10, (75 - newValue) * 6 / 75));
        let letterSpacing = Math.max(-20, Math.min(0, -(75 - newValue) * 12 / 75));
        console.log(blurValue)
        document.documentElement.style.setProperty("--font-blur", `${blurValue}px`)  // 动态栏的字模糊滤镜
        document.documentElement.style.setProperty("--letter-spacing", `${letterSpacing}px`)  // 动态栏的字字间距
    } else {
        // 当宽度大于75时，重置为默认值
        document.documentElement.style.setProperty("--font-blur", "0px")
        document.documentElement.style.setProperty("--letter-spacing", "0px")
    }

})
// 组件卸载时清理资源
onUnmounted(() => {
    if (resizeObserver && props.leftContainer) {
        resizeObserver.unobserve(props.leftContainer);
        resizeObserver.disconnect();
    }
});
// 定义更新事件
const emit = defineEmits<{
    (e: 'update:comVisibility', value: Object): void
}>()

// 定义获取图标的函数
const getIconComponent = (iconName: string) => {
    return (iconComponents.value as IconComponentsType)[iconName]
}

</script>
<style scoped>
.main-left {
    width: 100%;
    height: 100%;
    min-width: 0px;
    max-width: 25vw;
    min-height: 100px;
    max-height: 900px;
    border-radius: 2vh;
    font-size: clamp(6px, 1.75vmin, 32px);
    /* 最小值16px，理想值4vw，最大值32px */
    ;
    font-family: "楷体", 'Courier New', Courier, monospace;
    font-weight: 600;
    border-right: 1px solid var(--main-border);
    background: linear-gradient(to left, var(--icon-hover), var(--main-back-ground));
}

.sub-left-contain {
    padding: 2vh 0;
    width: 100%;
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
}

.single-item {
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 1vmin;
    box-shadow: 2px 4px 4px var(--tool-bar-color);
    filter: blur(var(--font-blur));
    letter-spacing: var(--letter-spacing);
    border-top: var(--title-min-icon-shadow) solid 0.3vmin;
    border-bottom: var(--main-border) solid 0.3vmin;
    background: var(--button-color);
    margin-top: 1vh;
    min-height: 20px;
    max-height: 100px;
    height: 5vh;
    width: 90%;
    transition: 0.1s;

}

.single-item:nth-child(1) {
    border-top-right-radius: 2.5vmin;
    border-top-left-radius: 2.5vmin;
}

.single-item:last-child {
    border-bottom-right-radius: 2.5vmin;
    border-bottom-left-radius: 2.5vmin;
}

.single-contain {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    border-radius: 1vmin;
    box-shadow: inset 0px 0px 3px 2px var(--tool-bar-color);
}

.single-item:nth-child(1) .single-contain {
    border-top-right-radius: 2.5vmin;
    border-top-left-radius: 2.5vmin;

}

.single-item:last-child .single-contain {
    border-bottom-right-radius: 2.5vmin;
    border-bottom-left-radius: 2.5vmin;
}

.name {

    user-select: none;
    /* 用户无法选择 */
    -webkit-user-select: none;
    /* Safari兼容性 */
    -moz-user-select: none;
    /* Firefox兼容性 */
    -ms-user-select: none;
    /* IE兼容性 */
}

.single-contain:hover {
    box-shadow: 3px 7px 7px var(--tool-bar-color);
}

.single-item:active {
    box-shadow: 2px 4px 4px var(--main-border);
    transform: scale(0.95);
}

.menu-icon {
    margin-right: 2vw;
}
</style>