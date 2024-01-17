<template lang="pug">
n-card
    n-h4(prefix="bar") 端口
    n-flex
        n-text 防火墙
            n-switch(:round="false" :value="true" style="margin-left:0.5rem" )
        n-text 禁ping
            n-switch(:round="false" style="margin-left:0.5rem")     
    n-divider       
    n-tabs(animated type="line" trigger="hover")
        n-tab-pane(name="0" tab="端口规则")
            n-flex
                n-button(size="small") 添加
                n-button(size="small") 导入
                n-button(size="small") 导出
            n-table(:bordered="true" :single-line="false" striped style="text-align:center;margin-top:1rem").table-striped
                thead
                    tr
                        th 协议
                        th 端口
                        th 状态
                        th 策略
                        th 来源
                        th 备注
                        th 时间
                        th 操作
                tbody
                    tr(v-for="(v,index) in data" :key="data.id")
                        td {{ v.protocol }} 
                        td {{ v.prot }} 
                        td {{ v.status==0?'正常':'待定' }} 
                        td {{ v.type }} 
                        td {{ v.address==""?'所有IP':v.address }} 
                        td {{ v.info }} 
                        td {{ v.time }} 
                        td
                            n-flex
                                n-icon(color="#536dfe" size="large")
                                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 20 20"><g fill="none"><path d="M13.245 2.817a2.783 2.783 0 0 1 4.066 3.796l-.13.14l-9.606 9.606a2.001 2.001 0 0 1-.723.462l-.165.053l-4.055 1.106a.5.5 0 0 1-.63-.535l.016-.08l1.106-4.054c.076-.28.212-.54.398-.76l.117-.128l9.606-9.606zm-.86 2.275L4.346 13.13a1 1 0 0 0-.215.321l-.042.123l-.877 3.21l3.212-.875a1 1 0 0 0 .239-.1l.107-.072l.098-.085l8.038-8.04l-2.521-2.52zm4.089-1.568a1.783 1.783 0 0 0-2.402-.11l-.12.11l-.86.86l2.52 2.522l.862-.86a1.783 1.783 0 0 0 .11-2.402l-.11-.12z" fill="currentColor"></path></g></svg>
                                n-icon(color="#536dfe" size="large" @click="check(data.sk)" :component="SettingsOutline")
                                    n-popconfirm(@positive-click="del(data.id)")
                                        template(#trigger)
                                            n-icon(color="#e84c85" size="large")                        
                                                <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 32 32"><path d="M12 12h2v12h-2z" fill="currentColor"></path><path d="M18 12h2v12h-2z" fill="currentColor"></path><path d="M4 6v2h2v20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8h2V6zm4 22V8h16v20z" fill="currentColor"></path><path d="M12 2h8v2h-8z" fill="currentColor"></path></svg>
                                        span 确认删除？
            n-space(justify="center" style="margin-top:12px")  
                n-pagination(v-model:page="page" :item-count="count" )    
        n-tab-pane(name="1" tab="IP规则")
            n-text 概念设计中
        n-tab-pane(name="2" tab="端口转发")
            n-text 概念设计中
        n-tab-pane(name="3" tab="地区规则")
            n-text 概念设计中                        
</template>

<script setup>
const data = ref([{
    "id": 1,
    "protocol": "tcp/udp",
    "prot": "80",
    "type": "accept",
    "address": "",
    "info": "\u7f51\u7ad9\u9ed8\u8ba4\u7aef\u53e3",
    "time": "0000-00-00 00:00:00",
    "status": 0
},
{
    "id": 2,
    "protocol": "tcp/udp",
    "prot": "8888",
    "type": "accept",
    "address": "",
    "info": "WEB\u9762\u677f",
    "time": "0000-00-00 00:00:00",
    "status": 0
}])
</script>

<style lang="stylus" scoped>

</style>
