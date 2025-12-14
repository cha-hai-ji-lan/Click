<template>
    <div v-show="comVisibility.LeftContain" class="main-left">
        
    </div>
</template>
<script setup lang="ts">
const props = defineProps({
    comVisibility: {
        type: Object,
        default: {}
    }
});

// 定义更新事件
const emit = defineEmits<{
    (e: 'update:comVisibility', value: Object): void
}>()

</script>
<style scoped>
.main-left {
    width: 100%;
    height: 100%;
    min-width: 0px;
    max-width: 25vw;
    min-height: 100px;
    max-height: 900px;
    border-radius: 2vh;
    border-right: 1px solid var(--main-border);
    background:linear-gradient(to left, var(--icon-hover), var(--main-back-ground));
}
</style>