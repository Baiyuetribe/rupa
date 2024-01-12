<template lang="pug">
n-grid(cols="4" item-responsive responsive="screen" x-gap="16" y-gap="4" style="margin:1.5rem;width:unset")
    n-gi(span="4 s:2 m:1 l:1 xl:1 2xl:1").card2.linear-red
        n-space(justify="center" align="center")
            n-icon(size="42" color="white" :component="EarthOutline")
            .count
                n-number-animation(:from="0" :to="123" show-separator)
                .title 网站
    n-gi(span="4 s:2 m:1 l:1 xl:1 2xl:1").card2.linear-purple
        n-space(justify="center" align="center")
            n-icon(size="42" color="white" :component="ServerOutline")
            .count
                n-number-animation(:from="0" :to="123" show-separator)
                .title 数据库
    n-gi(span="4 s:2 m:1 l:1 xl:1 2xl:1").card2.linear-blue
        n-space(justify="center" align="center")
            n-icon(size="42" color="white" :component="TimeOutline")
            .count
                n-number-animation(:from="0" :to="123" show-separator)
                .title 计划任务
    n-gi(span="4 s:2 m:1 l:1 xl:1 2xl:1").card2.linear-orange
        n-space(justify="center" align="center")
            n-icon(size="42" color="white" :component="HardwareChipOutline")
            .count
                n-number-animation(:from="0" :to="123" show-separator)
                .title 端口   
.card
    n-h4(prefix="bar") 状态
    n-flex(justify="space-evenly")
        Pie(id="load" percent='20' name="负载" footer='流畅运行')
        Pie(id="cpu" percent='20' name="cpu" footer='流畅运行')
        Pie(id="mem" percent='20' name="内存" footer='流畅运行')
        Pie(id="disk" percent='20' name="磁盘" footer='流畅运行')
        n-divider(vertical style="height:140px")
        n-flex(vertical size="large" ) 
            n-text 操作系统： Linux_x86_64
            n-text CPU信息： XXXX
            n-text 开机时间： 2023-05-16 12:00:11
            n-text 稳定运行 4545 天
.card
    //- 左侧为实时监控，右侧为CPU占用信息；前7个值
    n-h4(prefix="bar") 监控
    n-grid(cols="2" item-responsive responsive="screen" x-gap="16" y-gap="2")
        n-gi(span="2 s:2 m:1 l:1 xl:1 2xl:1")
            n-p 网络
            Line(:option="data")
        n-gi(span="2 s:2 m:1 l:1 xl:1 2xl:1")
            n-p 实时监控
            n-data-table(:columns="columns" :data="data2") 

</template>

<script setup>
import {
    BarChartOutline, TimeOutline, ServerOutline, EarthOutline, ShieldCheckmarkOutline, FolderOutline, AppsOutline, HardwareChipOutline, CardOutline, SettingsOutline, EaselOutline, ExtensionPuzzleOutline, LogOutOutline,
} from '@vicons/ionicons5'
const data = ref({
    xDatas: [1, 2, 3, 4, 5, 6],
    yDatas: [
        {
            name: "网络",
            data: [1, 4, 3, 4, 2, 3],
        },
    ],
    formatStr: 'KB/s',
})
const columns = ref([{
    title: '进程名',
    key: 'name',
}, {
    title: 'CPU(%)',
    key: 'cpu',
},
{
    title: '内存(Mb)',
    key: 'mem',
}, {
    title: '命令',
    key: 'command',
},
])
const data2 = ref([])
</script>

<style lang="stylus" scoped>
.card
    overflow: auto
.count
    font-size: 24px
    font-weight: bold
    color: #fff
    .title
        font-size: 12px
        font-weight: normal
        color: #f4f6f9
.card2
    background: #ffffffba;
    border-radius: 1rem;
    padding: 1.5rem;
    box-shadow: rgba(0, 0, 0, 0.03) 0px 4px 8px 0px;
.linear-red
    background-image: linear-gradient(to bottom right, #ec4786, #b955a4)
    box-shadow: 0 2px 6px #b5b3b3;
.linear-purple
    background-image: linear-gradient(to bottom right, #865ec0, #5144b4)
    box-shadow: 0 2px 6px #b5b3b3;
.linear-blue
    background-image: linear-gradient(to bottom right, #56cdf3, #719de3)
    box-shadow: 0 2px 6px #b5b3b3;
.linear-orange
    background-image: linear-gradient(to bottom right, #fcbc25, #f68057)
    box-shadow: 0 2px 6px #b5b3b3;
</style>
