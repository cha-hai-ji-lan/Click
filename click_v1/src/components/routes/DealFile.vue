<template>
     <div class="main-contain">
          <div class="title">
               <div class="tag-icon" @click="()=>{openFileDialog()}">
                    <svg t="1766153567513" class="icon" viewBox="0 0 1024 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="7556" width="200" height="200">
                         <path d="M910.096853 360.579647v-61.707521c0-52.999376-42.030493-112.28336-98.281728-112.28336H528.430288l-11.681508-22.048347c-11.685603-22.352441-34.588894-36.229158-59.807189-36.229158H154.570848c-50.436591 0-91.447294 42.168717-91.447294 94.010079v575.540275c0 51.814741 41.010703 93.969123 91.447294 93.969124h711.539892c50.436591 0 91.447294-42.154383 91.447294-93.969124V444.614011c-0.001024-30.633625-20.252452-63.054954-47.461181-84.034364z m-98.281728-117.546131c19.540851 0 41.838002 29.75206 41.838002 55.83861v39.040748c-0.956309-0.038908-1.913642-0.097269-2.865855-0.097269H608.55341l-50.21748-94.782089h253.479195z m89.297134 554.828099c0 20.684531-15.709473 37.524373-35.002543 37.524374h-711.538868c-19.292047 0-35.002544-16.839842-35.002544-37.524374V222.32134c0-20.712176 15.709473-37.565329 35.002544-37.565329h302.370743c4.134449 0 7.882893 2.274049 9.867182 6.07676l99.825748 188.420497a28.252068 28.252068 0 0 0 24.94287 15.007087h259.209881c20.229926 0 50.326011 30.12373 50.326011 50.353656v353.247604z"
                              p-id="7557" :fill="mainColor.iconColor">
                         </path>
                    </svg>
               </div>


          </div>
          <div class="explorer-act-path">
               <transition-group name="path-item" tag="div">

                    <div class="index-path coming-animation"
                         v-for="item in active_path" :key="item[1]">
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
                         <div v-else class="place-holder" >
                              <span>PS: 没有什么路径可操控，我没啥可干的</span>
                         </div>
                    </transition>
               </div>
               <div class="oper-1">
               </div>

          </div>


     </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";  // invoke：钩子方法 用于调用后端rust的函数
import { open } from '@tauri-apps/plugin-dialog';
import { ref, onMounted, onUnmounted } from "vue";
interface PathItem extends Array<string> {
     0: string; // 路径
     1: string; // 名称
}

const active_path = ref<PathItem[] | null>(null)
const selectedPaths = ref<PathItem[]>([]) // 选中的路径

let pollingInterval = ref<number | null>(null);
// 保存上一次获取的数据用于比较
const lastActivePathData = ref<string | null>(null);

const inputRefReplaceOldName = ref("")
const inputRefReplaceNewName = ref("")

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
     if (selectedPaths.value && selectedPaths.value.length > 0){
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
.title::after{
     content: '';
     position: absolute;
     bottom: 2px;
     left: 0;
     width: 100%;
     height: 2px;
     background: linear-gradient(to right, var(--main-border) 60%, var(--font-color) 62.5%,var(--button-color) 65%,var(--back-ground) 67.5%, var(--main-back-ground) 70%);
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

.submit-replace-name:hover{
     box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
     filter: drop-shadow(0 0 1em var(--title-min-icon-hover-shadow));
     
}
.submit-replace-name:active{
     background:var(--title-min-icon-shadow);
     box-shadow: 2px 2px var(--title-min-icon-hover-shadow);
     animation: active-icon 0.25s forwards;
     animation-timing-function: linear;
}


.coming-animation {
     animation: show-submit-replace-name 0.5s ease-in-out forwards;
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
     0%{
          height: 0%;
     }
     100%{
          height: 78%;
     }
     
}
</style>