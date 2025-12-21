<template>
     <div class="main-contain">
          <div class="title">
               <div class="tag-icon" @click="() => { openFileDialog() }">
                    <svg t="1766153567513" class="icon" viewBox="0 0 1024 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="7556" width="200" height="200">
                         <path d="M910.096853 360.579647v-61.707521c0-52.999376-42.030493-112.28336-98.281728-112.28336H528.430288l-11.681508-22.048347c-11.685603-22.352441-34.588894-36.229158-59.807189-36.229158H154.570848c-50.436591 0-91.447294 42.168717-91.447294 94.010079v575.540275c0 51.814741 41.010703 93.969123 91.447294 93.969124h711.539892c50.436591 0 91.447294-42.154383 91.447294-93.969124V444.614011c-0.001024-30.633625-20.252452-63.054954-47.461181-84.034364z m-98.281728-117.546131c19.540851 0 41.838002 29.75206 41.838002 55.83861v39.040748c-0.956309-0.038908-1.913642-0.097269-2.865855-0.097269H608.55341l-50.21748-94.782089h253.479195z m89.297134 554.828099c0 20.684531-15.709473 37.524373-35.002543 37.524374h-711.538868c-19.292047 0-35.002544-16.839842-35.002544-37.524374V222.32134c0-20.712176 15.709473-37.565329 35.002544-37.565329h302.370743c4.134449 0 7.882893 2.274049 9.867182 6.07676l99.825748 188.420497a28.252068 28.252068 0 0 0 24.94287 15.007087h259.209881c20.229926 0 50.326011 30.12373 50.326011 50.353656v353.247604z"
                              p-id="7557">
                         </path>
                    </svg>
               </div>
               <div class="tag-icon" @click="() => { click('choose-path-pool') }">
                    <svg t="1766335459475" class="icon" viewBox="0 0 1121 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="7571" width="200" height="200">
                         <path d="M1039.255327 23.915108a14.494005 14.494005 0 0 0-10.870503-23.915108H13.079777A13.044604 13.044604 0 0 0 3.658674 22.103358L362.385296 384.091131V797.170272l108.705038 108.705037a36.235012 36.235012 0 0 0 22.828057 10.508154 53.990168 53.990168 0 0 0 57.61367-52.540768v-4.348202a36.235012 36.235012 0 0 0-10.870504-26.813909l-78.629977-77.180576-3.623501-406.919189L200.052441 101.458035a9.058753 9.058753 0 0 1 6.159952-15.943406l644.62087-4.710551a9.421103 9.421103 0 0 1 6.884652 15.581055l-250.383935 253.645086-2.536451 625.053964a48.917267 48.917267 0 1 0 97.834534 0v-594.254203z"
                              p-id="7572"></path>
                         <path d="M743.577627 471.055161a44.569065 44.569065 0 0 0 44.569065 44.569065h289.880099a44.569065 44.569065 0 0 0 0-88.77578h-289.880099A44.569065 44.569065 0 0 0 743.577627 471.055161zM1077.664441 603.675306h-289.880099a44.569065 44.569065 0 1 0 0 88.77578h289.880099a44.569065 44.569065 0 0 0 0-88.77578zM1077.664441 793.184421h-289.880099a44.569065 44.569065 0 1 0 0 88.77578h289.880099a44.569065 44.569065 0 0 0 0-88.77578z"
                              p-id="7573"></path>
                    </svg>
               </div>


          </div>
          <div class="explorer-act-path">
               <transition-group name="path-item" tag="div">

                    <div class="index-path coming-animation" v-for="item in active_path" :key="item[1]">
                         <div class="choose">
                              <input type="checkbox" :value="item" :checked="isSelected(item)"
                                   @change="handleCheckboxChange(item)" />

                         </div>
                         <div class="name">{{ item[1] }}</div>
                         <div class="path">{{ item[0] }}</div>
                    </div>
               </transition-group>
          </div>
          <div class="oper">
               <div class="h3-head">
                    <h3>操作</h3>
               </div>
               <div class="oper-1">
                    <div class="item-title">批处理操作</div>
               </div>
               <div class="oper-1">
                    <transition name="replace-name-transition" mode="out-in">
                         <div v-if="active_path && active_path.length > 0" class="replace-name"
                              :class="{ 'hide-submit-replace-name': active_path && active_path.length <= 0 }">
                              <div class="item-title">修改名字</div>
                              <input class="input-box" type="text" v-model="inputRefReplaceOldName" placeholder="旧字段">
                              <input class="input-box" type="text" v-model="inputRefReplaceNewName" placeholder="新字段">
                              <div class="submit-replace-name" @click="() => { SubmitRepluceName('replace-name') }">修改
                              </div>
                         </div>
                         <div v-else class="place-holder">
                              <span>🔎 当前无可观察路径...</span>
                         </div>
                    </transition>
               </div>
               <div class="oper-1">
               </div>

          </div>
     </div>

     <div v-show="FloatingWindow['choose-path-pool']" class="choose-path-pool"
          :class="{ 'choose-path-pool-close': FloatingWindow['choose-path-pool-close'] }" ref="floatingWindowElement">
          <div class="drag-head-contain" @mousedown="startDrag">
               <svg t="1766338639348" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="9788" width="200" height="200">
                    <path d="M476.5 924V100c0-19.8 16.2-36 36-36s36 16.2 36 36v824c0 19.8-16.2 36-36 36s-36-16.2-36-36z"
                         p-id="9789"></path>
                    <path d="M100.5 476h824c19.8 0 36 16.2 36 36s-16.2 36-36 36h-824c-19.8 0-36-16.2-36-36s16.2-36 36-36zM690.1 797.6L538.4 949.3c-14.3 14.3-37.4 14.3-51.7 0L334.2 796.8c-14.3-14.3-14.6-37.9 0-52 14.1-13.5 36.5-13.4 50.4 0.5l117.6 117.6c5.8 5.8 15.1 5.8 20.8 0l115.6-115.6c14.3-14.3 37.9-14.6 52 0 13.5 14.1 13.4 36.5-0.5 50.3zM333.7 226.4L485.4 74.7c14.3-14.3 37.4-14.3 51.7 0l152.5 152.5c14.3 14.3 14.6 37.9 0 52-14.1 13.5-36.5 13.4-50.4-0.5L521.6 161.1c-5.8-5.8-15.1-5.8-20.8 0L385.1 276.7c-14.3 14.3-37.9 14.6-52 0-13.4-14.1-13.3-36.5 0.6-50.3z"
                         p-id="9790"></path>
                    <path d="M226.7 690.1L75.1 538.4c-14.3-14.3-14.3-37.4 0-51.7l152.5-152.5c14.3-14.3 37.9-14.6 52 0 13.5 14.1 13.4 36.5-0.5 50.4L161.5 502.2c-5.8 5.8-5.8 15.1 0 20.8l115.6 115.6c14.3 14.3 14.6 37.9 0 52-14.1 13.5-36.5 13.4-50.4-0.5zM798.1 333.7l151.7 151.7c14.3 14.3 14.3 37.4 0 51.7L797.3 689.6c-14.3 14.3-37.9 14.6-52 0-13.5-14.1-13.4-36.5 0.5-50.4l117.6-117.6c5.8-5.8 5.8-15.1 0-20.8L747.8 385.1c-14.3-14.3-14.6-37.9 0-52 14.1-13.4 36.5-13.3 50.3 0.6z"
                         p-id="9791"></path>
               </svg>

               <svg t="1766338714320" class="icon float-close-icon" @click="() => { click('choose-path-pool') }"
                    viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="10844" width="200"
                    height="200">
                    <path d="M199.36 572.768a31.904 31.904 0 0 0 22.624-9.376l294.144-294.144 285.728 285.728a31.968 31.968 0 1 0 45.248-45.248l-308.352-308.352a32 32 0 0 0-45.28 0l-316.768 316.768a31.968 31.968 0 0 0 22.656 54.624z"
                         p-id="10845"></path>
                    <path d="M538.784 457.376a32 32 0 0 0-45.28 0l-316.768 316.768a31.968 31.968 0 1 0 45.248 45.248l294.144-294.144 285.728 285.728a31.968 31.968 0 1 0 45.248-45.248l-308.32-308.352z"
                         p-id="10846"></path>
               </svg>
          </div>
          <div class="drag-contain">
               <div>当前选中的路径</div>
          </div>
     </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";  // invoke：钩子方法 用于调用后端rust的函数
