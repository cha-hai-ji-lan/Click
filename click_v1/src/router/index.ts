// 创建路由并暴露
import { createRouter, createWebHistory } from 'vue-router'
import DealFile from '../components/routes/DealFile.vue'
import MakeShutDown from '../components/routes/MakeShutDown.vue'

// 初始化路由器
const router = createRouter({
    history: createWebHistory(), // 路由器的工作模式
    routes: [{
        path:'/deal-file',
        component:DealFile
    },
    {
        path:'/make-shutdown',
        component:MakeShutDown
    }
       
    ]
})

// 暴露路由
export default router
