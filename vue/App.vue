<template lang="pug">
n-config-provider(:theme="appStore.theme" :theme-overrides="appStore.themeOverrides" :locale="zhCN")
  router-view
</template>
<script setup>
import { zhCN } from 'naive-ui'
import { useAppStore } from './stores/theme'
const appStore = useAppStore()

</script>

<style lang="stylus">
@import url(mosha-vue-toastify/dist/style.css); /* 引入原有的样式 */
body {
width: 100%;
height: 100%;
margin: 0;
padding: 0;
// font-family: monospace,sans-serif;
background-color: transparent;
}
.fade-enter-active,.fade-leave-active
  transition: opacity 0.3s ease;
.fade-enter-from,.fade-leave-to 
  opacity: 0;

.child-view {
  // position: absolute;
  transition: all .75s cubic-bezier(.55,0,.1,1);
}
.slide-left-enter, .slide-right-leave-active {
  opacity: 0;
  -webkit-transform: translate(30px, 0);
  transform: translate(30px, 0);
}
.slide-left-leave-active, .slide-right-enter {
  opacity: 0;
  -webkit-transform: translate(-30px, 0);
  transform: translate(-30px, 0);
}

a{
    text-decoration: none;
    color: #52527b;
}
.card{
    background: #ffffffba;
    border-radius: 1rem;
    padding: 1rem;
    box-shadow: rgba(0, 0, 0, 0.03) 0px 4px 8px 0px;
    // margin 1.5rem 1.5rem 3.5rem 1.5rem
    margin 1.5rem
    /* transition: all 0.3s ease-in-out; */
}
.card-title{
    font-size: 22px;
    border-left: 0.5rem solid #8b5cf6;
    padding-left:8px;
    color: #52527b;
    line-height: 1.4;
}
.n-button{
    box-shadow: 0 2px 6px #b5b3b3;
}
// 修正toast的部分样式
.mosha__toast__content__text
  font-size 14px
  font-weight 400
  color #000
  line-height 1.5
  padding-left 20px


</style>