import { open } from '@tauri-apps/plugin-dialog';
import { type PathItem } from "../../class/PathIndex"
import { ref, reactive, onMounted, onUnmounted } from "vue";


const active_path = ref<PathItem[] | null>(null)
const selectedPaths = ref<PathItem[]>([]) // 选中的路径

let pollingInterval = ref<number | null>(null);
// 保存上一次获取的数据用于比较
const lastActivePathData = ref<string | null>(null);

const inputRefReplaceOldName = ref("")
const inputRefReplaceNewName = ref("")

const FloatingWindow = reactive({
     "choose-path-pool": false,
     "choose-path-pool-close": false
})

const isDragging = ref(false);
const dragOffset = ref({ x: 0, y: 0 });
const floatingWindowElement = ref<HTMLElement | null>(null);



const props = defineProps({
     mainColor: {
          type: Object,
          default: () => ({})
     }
});

const get_explorer_active_path = async () => {
     try {
          const data = await invoke<PathItem[] | null>("active_explorer_path");

          // 将新数据转换为 JSON 字符串以便比较
          const newDataString = JSON.stringify(data);

          // 只有当数据真正改变时才更新 active_path
          if (lastActivePathData.value !== newDataString) {
               lastActivePathData.value = newDataString;
               active_path.value = data;
          }
     } catch (err) {
          console.error("无法处理文件:", err);
     }
};

