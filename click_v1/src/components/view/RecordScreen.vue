<template>
    <div ref="chartRef" class="chart"></div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue'
  import * as echarts from 'echarts'
  
  const chartRef = ref<HTMLDivElement>()
  
  let chart: echarts.ECharts | null = null
  
  onMounted(() => {
    if (chartRef.value) {
      chart = echarts.init(chartRef.value)
      
      const option = {
        title: {
          text: 'ECharts 示例'
        },
        tooltip: {},
        xAxis: {
          type: 'category',
          data: ['A', 'B', 'C']
        },
        yAxis: {
          type: 'value'
        },
        series: [{
          data: [10, 20, 15],
          type: 'bar'
        }]
      }
      
      chart.setOption(option)
    }
  })
  
  onUnmounted(() => {
    chart?.dispose()
  })
  </script>
  
  <style scoped>
  .chart {
    width: 100%;
    height: 400px;
  }
  </style>