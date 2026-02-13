<template>
    <ul id="sortable-list" class="sortable-list">
        <li class="sortable-item">Item 1</li>
        <li class="sortable-item">Item 2</li>
        <li class="sortable-item">Item 3</li>
    </ul>
</template>

<script>
import Sortable from 'sortablejs';

export default {
    mounted() {
        // 禁止其他库干扰
        const el = document.getElementById('sortable-list');
        new Sortable(el, {
            animation: 150,
            ghostClass: 'sortable-ghost'
        });
    }
};
</script>

<style>
.sortable-list {
    list-style: none;
    padding: 0;
}

.sortable-item {
    padding: 10px;
    margin: 5px 0;
    background-color: #f0f0f0;
    border: 1px solid #ccc;
    cursor: move;
}
</style>