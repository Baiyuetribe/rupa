<template lang="pug">
n-card
    n-flex(justify="space-between" align="center")
        n-h4(prefix="bar") 网站
        n-button(type="primary" style="margin-bottom:1rem" @click="state.showAdd = true") 新增
            template(#icon)
                n-icon(size="")
                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M256 112v288"></path><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M400 256H112"></path></svg>

    n-table(:bordered="true" :single-line="false" striped ).table-striped
        thead
            tr
                th(width="20px") 目录
                th 网站
                th 状态
                th SSL证书
                th 备注
                th 操作
        tbody
            tr(v-for="(v,index) in data" :key="data.id")
                td(style="text-align:center")  
                    n-icon(color="#536dfe" size="large" @click="check(data.sk)" :component="FolderOutline")
                td {{ v.name }} 
                td {{ v.status }}
                td {{ v.ssl }}
                td
                    n-text(depth="3") {{ v.info }}
                td
                    n-flex
                        n-icon(color="#536dfe" size="large" @click="check(data.sk)" :component="SettingsOutline")
                        n-popconfirm(@positive-click="del(data.id)")
                            template(#trigger)
                                n-icon(color="#e84c85" size="large")                        
                                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 32 32"><path d="M12 12h2v12h-2z" fill="currentColor"></path><path d="M18 12h2v12h-2z" fill="currentColor"></path><path d="M4 6v2h2v20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8h2V6zm4 22V8h16v20z" fill="currentColor"></path><path d="M12 2h8v2h-8z" fill="currentColor"></path></svg>
                            span 确认删除？
    n-space(justify="center" style="margin-top:12px")  
        n-pagination(v-model:page="page" :item-count="count" )                       
</template>

<script setup>
import { BarChartOutline, RemoveCircleOutline, ServerOutline, EarthOutline, ShieldCheckmarkOutline, FolderOutline, AppsOutline, HardwareChipOutline, CardOutline, SettingsOutline, EaselOutline, ExtensionPuzzleOutline, LogOutOutline, } from '@vicons/ionicons5'

// h函数用法
// 一个字符串，表示元素的标签类型。
// 一个对象 (可选)，表示元素的属性。
// 一个字符串或数组 (可选)，表示元素的子节点。

const page = ref(1)
const count = ref(10)
const data = ref([{
    key: 0,
    name: "demo.com",
    status: 0,
    ssl: -1, // -1 无证书，到期时间，可以设置为一个日期
    info: "博客",
}, {
    key: 1,
    name: "example.com",
    status: 0,
    ssl: -1,
    info: "社区",
},])
</script>

<style lang="stylus" scoped>

</style>
