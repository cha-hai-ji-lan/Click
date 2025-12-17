<template>
    <div v-show="comVisibility.LeftContain['LeftContain-open']" class="main-left">
        <div class="sub-left-contain">
            <div v-for="(item) in comVisibility.LeftContain['LeftContain-data']['item']" :key="item['index']"
                class="single-item">
                <div class="name">{{ item["name"] }}</div>
                
            </div>
        </div>

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
    font-family: "楷体", 'Courier New', Courier, monospace;
    font-weight: 600;
    border-right: 1px solid var(--main-border);
    background: linear-gradient(to left, var(--icon-hover), var(--main-back-ground));
}
.sub-left-contain {
    padding: 2vh 0;
    width: 100%;
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
}

.single-item {
    display:flex;
    justify-content: center;
    align-items: center;
    border-radius: 2vmin;
    background:var(--main-back-ground);
    margin-top: 0.75vh;
    height: 30px;
    width: 90%;

}


</style>