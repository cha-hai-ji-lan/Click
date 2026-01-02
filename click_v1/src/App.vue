<!-- TODO:: 重构 左侧栏唤醒按钮，移除全部 transform 尝试完善左侧栏的拉动效果 -->
<template>
  <div class="main">
    <div data-tauri-drag-region class="titlebar">
      <div data-tauri-drag-region class="left-button">
        <div class="main-icon-box">
          <RouterLink to="/">
            <img class="main-icon" src="./assets/ico.svg" alt="卡塔">

          </RouterLink>
        </div>
      </div>
      <div data-tauri-drag-region class="mid-button">
        <div class="titlebar-button tooltip" data-tooltip="设置 施工ing🔨" @click="() => { openWindow('setting') }">
          <svg t="1766646139964" class="icon" viewBox="0 0 1084 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
            p-id="5350" width="200" height="200">
            <path
              d="M1072.147851 406.226367c-6.331285-33.456782-26.762037-55.073399-52.047135-55.073399-0.323417 0-0.651455 0.003081-0.830105 0.009241l-4.655674 0c-73.124722 0-132.618162-59.491899-132.618162-132.618162 0-23.731152 11.447443-50.336101 11.546009-50.565574 13.104573-29.498767 3.023185-65.672257-23.427755-84.127081l-1.601687-1.127342-134.400039-74.661726-1.700252-0.745401c-8.753836-3.805547-18.334698-5.735272-28.479231-5.735272-20.789593 0-41.235746 8.344174-54.683758 22.306575-14.741683 15.216028-65.622973 58.649474-104.721083 58.649474-39.450789 0-90.633935-44.286652-105.438762-59.784516-13.518857-14.247316-34.128258-22.753199-55.127302-22.753199-9.945862 0-19.354234 1.861961-27.958682 5.531982l-1.746455 0.74078-139.141957 76.431283-1.643269 1.139662c-26.537186 18.437884-36.675557 54.579032-23.584845 84.062398 0.115506 0.264895 11.579891 26.725075 11.579891 50.634877 0 73.126262-59.491899 132.618162-132.618162 132.618162l-4.581749 0c-0.318797-0.00616-0.636055-0.01078-0.951772-0.01078-25.260456 0-45.672728 21.618157-52.002472 55.0811-0.462025 2.453354-11.313456 60.622322-11.313456 106.117939 0 45.494078 10.85143 103.659965 11.314996 106.119479 6.334365 33.458322 26.758957 55.076479 52.036353 55.076479 0.320337 0 0.651455-0.00616 0.842426-0.012321l4.655674 0c73.126262 0 132.618162 59.491899 132.618162 132.616622 0 23.760413-11.444363 50.333021-11.546009 50.565574-13.093793 29.474125-3.041666 65.646075 23.395414 84.151722l1.569346 1.093459 131.838879 73.726895 1.675611 0.7377c8.750757 3.84251 18.305437 5.790715 28.397607 5.790715 21.082208 0 41.676209-8.706094 55.0888-23.290689 18.724339-20.347588 69.527086-62.362616 107.04815-62.362616 40.625872 0 92.72537 47.100385 107.759669 63.583903 13.441852 14.831008 34.176001 23.689571 55.470741 23.695731l0.00616 0c9.895039 0 19.27877-1.883523 27.893999-5.598205l1.711034-0.73924 136.659342-75.531873 1.617088-1.128882c26.492523-18.456365 36.601633-54.600594 23.538642-84.016195-0.115506-0.267974-11.595291-27.082374-11.595291-50.67646 0-73.124722 59.49344-132.616622 132.618162-132.616622l4.517066-0.00154c0.300316 0.00616 0.599092 0.009241 0.899409 0.009241 25.331299-0.00154 45.785153-21.619697 52.107197-55.054918 0.112426-0.589852 11.325776-59.507301 11.325776-106.14104C1083.464388 466.640776 1072.609877 408.67356 1072.147851 406.226367zM377.486862 945.656142l-115.32764-64.487932c5.082277-13.052211 15.437801-43.51815 15.437801-75.017486 0-109.382917-84.176364-199.816642-192.587488-208.134635-2.647404-15.427021-8.873963-54.967133-8.873963-85.667166 0-30.65691 6.223479-70.232445 8.869343-85.671786 108.415744-8.311832 192.592108-98.745557 192.592108-208.134635 0-31.416171-10.300081-61.797405-15.371577-74.854236l122.721583-67.40331c0.003081 0 0.00462 0.00154 0.007701 0.00154 4.423121 4.518606 22.121764 22.080182 46.558275 39.493911 39.929754 28.46229 77.952885 42.894416 113.014434 42.894416 34.716571 0 72.437845-14.151831 112.115025-42.06431 24.282503-17.07953 41.896442-34.302288 46.308782-38.74543 0.009241-0.00154 0.018481-0.00462 0.026182-0.00616l118.301542 65.726159c-5.077657 13.055291-15.416239 43.499669-15.416239 74.958962 0 109.389077 84.174824 199.822802 192.590568 208.134635 2.645865 15.462442 8.872423 55.107281 8.872423 85.671786 0 30.687711-6.223479 70.241685-8.869343 85.673326C890.042174 606.334084 805.86427 696.767809 805.86427 806.158426c0 31.450053 10.317022 61.851309 15.393138 74.903519l-119.783103 66.198965c-5.168521-5.490399-22.603811-23.363073-46.740005-41.288109-40.701336-30.224145-79.662378-45.549521-115.800446-45.549521-35.79155 0-74.458435 15.038919-114.927219 44.694774C400.22004 922.554885 382.666163 940.255068 377.486862 945.656142zM731.271848 511.646647c0-105.803762-86.081448-191.88059-191.888289-191.88059-105.803762 0-191.88059 86.076827-191.88059 191.88059 0 105.803762 86.076827 191.882129 191.88059 191.882129C645.19194 703.528777 731.271848 617.450409 731.271848 511.646647zM539.383558 395.903184c63.825696 0 115.751164 51.922387 115.751164 115.743463 0 63.825696-51.925468 115.751164-115.751164 115.751164-63.821076 0-115.743463-51.925468-115.743463-115.751164C423.640095 447.824031 475.562482 395.903184 539.383558 395.903184z"
              p-id="5351"></path>
          </svg>
        </div>
        <a class="tooltip" href="mailto:shi2760992374@outlook.com?subject=BUG反馈&body=请发送反馈内容" data-tooltip="反馈邮件">
          <div class="mid-version">
            <h3>Bug反馈</h3>
          </div>
        </a>

        <div class="titlebar-button tooltip" data-tooltip="跳转仓库"
          @click="() => { opeanLinkling('https://github.com/cha-hai-ji-lan/Click') }">
          <svg t="1765561345418" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"
            p-id="4805" width="200" height="200">
            <path
              d="M0 524.992q0 166.016 95.488 298.496t247.488 185.504q6.016 0.992 10.016 0.992t6.496-1.504 4-3.008 2.016-4.992 0.512-4.992v-100.512q-36.992 4-66.016-0.512t-45.504-14.016-28.992-23.488-16.992-25.504-8.992-24-5.504-14.496q-8.992-15.008-27.008-27.488t-27.008-20-2.016-14.496q50.016-26.016 112.992 66.016 34.016 51.008 119.008 30.016 10.016-40.992 40-70.016Q293.984 736 237.984 670.976t-56-158.016q0-87.008 55.008-151.008-22.016-64.992 6.016-136.992 28.992-2.016 64.992 11.488t50.496 23.008 25.504 17.504q56.992-16 128.512-16t129.504 16q12.992-8.992 28.992-19.008t48.992-21.504 60.992-9.504q27.008 71.008 7.008 135.008 56 64 56 151.008 0 92.992-56.992 158.496t-172 85.504q43.008 43.008 43.008 104v128.992q0 0.992 0.992 3.008 0 6.016 0.512 8.992t4.512 6.016 12 3.008q152.992-52 250.496-185.504t97.504-300.512q0-104-40.512-199.008t-108.992-163.488-163.488-108.992T512.032 12.96 313.024 53.472 149.536 162.464t-108.992 163.488-40.512 199.008z"
              p-id="4806"></path>
          </svg>
        </div>
        <div class="titlebar-button split-icon tooltip" data-tooltip="置顶" @click="() => { pin_screen() }">
          <svg t="1765954787870" class="icon" :class="{ 'active-pin': isPin }" viewBox="0 0 1024 1024" version="1.1"
            xmlns="http://www.w3.org/2000/svg" p-id="1814" width="200" height="200">
            <path
              d="M381.298 418.828h-157.703l-37.575 38.272 155.61 158.377-278.212 345.128 356.040-265.838 154.71 157.41 38.813-39.51 2.407-157.972 238.838-313.29 71.685 73.013 34.695-35.28-310.185-315.743-34.672 35.257 77.287 79.402-311.737 240.773z"
              p-id="1815"></path>
          </svg>
        </div>

      </div>
      <div data-tauri-drag-region class="right-button">
        <div class="titlebar-button" @click="() => { title_bar_click('min-size') }">
          <svg t="1765104088218" class="icon min-size-icon" viewBox="0 0 1024 1024" version="1.1"
            xmlns="http://www.w3.org/2000/svg" p-id="6114" width="200" height="200">
            <path
              d="M85.333333 384c25.6 0 42.666667-17.066667 42.666667-42.666667V128h213.333333c25.6 0 42.666667-17.066667 42.666667-42.666667s-17.066667-42.666667-42.666667-42.666666H85.333333c-25.6 0-42.666667 17.066667-42.666666 42.666666v256c0 25.6 17.066667 42.666667 42.666666 42.666667zM938.666667 640c-25.6 0-42.666667 17.066667-42.666667 42.666667v213.333333h-213.333333c-25.6 0-42.666667 17.066667-42.666667 42.666667s17.066667 42.666667 42.666667 42.666666h256c25.6 0 42.666667-17.066667 42.666666-42.666666v-256c0-25.6-17.066667-42.666667-42.666666-42.666667zM601.6 401.066667c4.266667 8.533333 12.8 17.066667 21.333333 21.333333 4.266667 4.266667 12.8 4.266667 17.066667 4.266667h256c25.6 0 42.666667-17.066667 42.666667-42.666667s-17.066667-42.666667-42.666667-42.666667h-153.6l226.133333-226.133333c17.066667-17.066667 17.066667-42.666667 0-59.733333-8.533333-8.533333-17.066667-12.8-29.866666-12.8s-21.333333 4.266667-29.866667 12.8L682.666667 281.6V128c0-25.6-17.066667-42.666667-42.666667-42.666667s-42.666667 17.066667-42.666667 42.666667v256c0 4.266667 0 12.8 4.266667 17.066667zM115.2 968.533333L341.333333 742.4V896c0 25.6 17.066667 42.666667 42.666667 42.666667s42.666667-17.066667 42.666667-42.666667v-256c0-4.266667 0-12.8-4.266667-17.066667-4.266667-8.533333-12.8-17.066667-21.333333-21.333333-4.266667-4.266667-12.8-4.266667-17.066667-4.266667H128c-25.6 0-42.666667 17.066667-42.666667 42.666667s17.066667 42.666667 42.666667 42.666667h153.6l-226.133333 226.133333c-17.066667 17.066667-17.066667 42.666667 0 59.733333s42.666667 17.066667 59.733333 0z"
              p-id="6115"></path>
          </svg>
        </div>
        <div class="titlebar-button split-icon" @click="() => { title_bar_click('switch-size') }">
          <svg v-show="switchScreenSize === true" t="1765104152369" class="icon max-size-icon " viewBox="0 0 1024 1024"
            version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="8951" width="200" height="200">
            <path
              d="M384 128c23.564433 0 42.666667 19.102234 42.666667 42.666667s-19.102234 42.666667-42.666667 42.666666l-110.327523-0.00113 161.830428 161.831559c16.66317 16.66204 16.66317 43.677104 0 60.339143-16.66204 16.66317-43.677104 16.66317-60.339143 0L213.333333 273.673607v110.326393c0 23.564433-19.102234 42.666667-42.666666 42.666667s-42.666667-19.102234-42.666667-42.666667v-170.666667c0-47.127735 38.205598-85.333333 85.333333-85.333333h170.666667z m-8.836238 460.497095c16.66204-16.66317 43.677104-16.66317 60.339143 0 16.66317 16.66204 16.66317 43.677104 0 60.339143L273.672477 810.666667h110.327523c23.328212 0 42.284645 18.722472 42.661015 41.961395l0.005652 0.705271c0 23.564433-19.102234 42.666667-42.666667 42.666667h-170.666667c-47.127735 0-85.333333-38.205598-85.333333-85.333333v-170.666667c0-23.564433 19.102234-42.666667 42.666667-42.666667s42.666667 19.102234 42.666666 42.666667v110.325263zM810.666667 128c47.127735 0 85.333333 38.205598 85.333333 85.333333v170.666667c0 23.564433-19.102234 42.666667-42.666667 42.666667s-42.666667-19.102234-42.666666-42.666667v-110.327523L648.836238 435.502905c-16.66204 16.66317-43.677104 16.66317-60.339143 0-16.66317-16.66204-16.66317-43.677104 0-60.339143l161.828168-161.830429H640c-23.564433 0-42.666667-19.102234-42.666667-42.666666s19.102234-42.666667 42.666667-42.666667h170.666667zM588.497095 588.497095c16.66204-16.66317 43.677104-16.66317 60.339143 0L810.666667 750.326393V640c0-23.328212 18.722472-42.284645 41.961395-42.661015l0.705271-0.005652c23.564433 0 42.666667 19.102234 42.666667 42.666667v170.666667c0 47.127735-38.205598 85.333333-85.333333 85.333333h-170.666667c-23.564433 0-42.666667-19.102234-42.666667-42.666667s19.102234-42.666667 42.666667-42.666666h110.326393L588.497095 648.836238c-16.495894-16.495894-16.660909-43.137978-0.495046-59.836185z"
              p-id="8952"></path>
          </svg>
          <svg v-show="switchScreenSize === false" t="1765104453226" class="icon max-size-icon" viewBox="0 0 1024 1024"
            version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="11909" width="200" height="200">
            <path
              d="M631.06 687.1v182.01c0 21.91-17.76 39.67-39.67 39.67s-39.67-17.76-39.67-39.67v-277.7c0-21.91 17.76-39.67 39.67-39.67h277.7c21.91 0 39.67 17.76 39.67 39.67s-17.76 39.67-39.67 39.67H687.08l261.31 261.24c15.49 15.48 15.48 40.6-0.02 56.07-15.48 15.46-40.56 15.45-56.03-0.02L631.06 687.1zM432.71 472.32h-277.7c-21.91 0-39.67-17.76-39.67-39.67s17.76-39.67 39.67-39.67h181.93L75.61 131.72c-15.48-15.48-15.48-40.57 0-56.05l0.04-0.04c15.48-15.48 40.58-15.48 56.05 0l261.26 261.33V154.95c0-21.91 17.76-39.67 39.67-39.67s39.67 17.76 39.67 39.67v277.7c0 21.87-17.71 39.62-39.59 39.67z"
              p-id="11910"></path>
          </svg>
        </div>
        <div class="titlebar-button split-icon" @click="() => { title_bar_click('close-screen') }">
          <svg t="1765102437899" class="icon close-icon" viewBox="0 0 1024 1024" version="1.1"
            xmlns="http://www.w3.org/2000/svg" p-id="1674" width="200" height="200">
            <path
              d="M704 108.8c6.4 0 12.8 6.4 19.2 6.4 19.2 12.8 44.8 6.4 57.6-12.8s6.4-44.8-12.8-57.6c-6.4-6.4-12.8-6.4-19.2-12.8-19.2-12.8-44.8 0-51.2 19.2-19.2 19.2-12.8 44.8 6.4 57.6z m230.4 96c-12.8-19.2-38.4-25.6-57.6-12.8-6.4 6.4-12.8 12.8-19.2 25.6 0 12.8 0 19.2 6.4 32 44.8 70.4 70.4 153.6 70.4 236.8a428.8 428.8 0 0 1-857.6 0c0-160 89.6-307.2 230.4-384 19.2-12.8 25.6-32 19.2-51.2-12.8-19.2-32-25.6-51.2-19.2C102.4 121.6 0 294.4 0 486.4c0 281.6 230.4 512 512 512s512-230.4 512-512c-6.4-96-32-198.4-89.6-281.6z"
              p-id="1675"></path>
            <path
              d="M307.2 332.8L454.4 480c6.4 6.4 6.4 6.4 0 12.8L307.2 640c-12.8 12.8-12.8 32-6.4 44.8 12.8 25.6 38.4 25.6 57.6 6.4 51.2-51.2 102.4-96 147.2-147.2 6.4-6.4 6.4-6.4 12.8 0 51.2 51.2 102.4 102.4 153.6 147.2 19.2 19.2 38.4 19.2 57.6 0 12.8-12.8 12.8-38.4 0-51.2l-70.4-70.4-76.8-76.8c-6.4-6.4-6.4-6.4 0-12.8 0 0 0-6.4 6.4-6.4l134.4-134.4c19.2-19.2 19.2-38.4 0-51.2-12.8-12.8-38.4-12.8-51.2 0L524.8 435.2c-6.4 6.4-6.4 6.4-12.8 0l-57.6-57.6-96-96c-12.8-12.8-32-12.8-44.8-6.4-19.2 12.8-25.6 38.4-6.4 57.6z"
              p-id="1676"></path>
          </svg>
        </div>
      </div>
    </div>

    <main class="container">
      <div v-show="comVisibility.LeftContain['LeftContain-open']" class="left-contain" ref="leftContainer"
        :class="{ 'close-left-contain': comVisibility.LeftContain['LeftContain-close'] }" @mousedown="startResize">

        <LeftContain v-model:comVisibility="comVisibility" :leftContainer="leftContainer" :mainColor="mainColor"
          :open_sidebar_left="() => { open_sidebar('left-contain') }"></LeftContain>

      </div>
      <div class="mid-contain">
        <div class="left-call-icon" @click="() => { open_sidebar('left-contain') }">
          <svg t="1765631009180" class="icon "
            :class="{ 'left-called-icon1': !comVisibility.LeftContain['LeftContain-close'] }" viewBox="0 0 1024 1024"
            version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2824" width="200" height="200">
            <path
              d="M368.576 116.672a16 16 0 0 0-22.656 0l-45.248 45.248a16 16 0 0 0 0 22.656l327.936 327.936-327.936 327.936a16 16 0 0 0 0 22.592l45.248 45.248a16 16 0 0 0 22.656 0l384.512-384.448a16 16 0 0 0 0-22.656L368.576 116.672z"
              p-id="2825"></path>
          </svg>
        </div>
        <RouterView></RouterView>
      </div>
      <div class="right-contain">

      </div>
    </main>
    <Setting :comVisibility="comVisibility" :openWindow="openWindow"></Setting>
  </div>

