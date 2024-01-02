import { createPinia } from 'pinia'
import { App } from 'vue'

export default (app: App) => app.use(createPinia())
