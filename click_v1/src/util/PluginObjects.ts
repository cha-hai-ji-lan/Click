import {reactive, shallowRef, markRaw } from "vue";
import IconFile from '../icon/IconFile.vue'
import IconShutdown from '../icon/IconShutDown.vue'
import LivePicture from '../icon/LivePicture.vue'
import RecordScreen from '../icon/RecordScreen.vue'
export const comVisibility = reactive({
  "LeftContain": {
    "LeftContain-data": {
      "item": [
        {
          "name": "文件处理",
          "index": 0,
          "is-focus": false,
          "icon": "IconFile",
          "router": "/deal-file",
          "props": true,
        },
        {
          "name": "关机问询",
          "index": 0,
          "is-focus": false,
          "icon": "IconShutdown",
          "router": "/make-shutdown",
          "props": true,
        },
        {
          "name": "未分类功能",
          "index": 0,
          "is-focus": false,
          "icon": "LivePicture",
          "router": "/live-picture",
          "props": true,
        },
        {
          "name": "屏幕录制",
          "index": 0,
          "is-focus": false,
          "icon": "RecordScreen",
          "router": "/record-screen",
          "props": true,
        }
      ]
    },
    "LeftContain-open": false,
    "LeftContain-close": true, // 保障关闭侧边栏按钮可以正常运转
  },
  "setting":{
    "setting-open": false,
    "setting-close": false,
  }
})

export const iconComponents = shallowRef({
  'IconFile': markRaw(IconFile),
  'IconShutdown': markRaw(IconShutdown),
  'LivePicture': markRaw(LivePicture),
  'RecordScreen':markRaw(RecordScreen),
})

export const set_color_flag = (mainColor : any, flag: String) => {
  mainColor.set_theme_flag(flag)
  
}

export const set_focus_color_palette = (mainColor : any) => {
  // 特殊颜色
  document.documentElement.style.setProperty("--icon-color", `rgba(${mainColor.iconColorRGBA})`)
  document.documentElement.style.setProperty("--title-close-icon-shadow", `rgba(${mainColor.colseIconColorRGBA})`)
  document.documentElement.style.setProperty("--title-close-icon-hover-shadow", `rgba(${mainColor.colseIconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--title-close-icon-active-shadow", `rgba(${mainColor.colseIconActiveColorRGBA})`)
  document.documentElement.style.setProperty("--title-min-icon-shadow", `rgba(${mainColor.minSizeIconColorRGBA})`)
  document.documentElement.style.setProperty("--title-min-icon-hover-shadow", `rgba(${mainColor.minSizeIconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--title-min-icon-active-shadow", `rgba(${mainColor.minSizeIconActiveColorRGBA})`)
  document.documentElement.style.setProperty("--title-max-icon-shadow", `rgba(${mainColor.maxSizeIconColorRGBA})`)
  document.documentElement.style.setProperty("--title-max-icon-hover-shadow", `rgba(${mainColor.maxSizeIconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--title-max-icon-active-shadow", `rgba(${mainColor.maxSizeIconActiveColorRGBA})`)
  // 总背景颜色
  document.documentElement.style.setProperty("--back-ground", `rgba(${mainColor.backGroundColorRGBA})`)
  // 标题栏颜色
  document.documentElement.style.setProperty("--title-bar-lg-1", `rgba(${mainColor.titleBarColorRGBA})`)
  document.documentElement.style.setProperty("--title-bar-lg-2", `rgba(${mainColor.backGroundColorRGBA})`)
  // 主区域颜色
  document.documentElement.style.setProperty("--main-border", `rgba(${mainColor.borderColorRGBA})`)  // 边框色
  document.documentElement.style.setProperty("--main-back-ground", `rgba(${mainColor.midGroundColorRGBA})`)  // 背景色
  document.documentElement.style.setProperty("--tool-bar-color", `rgba(${mainColor.toolBarColorRGBA})`)  // 背景色
  document.documentElement.style.setProperty("--icon-hover", `rgba(${mainColor.foreGroundColorRGBA})`)
  document.documentElement.style.setProperty("--icon-hover-shadow", `rgba(${mainColor.iconHoverColorRGBA})`)
  document.documentElement.style.setProperty("--icon-active-shadow", `rgba(${mainColor.iconActiveColorRGBA})`)
  document.documentElement.style.setProperty("--font-color", `rgba(${mainColor.fontColorRGBA})`)
  document.documentElement.style.setProperty("--button-color", `rgba(${mainColor.buttonColorRGBA})`)  // 按钮颜色
  document.documentElement.style.setProperty("--unite-but-color", `rgba(${mainColor.iconColorRGBA})`)

  // 注视颜色
  document.documentElement.style.setProperty("--normal-attention-color", `rgba(${mainColor.normalAttentionRGBA})`)
  document.documentElement.style.setProperty("--active-attention-color", `rgba(${mainColor.activeAttentionRGBA})`)

  document.documentElement.style.setProperty("--positive-show-color", `rgba(${mainColor.positiveShowRGBA})`)
  document.documentElement.style.setProperty("--normal-show-color", `rgba(${mainColor.normalShowRGBA})`)
  document.documentElement.style.setProperty("--negative-show-color", `rgba(${mainColor.negativeShowRGBA})`)



  // 左侧边框颜色


}

export const set_special_style = () => {
  document.documentElement.style.setProperty("--left-contain-width", `30vw`)  // 侧边栏宽度
  document.documentElement.style.setProperty("--font-blur", `5`)  // 动态栏的字模糊滤镜
  document.documentElement.style.setProperty("--letter-spacing", `normal`)  // 动态栏的字字间距
  document.documentElement.style.setProperty("--grid-size", `4vmin`)  // 模拟纸面网格大小
  document.documentElement.style.setProperty("--timing-schedule", `0%`)  // 模拟纸面网格大小
  document.documentElement.style.setProperty("--float-window2-height", `90vh`)  // 浮动窗口2起始高度

}