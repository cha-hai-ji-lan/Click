<template>
  <div class="shutdown-main">
    <div class="shutdown-title">

    </div>
    <div class="shutdown-contain">
      <form action="" class="timing-form">
        <input class="hold-time" type="number" placeholder="时（h）">
        <input class="hold-time" type="number" placeholder="分（m）">
        <input class="hold-time" type="number" placeholder="秒（s）">
        <div class="submin-time" @click="() => { }"><span>确定</span></div>
        <div class="reset-time" @click="() => { }"><span>重置</span></div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

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
}

.shutdown-title {
  width: 100%;
  margin-top: 3%;
  height: 16%;
  position: relative;

}

.shutdown-title::after {
  position: absolute;
  content: '';
  bottom: 2px;
  left: 0;
  width: 100%;
  height: 2px;
  background: linear-gradient(to right, var(--main-border), var(--title-bar-lg-1) var(--timing-schedule));
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
  display: flex;
  height: 4vmin;
  justify-content: center;
  align-content: center;
  border-top: 1px dashed var(--unite-but-color);
  border-bottom: 1px dashed var(--unite-but-color);

}

.hold-time {
  flex: 1;
  font-family: "楷体", 'Courier New', Courier, monospace;
  font-size: 2vmin;
  text-align: center;
  border-radius: 1vmin;
  border-top: 1px dashed var(--unite-but-color);
  border-bottom: 1px dashed var(--unite-but-color);
  border-left: 1px dashed var(--unite-but-color);
  border-right: 1px dashed var(--unite-but-color);

}

.reset-time,
.submin-time {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 15%;
  border: 0px solid var(--main-border);
  border-left: 1px solid var(--main-border);
  border-right: 1px solid var(--main-border);
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


@keyframes show-paper {
  0% {
    height: 0%;
  }

  100% {
    height: 78%;
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
</style>