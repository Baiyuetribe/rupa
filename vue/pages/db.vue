<template lang="pug">
n-card
    n-flex(justify="space-between" align="center")
        n-h4(prefix="bar") 数据库
        n-button(type="primary" style="margin-bottom:1rem" @click="state.showAdd = true") 新增
            template(#icon)
                n-icon(size="")
                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M256 112v288"></path><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M400 256H112"></path></svg>

    n-table(:bordered="true" :single-line="false" striped style="text-align:center").table-striped
        thead
            tr
                th 数据库名
                th 用户名
                th 密码
                th 备份
                th 备注
                th 操作
        tbody
            tr(v-for="(v,index) in data" :key="data.id")
                td {{ v.db_name }} 
                td {{ v.user_name }} 
                td 
                    n-flex
                        n-text {{ v.password }}
                        n-button(text)
                            n-icon(color="#536dfe" @click="check(data.sk)" :component="EyeOffOutline")
                        n-button(text)
                            n-icon(color="#536dfe" @click="check(data.sk)" :component="CopyOutline")
                        n-button(text)
                            n-icon(color="#536dfe" @click="check(data.sk)" :component="BuildOutline")

                td(style="text-align:center")  备份|导入
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
import { BarChartOutline, RemoveCircleOutline, EyeOffOutline, CopyOutline, BuildOutline, FolderOutline, AppsOutline, HardwareChipOutline, CardOutline, SettingsOutline, EaselOutline, ExtensionPuzzleOutline, LogOutOutline, } from '@vicons/ionicons5'

// h函数用法
// 一个字符串，表示元素的标签类型。
// 一个对象 (可选)，表示元素的属性。
// 一个字符串或数组 (可选)，表示元素的子节点。

const page = ref(1)
const count = ref(10)
const data = ref([{
    key: 0,
    db_name: "rupa",
    user_name: "rupa",
    password: "rupa123",
    info: "演示",
}, {
    key: 1,
    db_name: "rupa55",
    user_name: "rupa66",
    password: "rupa123",
    info: "演示2",
},])
</script>

<style lang="stylus" scoped>

</style>
    