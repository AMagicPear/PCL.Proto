/**
 ____   ____ _       ____            _        
|  _ \ / ___| |     |  _ \ _ __ ___ | |_ ___  
| |_) | |   | |     | |_) | '__/ _ \| __/ _ \ 
|  __/| |___| |___ _|  __/| | | (_) | || (_) |
|_|    \____|_____(_)_|   |_|  \___/ \__\___/ 
 */
import './assets/main.css'
import { isTauri } from '@tauri-apps/api/core'

import App from './App.vue'
import { initializePclLocale } from './api/pclLocale'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from '@/router/index'
import { useAccountInfo } from './stores/account'
import { attachConsole } from '@tauri-apps/plugin-log'
import { useSelectedInstance } from './stores/gameLaunch'

const pinia = createPinia()

const app = createApp(App)

app.use(pinia)
app.use(router)

// ignore non-emit warnnings
if (import.meta.env.DEV) {
  app.config.warnHandler = (msg, instance, trace) => {
    if (msg.includes('Extraneous non-emits event listeners')) {
      return
    }
    console.warn(msg, trace)
  }
}

document.documentElement.classList.add('theme-blue')
document.documentElement.classList.toggle('website', !isTauri())

void initializePclLocale().then(() => app.mount('#app'))

// some async actions
if (isTauri()) void useAccountInfo().initialize()
if (isTauri()) {
  void attachConsole()
  void useSelectedInstance().fetch().catch(console.error)
}
