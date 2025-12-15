<template>
  <div class="main">
    <div data-tauri-drag-region class="titlebar">
      <div data-tauri-drag-region class="left-button">
        <div class="main-icon-box">
          <img class="main-icon" src="./assets/ico.svg" alt="">
        </div>
      </div>
      <div data-tauri-drag-region class="mid-button">
        <div class="mid-version">
          <h3>Click 0.1.2</h3>
        </div>
        <svg @click="() => { opeanLinkling('https://github.com/cha-hai-ji-lan/Click') }" t="1765561345418" class="icon"
          viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="4805" width="200" height="200">
          <path
            d="M0 524.992q0 166.016 95.488 298.496t247.488 185.504q6.016 0.992 10.016 0.992t6.496-1.504 4-3.008 2.016-4.992 0.512-4.992v-100.512q-36.992 4-66.016-0.512t-45.504-14.016-28.992-23.488-16.992-25.504-8.992-24-5.504-14.496q-8.992-15.008-27.008-27.488t-27.008-20-2.016-14.496q50.016-26.016 112.992 66.016 34.016 51.008 119.008 30.016 10.016-40.992 40-70.016Q293.984 736 237.984 670.976t-56-158.016q0-87.008 55.008-151.008-22.016-64.992 6.016-136.992 28.992-2.016 64.992 11.488t50.496 23.008 25.504 17.504q56.992-16 128.512-16t129.504 16q12.992-8.992 28.992-19.008t48.992-21.504 60.992-9.504q27.008 71.008 7.008 135.008 56 64 56 151.008 0 92.992-56.992 158.496t-172 85.504q43.008 43.008 43.008 104v128.992q0 0.992 0.992 3.008 0 6.016 0.512 8.992t4.512 6.016 12 3.008q152.992-52 250.496-185.504t97.504-300.512q0-104-40.512-199.008t-108.992-163.488-163.488-108.992T512.032 12.96 313.024 53.472 149.536 162.464t-108.992 163.488-40.512 199.008z"
            p-id="4806" :fill="mainColor.iconColor"></path>
        </svg>
      </div>
      <div data-tauri-drag-region class="right-button">
        <div class="titlebar-button" @click="() => { title_bar_click('min-size') }">
          <svg t="1765104088218" class="icon min-size-icon" viewBox="0 0 1024 1024" version="1.1"
            xmlns="http://www.w3.org/2000/svg" p-id="6114" width="200" height="200">
            <path
              d="M85.333333 384c25.6 0 42.666667-17.066667 42.666667-42.666667V128h213.333333c25.6 0 42.666667-17.066667 42.666667-42.666667s-17.066667-42.666667-42.666667-42.666666H85.333333c-25.6 0-42.666667 17.066667-42.666666 42.666666v256c0 25.6 17.066667 42.666667 42.666666 42.666667zM938.666667 640c-25.6 0-42.666667 17.066667-42.666667 42.666667v213.333333h-213.333333c-25.6 0-42.666667 17.066667-42.666667 42.666667s17.066667 42.666667 42.666667 42.666666h256c25.6 0 42.666667-17.066667 42.666666-42.666666v-256c0-25.6-17.066667-42.666667-42.666666-42.666667zM601.6 401.066667c4.266667 8.533333 12.8 17.066667 21.333333 21.333333 4.266667 4.266667 12.8 4.266667 17.066667 4.266667h256c25.6 0 42.666667-17.066667 42.666667-42.666667s-17.066667-42.666667-42.666667-42.666667h-153.6l226.133333-226.133333c17.066667-17.066667 17.066667-42.666667 0-59.733333-8.533333-8.533333-17.066667-12.8-29.866666-12.8s-21.333333 4.266667-29.866667 12.8L682.666667 281.6V128c0-25.6-17.066667-42.666667-42.666667-42.666667s-42.666667 17.066667-42.666667 42.666667v256c0 4.266667 0 12.8 4.266667 17.066667zM115.2 968.533333L341.333333 742.4V896c0 25.6 17.066667 42.666667 42.666667 42.666667s42.666667-17.066667 42.666667-42.666667v-256c0-4.266667 0-12.8-4.266667-17.066667-4.266667-8.533333-12.8-17.066667-21.333333-21.333333-4.266667-4.266667-12.8-4.266667-17.066667-4.266667H128c-25.6 0-42.666667 17.066667-42.666667 42.666667s17.066667 42.666667 42.666667 42.666667h153.6l-226.133333 226.133333c-17.066667 17.066667-17.066667 42.666667 0 59.733333s42.666667 17.066667 59.733333 0z"
              p-id="6115" :fill="mainColor.iconColor"></path>
          </svg>
        </div>
        <div class="titlebar-button " @click="() => { title_bar_click('switch-size') }">
          <svg v-show="switchScreenSize === true" t="1765104152369" class="icon max-size-icon" viewBox="0 0 1024 1024"
            version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="8951" width="200" height="200">
            <path
              d="M384 128c23.564433 0 42.666667 19.102234 42.666667 42.666667s-19.102234 42.666667-42.666667 42.666666l-110.327523-0.00113 161.830428 161.831559c16.66317 16.66204 16.66317 43.677104 0 60.339143-16.66204 16.66317-43.677104 16.66317-60.339143 0L213.333333 273.673607v110.326393c0 23.564433-19.102234 42.666667-42.666666 42.666667s-42.666667-19.102234-42.666667-42.666667v-170.666667c0-47.127735 38.205598-85.333333 85.333333-85.333333h170.666667z m-8.836238 460.497095c16.66204-16.66317 43.677104-16.66317 60.339143 0 16.66317 16.66204 16.66317 43.677104 0 60.339143L273.672477 810.666667h110.327523c23.328212 0 42.284645 18.722472 42.661015 41.961395l0.005652 0.705271c0 23.564433-19.102234 42.666667-42.666667 42.666667h-170.666667c-47.127735 0-85.333333-38.205598-85.333333-85.333333v-170.666667c0-23.564433 19.102234-42.666667 42.666667-42.666667s42.666667 19.102234 42.666666 42.666667v110.325263zM810.666667 128c47.127735 0 85.333333 38.205598 85.333333 85.333333v170.666667c0 23.564433-19.102234 42.666667-42.666667 42.666667s-42.666667-19.102234-42.666666-42.666667v-110.327523L648.836238 435.502905c-16.66204 16.66317-43.677104 16.66317-60.339143 0-16.66317-16.66204-16.66317-43.677104 0-60.339143l161.828168-161.830429H640c-23.564433 0-42.666667-19.102234-42.666667-42.666666s19.102234-42.666667 42.666667-42.666667h170.666667zM588.497095 588.497095c16.66204-16.66317 43.677104-16.66317 60.339143 0L810.666667 750.326393V640c0-23.328212 18.722472-42.284645 41.961395-42.661015l0.705271-0.005652c23.564433 0 42.666667 19.102234 42.666667 42.666667v170.666667c0 47.127735-38.205598 85.333333-85.333333 85.333333h-170.666667c-23.564433 0-42.666667-19.102234-42.666667-42.666667s19.102234-42.666667 42.666667-42.666666h110.326393L588.497095 648.836238c-16.495894-16.495894-16.660909-43.137978-0.495046-59.836185z"
              p-id="8952" :fill="mainColor.iconColor"></path>
          </svg>
          <svg v-show="switchScreenSize === false" t="1765104453226" class="icon max-size-icon" viewBox="0 0 1024 1024"
            version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="11909" width="200" height="200">
            <path
              d="M631.06 687.1v182.01c0 21.91-17.76 39.67-39.67 39.67s-39.67-17.76-39.67-39.67v-277.7c0-21.91 17.76-39.67 39.67-39.67h277.7c21.91 0 39.67 17.76 39.67 39.67s-17.76 39.67-39.67 39.67H687.08l261.31 261.24c15.49 15.48 15.48 40.6-0.02 56.07-15.48 15.46-40.56 15.45-56.03-0.02L631.06 687.1zM432.71 472.32h-277.7c-21.91 0-39.67-17.76-39.67-39.67s17.76-39.67 39.67-39.67h181.93L75.61 131.72c-15.48-15.48-15.48-40.57 0-56.05l0.04-0.04c15.48-15.48 40.58-15.48 56.05 0l261.26 261.33V154.95c0-21.91 17.76-39.67 39.67-39.67s39.67 17.76 39.67 39.67v277.7c0 21.87-17.71 39.62-39.59 39.67z"
              p-id="11910" :fill="mainColor.iconColor"></path>
          </svg>
        </div>
        <div class="titlebar-button" @click="() => { title_bar_click('close-screen') }">
          <svg t="1765102437899" class="icon close-icon" viewBox="0 0 1024 1024" version="1.1"
            xmlns="http://www.w3.org/2000/svg" p-id="1674" width="200" height="200">
            <path
              d="M704 108.8c6.4 0 12.8 6.4 19.2 6.4 19.2 12.8 44.8 6.4 57.6-12.8s6.4-44.8-12.8-57.6c-6.4-6.4-12.8-6.4-19.2-12.8-19.2-12.8-44.8 0-51.2 19.2-19.2 19.2-12.8 44.8 6.4 57.6z m230.4 96c-12.8-19.2-38.4-25.6-57.6-12.8-6.4 6.4-12.8 12.8-19.2 25.6 0 12.8 0 19.2 6.4 32 44.8 70.4 70.4 153.6 70.4 236.8a428.8 428.8 0 0 1-857.6 0c0-160 89.6-307.2 230.4-384 19.2-12.8 25.6-32 19.2-51.2-12.8-19.2-32-25.6-51.2-19.2C102.4 121.6 0 294.4 0 486.4c0 281.6 230.4 512 512 512s512-230.4 512-512c-6.4-96-32-198.4-89.6-281.6z"
              p-id="1675" :fill="mainColor.iconColor"></path>
            <path
              d="M307.2 332.8L454.4 480c6.4 6.4 6.4 6.4 0 12.8L307.2 640c-12.8 12.8-12.8 32-6.4 44.8 12.8 25.6 38.4 25.6 57.6 6.4 51.2-51.2 102.4-96 147.2-147.2 6.4-6.4 6.4-6.4 12.8 0 51.2 51.2 102.4 102.4 153.6 147.2 19.2 19.2 38.4 19.2 57.6 0 12.8-12.8 12.8-38.4 0-51.2l-70.4-70.4-76.8-76.8c-6.4-6.4-6.4-6.4 0-12.8 0 0 0-6.4 6.4-6.4l134.4-134.4c19.2-19.2 19.2-38.4 0-51.2-12.8-12.8-38.4-12.8-51.2 0L524.8 435.2c-6.4 6.4-6.4 6.4-12.8 0l-57.6-57.6-96-96c-12.8-12.8-32-12.8-44.8-6.4-19.2 12.8-25.6 38.4-6.4 57.6z"
              p-id="1676" :fill="mainColor.iconColor"></path>
          </svg>
        </div>
      </div>
    </div>

    <main class="container">
      <div v-show="comVisibility.LeftContain" class="left-contain" ref="leftContainer"
        :class="{ 'close-left-contain': comVisibility['LeftContain-close'] }" @mousedown="startResize">

        <LeftContain v-model:comVisibility="comVisibility"></LeftContain>

      </div>
      <!-- <div v-show="comVisibility.LeftContain" 
       class="resize-handle" 
       ></div> -->
      <div class="mid-contain">
        <div class="left-call-icon" @click="() => { open_sidebar('left-contain') }">
          <svg t="1765631009180" class="icon "
            :class="{ 'left-called-icon1': !comVisibility['LeftContain-close'], 'left-called-icon2': comVisibility['LeftContain-close'] }"
            viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2824" width="200"
            height="200">
            <path
              d="M368.576 116.672a16 16 0 0 0-22.656 0l-45.248 45.248a16 16 0 0 0 0 22.656l327.936 327.936-327.936 327.936a16 16 0 0 0 0 22.592l45.248 45.248a16 16 0 0 0 22.656 0l384.512-384.448a16 16 0 0 0 0-22.656L368.576 116.672z"
              p-id="2825" :fill="mainColor.iconColor"></path>
          </svg>
        </div>

      </div>
      <div class="right-contain">

      </div>

      <!-- <form class="row" @submit.prevent="greet"> -->
      <!-- <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form> -->
    </main>
  </div>

