<template>
  <div class="select-container">
    <div class="select-wrapper" @click="toggleOptions">
      <div class="select-selected">
        {{ selectedOption ? selectedOption.label : placeholder }}
      </div>
      <div class="select-arrow" :class="{ 'arrow-up': isOpen }"></div>
    </div>
    <div class="select-options" v-show="isOpen">
      <div v-for="option in options" :key="option.value" class="select-option"
        :class="{ 'selected': selectedValue === option.value }" @click="selectOption(option)">
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
  height: 100%;
  margin: 0 0;
}

.select-label {
  display: block;
  font-weight: bold;
}

.select-wrapper {
  height: 100%;

  display: flex;
  justify-content: space-between;
  align-items: center;
  /* padding-top: 1px; */
  border-top: 0px dashed var(--unite-but-color);
  border-bottom: 2px dashed var(--unite-but-color);
  border-left: 0px dashed var(--unite-but-color);
  border-right: 0px dashed var(--unite-but-color);
  background-color: transparent;
  cursor: pointer;
  transition: border-color 0.3s ease;
}

.select-wrapper:hover {
  border-color: var(--active-attention-color);
}

.select-selected {
  flex-grow: 1;
}

.select-arrow {
  width: 0;
  height: 0;
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-top: 5px solid var(--positive-show-color);
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
  border: 1px dashed var(--unite-but-color);
  border-top: none;
  border-radius: 0 0 1vmin 1vmin;
  background-color: rgb(0,0,0,0);
  backdrop-filter: blur(30px);
  z-index: 5;
  max-height: 20vmin;
  overflow-y: auto;
}

.select-option {
  /* padding: 0px 15px; */
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.select-option:hover {
  background-color: var(--main-back-ground);
}

.select-option.selected {
  background-color: var(--active-attention-color);
  color: var(--title-bar-lg-1);
}
</style>