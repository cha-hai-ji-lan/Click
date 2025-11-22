# 0.0.2目标
1. 把面向Android平台和Windows平台的自定义菜单栏做出来
> 把main.ts的控制开关，缩小放大窗口，最小化按钮控制以及控制是否生效代码转移到App.vue
2. 做好平台检测
> 在App.ve根组件中检测平台
``` typescript
    import { ref, onMounted } from 'vue';

    const isWindows = ref(false);
    const isAndroid = ref(false);

    onMounted(() => {
    // 检测用户代理
    const userAgent = navigator.userAgent.toLowerCase();
    isWindows.value = userAgent.includes('windows');
    isAndroid.value = userAgent.includes('android');
    
    // 或者检测平台
    isWindows.value = navigator.platform.includes('Win');
    isAndroid.value = navigator.platform.includes('Android');
    });
```
3. 【选】为Android做好最下部分的窗口切换的按钮放置预留框