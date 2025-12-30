// 创建路由并暴露
import { createRouter, createWebHistory } from 'vue-router'
import DealFile from '../components/view/DealFile.vue'
import MakeShutDown from '../components/view/MakeShutDown.vue'
import LivePicture from '../components/view/LivePicture.vue'
import Temp from '../components/view/Temp.vue'
import Home from '../components/view/Home.vue'

// 初始化路由器
const router = createRouter({
    history: createWebHistory(), // 路由器的工作模式
    routes: [
        {
            path: '/', // 添加根路径路由
            component: Home // 可以选择一个组件作为默认页面
        },
        {
            path: '/deal-file',
            component: DealFile
        },
        {
            path: '/make-shutdown',
            component: MakeShutDown
        },
        {
            path: '/live-picture',
            component: LivePicture
        },
        {
            path: '/child',
            component: Temp
        }

    ]
})

// 暴露路由
export default router
