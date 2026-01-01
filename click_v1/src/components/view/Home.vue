<template>
    <div class="home-main">
        <div class="home-title tooltip" data-tooltip="connect：https://space.bilibili.com/419772620">
            <img class="home-main-icon" src="../../assets/ico.svg">
            <div class="welcome-msg">
                <h2>{{ titleMessage }}</h2>
                <div class="version">Click Version: 0.1.2</div>
            </div>
        </div>
        <div class="detail">
            <div class="storehouse tooltip" data-tooltip="跳转仓库" @click="() => { opeanLinkling('https://github.com/cha-hai-ji-lan/Click') }">
                <div class="msg">远程仓库</div>
                <div class="titlebar-button">
                    <svg t="1765561345418" class="big-icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="4805" width="200" height="200">
                        <path
                            d="M0 524.992q0 166.016 95.488 298.496t247.488 185.504q6.016 0.992 10.016 0.992t6.496-1.504 4-3.008 2.016-4.992 0.512-4.992v-100.512q-36.992 4-66.016-0.512t-45.504-14.016-28.992-23.488-16.992-25.504-8.992-24-5.504-14.496q-8.992-15.008-27.008-27.488t-27.008-20-2.016-14.496q50.016-26.016 112.992 66.016 34.016 51.008 119.008 30.016 10.016-40.992 40-70.016Q293.984 736 237.984 670.976t-56-158.016q0-87.008 55.008-151.008-22.016-64.992 6.016-136.992 28.992-2.016 64.992 11.488t50.496 23.008 25.504 17.504q56.992-16 128.512-16t129.504 16q12.992-8.992 28.992-19.008t48.992-21.504 60.992-9.504q27.008 71.008 7.008 135.008 56 64 56 151.008 0 92.992-56.992 158.496t-172 85.504q43.008 43.008 43.008 104v128.992q0 0.992 0.992 3.008 0 6.016 0.512 8.992t4.512 6.016 12 3.008q152.992-52 250.496-185.504t97.504-300.512q0-104-40.512-199.008t-108.992-163.488-163.488-108.992T512.032 12.96 313.024 53.472 149.536 162.464t-108.992 163.488-40.512 199.008z"
                            p-id="4806"></path>
                    </svg>
                </div>
            </div>
            <div class="about tooltip" data-tooltip="关于我 施工ing🔨">
                <div class="about-msg">关于</div>
                <div class="titlebar-button"
                    @click="() => { opeanLinkling('https://github.com/cha-hai-ji-lan/Click') }">
                    <svg t="1766736987880" class="about-icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="12616" width="200" height="200">
                        <path
                            d="M514.432 74.688c93.44 0 166.912 0 224.192 7.68 58.752 7.872 105.344 24.384 141.952 61.056 36.608 36.608 53.12 83.2 61.056 141.952 7.68 57.28 7.68 130.688 7.68 224.192v4.864c0 93.44 0 166.912-7.68 224.192-7.872 58.752-24.384 105.344-61.056 141.952-36.608 36.608-83.2 53.12-141.952 61.056-57.28 7.68-130.688 7.68-224.192 7.68h-4.864c-93.44 0-166.912 0-224.192-7.68-58.752-7.872-105.344-24.384-141.952-61.056-36.672-36.608-53.12-83.2-61.056-141.952-7.68-57.28-7.68-130.688-7.68-224.192v-4.864c0-93.44 0-166.912 7.68-224.192 7.872-58.752 24.384-105.344 61.056-141.952 36.608-36.672 83.2-53.12 141.952-61.056 57.28-7.68 130.688-7.68 224.192-7.68h4.864z m-16 374.464C489.6 448 479.552 448 471.04 448h-1.728a42.688 42.688 0 1 0 0 85.312V704a42.688 42.688 0 1 0 85.376 0V531.648c0-8.512 0-18.688-1.216-27.392a63.488 63.488 0 0 0-17.536-37.504 63.488 63.488 0 0 0-37.504-17.6z m13.376-171.84A42.56 42.56 0 0 0 469.312 320c0 23.552 19.008 42.688 42.496 42.688h0.384A42.56 42.56 0 0 0 554.688 320a42.56 42.56 0 0 0-42.496-42.688h-0.384z"
                            p-id="12617"></path>
                    </svg>
                </div>
            </div>

        </div>

    </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
const titleMessage = ref("咔嗒")



const opeanLinkling = async (url: String) => {
    try {
        await invoke('open_url', { url });
    } catch (err) {
        console.error('无法打开链接:', err);
    }
}

</script>
<style scoped>
.home-main {

    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
    width: 100%;
    transition: all;
    user-select: none;
    /* 用户无法选择 */
    -webkit-user-select: none;
    /* Safari兼容性 */
    -moz-user-select: none;
    /* Firefox兼容性 */
    -ms-user-select: none;
    /* IE兼容性 */

    animation: show-home 0.5s both;
    animation-timing-function: ease-in-out;
}


.home-title {
    display: flex;
    width: 90%;
    height: 30vmin;
    margin-top: 10vmin;
    padding: 2vmin;
    justify-content: center;
    align-items: center;
    border: 0.5vmin dashed var(--title-min-icon-shadow);
    background: var(--title-bar-lg-1);
    border-radius: 5vmin;
    filter: drop-shadow(0 0 1em var(--title-min-icon-hover-shadow));

}

.home-main-icon {
    width: 30vmin;
    height: 30vmin;
}

.welcome-msg {
    width: 50vmin;
}

.welcome-msg h2 {
    font-size: 8vmin;
    margin-left: 4vmin;
}

.welcome-msg .version {
    font-size: 3vmin;
    color: var(--normal-show-color);
    /* color */
}

.detail {
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: row;
}

.storehouse {
    margin-top: 5vmin;
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: fit-content;
    /* 自适应内容大小 */
    height: 14vmin;
    background: var(--title-bar-lg-2);
    border-radius: 2vmin;
    border: 0.5vmin dashed var(--positive-show-color);
    box-shadow: 1vmin 1vmin 1vmin var(--positive-show-color);

}

.about {
    margin-top: 5vmin;
    margin-left: 2vmin;
    display: flex;
    flex: 1;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: fit-content;
    /* 自适应内容大小 */
    height: 14vmin;
    background: var(--title-bar-lg-2);
    border-radius: 2vmin;
    border: 0.5vmin dashed var(--title-max-icon-hover-shadow);
    box-shadow: 1vmin 1vmin 1vmin var(--title-max-icon-active-shadow);
}

.msg {
    display: flex;
    width: 20vmin;
    height: 10vmin;
    font-size: 4vmin;
    font-weight: 900;
    justify-content: center;
    align-items: center;
    color: var(--normal-show-color);
}

.about-msg {
    display: flex;
    width: 20vmin;
    height: 10vmin;
    font-size: 4vmin;
    font-weight: 900;
    justify-content: center;
    align-items: center;
    color: var(--title-max-icon-shadow);
}

.titlebar-button {
    display: flex;

    width: 12vmin;
    height: 10vmin;
    justify-content: center;
    align-items: center;
}

.big-icon {
    width: 10vmin;
    height: 10vmin;
    fill: var(--normal-show-color);

}

.about-icon {
    width: 10vmin;
    height: 10vmin;
    fill: var(--title-max-icon-shadow);


}

/* --------------------------------------------------------动画区----------------------------------------------------- */
@keyframes show-home {
    0% {
        opacity: 0;
    }

    100% {
        opacity: 1;
    }

}
</style>