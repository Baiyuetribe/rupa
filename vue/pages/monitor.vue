<template lang="pug">
n-card
    n-h4(prefix="bar") 负载均衡
    Line(:option="data" id="load")
n-grid(cols="4" item-responsive responsive="screen" )
    n-gi(span="4 s:4 m:4 l:2 xl:2 2xl:1")
        n-card
            n-h4(prefix="bar") CPU
            Line(:option="data" id="cpu")
    n-gi(span="4 s:4 m:4 l:2 xl:2 2xl:1")
        n-card
            n-h4(prefix="bar") 内存
            Line(:option="data" id="mem")
    n-gi(span="4 s:4 m:4 l:2 xl:2 2xl:1")
        n-card
            n-h4(prefix="bar") 磁盘
            Line(:option="data" id="disk")
    n-gi(span="4 s:4 m:4 l:2 xl:2 2xl:1")
        n-card
            n-h4(prefix="bar") 网络
            Line(:option="data" id="network")            
</template>

<script setup>
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
</script>
    
<style lang="stylus" scoped>
    
</style>
    