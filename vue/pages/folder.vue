<template lang="pug">

n-card
    n-h4(prefix="bar") 文件管理
    n-input-group
        n-button(type="primary" ghost style="margin-bottom:1rem" @click="state.showAdd = true") <
        n-input(v-model:value="path" )
        n-button(type="primary" ghost @click="state.showAdd = true")
            n-icon(:component="RefreshOutline") 
    n-space(justify="space-between" align="center")
        n-flex
            n-button 上传
            n-button 新建
            n-button 远程下载
        n-button(type="primary" ghost @click="state.showAdd = true") 回收站
    n-table(:bordered="true" :single-line="false" striped style="margin-top:1rem").table-striped
        thead
            tr
                th() 文件名
                th 权限/所有者
                th 大小
                th 修改时间
                th 操作
        tbody
            tr(v-for="(v,index) in data" :key="data.id")
                td
                    n-flex
                        n-icon(color="#536dfe" size="large" :component="renderFile(v)") 
                        n-text {{ v.filename }}
                td {{ v.chmod }}
                td {{ v.type == 'dir' ? '计算' : v.size  }}
                td {{ v.time }} 
                td
                    n-flex
                        n-icon(color="#536dfe" size="large")
                            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 28 28"><g fill="none"><path d="M11.75 2a.75.75 0 0 0 0 1.5h1.5v21h-1.5a.75.75 0 0 0 0 1.5h4.5a.75.75 0 0 0 0-1.5h-1.5v-21h1.5a.75.75 0 0 0 0-1.5h-4.5zm-5.5 4.02h6v1.5h-6A1.75 1.75 0 0 0 4.5 9.27v9.5c0 .966.784 1.75 1.75 1.75h6v1.5h-6A3.25 3.25 0 0 1 3 18.77v-9.5a3.25 3.25 0 0 1 3.25-3.25zm15.5 14.5h-6v1.5h6A3.25 3.25 0 0 0 25 18.77v-9.5a3.25 3.25 0 0 0-3.25-3.25h-6v1.5h6c.966 0 1.75.783 1.75 1.75v9.5a1.75 1.75 0 0 1-1.75 1.75z" fill="currentColor"></path></g></svg>
                        n-icon(color="#536dfe" size="large")
                            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 20 20"><g fill="none"><path d="M13.245 2.817a2.783 2.783 0 0 1 4.066 3.796l-.13.14l-9.606 9.606a2.001 2.001 0 0 1-.723.462l-.165.053l-4.055 1.106a.5.5 0 0 1-.63-.535l.016-.08l1.106-4.054c.076-.28.212-.54.398-.76l.117-.128l9.606-9.606zm-.86 2.275L4.346 13.13a1 1 0 0 0-.215.321l-.042.123l-.877 3.21l3.212-.875a1 1 0 0 0 .239-.1l.107-.072l.098-.085l8.038-8.04l-2.521-2.52zm4.089-1.568a1.783 1.783 0 0 0-2.402-.11l-.12.11l-.86.86l2.52 2.522l.862-.86a1.783 1.783 0 0 0 .11-2.402l-.11-.12z" fill="currentColor"></path></g></svg>
                        n-icon(color="#536dfe" size="large" :component="CopyOutline")
                        n-icon(color="#536dfe" size="large")
                            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 32 32"><path d="M28 9h-6v14h2v-5h4a2 2 0 0 0 2-2v-5a2 2 0 0 0-2-2zm0 7h-4v-5h4z" fill="currentColor"></path><path d="M12 9v2h3v10h-3v2h8v-2h-3V11h3V9h-8z" fill="currentColor"></path><path d="M10 9H2v2h6L2 21v2h8v-2H4l6-10V9z" fill="currentColor"></path></svg>
                        n-icon(color="#536dfe" size="large")
                            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 32 32"><path d="M11 23.18l-2-2.001l-1.411 1.41L11 26l6-6l-1.41-1.41L11 23.18z" fill="currentColor"></path><path d="M28 30h-4v-2h4V16h-4V8a4.005 4.005 0 0 0-4-4V2a6.007 6.007 0 0 1 6 6v6h2a2.002 2.002 0 0 1 2 2v12a2.002 2.002 0 0 1-2 2z" fill="currentColor"></path><path d="M20 14h-2V8A6 6 0 0 0 6 8v6H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V16a2 2 0 0 0-2-2zM8 8a4 4 0 0 1 8 0v6H8zm12 20H4V16h16z" fill="currentColor"></path></svg>
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
import { RefreshOutline, FolderOutline, DocumentTextOutline, CopyOutline } from '@vicons/ionicons5'
const path = ref('/opt/rupa')
const data = ref([{
    id: 0,
    filename: "demo",
    chmod: "777/www",
    size: "",
    time: "2021-05-16 12:00:11",
    type: "dir"
}, {
    id: 1,
    filename: "admin",
    chmod: "777/www",
    size: "",
    time: "2021-05-16 12:00:11",
    type: "dir"
}, {
    id: 2,
    filename: "a.txt",
    chmod: "777/www",
    size: "16kb",
    time: "2021-05-16 12:00:11",
    type: "file"
},
])

const renderFile = (data) => {
    if (data.type == 'dir') {
        return FolderOutline
    } else {
        return DocumentTextOutline
    }
}
</script>

<style lang="stylus" scoped>

</style>
