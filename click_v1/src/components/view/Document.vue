<template>
    <div class="main-doc">
        <div class="doc-index">

            <div class="index-contain">
                <ul>
                    <li>
                        <div class="index-item">
                            <MeIcon></MeIcon>
                            <span class="item-text">关于我</span>
                        </div>

                    </li>
                </ul>
            </div>
        </div>
        <div class="doc-text explorer-act-path">
            <h1># 我?</h1>
            <p class="click-say">
                <span>咔嗒目前致力于学习更多的东西,目前其实并没有明确的专攻方向😶‍🌫️,我还需要不断地学习充实自己.</span>
            </p>
            <p class="click-say"><span>我目前会做什么呢?</span></p>
            <p class="click-say"><span>我目前会:</span></p>
            <div class="table-wrapper">
                <table>
                    <thead>
                        <tr>
                            <th>
                                <MeIcon></MeIcon>
                            </th>
                            <th>
                                <IconFile></IconFile><br>文件处理
                            </th>
                            <th>
                                <IconShutDown></IconShutDown><br>关机问询
                            </th>
                            <th>
                                <LivePicture></LivePicture><br>实况GO图
                            </th>
                            <th>
                                <RecordScreen></RecordScreen><br>屏幕录制
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="positive-dia">能做什么?</td>
                            <td class="positive-dia">面向Explorer辅助功能</td>
                            <td class="positive-dia">目前仅能处理因情况关机</td>
                            <td class="positive-dia">用于编辑实况图片即(LivePhoto)</td>
                            <td class="positive-dia">面对屏幕的延迟摄影</td>
                        </tr>
                        <tr>
                            <td class="positive-dia">功能1</td>
                            <td>@修改名字字段</td>
                            <td>定时关机</td>
                            <td style="color:red">作者太懒还没实现功能</td>
                            <td style="color:red">作者太懒还没实现功能</td>
                        </tr>
                        <tr>
                            <td class="positive-dia">功能2</td>
                            <td>@改名排序</td>
                            <td>潮汐锁定</td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="positive-dia">功能3</td>
                            <td>@搜集存放</td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="positive-dia">功能4</td>
                            <td>@格式转化</td>
                            <td></td>
                            <td></td>
                            <td></td>
                        </tr>
                        <tr>
                            <td class="positive-dia">注意</td>
                            <td class="positive-dia" colspan="4">@:表示功能支持批处理</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <p class="click-say"><span>我不断进步,但并不是从零开始的,我也有自己的动力,赋予我能力驱动我前进的技术有:</span></p>
            <table>
                <tbody>
                    <tr>
                        <td class="positive-dia">
                            <div class="composition">骨架</div>
                        </td>
                        <td><img class="show-img tauri" src="../../assets/tauri.svg" alt=""
                                @click="() => { opeanLinkling('https://tauri.app/') }"></td>
                        <td><img class="show-img vue3" src="../../assets/vue.svg" alt=""
                                @click="() => { opeanLinkling('https://cn.vuejs.org/') }"></td>
                        <td><img class="show-img vite" src="../../assets/vite.svg" alt=""
                                @click="() => { opeanLinkling('https://vite.org.cn/') }"></td>
                    </tr>
                    <tr>
                        <td class="positive-dia">
                            <div class="composition">名称</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://tauri.app/') }">
                            <div class="composition">Tauri</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://cn.vuejs.org/') }">
                            <div class="composition">Vue3</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://vite.org.cn/') }">
                            <div class="composition">Vite</div>
                        </td>
                    </tr>
                    <tr>
                        <td class="positive-dia">
                            <div class="composition">血肉</div>
                        </td>
                        <td><img class="show-img ffmpeg" src="../../assets/ffmpeg.svg
                            " alt="" @click="() => { opeanLinkling('https://ffmpeg.org/') }"></td>
                        <td><img class="show-img image-magick" src="../../assets/ImageMagick.svg" alt=""
                                @click="() => { opeanLinkling('https://imagemagick.org/script/download.php#windows') }">
                        </td>
                        <td><img class="show-img seven-zip" src="../../assets/7zip.svg" alt=""
                                @click="() => { opeanLinkling('https://www.7-zip.org/') }">
                        </td>

                    </tr>
                    <tr>
                        <td class="positive-dia">
                            <div class="composition">名称</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://ffmpeg.org/') }">
                            <div class="composition">FFmpeg</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://imagemagick.org/script/download.php#windows') }">
                            <div class="composition">ImageMagick</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://www.7-zip.org/') }">
                            <div class="composition">7zip</div>
                        </td>
                    </tr>
                    <tr>
                        <td class="positive-dia">
                            <div class="composition">灵魂</div>
                        </td>
                        <td><img class="show-img rust" src="../../assets/rust.svg" alt=""
                                @click="() => { opeanLinkling('https://rust-lang.org/zh-CN/') }"></td>
                        <td><img class="show-img type-script" src="../../assets/typescript.svg" alt=""
                                @click="() => { opeanLinkling('https://www.typescriptlang.org/') }"></td>
                        <td><img class="show-img python" src="../../assets/Python.svg" alt=""
                                @click="() => { opeanLinkling('https://rust-lang.org/zh-CN/') }"></td>
                    </tr>
                    <tr>
                        <td class="positive-dia">
                            <div class="composition">名称</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://rust-lang.org/zh-CN/') }">
                            <div class="composition">Rust</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://www.typescriptlang.org/') }">
                            <div class="composition">TypeScript</div>
                        </td>
                        <td @click="() => { opeanLinkling('https://rust-lang.org/zh-CN/') }">
                            <div class="composition">Python</div>
                        </td>
                    </tr>
                </tbody>
            </table>
            <p class="click-say"><span>辅助工具:</span></p>


        </div>

    </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import IconFile from '../../icon/IconFile.vue';
