<template>
    <div class="selector-bar-container">
      <div class="selector-bar" @mouseenter="startAnimation" @mouseleave="resetAnimation">
        <div class="center-line" :class="{ 'animate': isAnimating }">
          <div class="line"></div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue';
  
  const isAnimating = ref(false);
  
  const startAnimation = () => {
    isAnimating.value = true;
  };
  
  const resetAnimation = () => {
    isAnimating.value = false;
  };
  </script>
  
  <style scoped>
  .selector-bar-container {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 10px 0;
  }
  
  .selector-bar {
    height: 80%;
    position: relative;
    height: 2px;
    cursor: pointer;
  }
  
  .center-line {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 0;
    height: 100%;
    overflow: hidden;
  }
  
  .center-line.animate {
    animation: expandLine 0.6s ease-out forwards;
  }
  
  .line {
    width: 100%;
    height: 100%;
    background: linear-gradient(
      to right,
      rgba(0, 0, 255, 0) 0%, 
      rgba(0, 0, 255, 1) 30%, 
      rgba(0, 0, 255, 1) 70%, 
      rgba(0, 0, 255, 0) 100%
    );
  }
  
  @keyframes expandLine {
    0% {
      height: 0;
    }
    100% {
      height: 100%;
    }
  }
  </style>