</template>
<script setup lang="ts">

// TODO: JPG 动图显示
import { onMounted, ref } from "vue";
import { Window } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";  // invoke：钩子方法 用于调用后端rust的函数
import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { RouterView } from "vue-router"
import { ColorCtr } from './util/Colors';
import LeftContain from "./components/LeftContain.vue";
import Setting from "./components/Setting.vue";
import { comVisibility, set_focus_color_palette, set_special_style } from "./util/PluginObjects"
const mainColor = ref(ColorCtr());
const appWindow = Window.getCurrent()
const switchScreenSize = ref(true)
const isPin = ref(false)
const leftContainer = ref<HTMLElement | null>(null)  // 左侧容器DOM元素
let appIcon = null  //  在挂载组件时获取软件的图标
let isResizing = false
let leftContainWidth = 0  // 左侧容器宽度 方便改写和关闭按钮调用





onMounted(async () => {
  mainColor.value.set_color_palette()
  set_focus_color_palette(mainColor.value)  // 设置调色板
  set_special_style()  // 设置特殊样式

  try {
    appIcon = await defaultWindowIcon();
    const options = appIcon ? { icon: appIcon } : {};
    await TrayIcon.new(options);  // 加载小托盘图标 为默认图标
  } catch (error) {
    console.error("Failed to create tray icon:", error);
  }
})


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
      if (comVisibility.LeftContain["LeftContain-open"]) {
        if (!leftContainer.value) return
        leftContainer.value.style.width = ''  // 移除width 方便动画播放
        setTimeout(() => {
          comVisibility.LeftContain["LeftContain-close"] = true
        }, 50)
        setTimeout(() => {
          comVisibility.LeftContain["LeftContain-open"] = !comVisibility.LeftContain["LeftContain-open"];
        }, 800)

      }
      else {
        comVisibility.LeftContain["LeftContain-close"] = false
        comVisibility.LeftContain["LeftContain-open"] = !comVisibility.LeftContain["LeftContain-open"];

      }
      break;

    default:
      break;
  }

}