import MeIcon from '../../icon/MeIcon.vue';
import IconShutDown from '../../icon/IconShutDown.vue';
import LivePicture from '../../icon/LivePicture.vue';
import RecordScreen from '../../icon/RecordScreen.vue';

const opeanLinkling = async (url: String) => {
    try {
        await invoke('open_url', { url });
    } catch (err) {
        console.error('无法打开链接:', err);
    }
}

</script>
<style scoped>
h1,
h2,
h3,
h4,
h5,
h6 {
    color: var(--normal-agree-color);
    text-shadow: 0.5px 0.5px 0.5px var(--negative-agree-color);
    border-bottom: 2px dashed var(--active-attention-color);
}

.table-wrapper {
    margin: 0 auto;
    width: fit-content;
    border: 2px solid var(--main-border);
    border-radius: 1vmin;
    /* 圆角 */
    overflow: hidden;
    /* 裁剪内容 */
    /* display: inline-block; */
    /* 防止容器宽度塌陷 */

}

table {
    margin: 0 auto;
    user-select: none;
    border-collapse: collapse;
    /* 合并表格边框 */
    border: 2px solid var(--main-border);
    border-radius: 10px;
    /* 设置圆角 */
    overflow: hidden;
    /* 确保圆角裁剪内容 */

}

tr,
th,
td {
    border: 2px dashed var(--main-border);

    text-align: center;
    /* 水平居中 */
    vertical-align: middle;
    /* 垂直居中 */

}

.composition {
    margin: 1vmin;
    /* 垂直居中 */

}


.main-doc {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 100%;
    font-size: 2vmin;
}

.doc-index {
    background: var(--main-border);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    width: 20vmin;
    height: 96%;
    margin: 2% 0;
    border-radius: 2.5vmin;
}

.index-contain {
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
    margin: 0.5vmin;
    flex: 1;
    border-radius: 2vmin;
    background: var(--title-bar-lg-2);
    width: 19vmin;


}

.index-contain ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

.index-contain li {
    padding: 0;
    margin: 0;
}

.index-item {
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: row;
    margin: 0.5vmin;

}

.index-item:active {
    animation: an-index-item 0.1s forwards;
    animation-timing-function: ease-in-out;

}

.item-text {
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
}

.doc-text {
    background: var(--title-bar-lg-1);
    border-radius: 2vmin;
    border: 2px solid var(--main-border);
    margin: 1%;
    padding: 1%;
    flex: 1;
    height: 85vmin;
    max-height: 85vmin;
    overflow-y: auto;
}

.click-say span {
    color: var(--normal-attention-color);
    background: linear-gradient(to right, var(--normal-attention-color), var(--active-attention-color)) no-repeat left bottom;
    background-size: 100% 1px;
}

.positive-dia {
    color: var(--negative-show-color);
    font-size: 2vmin;
    font-weight: bold;
}


.explorer-act-path::-webkit-scrollbar {
    width: 1vmin;
    /* 垂直滚动条宽度 */
    height: 1vmin;
    /* 水平滚动条高度 */

}

.show-img {
    width: 15vmin;
    height: 15vmin;
    margin: 1vmin;
    will-change: filter;
    transition: 0.75s;
}

.tauri:hover {
    filter: drop-shadow(0 0 0.75em rgba(36, 200, 219, 0.75));

}
.vue3:hover {
    filter: drop-shadow(0 0 0.75em rgba(65, 184, 131, 0.75));

}
.vite:hover {
    filter: drop-shadow(0 0 0.75em rgba(189, 52, 254, 0.75));

}
.ffmpeg:hover {
    filter: drop-shadow(0 0 0.75em rgba(17, 106, 32, 0.75));
}
.image-magick:hover {
    filter: drop-shadow(0 0 0.75em rgba(252, 252, 0, 0.75));
}
.seven-zip:hover {
    filter: drop-shadow(0 0 0.75em rgba(3, 3, 3, 0.75));
}
.rust:hover {
    filter: drop-shadow(0 0 0.75em rgba(255, 112, 67, 0.75));
}
.type-script:hover {
    filter: drop-shadow(0 0 0.75em rgba(0, 122, 204, 0.75));
}
.python:hover {
    filter: drop-shadow(0 0 0.75em rgba(0, 117, 170, 0.75));
}

.explorer-act-path::-webkit-scrollbar-track {
    background: var(--title-bar-lg-1);
    /* 滚动条轨道背景色 */
    border-radius: 4px;
}

.explorer-act-path::-webkit-scrollbar-thumb {
    background: var(--normal-attention-color);
    /* 滚动条滑块颜色 */
    border-radius: 4px;
}

.explorer-act-path::-webkit-scrollbar-thumb:hover {
    background: var(--active-attention-color);
    /* 滑块悬停时的颜色 */
}



@keyframes an-index-item {
    0% {
        transform: scale(1);
    }

    50% {
        transform: scale(0.75);
    }

    100% {
        transform: scale(1);
    }

}
</style>