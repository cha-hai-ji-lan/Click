<template>
  <!-- 页面容器 -->
  <div class="container" @contextmenu.prevent="showMenu">
    <p>右键点击此处打开菜单</p>

    <!-- 自定义右键菜单 -->
    <div
      v-if="menuVisible"
      class="context-menu"
      :style="{ top: menuY + 'px', left: menuX + 'px' }"
    >
      <ul>
        <li @click="handleMenuClick('option1')">选项 1</li>
        <li @click="handleMenuClick('option2')">选项 2</li>
        <li @click="handleMenuClick('option3')">选项 3</li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

// 菜单可见性
const menuVisible = ref(false);
// 菜单坐标
const menuX = ref(0);
const menuY = ref(0);

// 显示菜单
const showMenu = (event: MouseEvent) => {
  menuVisible.value = true;
  menuX.value = event.clientX;
  menuY.value = event.clientY;

  // 点击其他区域关闭菜单
  document.addEventListener('click', closeMenu);
};

// 关闭菜单
const closeMenu = () => {
  menuVisible.value = false;
  document.removeEventListener('click', closeMenu);
};

// 处理菜单点击事件
const handleMenuClick = (option: string) => {
  console.log(`点击了 ${option}`);
  closeMenu();
};
</script>

<style scoped>
.container {
  width: 100vw;
  height: 100vh;
  background-color: #f0f0f0;
  display: flex;
  justify-content: center;
  align-items: center;
}

.context-menu {
  position: fixed;
  background-color: white;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  z-index: 1000;
}

.context-menu ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

.context-menu li {
  padding: 8px 16px;
  cursor: pointer;
}

.context-menu li:hover {
  background-color: #f5f5f5;
}
</style>