const openWindow = (windowName: string) => {
  switch (windowName) {
    case 'setting':
      if (comVisibility.setting["setting-open"] === false) {
        comVisibility.setting["setting-open"] = true;

      } else {
        comVisibility.setting["setting-close"] = true
        setTimeout(() => {
          comVisibility.setting["setting-open"] = false;
          comVisibility.setting["setting-close"] = false

        }, 500)
      }
  }


}

const opeanLinkling = async (url: String) => {
  try {
    await invoke('open_url', { url });
  } catch (err) {
    console.error('无法打开链接:', err);
  }
}

const pin_screen = async () => {
  isPin.value = !isPin.value
  console.log("屏幕钉住了", isPin.value)
  await appWindow.setAlwaysOnTop(isPin.value);
}

const startResize = (_: MouseEvent) => {
  isResizing = true
  document.addEventListener('mousemove', resize)
  document.addEventListener('mouseup', stopResize)
}

const resize = (e: MouseEvent) => {
  if (!isResizing || !leftContainer.value) return

  const containerRect = leftContainer.value?.getBoundingClientRect()

  if (!containerRect) return
  leftContainWidth = e.clientX * 1.2371134 // 1.2371134是修正系数
  leftContainer.value.style.cssText += `width: ${leftContainWidth}px !important;`
  document.documentElement.style.setProperty("--left-contain-width", `${leftContainWidth}px`)  // 侧边栏宽度  用于动画属性 因为动画属性优先级过高
}

