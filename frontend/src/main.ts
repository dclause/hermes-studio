import './assets/main.css'
import { createApp } from 'vue'
import { registerPlugins } from '@/plugins'
import App from './App.vue'

const app = createApp(App)
registerPlugins(app)

app.config.performance = true
app.mount('#app')
