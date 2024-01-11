<template lang="pug">
n-layout-header
    n-flex(justify="right" style="padding: 0.5rem;box-shadow: 0 0 10px rgba(0,0,0,.1)")
        router-link(to="/dashboard")
            n-tooltip(placement="bottom-start" trigger="hover" )
                template(#trigger)
                    n-icon(size="24" router-view="dashboard")
                        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" data-v-600e32da=""><rect x="48" y="48" width="176" height="176" rx="20" ry="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32"></rect><rect x="288" y="48" width="176" height="176" rx="20" ry="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32"></rect><rect x="48" y="288" width="176" height="176" rx="20" ry="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32"></rect><rect x="288" y="288" width="176" height="176" rx="20" ry="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32"></rect></svg>
                span 返回首页 
            //- router-link(to="/home") Home
</template>
<script  setup>
// console.log(appStore);

// const languageOptions = [
// { label: '中文', key: 'zh', icon: renderIcon('zh') },
// { label: 'English', key: 'en', icon: renderIcon('en') }
// ]

</script>