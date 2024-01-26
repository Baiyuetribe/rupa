<template lang="pug">
n-card
    n-h4(prefix="bar") 应用
    n-tabs(animated type="line" trigger="hover")
        n-tab-pane(name="0" tab="运行环境")
            n-table(:bordered="true" :single-line="false" striped style="text-align:center").table-striped
                thead
                    tr
                        th 名称
                        th 说明
                        th 状态
                        th 操作
                tbody
                    tr(v-for="(v,index) in data" :key="data.id")
                        td {{ v.name }} 
                        td {{ v.info }}             
                        td {{ v.status }}             
                        td 
                            n-flex(v-if="v.status==0")
                                n-button(text) 安装   
                            n-flex(v-else)
                                n-button(text) 设置   
                                n-button(text) 卸载   
        n-tab-pane(name="1" tab="热门应用")
            n-table(:bordered="true" :single-line="false" striped style="text-align:center").table-striped
                thead
                    tr
                        th 名称
                        th 简介
                        th 语言
                        th 操作
                tbody
                    tr(v-for="(v,index) in hotapp" :key="data.id")
                        td {{ v.name }} 
                        td {{ v.info }}             
                        td {{ v.lang }}             
                        td 
                            n-button(text) 创建   
</template>

<script setup>
const data = ref([{
    key: 0,
    name: "Nginx",
    info: "Nginx是一个高性能的HTTP和反向代理服务器",
    status: 0, // 0 代表未安装；1代表运行，2代表停止
}, {
    key: 1,
    name: "Mysql",
    info: "MySQL是一个关系型数据库管理系统",
    status: 1,
},])

const hotapp = ref([{
    key: 0,
    name: "wordpress",
    info: "WordPress是一个使用PHP语言开发的博客平台",
    lang: "php",
}, {
    key: 1,
    name: "hexo",
    info: "Hexo是一个快速、简洁且高效的博客框架",
    lang: "js",
},])
</script>

<style lang="stylus" scoped>

</style>
