<template>
    <div v-show="comVisibility.LeftContain['LeftContain-open']" class="main-left">
        <div class="navigation">
            <div class="contain">
                <RouterLink to="/">/</RouterLink>
                <RouterLink v-for="(item) in routePath" :key="item[0]" :to="item[1]">{{ routeName[item[1]] }}</RouterLink>
            </div>

        </div>
        <div class="sub-left-contain">

            <div v-if="menuFocus === 'normal'" v-for="(item) in comVisibility.LeftContain['LeftContain-data']['item']" :key="item['index']"
                class="item">
                <RouterLink :to="item.router" @click="" class="single-item">
                    <!-- 注意给路由提供图标颜色 -->
                    <div class="icon menu-icon">
                        <component :is="getIconComponent(item.icon)" :mainColor="mainColor" />
                    </div>
                    <div class="name">{{ item["name"] }}</div>
                </RouterLink>
            </div>
            <!-- 命名路由视图 - 侧边栏 -->
            <RouterView name="sidebar"></RouterView>
        </div>

    </div>
</template>
<script setup lang="ts">
import { RouterLink, useRoute } from "vue-router"
import { iconComponents } from "../util/PluginObjects"
import { IconComponentsType } from "../class/IconIndex"
import { ref, watch } from "vue"
import { type PathItem } from "../class/PathIndex"

const menuFocus = ref("normal")
const routePath = ref<PathItem[] | null>([])
// 添加 routeName 类型定义
interface RouteNameMap {
  [key: string]: string;
  "deal-file": string;
  "make-shutdown": string;
}

const routeName: RouteNameMap = {
  // 根据实际路由路径添加名称映射
  "deal-file": "处理文件",
  "make-shutdown": "关机问询",
  "live-picture": "未分类",
  // 可以根据需要添加更多路由映射
}
// 在组件的 setup 函数中
const route = useRoute()
watch(
    () => route.path,  // 监听路由路径变化
    (newPath) => {
        // 清空之前的数据
        routePath.value = []
        // 分割路径并添加到数组中
        const segments = newPath.split('/').filter(segment => segment !== '')  // 过滤空字符串
        segments.forEach((segment, index) => {
            routePath.value?.push([index.toString(), segment])
        })
        console.log(route.path)
        console.log(route.fullPath)
        console.log(route.meta)
        

    },
    { immediate: true }  // 立即执行一次以设置初始值
)