// 检查项目是否被选中
const isSelected = (item: PathItem) => {
     return selectedPaths.value.some(selectedItem =>
          selectedItem[0] === item[0] && selectedItem[1] === item[1]
     );
};

// 处理复选框变化
const handleCheckboxChange = (item: PathItem) => {
     const index = selectedPaths.value.findIndex(selectedItem =>
          selectedItem[0] === item[0] && selectedItem[1] === item[1]
     );

     if (index === -1) {
          // 如果未选中，则添加到选中列表
          selectedPaths.value.push(item);
     } else {
          // 如果已选中，则从选中列表移除
          selectedPaths.value.splice(index, 1);
     }
};
const startPolling = () => {
     // 每隔1秒调用一次
     pollingInterval.value = setInterval(get_explorer_active_path, 1000);
};

const stopPolling = () => {
     if (pollingInterval.value) {
          clearInterval(pollingInterval.value);
          pollingInterval.value = null;
     }
};

onMounted(() => {
     get_explorer_active_path();
     startPolling();
});

onUnmounted(() => {
     stopPolling();
});


const openFileDialog = async () => {
     let default_path = 'C:\\'  // 设置默认打开路径，可以根据需要修改
     if (selectedPaths.value && selectedPaths.value.length > 0) {
          default_path = selectedPaths.value[0][0]
     }
     try {
          const selected = await open({
               multiple: true,
               filters: [{
                    name: 'All Files',
                    extensions: ['*']
               }],
               defaultPath: default_path
          });

          if (selected === null) {
               console.log('用户取消了选择');
          } else {
               console.log('选中的文件:', selected);
               // 在这里可以处理选中的文件，比如将其添加到 active_path 中
          }
     } catch (error) {
          console.error('打开文件对话框时出错:', error);
     }
};


