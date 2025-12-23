<template>
    <div class="select-container">
      <div class="select-wrapper" @click="toggleOptions">
        <div class="select-selected">
          {{ selectedOption ? selectedOption.label : placeholder }}
        </div>
        <div class="select-arrow" :class="{ 'arrow-up': isOpen }"></div>
      </div>
      <div class="select-options" v-show="isOpen">
        <div 
          v-for="option in options" 
          :key="option.value"
          class="select-option"
          :class="{ 'selected': selectedValue === option.value }"
          @click="selectOption(option)"
        >
          {{ option.label }}
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed } from 'vue'
  
  interface SelectOption {
    value: string | number
    label: string
  }
  
  const props = defineProps<{
    options: SelectOption[]
    modelValue?: string | number
    placeholder?: string
  }>()
  
  const emit = defineEmits<{
    (e: 'update:modelValue', value: string | number): void
  }>()
  
  const isOpen = ref(false)
  const selectedValue = computed(() => props.modelValue)
  
  const selectedOption = computed(() => {
    return props.options.find(option => option.value === selectedValue.value)
  })
  
  const toggleOptions = () => {
    isOpen.value = !isOpen.value
  }
  
  const selectOption = (option: SelectOption) => {
    emit('update:modelValue', option.value)
    isOpen.value = false
  }
  
  // 点击外部关闭选项列表
  document.addEventListener('click', (e) => {
    const selectContainer = document.querySelector('.select-container')
    if (selectContainer && !selectContainer.contains(e.target as Node)) {
      isOpen.value = false
    }
  })
  </script>
  
  <style scoped>
  .select-container {
    position: relative;
    width: 100%;
    margin: 0 0;
    font-family: Arial, sans-serif;
  }
  
  .select-label {
    display: block;
    margin-bottom: 5px;
    font-weight: bold;
    color: #333;
  }
  
  .select-wrapper {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1px 15px;
    border: 2px solid #ddd;
    border-radius: 4px;
    background-color: white;
    cursor: pointer;
    transition: border-color 0.3s ease;
  }
  
  .select-wrapper:hover {
    border-color: var(--active-attention-color);
  }
  
  .select-selected {
    flex-grow: 1;
    color: #333;
  }
  
  .select-arrow {
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 5px solid #999;
    transition: transform 0.3s ease;
  }
  
  .arrow-up {
    transform: rotate(180deg);
  }
  
  .select-options {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    border: 2px solid #ddd;
    border-top: none;
    border-radius: 0 0 4px 4px;
    background-color: white;
    z-index: 100;
    max-height: 200px;
    overflow-y: auto;
  }
  
  .select-option {
    padding: 1px 15px;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }
  
  .select-option:hover {
    background-color: #f8f9fa;
  }
  
  .select-option.selected {
    background-color: var(--active-attention-color);
    color: white;
  }
  </style>