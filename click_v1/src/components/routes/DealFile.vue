<template>
     <div class="main-contain">
          <div class="explorer-act-path">
               <div class="index-path" v-for="item in active_path" :key="item[1]">
                    <div class="choose">
                         <input type="checkbox" :value="item" :checked="isSelected(item)"
                              @change="handleCheckboxChange(item)" />

                    </div>
                    <div class="name">{{ item[1] }}</div>
                    <div class="path">{{ item[0] }}</div>
               </div>


          </div>

          <div class="replace-name">
               <div>修改名字</div>
               <input type="text" v-model="inputRefReplaceOldName" placeholder="旧字段">
               <input type="text" v-model="inputRefReplaceNewName" placeholder="新字段">
               <div class="submit-replace-name" @click="() => { SubmitRepluceName('replace-name') }">修改</div>
          </div>

     </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";  // invoke：钩子方法 用于调用后端rust的函数
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

               }else {
                    alert("请输入要替换的字段")
               }

               break;

          default:
               break;
     }

}


</script>
<style scoped>
.main-contain {
     width: 100%;
     display: flex;
     justify-content: start;
     align-items: center;
     flex-direction: column;
}


.explorer-act-path {
     margin-top: 2vh;
     width: 100%;
     height: 30%;
     overflow-y: auto;


}

.index-path {
     display: grid;
     grid-template-columns: 5% 20% 75%;
     position: relative;
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

.replace-name {
     display: grid;
     grid-template-columns: 25% 30% 30% 15%;
}

.submit-replace-name {
     border: 1px solid var(--main-border);
}
</style>