const click = (whichOne: string) => {
     switch (whichOne) {
          case 'choose-path-pool':
               if (FloatingWindow["choose-path-pool"] === false) {
                    FloatingWindow["choose-path-pool"] = true
                    if (floatingWindowElement.value && parseFloat(floatingWindowElement.value.style.left) < 0) {
                         floatingWindowElement.value.style.left = `0px`;
                    }
               } else {
                    FloatingWindow["choose-path-pool-close"] = true;
               setTimeout(() => {
                    FloatingWindow["choose-path-pool"] = false;
                    FloatingWindow["choose-path-pool-close"] = false;
               }, 500)
               }


               break;
          default:
               break;
     }
}

const SubmitRepluceName = (tag: String) => {
     switch (tag) {
          case 'replace-name':
               if (inputRefReplaceOldName.value !== "") {
                    selectedPaths.value.forEach((item, _) => {
                         if (item[0] !== "") {
                              console.log(item[0])
                              console.log(inputRefReplaceOldName.value)
                              console.log(inputRefReplaceNewName.value)
                              invoke("replace_all_name", { dirPath: item[0], oldNameSign: inputRefReplaceOldName.value, newNameSign: inputRefReplaceNewName.value })
                                   .then(() => {
                                        console.log("成功替换文件名");
                                   })
                                   .catch((err) => {
                                        console.error("无法处理文件:", err);
                                   });
                         }
                    });

               } else {
                    alert("请输入要替换的字段")
               }

               break;

          default:
               break;
     }

}


//  ------------------------------------悬浮窗拖拽-----------------------------------
// 开始拖拽
const startDrag = (event: MouseEvent) => {
     if (!floatingWindowElement.value) return;

     isDragging.value = true;

     // 计算鼠标相对于悬浮窗左上角的偏移量
     const rect = floatingWindowElement.value.getBoundingClientRect();
     dragOffset.value = {
          x: event.clientX - rect.left,
          y: event.clientY - rect.top
     };

     // 添加全局事件监听器
     document.addEventListener('mousemove', drag);
     document.addEventListener('mouseup', stopDrag);
};

// 拖拽过程
const drag = (event: MouseEvent) => {
     if (!isDragging.value || !floatingWindowElement.value) return;

     // 计算新的位置
     const newX = event.clientX - dragOffset.value.x;
     const newY = event.clientY - dragOffset.value.y;

     // 应用新位置
     if (floatingWindowElement.value) {
          floatingWindowElement.value.style.left = `${newX}px`;
          floatingWindowElement.value.style.top = `${newY}px`;
     }
};

// 停止拖拽
const stopDrag = () => {
     isDragging.value = false;

     // 移除全局事件监听器
     document.removeEventListener('mousemove', drag);
     document.removeEventListener('mouseup', stopDrag);
};

</script>
<style scoped>
h3 {
     width: 100%;
     margin: 1vh 0;
     font-size: 3.5vmin;
     text-align: center;
     user-select: none;
     /* 用户无法选择 */
     -webkit-user-select: none;
     /* Safari兼容性 */
     -moz-user-select: none;
     /* Firefox兼容性 */
     -ms-user-select: none;
     /* IE兼容性 */
}

.main-contain {
     width: 100%;
     display: flex;
     justify-content: start;
     align-items: center;
     flex-direction: column;
     transition: all;
     margin-right: 1vw;
     margin-bottom: 1vw;

}

.title {
     display: flex;
     height: 4vh;
     margin-top: 2vh;
     width: 100%;
     justify-content: start;
     align-items: center;
     flex-direction: row;
     position: relative;
}

.title::after {
     content: '';
     position: absolute;
     bottom: 2px;
     left: 0;
     width: 100%;
     height: 2px;
     background: linear-gradient(to right, var(--main-border) 60%, var(--font-color) 62.5%, var(--button-color) 65%, var(--back-ground) 67.5%, var(--main-back-ground) 70%);
}


