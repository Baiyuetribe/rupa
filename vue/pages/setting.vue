<template lang="pug">
n-card
    n-h4(prefix="bar") 通用设置
    n-form(label-placement="left" label-width="auto")
        n-form-item(label="管理账号")
            n-input-group
                n-input(v-model:value="username")
                n-button(type="primary" ghost) 修改
        n-form-item(label="管理密码")
            n-input-group
                n-input(v-model:value="password")
                n-button(type="primary" ghost) 修改
        n-form-item(label="主题")
            n-radio(value="light" v-model:value="color" :disabled="true") 暗色
            n-radio(value="dark") 亮色
            n-radio(value="dark") 跟随系统
        n-form-item(label="主题色" name="radiogroup")
            n-radio-group(v-model:value="color")
                n-space 
                    n-radio(v-for="i in color_data" :key="i.id" :value="i") {{ i }}
       
</template>

<script setup>
const username = ref("admin")
const password = ref("admin123")
const color = ref("红")
const color_data = ref(["红", "蓝", "绿", "黄", "紫", "橙"])
</script>

<style lang="stylus" scoped>

</style>
