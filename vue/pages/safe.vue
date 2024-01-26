<template lang="pug">
n-card
    n-flex(justify="space-between" align="center")
        n-h4(prefix="bar") 进程守护
        n-button(type="primary" style="margin-bottom:1rem" @click="state.showAdd = true") 新增
            template(#icon)
                n-icon(size="")
                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M256 112v288"></path><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M400 256H112"></path></svg>

    n-table(:bordered="true" :single-line="false" striped style="text-align:center").table-striped
        thead
            tr
                th 名称
                th 启动命令
                th 进程ID
                th 优先级
                th 状态
                th 备注
                th 工作目录
                th 操作
        tbody
            tr(v-for="(v,index) in data" :key="data.id")
                td {{ v.name }} 
                td {{ v.cmd }} 
                td {{ v.pid }} 
                td {{ v.priority }} 
                td {{ v.status }} 
                td {{ v.work_dir }} 
                td
                    n-flex
                        n-button(text) 日志 
                        n-button(text) 重启 
                        n-button(text) 修改 
                        n-button(text) 配置文件 
                        //- n-icon(color="#536dfe" size="large" @click="check(data.sk)" :component="SettingsOutline")
                        n-popconfirm(@positive-click="del(data.id)")
                            template(#trigger)
                                n-icon(color="#e84c85" size="large")                        
                                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 32 32"><path d="M12 12h2v12h-2z" fill="currentColor"></path><path d="M18 12h2v12h-2z" fill="currentColor"></path><path d="M4 6v2h2v20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8h2V6zm4 22V8h16v20z" fill="currentColor"></path><path d="M12 2h8v2h-8z" fill="currentColor"></path></svg>
                            span 确认删除？
    n-space(justify="center" style="margin-top:12px")  
        n-pagination(v-model:page="page" :item-count="count" )                       
</template>
    

<script setup>
const data = ref([{
    key: 0,
    name: "nuxt3_naiveui",
    cmd: "node /path/server.mjs",
    pid: "54541",
    priority: "1",
    status: "run", // 运行状态，run|stop|error
    info: "nodejs云渲染服务",
    work_dir: "/opt/rupa",
}, {
    key: 1,
    name: "nuxt3_web",
    cmd: "node /path/server.mjs",
    pid: "54541",
    priority: "1",
    status: "stop", // 运行状态，run|stop|error
    info: "nodejs云渲染服务",
    work_dir: "/opt/rupa",
},])
</script>

<style lang="stylus" scoped>

</style>
