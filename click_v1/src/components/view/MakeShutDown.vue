<template>
  <div class="shutdown-main">
    <div class="shutdown-title">
      <div class="tag-icon tooltip" data-tooltip="操作栏" @click="() => { click('choose-function') }">
        <svg t="1766718761251" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
          p-id="7191" width="200" height="200">
          <path
            d="M802.864762 131.462095a97.52381 97.52381 0 0 1 97.523809 97.52381v547.84a97.52381 97.52381 0 0 1-97.523809 97.523809H188.025905a97.52381 97.52381 0 0 1-97.52381-97.523809V228.985905a97.52381 97.52381 0 0 1 97.52381-97.52381h614.887619z m0 73.142857H188.025905a24.380952 24.380952 0 0 0-24.380953 24.380953v547.84a24.380952 24.380952 0 0 0 24.380953 24.380952h614.887619a24.380952 24.380952 0 0 0 24.380952-24.380952V228.985905a24.380952 24.380952 0 0 0-24.380952-24.380953z"
            p-id="7192"></path>
          <path
            d="M472.795429 147.456v707.047619h-73.142858v-707.047619zM354.889143 323.486476v73.142857h-170.666667v-73.142857zM354.889143 466.358857v73.142857h-170.666667v-73.142857z"
            p-id="7193"></path>
        </svg>
      </div>
    </div>
    <div class="shutdown-contain">
      <form v-show="focusMethod === '0'" action="" class="timing-form">
        <div class="item-title tooltip" data-tooltip="------示例 施工ing🔨">
          <svg t="1766481043465" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
            p-id="7054" width="200" height="200">
            <path
              d="M512 145.92c201.728 0 366.08 164.352 366.08 366.08s-164.352 366.08-366.08 366.08S145.92 713.728 145.92 512 310.272 145.92 512 145.92M512 87.04c-234.496 0-424.96 190.464-424.96 424.96s189.952 424.96 424.96 424.96c234.496 0 424.96-190.464 424.96-424.96S746.496 87.04 512 87.04L512 87.04z"
              p-id="7055"></path>
            <path
              d="M578.56 721.92c-16.384 0-29.184-13.312-29.184-29.184l0-40.96c0-12.288 7.68-23.04 18.944-27.648 64-23.552 107.008-84.992 107.008-153.088 0-90.112-73.216-163.328-163.328-163.328-90.112 0-163.328 73.216-163.328 163.328 0 68.096 43.008 129.536 107.008 153.088 11.776 4.096 18.944 15.36 18.944 27.648l0 40.96c0 16.384-13.312 29.184-29.184 29.184-16.384 0-29.184-13.312-29.184-29.184L416.256 670.72c-76.288-36.352-126.464-114.688-126.464-200.192 0-122.368 99.328-221.696 221.696-221.696s221.696 99.328 221.696 221.696c0 85.504-50.176 163.328-126.464 200.192l0 21.504C607.744 708.608 594.432 721.92 578.56 721.92z"
              p-id="7056"></path>
            <path d="M578.56 742.912" p-id="7057"></path>
            <path d="M445.952 742.912" p-id="7058"></path>
            <path
              d="M559.104 818.688 465.408 818.688c-16.384 0-29.184-13.312-29.184-29.184s13.312-29.184 29.184-29.184l93.696 0c16.384 0 29.184 13.312 29.184 29.184S574.976 818.688 559.104 818.688z"
              p-id="7059"></path>
          </svg>
        </div>
        <div class="item-title tooltip" data-tooltip="配置 施工ing🔨">
          <svg t="1766504502772" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
            p-id="9296" width="200" height="200">
            <path
              d="M815.98 379.92c21.84-0.2 41.83-12.04 51.39-31.48l77.52-157.47c10.42-21.18 6.46-46.86-9.88-63.91L894.3 84.57c-18.3-19.09-46.97-22.94-69.62-9.4l-156.63 93.49c-16.94 10.12-27.47 28.67-27.47 48.41v70.13l-100.32 96.14c-30.02-32.81-56.01-61.27-69.8-76.49 10.82-91.02-6.52-157.53-51.59-197.79-63.69-56.86-158.93-40.02-196.93-30.12A37.72 37.72 0 0 0 194.92 106c-3.44 13.3 0.62 27.45 10.58 36.91l62.45 59.41c-1.18 16.28-8.15 31.38-19.89 42.97-10.64 10.5-24.11 17.02-38.66 18.86l-68.29-66.01c-10.3-9.94-25.07-13.21-38.63-8.54-13.49 4.66-23.08 16.34-25.05 30.45-6.29 45.12-13.59 156.76 54.53 218.73 40.81 37.14 99.25 48.39 173.87 33.53 2.06 2.16 4.34 4.57 6.79 7.14 13.14 13.81 29.93 31.45 43.35 44.9L114.99 765.29c-22.6 22.6-36.69 51.38-39.68 81.04-3.19 31.61 6.92 61.67 27.74 82.48 18.28 18.29 43.72 28.31 71.06 28.31 3.78 0 7.59-0.19 11.42-0.57 29.66-2.99 58.44-17.09 81.03-39.68l252.02-252.01c9.58 10.32 21.64 23.9 32.87 38.29-11.86 95.35 5.6 164.39 51.94 205.3 34.38 30.35 77.67 39.1 116.13 39.1 32.77 0 62.02-6.34 79.31-11.09 13.2-3.63 23.4-14.15 26.64-27.45 3.24-13.34-1.01-27.42-11.08-36.71l-60.43-55.97a61.251 61.251 0 0 1 18.28-42.72 61.258 61.258 0 0 1 38.8-17.45c17.16 17.36 45.84 46.04 61.23 59.6 9.95 8.77 23.99 11.7 36.59 7.56 12.69-4.12 22.38-14.75 25.28-27.72 21.94-97.55 10.13-167.95-35.08-209.25-58.66-53.6-151.71-37.85-196.52-25.9l-70.59-76.98 109.16-104.59 74.87 1.04zM358.8 435.47c-8.68-9.12-15.47-16.23-18.03-18.74-9.26-9.07-22.36-12.66-35.09-9.68-59.9 14.22-103.91 9.01-130.81-15.46-25.92-23.59-36.17-65.75-37.37-108.22l35.6 34.42a37.609 37.609 0 0 0 25.8 10.67c34.92 0.47 68.73-12.92 93.94-37.76 25.2-24.85 39.08-58.02 39.08-93.42v-5.95c0-10.33-4.3-20.34-11.75-27.39l-32.68-31.08c29.48-1.76 64.73 2.19 88.91 23.79 30.15 26.93 40.01 78.55 29.33 153.37-1.6 11.04 1.7 22.1 9.08 30.37 8.99 10.07 41.01 45.15 79.39 87.1l-12.5 11.98-20.44-20.44-60.21 60.21c-12.98-13.01-29.99-30.89-42.25-43.77zM221.47 871.78c-23.12 23.09-56.67 28.54-73.32 11.94-9.87-9.88-10.11-23.66-9.37-30.99 1.49-14.85 9.26-30.27 21.31-42.33l301.16-301.17 61.38 61.38-301.16 301.17z m445.07-256.21c9.97 10.83 25.43 14.91 39.29 10.37 10.73-3.44 106.26-32.64 150.22 7.51 23.92 21.85 27.62 61.79 23.68 101.23a3202.444 3202.444 0 0 1-30.7-30.87c-6.35-6.48-14.86-10.45-23.98-11.18-36.18-2.94-71.68 10.09-97.52 35.51a124.98 124.98 0 0 0-37.11 96.93l0.35 5.65c0.66 9.74 4.98 18.72 12.04 25.19l29.6 27.42c-28.91 1.85-63.07-1.73-86.8-22.68-38.51-34-37.34-105.29-29.57-159.16 1.4-9.82-1.17-20.05-7.11-28.12-15.2-20.54-32.45-39.95-45.22-53.62l49.13-49.13-35.15-35.15 8.2-7.86 80.65 87.96z m32.65-284.84l-166.6 159.62-5.77-5.77 160.11-153.44c11.09-10.59 17.44-25.46 17.44-40.78v-69.1l147.95-88.31 33.06 34.5-73.16 148.62-73.27-1.02c-15.21-0.32-29.05 5.38-39.76 15.68z"
              p-id="9297"></path>
          </svg>
        </div>
        <input v-model="timing.h" class="input-box" type="number" placeholder="时（h）" min="0" max="23">
        <input v-model="timing.m" class="input-box" type="number" placeholder="分（m）" min="0" max="59">
        <input v-model="timing.s" class="input-box" type="number" placeholder="秒（s）" min="0" max="59">
        <div class="submin-time" @click="() => { takeTiming(0 ,0, 0) }"><span>确定</span></div>
        <div class="reset-time" @click="() => { cleanInputTiming() }"><span>重置</span></div>
      </form>
    </div>
  </div>
  <MSDChooseMethoadFW v-model:FloatingWindow="FloatingWindow" :click="click" :changeMethod="changeMethod">
  </MSDChooseMethoadFW>
