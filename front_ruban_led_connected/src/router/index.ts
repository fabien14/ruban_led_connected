import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/views/TheWelcome.vue'
import DevicesScanView from '@/views/DevicesScanView.vue'
import DevicesView from '@/views/DevicesView.vue'
import Devices from '@/components/Devices.vue'

const routes = [
    {
        path: '/',
        name: 'Home',
        component: Home
    },
    {
        path: '/scan',
        name: 'Scan',
        component: DevicesScanView
    },
    {
        path: '/devices',
        name: 'Devices',
        component: DevicesView,
        children: [
            {
                path: ':id',
                name: 'Device',
                component: Devices
            }
        ]
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router
