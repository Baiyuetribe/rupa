<template lang="pug">
n-config-provider(:theme-overrides="themeOverrides" :locale="zhCN")
  router-view
</template>
<script setup>
import { zhCN } from 'naive-ui'

const themeOverrides = {
  "common": {

    "textColor1": "#52527b",  //字体颜色-- 常规字体比如icon或lable字体
    "textColor2": "#52527b",  //字体颜色-- 常规字体--lable下的字体
    // 初始色彩
    "primaryColor": "#8b5cf6",
    "primaryColorHover": "#9c74f7",
    "primaryColorPressed": "#5714f2",
    "primaryColorSuppl": "#3c0aaf",
    // success积极颜色
    "successColor": "#22c55e",
    "successColorHover": "#27da69",
    "successColorPressed": "#17843f",
    "successColorSuppl": "#0b4320",
    // error消极颜色
    "errorColor": "#ef4444",
    "errorColorHover": "#f15b5b",
    "errorColorPressed": "#d41212",
    "errorColorSuppl": "#8e0c0c",
    // info信息颜色
    "infoColor": "#3b82f6",
    "infoColorHover": "#5392f7",
    "infoColorPressed": "#0a59da",
    "infoColorSuppl": "#073b91",
    // warning 警告颜色
    "warningColor": "#ff971e",
    "warningColorHover": "#ffa338",
    "warningColorPressed": "#d17000",
    "warningColorSuppl": "#844700",

  },
  "Menu": {
    // 字体颜色
    "itemTextColor": "#52527b",
  },
  "Table": {
    // 字体颜色
    "tdTextColor": "#52527b",
  },
}
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
    margin 1.5rem 1.5rem 3.5rem 1.5rem
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