</template>

<script setup lang="ts">
import { ref, reactive , onMounted } from 'vue';
import MSDChooseMethoadFW from "./components-view/MSDChooseMethoadFW.vue"
import { invoke } from "@tauri-apps/api/core";  
import { alertMsg } from '../../util/PluginObjects'
const focusMethod = ref<string>("")
const FloatingWindow = reactive({
  "choose-function": false,
  "choose-function-close": false,
})

const timing = reactive({
  "h": null,
  "m": null,
  "s": null,
})

const errorMsg = ref("")
const closeerrorMsg = ref("")
const errorLevel = ref<number>(0) // 初始化为 0，确保类型为 number

onMounted(()=>{
  click('choose-function'); // 默认打开操作窗口
})
const takeTiming = (target_time: number, fn_mode: number, mode: number) => {
  const measure_time = (timing.h !== null ? Number(timing.h) : 0) * 3600 + 
                      (timing.m !== null ? Number(timing.m) : 0) * 60 + 
                      (timing.s !== null ? Number(timing.s) : 0);
  alertMsg(errorMsg, closeerrorMsg, `预备关机剩余时间\t${measure_time}s`, errorLevel, 0);
  switch (mode) {
    case 0:
      invoke("during_time_do_something",{time:measure_time,targetTime:target_time ,fnMode:fn_mode, mode:mode}  )
      break;
    case 1:
      
      break;
  
    default:
      break;
  }
  
}
const cleanInputTiming = () => {
  timing.h = null
  timing.m = null
  timing.s = null
}
const changeMethod = (methodName: string) => {
  focusMethod.value = methodName
}
const click = (whichOne: string, _: number = 0) => {
  switch (whichOne) {
    case 'choose-function':
      if (FloatingWindow["choose-function"] === false) {
        FloatingWindow["choose-function"] = true
      } else {
        FloatingWindow["choose-function-close"] = true;
        setTimeout(() => {
          FloatingWindow["choose-function"] = false;
          FloatingWindow["choose-function-close"] = false;
        }, 500)
      }
      break;
    default:
      break;
  }
}
</script>