// import { watch, ref, onMounted, onUnmounted } from "vue"
defineProps({
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

// // 使用一个响应式引用来存储宽度值
// const containerWidth = ref(0);

// // 创建一个ResizeObserver来监听元素尺寸变化
// let resizeObserver: ResizeObserver | null = null;


// onMounted(() => {
//     setTimeout(
//         () => {
//             if (props.leftContainer) {
//                 // 初始化宽度
//                 containerWidth.value = props.leftContainer.getBoundingClientRect().width;

//                 // 创建ResizeObserver实例
//                 resizeObserver = new ResizeObserver(entries => {
//                     for (let entry of entries) {
//                         containerWidth.value = entry.contentRect.width;
//                     }
//                 });

//                 // 开始观察元素
//                 resizeObserver.observe(props.leftContainer);
//                 console.log("开始观察元素")
//             }
//         }, 300
//     )

// });

// watch(containerWidth, (newValue) => {
//     if (newValue <= 130) {
//         // 当宽度小于等于160时，根据宽度动态调整blur和letter-spacing
//         let blurValue = Math.max(0, Math.min(10, (130 - newValue) * 10 / 75));
//         let letterSpacing = Math.max(-20, Math.min(0, -(130 - newValue) * 25 / 75));
//         document.documentElement.style.setProperty("--font-blur", `${blurValue}px`)  // 动态栏的字模糊滤镜
//         document.documentElement.style.setProperty("--letter-spacing", `${letterSpacing}px`)  // 动态栏的字字间距
//     } else {
//         // 当宽度大于160时，重置为默认值
//         document.documentElement.style.setProperty("--font-blur", "0px")
//         document.documentElement.style.setProperty("--letter-spacing", "0px")
//     }

// })
// // 组件卸载时清理资源
// onUnmounted(() => {
//     if (resizeObserver && props.leftContainer) {
//         resizeObserver.unobserve(props.leftContainer);
//         resizeObserver.disconnect();
//     }
// });
// 定义更新事件
// const emit = defineEmits<{
//     (e: 'update:comVisibility', value: Object): void
// }>()

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
    border-radius: 2vh;
    font-size: clamp(6px, 1.75vmin, 32px);
    /* 最小值16px，理想值4vw，最大值32px */
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

.navigation {
    margin-top: 2vmin;
    margin-left: 3vmin;
    display: flex;
    height: 4vmin;
    max-width: 80%;
    justify-content: start;
    align-items: center;
    position: relative;
    background: linear-gradient(to top right, rgb(161, 201, 115), rgba(115, 201, 131, 1));
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    color: transparent;
    /* border-left: 1px solid var(--main-border); */
    border-bottom: 1px solid var(--main-border);
    border-bottom-right-radius: 1vmin;
    border-bottom-left-radius: 1vmin;

    white-space: nowrap;
    /* - 防止文本换行 */
    overflow: hidden;
    /* - 隐藏超出容器的文本 */
    text-overflow: ellipsis;
    /* - 当文本溢出时显示省略号 */
    box-sizing: border-box;
    /* - 确保padding被包含在元素的总宽度内 */
}

.navigation::before {
    content: "";
    position: absolute;
    /* top: 50%; */
    left: 0vw;
    width: 0.3vmin;
    max-width: 3px;
    border-radius: 1vmin;
    height: 4vmin;
    max-height: 20px;
    background-color: var(--active-attention-color);
    /* transform: translateY(50%); */
}

.navigation .contain {
    display: flex;
    justify-content: start;
    align-items: center;
    font-size: 1.5vmin;
    margin-left: 1vmin;
    user-select: none;
    /* 用户无法选择 */
    -webkit-user-select: none;
    /* Safari兼容性 */
    -moz-user-select: none;
    /* Firefox兼容性 */
    -ms-user-select: none;
}

.item {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
}

.single-item {
    filter: blur(var(--font-blur));
    letter-spacing: var(--letter-spacing);
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 5vmin;

    border-bottom-right-radius: 1vmin;
    box-shadow: 2px 4px 4px var(--tool-bar-color);

    border-top: var(--title-min-icon-shadow) solid 0.3vmin;
    border-bottom: var(--main-border) solid 0.3vmin;
    background: transparent;
    margin-top: 2vmin;
    min-height: 20px;
    max-height: 100px;
    height: 5vh;
    width: 80%;
    transition: 0.1s;
    white-space: nowrap;
    /* - 防止文本换行 */
    overflow: hidden;
    /* - 隐藏超出容器的文本 */
}

.single-item:active {
    animation: active-icon 0.25s forwards;
    animation-timing-function: linear;
}


.menu-icon {
    display: flex;
    justify-content: start;
    align-items: center;
    margin-right: 2vmin;
    margin-left: 2vmin;
    width: 40%;
}

.name {
    filter: blur(var(--font-blur));
    letter-spacing: var(--letter-spacing);
    display: flex;
    justify-content: start;
    align-items: center;
    width: 60%;
    height: 5vmin;
    user-select: none;
    /* 用户无法选择 */
    -webkit-user-select: none;
    /* Safari兼容性 */
    -moz-user-select: none;
    /* Firefox兼容性 */
    -ms-user-select: none;
    /* IE兼容性 */
    white-space: nowrap;
    /* - 防止文本换行 */
    overflow: hidden;
    /* - 隐藏超出容器的文本 */

}

.icon {
    height: 3vmin;
    width: 3vmin;
    min-height: 10px;
    max-height: 25px;
    min-width: 15px;
    max-width: 30px;
}

.icon:active {
    animation: active-icon 0.25s forwards;
    animation-timing-function: linear;
}

/* ----------------------------------------动画区----------------------------------------------- */


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

<style></style>