</template>
<script setup lang="ts">

// TODO: JPG 动图显示
import { onMounted, reactive, ref } from "vue";
import { Window } from "@tauri-apps/api/window";
import { ColorCtr } from './util/Colors'
import LeftContain from "./components/LeftContain.vue";
import { invoke } from "@tauri-apps/api/core";  // invoke：钩子方法 用于调用后端rust的函数
const mainColor = ref(ColorCtr());
const appWindow = Window.getCurrent()
const switchScreenSize = ref(true)
const comVisibility = reactive({
  "LeftContain": false,
  "LeftContain-close": true  // 保障关闭侧边栏按钮可以正常运转
})
const leftContainer = ref<HTMLElement | null>(null)
let isResizing = false
let leftContainWidth = 0  // 左侧容器宽度 方便改写和关闭按钮调用


onMounted(() => {
  mainColor.value.set_color_palette()
  set_focus_color_palette()  // 设置调色板
  set_special_style()  // 设置特殊样式
})

const set_focus_color_palette = () => {
  // 特殊颜色
  document.documentElement.style.setProperty("--title-close-icon-shadow", `rgba(${mainColor.value.colseIconColorRGBA})`)
  document.documentElement.style.setProperty("--title-close-icon-hover-shadow", `rgba(${mainColor.value.colseIconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--title-close-icon-active-shadow", `rgba(${mainColor.value.colseIconActiveColorRGBA})`)
  document.documentElement.style.setProperty("--title-min-icon-shadow", `rgba(${mainColor.value.minSizeIconColorRGBA})`)
  document.documentElement.style.setProperty("--title-min-icon-hover-shadow", `rgba(${mainColor.value.minSizeIconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--title-min-icon-active-shadow", `rgba(${mainColor.value.minSizeIconActiveColorRGBA})`)
  document.documentElement.style.setProperty("--title-max-icon-shadow", `rgba(${mainColor.value.maxSizeIconColorRGBA})`)
  document.documentElement.style.setProperty("--title-max-icon-hover-shadow", `rgba(${mainColor.value.maxSizeIconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--title-max-icon-active-shadow", `rgba(${mainColor.value.maxSizeIconActiveColorRGBA})`)
  // 总背景颜色
  document.documentElement.style.setProperty("--back-ground", `rgba(${mainColor.value.backGroundColorRGBA})`)
  // 标题栏颜色
  document.documentElement.style.setProperty("--title-bar-lg-1", `rgba(${mainColor.value.titleBarColorRGBA})`)
  document.documentElement.style.setProperty("--title-bar-lg-2", `rgba(${mainColor.value.backGroundColorRGBA})`)
  // 主区域颜色
  document.documentElement.style.setProperty("--main-border", `rgba(${mainColor.value.borderColorRGBA})`)  // 边框色
  document.documentElement.style.setProperty("--main-back-ground", `rgba(${mainColor.value.midGroundColorRGBA})`)  // 背景色
  document.documentElement.style.setProperty("--icon-hover", `rgba(${mainColor.value.foreGroundColorRGBA})`)
  document.documentElement.style.setProperty("--icon-hover-shadow", `rgba(${mainColor.value.iconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--icon-active-shadow", `rgba(${mainColor.value.iconActiveColorRGBA})`)
  document.documentElement.style.setProperty("--font-color", `rgba(${mainColor.value.fontColorRGBA})`)

}

const set_special_style = () => {
  document.documentElement.style.setProperty("--left-contain-width", `30vw`)  // 侧边栏宽度

}
const title_bar_click = (mode: string) => {

  switch (mode) {
    case 'min-size':
      appWindow.minimize()
      break;
    case 'switch-size':
      switchScreenSize.value = !switchScreenSize.value;
      appWindow.toggleMaximize();
      break;
    default:
      appWindow.close();
      break;
  }

}

const open_sidebar = (operObj: string) => {
  switch (operObj) {
    case 'left-contain':
      if (comVisibility.LeftContain) {
        if (!leftContainer.value) return
        leftContainer.value.style.width = ''  // 移除width 方便动画播放
        setTimeout(() => {
          comVisibility["LeftContain-close"] = true
        }, 50)
        setTimeout(() => {
          comVisibility.LeftContain = !comVisibility.LeftContain;
        }, 800)

      }
      else {
        comVisibility["LeftContain-close"] = false
        comVisibility.LeftContain = !comVisibility.LeftContain;

      }
      break;

    default:
      break;
  }

}

const opeanLinkling = async (url: String) => {
  try {
    await invoke('open_url', { url });
  } catch (err) {
    console.error('无法打开链接:', err);
  }
}

const startResize = (e: MouseEvent) => {
  isResizing = true
  document.addEventListener('mousemove', resize)
  document.addEventListener('mouseup', stopResize)
}

const resize = (e: MouseEvent) => {
  if (!isResizing || !leftContainer.value) return

  const containerRect = leftContainer.value?.getBoundingClientRect()

  if (!containerRect) return
  leftContainWidth = e.clientX * 1.2371134 // 1.2371134是修正系数

  // const newWidth = (e.clientX - containerRect.left) + 15
  // const newWidth = ((e.clientX - containerRect.left) / containerRect.width) * 100
  // 限制最小和最大宽度
  // const clampedWidth = Math.min(Math.max(newWidth, 10), 300)
  leftContainer.value.style.cssText += `width: ${leftContainWidth}px !important;`
  document.documentElement.style.setProperty("--left-contain-width", `${leftContainWidth}px`)  // 侧边栏宽度
}

const stopResize = () => {
  isResizing = false
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
}
</script>
<style scoped>
.main {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100vh;
  margin: 0;
  padding: 0;

  background-color: var(--back-ground);
}

.titlebar {
  display: flex;
  height: 6vh;
  min-height: 15px;
  max-height: 30px;
  justify-content: center;
  /* 左右 居中 */
  align-items: center;
  /* 上下 居中 */

  background: linear-gradient(to bottom, var(--title-bar-lg-1), var(--title-bar-lg-2))
}

.left-button,
.mid-button,
.right-button {
  display: flex;
  flex-direction: row;
  flex: 1;
  height: 4vh;
  min-height: 15px;
  max-height: 30px;
  justify-content: center;
  /* 左右 居中 */
  align-items: center;
  /* 上下 居中 */

}
.left-button{
  justify-content: left;
  /* 左右 居中 */
  align-items: center;
}

.mid-version {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 20vw;
  height: 4vh;
  min-height: 15px;
  max-height: 30px;
  border-radius: 1vh;
  border-left: 0.1vh solid var(--main-border);
  border-right: 0.1vh solid var(--main-border);
  margin-right: 1vw;
}

.mid-version:hover {
  border-left: 0.1vh solid var(--title-min-icon-shadow);
  border-right: 0.1vh solid var(--title-min-icon-shadow);
  filter: drop-shadow(0 0 1em var(--title-min-icon-hover-shadow));
}

.mid-version:active {
  border-left: 0.1vh solid var(--title-min-icon-hover-shadow);
  border-right: 0.1vh solid var(--title-min-icon-hover-shadow);
  filter: blur(1px);
}

.mid-version h3 {
  font: bold 1.5vh Arial, sans-serif;

  user-select: none;
  /* 用户无法选择 */
  -webkit-user-select: none;
  /* Safari兼容性 */
  -moz-user-select: none;
  /* Firefox兼容性 */
  -ms-user-select: none;
  /* IE兼容性 */
}


.right-button {
  justify-content: flex-end;
  padding-right: 1.5vw;

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

.min-size-icon,
.max-size-icon,
.close-icon {
  will-change: filter;
  transition: 0.75s;
}

.titlebar-button .min-size-icon:hover {
  animation: zoom-in-icon 0.75s forwards;
  animation-timing-function: ease;
  background: radial-gradient(circle at center, var(--title-min-icon-shadow) 45%, rgba(0, 0, 0, 0) 75%);
  filter: drop-shadow(0 0 0.75em var(--title-min-icon-hover-shadow));
}

.titlebar-button .max-size-icon:hover {
  animation: zoom-out-icon 0.75s forwards;
  animation-timing-function: ease;
  background: radial-gradient(circle at center, var(--title-max-icon-shadow) 45%, rgba(0, 0, 0, 0) 75%);
  filter: drop-shadow(0 0 0.75em var(--title-max-icon-hover-shadow));
}

.titlebar-button .close-icon:hover {
  animation: roll-icon 0.5s forwards;
  animation-timing-function: ease;
  background: radial-gradient(circle at center, var(--title-close-icon-shadow) 45%, rgba(0, 0, 0, 0) 75%);
  filter: drop-shadow(0 0 0.75em var(--title-close-icon-hover-shadow));
}

.titlebar-button .min-size-icon:active {
  background: radial-gradient(circle at center, var(--title-min-icon-active-shadow) 45%, rgba(0, 0, 0, 0) 75%);
  filter: drop-shadow(0 0 0.75em var(--title-min-icon-active-shadow));
}

.titlebar-button .max-size-icon:active {
  background: radial-gradient(circle at center, var(--title-max-icon-active-shadow) 45%, rgba(0, 0, 0, 0) 75%);
  filter: drop-shadow(0 0 0.75em var(--title-max-icon-active-shadow));
}

.titlebar-button .close-icon:active {
  background: radial-gradient(circle at center, var(--title-close-icon-active-shadow) 45%, rgba(0, 0, 0, 0) 75%);
  filter: drop-shadow(0 0 0.75em var(--title-close-icon-active-shadow));
}

.titlebar-button:nth-child(2):after {
  content: "";
  position: absolute;
  /* top: 50%; */
  left: -0.5vw;
  width: 0.2vw;
  max-width: 2px;
  height: 3vh;
  max-height: 20px;
  background-color: var(--main-border);
  /* transform: translateY(50%); */
}

.titlebar-button:nth-child(3):after {
  content: "";
  position: absolute;
  /* top: 50%; */
  left: -0.5vw;
  width: 0.2vw;
  max-width: 2px;
  height: 3vh;
  max-height: 20px;
  background-color: var(--main-border);
  /* transform: translateY(50%); */
}
.main-icon-box {
  display:flex;
  justify-content: center;
  align-items: center;
  height: 4vh;
  width: 4vh;
  min-height: 15px;
  max-height: 30px;
  min-width: 15px;
  max-width: 30px;
  margin-left: 0.5vw;
}
.main-icon {
  height: 4vh;
  width: 4vh;
  min-height: 15px;
  max-height: 30px;
  min-width: 15px;
  max-width: 30px;
}

.icon {
  height: 3vh;
  width: 3vh;
  min-height: 10px;
  max-height: 25px;
  min-width: 15px;
  max-width: 30px;
}

/* -------------------------------------主区域样式----------------------------------------- */
.container {
  transition: 0.5s;
  display: flex;
  flex: 1;
  /* position: relative; */
  flex-direction: row;
  margin: 0 5px 5px 5px;
  border-radius: 2vh;
  border: 1px solid var(--main-border);
  background: var(--main-back-ground);
}

.left-contain,
.mid-contain,
.right-contain {
  display: flex;
  position: relative;
}

.left-contain {
  margin: 1% 0 1% 1%;
  min-width: 0px;
  max-width: 25vw;
  min-height: 120px;
  max-height: 900px;
  animation: left-expand 0.75s forwards;
  animation-timing-function: ease;
}

.close-left-contain {
  animation: left-rexpand 0.75s forwards;
  animation-timing-function: ease;
}

.resize-handle {
  width: 5px;
  background: var(--main-border);
  cursor: col-resize;
  transition: background 0.3s;
}

.resize-handle:hover {
  background: var(--icon-hover-shadow);
}

.mid-contain {
  width: 100%;
  height: 100%;
}

.left-call-icon {
  display: flex;
  position: relative;
  background: var(--icon-hover);
  border-right: 1px solid var(--main-border);
  border-radius: 0 2vh 2vh 0;
  width: 2vh;
  top: 50%;
  height: 6vh;
  min-height: 10px;
  max-height: 50px;
  min-width: 10px;
  max-width: 30px;
  /* 左上角定位到中心 */
  translate: 0 -50%;
  /* 自己大小反向定位50% */
  justify-content: center;
  align-items: center;
  will-change: background, filter;
  transition: 0.75s
}

.left-call-icon:hover {
  filter: drop-shadow(0 0 0.75em var(--icon-hover-shadow));
  /* filter: blur(1px); 高斯模糊 --做毛玻璃 */
}

.left-call-icon:active {
  background: var(--icon-hover-shadow);
  filter: drop-shadow(0 0 0.75em var(--icon-active-shadow));
  /* filter: blur(1px); 高斯模糊 --做毛玻璃 */
}

.left-call-icon:before {
  content: "";
  position: absolute;
  /* top: 50%; */
  left: 0vw;
  width: 0.15vw;
  max-width: 2px;
  height: 3vh;
  border-radius: 0 5vh 5vh 0;
  background-color: var(--main-border);
  /* transform: translateY(50%); */


}

.left-call-icon:hover:before {
  animation: expand 0.75s forwards;
  animation-timing-function: linear;
}

.left-call-icon:active:before {
  background: radial-gradient(circle at center, var(--title-min-icon-active-shadow), var(--title-min-icon-hover-shadow), var(--title-min-icon-shadow))
}

.left-called-icon1 {
  animation: roll-0-180 0.75s forwards;
  animation-timing-function: linear;
}

.left-called-icon2 {
  animation: roll-180-0 0.75s forwards;
  animation-timing-function: linear;
}

.right-contain {
  top: 0;
  left: 100%;
}

/* ==============================================动画实现============================================== */
@keyframes expand {
  0% {

    transform: scaleY(2);
  }

  25% {
    transform: scaleY(4);
  }

  50% {
    transform: scaleY(8);
  }

  75% {
    transform: scaleY(16);
  }

  100% {
    transform: scaleY(28);
  }

}

@keyframes roll-icon {
  0% {
    transform: rotate(0);
  }

  100% {
    transform: rotate(360deg);
  }
}

@keyframes zoom-out-icon {
  0% {
    transform: scale(1.1);
  }

  25% {
    transform: scale(1.25);
  }

  50% {
    transform: scale(1.1);
  }

  75% {
    transform: scale(1.25);
  }

  100% {
    transform: scale(1);
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

@keyframes left-expand {
  0% {
    opacity: 0;
    /* opacity: calc(); 
    这段Vue代码中的opacity: calc();是一个CSS样式声明，用于设置元素的透明度。
    calc()函数允许通过计算来动态确定不透明度值，可以结合变量、数学运算符(+、-、*、/)来实现响应式或动态的透明度效果。
    例如：opacity: calc(1 - var(--scroll-progress));可以根据滚动进度动态调整元素透明度。
    */
    width: 0;
  }

  100% {
    opacity: 1;
    width: var(--left-contain-width);
  }
}

@keyframes left-rexpand {
  0% {
    opacity: 1;
    width: var(--left-contain-width);
  }

  100% {
    opacity: 0;
    width: 0;
  }
}

@keyframes roll-0-180 {
  0% {
    transform: rotate(0);
  }

  100% {
    transform: rotate(180deg);
  }
}

@keyframes roll-180-0 {
  0% {
    transform: rotate(180deg);
  }

  100% {
    transform: rotate(0);
  }
}
</style>
<style>
html {
  color: var(--font-color);
  background: transparent;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}

a {
  text-decoration: none;
  /* 移除下划线 */
  color: inherit;
  /* 继承父元素的颜色 */
  background-color: transparent;
  /* 背景透明 */
  border: none;
  /* 移除边框 */
}

body {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}
</style>