<style scoped>
.shutdown-main {
  width: 100%;
  display: flex;
  justify-content: start;
  align-items: center;
  flex-direction: column;
  transition: all;
  margin-right: 1vw;
  margin-bottom: 1vw;
  font-size: 2vmin;
}

.shutdown-title {
  width: 100%;
  margin-top: 3%;
  height: 4vmin;
  position: relative;

}

.shutdown-title::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  width: 100%;
  height: 2px;
  background: linear-gradient(to right, var(--normal-attention-color) 20%, var(--active-attention-color) 40%, var(--main-border) 60%, var(--main-back-ground) 80%);
}


.tag-icon {
  width: 4vmin;
  height: 4vmin;
  display: flex;
  position: relative;
  justify-content: start;
  align-items: center;
}

.shutdown-contain {
  width: 100%;
  display: flex;
  margin-top: 3%;
  justify-content: start;
  align-content: center;
  flex-direction: column;
  border-radius: 2vmin;
  border: 1px solid var(--unite-but-color);
  background: var(--title-bar-lg-2);
  /* 画格子模拟纸张 */
  background-image:
    linear-gradient(to right, var(--button-color) 1px, transparent 1px),
    linear-gradient(to bottom, var(--button-color) 1px, transparent 1px);
  background-size: var(--grid-size) var(--grid-size);
  overflow: auto;
  animation: show-paper 0.5s ease-in-out forwards;
}

.timing-form {
  margin: 2vmin 2vmin;
  margin-bottom: 0vmin;
  display: grid;
  grid-template-columns: 5% 5% 18% 18% 18% 18% 18%;
  height: 4vmin;
  justify-content: center;
  align-content: center;
  border-top: 1px dashed var(--unite-but-color);
  border-bottom: 1px dashed var(--unite-but-color);
  animation: show-method 0.5s ease-in-out forwards;


}

.item-title {
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

.reset-time,
.submin-time {
  height: 4vmin;
  display: flex;
  justify-content: center;
  align-items: center;
  border: 0px solid var(--main-border);
  border-left: 2px solid var(--main-border);
  border-right: 2px solid var(--main-border);
  border-radius: 1vmin;
  user-select: none;
  /* 用户无法选择 */
  -webkit-user-select: none;
  /* Safari兼容性 */
  -moz-user-select: none;
  /* Firefox兼容性 */
  -ms-user-select: none;
  /* IE兼容性 */
  transition: 0.25s;
}

.reset-time:hover,
.submin-time:hover {
  box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
  filter: drop-shadow(0 0 1em var(--title-min-icon-hover-shadow));

}

.reset-time:active,
.submin-time:active {
  background: var(--title-min-icon-shadow);
  box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
  animation: active-icon 0.25s forwards;
  animation-timing-function: linear;
}

/* --------------------------------------------------------动画区----------------------------------------------------- */

@keyframes show-paper {
  0% {
    height: 0%;
  }

  100% {
    height: 88%;
  }

}

@keyframes show-method {
  0% {
    opacity: 0;
    filter: blur(10px);
  }

  100% {
    opacity: 1;
    filter: blur(0px);
  }
}
</style>