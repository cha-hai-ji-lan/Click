<template>
    <div class="main-doc">
        <div class="doc-index">

            <div class="index-contain">
                <ul>
                    <li>
                        <div class="index-item">
                            <svg class="icon" width="32" height="32"  viewBox="0 0 32 32"  xmlns="http://www.w3.org/2000/svg">
                                <g>
                                    <circle r="15" cx="16" cy="16" stroke="black" fill-opacity="0.6" stroke-width="1.25"
                                        opacity="1" fill="#ecfbff" />
                                    <circle r="10" cx="16" cy="16" fill="#29fbf1" stroke-width="1.5" stroke="#90ebfd"
                                        stroke-opacity="0.75" />
                                </g>
                                <path d="M9 24C0 40 40 0 24 9" fill="none" stroke="#28e9ff" stroke-opacity="1"
                                    stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
                                <polygon
                                    points="15.76 4.501 16.492 4.751 16.776 5.522 17.005 4.779 17.758 4.527 16.995 4.259 16.755 3.508 16.521 4.232"
                                    stroke="#bff8ff" fill="#58c3ff" stroke-width="0.2" stroke-linejoin="round" />
                                <polygon
                                    points="16.84 26.07 19.16 26.89 19.99 28.99 20.61 26.93 22.95 25.97 20.77 25.43 19.98 22.95 19.05 25.46"
                                    stroke="#bff8ff" fill="#58c3ff" stroke-width="0.75" stroke-linejoin="round" />
                                <polygon
                                    points="3.456 10.513 4.992 11.01 5.508 12.518 6.009 11.01 7.503 10.497 6.001 10.004 5.497 8.513 4.996 9.982"
                                    stroke="#bff8ff" fill="#58c3ff" stroke-width="0.5" stroke-linejoin="round" />
                            </svg>

                            <span class="item-text">关于我</span>
                        </div>

                    </li>
                </ul>
            </div>
        </div>
        <div class="doc-text">
            正文
        </div>

    </div>
</template>

<script setup lang="ts">

</script>
<style scoped>
.main-doc {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    width: 100%;
    height: 100%;
}

.doc-index {
    background: var(--main-border);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    width: 20vmin;
    height: 96%;
    margin: 2% 0;
    border-radius: 2.5vmin;
}

.index-contain {
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: column;
    margin: 0.5vmin;
    flex: 1;
    border-radius: 2vmin;
    background: var(--title-bar-lg-2);
    width: 19vmin;


}

.index-contain ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

.index-contain li {
    padding: 0;
    margin: 0;
}

.index-item {
    display: flex;
    justify-content: start;
    align-items: center;
    flex-direction: row;
    margin: 0.5vmin;
}

.item-text{
    user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}

.doc-text {
    flex: 1;
    width: 79%;
    height: 100%;
}
</style>