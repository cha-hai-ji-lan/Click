import { createApp } from "vue";
import App from "./App.vue";

createApp(App).mount("#app");

// 自定义标题栏

import {Window} from '@tauri-apps/api/window';

const appWindow = new Window('main');
document
    .getElementById('titlebar-minimize')
    ?.addEventListener('click', () => appWindow.minimize());
document
    .getElementById('titlebar-maximize')
    ?.addEventListener('click', () => appWindow.toggleMaximize());
document
    .getElementById('titlebar-close')
    ?.addEventListener('click', () => {
    appWindow.close()
    });