const stopResize = () => {
  isResizing = false
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
  if (leftContainer.value) {
    if (leftContainer.value.getBoundingClientRect().width <= 50) {
      open_sidebar("left-contain")
      setTimeout(() => {
        document.documentElement.style.setProperty("--left-contain-width", `200px`)  // 侧边栏宽度  用于动画属性 因为动画属性优先级过高
        document.documentElement.style.setProperty("--letter-spacing", `0px`)  // 动态栏的字字间距
        document.documentElement.style.setProperty("--font-blur", `0px`)

      }, 800)
    }
  }

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

/* ------------------------------------------------------标题栏样式区--------------------------------------------------------- */
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

.active-pin {
  animation: active-pin 0.25s forwards;
  animation-timing-function: linear;
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

.left-button {
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

.split-icon:before {
  content: "";
  position: absolute;
  /* top: 50%; */
  left: -0.5vw;
  width: 0.2vw;
  max-width: 2px;
  height: 3vh;
  max-height: 20px;
  background-color: var(--main-border);
}

.main-icon-box {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 4vh;
  width: 4vh;
  min-height: 15px;
  max-height: 30px;
  min-width: 15px;
  max-width: 30px;
  margin-left: 0.5vw;

  user-select: none;
  /* 用户无法选择 */
  -webkit-user-select: none;
  /* Safari兼容性 */
  -moz-user-select: none;
  /* Firefox兼容性 */
  -ms-user-select: none;
  /* IE兼容性 */
}

.main-icon {
  height: 4vmin;
  width: 4vmin;
  min-height: 15px;
  max-height: 30px;
  min-width: 15px;
  max-width: 30px;
}

.main-icon:active {
  animation: active-icon 0.25s forwards;
  animation-timing-function: linear;
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
  transition: 0.75s;
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
  height: 3vmin;
  border-radius: 0 5vh 5vh 0;
  background-color: var(--main-border);
  /* transform: translateY(50%); */


}

.left-call-icon:hover:before {
  animation: expand 0.25s both;
  animation-timing-function: linear;
}

.left-call-icon:active:before {
  background: radial-gradient(circle at center, var(--title-min-icon-active-shadow), var(--title-min-icon-hover-shadow), var(--title-min-icon-shadow))
}

.left-called-icon1 {
  animation-delay: 50ms;
  animation: roll-0-180 0.75s both;
  animation-timing-function: linear;
}

.right-contain {
  top: 0;
  left: 100%;
}


/* ==============================================动画实现============================================== */
@keyframes active-pin {
  0% {
    transform: rotate(0);
  }

  100% {
    transform: rotate(-45deg);
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

@keyframes expand {
  0% {
    height: 3vmin;
  }

  100% {
    height: 90vmin;
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

</style>
<style>
html {
  font-family: "楷体", 'Courier New', Courier, monospace;
  color: var(--font-color);
  background: transparent;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}

svg {
  fill: var(--icon-color)
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

/* --------------------------------------------------------一般图标----------------------------------------------------- */

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
/* --------------------------------------------------------小图标----------------------------------------------------- */


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

/* --------------------------------------------------------错误提示----------------------------------------------------- */
.show-err-msg{
    animation: show-err 0.25s forwards;

}

.close-err-msg {
    animation: hide-err 0.25s forwards;

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

/* --------------------------------------------------------可拖缩放动框----------------------------------------------------- */


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

/* ==============================================动画实现============================================== */

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
@keyframes show-bubbles {
  0% {
    opacity: 0.25;
  }

  100% {
    opacity: 1;
  }

}
@keyframes show-err {
  0% {
    opacity: 0;
  }

  100% {
    opacity: 1;
  }

}
@keyframes hide-err {
  0% {
    opacity: 1;
  }

  100% {
    opacity: 0;
  }

}
</style>