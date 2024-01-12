<template lang="pug">
n-layout(has-sider)
  n-layout-sider.sider(width="180"  collapse-mode="width" bordered :collapsed-width="82" show-trigger="bar" :collapsed="state.collapsed" @collapse="state.collapsed = true" @expand="state.collapsed = false" style="background: hsla(0, 0%, 100%, .73);")
    n-flex(justify="center" )
      n-icon(size="42" color="#536dfe" style="margin-top:1rem;")
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path d="M448 112s-17.62 0-30.51 1.39c-19 2-42.06 8-59.73 13.22c-35.06 10.39-69.33 23.92-107.85 42.59c-18.62 9.05-26 13.35-48 26.13l-4.5 2.67c-32.95 19-57.09 40-73.79 64.3C105.29 288.89 96 320 96 354.64c0 40.74 15.71 77.1 44.24 102.37C169 482.52 209.06 496 256 496c46.76 0 86.89-14.32 116-41.43c28.35-26.35 44-63.39 44-104.29c0-25-6.19-47-12.17-68.22c-12.59-44.69-23.46-83.29 24.71-144.13C432.75 132.62 448 112 448 112z" fill="currentColor"></path><path d="M219 119.55C168.47 92.08 104.72 80 80 80c0 0 23.23 28.19 29.15 55.4s6.54 48.61 2.91 88.6c17.94-20.48 40.59-37.15 69.32-53.73l4.48-2.6C208 154.8 216.23 150 236 140.41c2.88-1.4 5.74-2.76 8.58-4.11A170.77 170.77 0 0 0 219 119.55z" fill="currentColor"></path><path d="M345.25 48s-42.53.36-86.12 21.3a280.36 280.36 0 0 0-32.27 18.27q3.73 1.89 7.4 3.88c3.44 1.87 7.09 4 10.9 6.29a189.7 189.7 0 0 1 31.46 24.16c24.57-10.41 73-26.1 90.77-31.28c-8-19.15-22.14-42.62-22.14-42.62z" fill="currentColor"></path><path d="M176 16c-16 10.83-33.24 41.1-33.24 41.1a494.22 494.22 0 0 1 48.92 15.25l17.65-11.56c8.18-5.35 16.55-10.29 25-14.77C234.31 46 202.59 24.17 176 16z" fill="currentColor"></path></svg>
    n-menu(:collapsed="state.collapsed" default-expand-all v-model:value="state.activeKey"  :collapsed-width="82" :collapsed-icon-size="32" :options="state.options" @update:value="goRoute")
  //- 需要重新设置一个layout
  n-layout(style="height: 100vh;background-color: #f4f6f9;")
    Header 
    n-layout-content(style="background:none")
      router-view
    Footer
</template>

<script setup>
import { NIcon } from 'naive-ui'
import {
  BarChartOutline, TimeOutline, ServerOutline, EarthOutline, ShieldCheckmarkOutline, FolderOutline, AppsOutline, HardwareChipOutline, CardOutline, SettingsOutline, EaselOutline, ExtensionPuzzleOutline, LogOutOutline,
} from '@vicons/ionicons5'
const router = useRouter()

function renderIcon(icon) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const goRoute = (key, item) => {
  switch (key) {
    case 'back':
      window.open(location.origin, 'top')
      break;
    case 'logout':
      window.localStorage.removeItem('rupa_token')
      router.push('/login')
      break;
    default:
      router.push({ 'name': key })
      break;
  }
}
const state = reactive({
  collapsed: false,
  activeKey: router.currentRoute.value.name,
  // activeKey: '',
  options: [
    {
      label: '仪表盘',
      key: 'dashboard',
      icon: renderIcon(BarChartOutline),
    },
    {
      label: '网站',
      key: 'website',
      icon: renderIcon(EarthOutline),
    },
    {
      label: '数据库',
      key: 'db',
      icon: renderIcon(ServerOutline),
    },
    {
      label: '文件',
      key: 'folder',
      icon: renderIcon(FolderOutline),
    },
    {
      label: '日志',
      key: 'log',
      icon: renderIcon(SettingsOutline),
    },
    {
      label: '端口',
      key: 'port',
      icon: renderIcon(HardwareChipOutline),
    },
    {
      label: '安全',
      key: 'safe',
      icon: renderIcon(ShieldCheckmarkOutline),
    },
    {
      label: '应用',
      key: 'app',
      icon: renderIcon(AppsOutline),
    },
    {
      label: '监控',
      key: 'monitor',
      icon: renderIcon(EaselOutline),
    },
    {
      label: '计划任务',
      key: 'cron',
      icon: renderIcon(TimeOutline),
    },
    {
      label: '其他',
      key: 'setting',
      icon: renderIcon(SettingsOutline),
    },
    {
      label: '退出登录',
      key: 'logout',
      icon: renderIcon(LogOutOutline)
    }
  ]

})

</script>

<style lang="stylus" scoped>
// .sider
//   min-height: 100vh
//   transition: all 0.2s ease-in-out
//   padding-top: 12px
//   box-shadow: 2px 0 8px #1d23290d
:deep(.n-menu-item--selected)
  border-right: 6px solid #6078FF;
:deep(.n-menu-item--selected)
  border-right: 6px solid #6078FF;
</style>