.tag-icon {
     width: 4vh;
     height: 4vh;
     display: flex;
     justify-content: start;
     align-items: center;
}

.icon {
     height: 3vh;
     width: 3vh;
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

.explorer-act-path {
     margin-top: 2vh;
     width: 100%;
     height: 22%;
     overflow-y: auto;


}

.index-path {
     display: grid;
     grid-template-columns: 5% 20% 75%;
     position: relative;
     font-size: 2vmin;
}

.index-path:nth-child(1)::before {
     content: '';
     position: absolute;
     top: 0;
     left: 0;
     width: 100%;
     height: 2px;
     height: 2px;
     background: radial-gradient(circle at center, var(--title-min-icon-active-shadow), var(--title-min-icon-hover-shadow), var(--title-min-icon-shadow));
}

.index-path::after {
     content: '';
     position: absolute;
     bottom: 0;
     left: 0;
     width: 100%;
     height: 2px;
     background: radial-gradient(circle at center, var(--title-min-icon-active-shadow), var(--title-min-icon-hover-shadow), var(--title-min-icon-shadow));
}

.choose {
     display: flex;
     justify-content: center;
     align-items: center;
}

/* 自定义复选框样式 */
.choose input[type="checkbox"] {
     appearance: none;
     width: 18px;
     height: 18px;
     border: 2px solid #ccc;
     border-radius: 3px;
     position: relative;
     cursor: pointer;
     display: flex;
     justify-content: center;
     align-items: center;
     transition: all 0.2s ease;
}

/* 添加悬停效果 */
.choose input[type="checkbox"]:hover {
     border-color: #1e90ff;
     box-shadow: 0 0 4px rgba(30, 144, 255, 0.3);
}

/* 添加焦点效果 */
.choose input[type="checkbox"]:focus {
     outline: none;
     box-shadow: 0 0 6px rgba(30, 144, 255, 0.5);
}

.choose input[type="checkbox"]:checked {
     background-color: #1e90ff;
     border-color: #1e90ff;
}

.choose input[type="checkbox"]:checked::after {
     content: '';
     position: absolute;
     width: 5px;
     height: 10px;
     border: solid white;
     border-width: 0 2px 2px 0;
     transform: rotate(45deg);
}

.name {
     height: 4vh;
     max-height: 30px;
     min-height: 10px;
     display: flex;
     justify-content: center;
     align-items: center;
     flex-direction: row;
     overflow: hidden;
     text-overflow: ellipsis;
     white-space: nowrap;
     border-right: 2px solid var(--title-min-icon-hover-shadow);
     border-left: 2px solid var(--title-min-icon-shadow);
     -webkit-box-orient: vertical;
}

.path {
     height: 4vh;
     max-height: 30px;
     min-height: 10px;
     display: flex;
     justify-content: center;
     align-items: center;
     flex-direction: row;
     overflow: hidden;
     text-overflow: ellipsis;
     white-space: nowrap;
     -webkit-box-orient: vertical;

}

.path::before {
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

.oper {
     display: flex;
     width: 100%;
     /* flex:1; */
     border-radius: 1vmin;
     justify-content: start;
     align-items: center;
     flex-direction: column;
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

.h3-head {
     width: 100%;
     position: relative;

}

.h3-head::after {
     content: '';
     position: absolute;
     bottom: 4px;
     left: 12.5%;
     width: 75%;
     height: 2px;
     background: radial-gradient(circle at center, var(--font-color), var(--button-color), var(--title-bar-lg-2));
}

.oper-1 {
     margin: 0.75vh 0;
     text-align: center;
     display: flex;
     height: 4vh;
     width: 100%;
     font-size: 2.5vmin;
     /* border-radius: 1vmin; */
     justify-content: center;
     align-items: center;
     flex-direction: column;
     border-top: 1px dashed var(--unite-but-color);
     border-bottom: 1px dashed var(--unite-but-color);



}

.replace-name {
     display: grid;
     width: 100%;
     height: 4vh;
     grid-template-columns: 15% 35% 35% 15%;
}

.place-holder {
     width: 100%;
     height: 4vh;
     font-size: medium;
     display: flex;
     justify-content: center;
     align-items: center;
     color: var(--font-color);

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
     text-align: center;
     border-radius: 1vmin;
     border-top: 1px dashed var(--unite-but-color);
     border-bottom: 2px solid var(--unite-but-color);
     border-left: 1px dashed var(--unite-but-color);
     border-right: 1px dashed var(--unite-but-color);




}

.submit-replace-name {
     border-radius: 1vmax;
     border: 1px solid var(--main-border);
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


.coming-animation {
     animation: show-submit-replace-name 0.5s ease-in-out forwards;
}


/* ----------------------------------------悬浮窗区----------------------------------------------- */
.choose-path-pool {
     position: fixed;
     display: flex;
     justify-self: center;
     align-items: center;
     flex-direction: column;
     top: 1vh;
     left: 1vw;
     height: 80vmin;
     width: 40vmin;
     transform: translateZ(0);
     will-change: transform;
     user-select: none;
     overflow: auto;
     z-index: 5;

     /* 添加以下样式使元素更像悬浮窗 */

     border: 1px solid var(--unite-but-color);
     background: var(--title-bar-lg-2);
     /* 画格子模拟纸张 */
     background-image:
          linear-gradient(to right, var(--button-color) 1px, transparent 1px),
          linear-gradient(to bottom, var(--button-color) 1px, transparent 1px);
     background-size: var(--grid-size) var(--grid-size);
     overflow: auto;

     border-radius: 8px;
     box-shadow: 0 4px 12px var(--font-color);
     border: 1px solid #e0e0e0;
     padding: 1vmin;
     overflow-x: hidden;
     animation: show-choose-pool 0.5s ease-in-out forwards;

}

.choose-path-pool-close {
     animation: hide-choose-pool 0.5s ease-in-out forwards;
}

.float-close-icon {
     float: right;
     margin-right: 2vmin;
}

.drag-head-contain {
     height: 10%;
     width: 100%;
}

.drag-contain {
     flex: 1;
     width: 100%;
     border: 1px dashed var(--unite-but-color);
     border-radius: 1vmin;
}



/* ----------------------------------------动画区----------------------------------------------- */

/* 替换名称区域的过渡动画 */
.replace-name-transition-leave-active {
     transition: all 0.5s ease;
}

.replace-name-transition-leave-from {
     opacity: 1;
     filter: blur(0px);
}

.replace-name-transition-leave-to {
     opacity: 0;
     filter: blur(10px);
}

/* 列表项的过渡动画 */
.path-item-leave-active {
     transition: all 0.5s ease;
}

.path-item-leave-from {
     opacity: 1;
     filter: blur(0px);
}

.path-item-leave-to {
     opacity: 0;
     filter: blur(10px);
}

/* 确保离开的元素在动画期间保持定位
.path-item-move {
     transition: transform 0.5s ease;
} */


@keyframes show-submit-replace-name {
     0% {
          opacity: 0;
          filter: blur(10px);
     }

     100% {
          opacity: 1;
          filter: blur(0px);
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

@keyframes show-paper {
     0% {
          height: 0%;
     }

     100% {
          height: 78%;
     }

}

@keyframes show-choose-pool {
     0% {
          height: 0vmin;
          opacity: 0;
     }

     100% {
          height: 80vmin;
          opacity: 1;

     }

}

@keyframes hide-choose-pool {
     0% {
          height: 80vmin;
          opacity: 1;

     }

     100% {
          height: 0vmin;
          opacity: 0;

     }


}
</style>