import { createI18n } from 'vue-i18n'
/*
 * All i18n resources specified in the plugin `include` option can be loaded
 * at once using the import syntax
 */
// 引入后可调用locals目录下的文件，否则只有vue文件中的i18n可用
// @ts-ignore
import messages from '@intlify/vite-plugin-vue-i18n/messages'
import { App } from 'vue'

const i18n = createI18n({
    locale: 'zh-CN',
    // locale: 'en',
    messages
})

export default (app: App) => app.use(i18n)
