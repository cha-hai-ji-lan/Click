import { reactive, shallowRef, markRaw} from "vue";
import IconFile from '../icon/IconFile.vue'
import IconShutdown from '../icon/IconShutDown.vue'
export const comVisibility = reactive({
      "LeftContain": {
        "LeftContain-data": {
          "item": [
            {
              "name": "文件处理",
              "index": 0,
              "is-focus": false,
              "icon": "IconFile",
              "router":"/deal-file"
            },
            {
              "name": "可控关机",
              "index": 0,
              "is-focus": false,
              "icon":"IconShutdown",
              "router":"/make-shutdown"
            }
          ]
        },
        "LeftContain-open": false,
        "LeftContain-close": true, // 保障关闭侧边栏按钮可以正常运转
      }
})

export const iconComponents = shallowRef({
  'IconFile': markRaw(IconFile),
  'IconShutdown': markRaw(IconShutdown),
})