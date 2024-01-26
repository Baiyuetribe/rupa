<template lang="pug">
n-result(status="404" title="404 资源不存在" description="期待您的创意")
    template(#footer)
        n-button(type="primary" @click="() => $router.push('/dashboard')") 返回首页

    n-flex(justify="center")
        n-text CC防护、以及类似fail2ban的设计+进程守护
</template>

<script setup>

</script>

<style lang="stylus" scoped>

</style>
