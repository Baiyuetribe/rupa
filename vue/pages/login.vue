<template lang="pug">
n-grid.bg4(cols="24" item-responsive responsive="screen")
    n-gi(span="0 s:2 m:3 l:5 xl:6 2xl:6")
    n-gi.auth(span="24 s:20 m:18 l:14 xl:12 2xl:12")
        n-flex(justify="center" style="padding-top:20px" vertical align="center")
            n-icon(size="40" color="white")
                <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path d="M56.79 200.58l12.36 7.5l7.35-13.58C93.07 166.75 143.78 102 256 102c115 0 164 70.32 180.1 93.46l8.16 12.7L469.88 192l-8.54-13.36c-8.88-12.85-27.52-39.53-60.78-63.1C360.15 86.82 311.5 72.25 256 72.25c-128.07 0-186.69 75.11-206 107.25L42.63 192L54 198.86a14.09 14.09 0 0 0 1.63 1.1a12.57 12.57 0 0 0 1.16.62z" fill="currentColor"></path><path d="M379.22 172.32c-35.54-28.93-78.12-44.25-123.22-44.25c-97.52 0-162.31 66-183.33 131.47C53.42 320 76.82 407.61 77.8 411.36l4.38 13.81l29.93-6.43l-4.74-15c-.21-.75-22.1-82.93-5.41-135.21c9-28.08 27.73-55.4 51.35-74.79C181.81 170.39 217.35 158 256 158c90.58 0 141.93 70.61 156.45 108.11c11.27 28.93 8.67 61.82-6.28 82c-5.53 7.39-15.28 16.07-30.12 15.32c-33.81-1.72-39.66-18.43-47.79-50.25c-3.9-15.32-7.9-31.18-17.87-44c-12.14-15.75-29.8-23.36-54.28-23.36c-26.33 0-46.27 8.68-59.38 25.72c-28.6 37.28-10 100.93-9.21 103.61l.22.85c1.41 3.86 36.08 96.65 128.93 119.68l14.77 3.21l8.09-28.71l-15.27-3.43c-74.22-18.43-105.21-94.39-107.59-100.39a152.44 152.44 0 0 1-5.1-29.79c-1.08-14.46-.32-34.39 9.43-47.14c7.15-9.32 18.64-13.82 35-13.82c29.79 0 34.78 14.57 42.58 44.79c7.58 29.46 18 69.85 75.84 72.75c22.21 1.07 42.26-8.79 56.34-27.65c21.13-28.28 25.14-71.57 10.19-110.14c-11.68-30.36-34.21-60.54-61.73-83.04z" fill="currentColor"></path><path d="M154.18 343.21c-3.47-28.28 1.41-71 26.55-98.78c17.44-19.29 42.79-29 75.19-29c37.49 0 65.87 16.72 84.51 49.61a154 154 0 0 1 17.88 53.25l1.43 14.69l30-2.2a112.63 112.63 0 0 0-1-15.6c-.11-1.28-3.57-32.46-21-63.75c-24.06-43.11-62.63-65.93-111.74-65.93c-41.5 0-74.55 13.18-98.06 39.11c-31.85 35.14-38.35 86.25-33.91 122.35v.33c7.97 54.53 28.97 98.14 66.12 137.14l11.6 11.22l20.95-21.79l-10.34-9.79c-32.72-34.28-51.25-72.64-58.18-120.86z" fill="currentColor"></path><path d="M132.47 72.66c11.08-6.72 50.27-26.77 123.53-26.77c87.54 0 126.44 28.72 126.87 28.93l13.9 8.86L413 58.47l-13.22-8.56c-.52-.38-1.06-.76-1.6-1.12C385.5 40.54 340.54 16 256 16c-87.71 0-132.75 26.48-143.41 33.71L99 58.52l16.2 25.21z" fill="currentColor"></path><path d="M390.59 415.21c-33.37 3.75-60.45-2.67-80.71-18.85c-34.24-27.43-38.68-75.11-38.79-76l-1.23-14.88l-30.53 2.23l1.31 15c.22 2.46 5.2 60.75 49.62 96.54c22.11 17.89 49.74 26.89 82.24 26.89a187 187 0 0 0 21.56-1.29l16.59-2.09l-6.1-29.71z" fill="currentColor"></path></svg>
            h2(style="color:white") RuPa面板
        n-flex(justify="center")
            n-form(label-placement="left" size="large")
                n-form-item
                    n-input(placeholder="请输入账号" v-model:value="state.login.user")
                n-form-item
                    n-input(placeholder="请输入密码" type="password" v-model:value="state.login.password" show-password-on="mousedown")
                n-form-item
                    n-input(placeholder="验证码" style="width: 70%" v-model:value="state.chaptcha")
                    img(:src="state.bs64" alt="点击此处刷新验证码" style="width:30%;border-bottom-right-radius: 3px;border-top-right-radius: 3px;" height="40" @click="getCaptcha")
        n-flex(justify="center" style="padding-bottom: 40px;")
            n-button(type="primary" size="large" style="padding:0 40px;box-shadow:none" @click="login") 登录
    n-gi(span="0 s:2 m:3 l:5 xl:6 2xl:6")
//- 背景特效
div
    .star0
    .star1
    .star2  
    .star3  
</template>

<script setup>
// const router = useRouter() // 用于跳转
const state = reactive({
    uuid: "",
    bs64: null,
    chaptcha: "",
    login: { user: "admin", password: "123456" }, // 默认admin 123456
})

const getCaptcha = async () => {
    const { data } = await useFetch('/api/captcha').json() // 不同于nuxt3,此处需要手动指定json()或bolb()，默认为text()
    if (data.value?.status == 200) {
        state.uuid = data.value.uuid
        state.bs64 = data.value.data
    } else {
        toast("获取验证码失败")
    }
}

const login = () => {
    if (!state.login.user) { toast("请输入账号"); return; }
    if (!state.login.password) { toast("请输入密码"); return; }
    if (!state.login.chaptcha) { toast("请输入验证码"); return; }
}

onMounted(() => {
    getCaptcha()
})
</script>


<style lang="stylus" scoped>
.bg4
    // background-image: radial-gradient(circle at 10% 20%, rgb(90, 92, 106) 0%, rgb(32, 45, 58) 81.3%);
    min-height: 100vh
    background: #181828
.auth
    z-index 100
    max-width: 800px
    margin: 0 auto
    padding: 0 40px    
    background: #fff
    max-height: 420px
    // background linear-gradient(90deg, #9600fb 0%, #00ffd0 100%);
    // background-image: linear-gradient(120deg, #e0c3fc 0%, #8ec5fc 100%);
    background-image: radial-gradient(circle at 10% 20%, rgb(90, 92, 106) 0%, rgb(32, 45, 58) 81.3%);
    // background-image: radial-gradient(circle at 10% 20%, rgb(14, 174, 87) 0%, rgb(12, 116, 117) 90%);
    // background: linear-gradient(-50deg, #F7A37B, #F7A37B, #FFDEA8, #FFDEA8, #D0E4B0, #D0E4B0, #7CC5D0, #7CC5D0, #00A2E1, #00A2E1, #0085C8, #0085C8);
    border-radius: 8px
    box-shadow: 0 0 10px rgba(0,0,0,.1)
    // vertical-align: middle
    // display inline-block
    top:calc(25%);
    position relative
    .auth-title
        font-size: 24px
        font-weight: bold
        margin-bottom: 20px
    
    // padding 25% 0
    // background-color: #fff;


.star0
{
    content: " ";
    position: absolute;
    width: 4px;
    height: 4px;
    background: transparent;
    box-shadow: 571px 173px #00BCD4, 1732px 143px #00BCD4, 1745px 454px #FF5722, 234px 784px #00BCD4, 1793px 1123px #FF9800, 1076px 504px #03A9F4, 633px 601px #FF5722, 350px 630px #FFEB3B, 1164px 782px #00BCD4, 76px 690px #3F51B5, 1825px 701px #CDDC39, 1646px 578px #FFEB3B, 544px 293px #2196F3, 445px 1061px #673AB7, 928px 47px #00BCD4, 168px 1410px #8BC34A, 777px 782px #9C27B0, 1235px 1941px #9C27B0, 104px 1690px #8BC34A, 1167px 1338px #E91E63, 345px 1652px #009688, 1682px 1196px #F44336, 1995px 494px #8BC34A, 428px 798px #FF5722, 340px 1623px #F44336, 605px 349px #9C27B0, 1339px 1344px #673AB7, 1102px 1745px #3F51B5, 1592px 1676px #2196F3, 419px 1024px #FF9800, 630px 1033px #4CAF50, 1995px 1644px #00BCD4, 1092px 712px #9C27B0, 1355px 606px #F44336, 622px 1881px #CDDC39, 1481px 621px #9E9E9E, 19px 1348px #8BC34A, 864px 1780px #E91E63, 442px 1136px #2196F3, 67px 712px #FF5722, 89px 1406px #F44336, 275px 321px #009688, 592px 630px #E91E63, 1012px 1690px #9C27B0, 1749px 23px #673AB7, 94px 1542px #FFEB3B, 1201px 1657px #3F51B5, 1505px 692px #2196F3, 1799px 601px #03A9F4, 656px 811px #00BCD4, 701px 597px #00BCD4, 1202px 46px #FF5722, 890px 569px #FF5722, 1613px 813px #2196F3, 223px 252px #FF9800, 983px 1093px #F44336, 726px 1029px #FFC107, 1764px 778px #CDDC39, 622px 1643px #F44336, 174px 1559px #673AB7, 212px 517px #00BCD4, 340px 505px #FFF, 1700px 39px #FFF, 1768px 516px #F44336, 849px 391px #FF9800, 228px 1824px #FFF, 1119px 1680px #FFC107, 812px 1480px #3F51B5, 1438px 1585px #CDDC39, 137px 1397px #FFF, 1080px 456px #673AB7, 1208px 1437px #03A9F4, 857px 281px #F44336, 1254px 1306px #CDDC39, 987px 990px #4CAF50, 1655px 911px #00BCD4, 1102px 1216px #FF5722, 1807px 1044px #FFF, 660px 435px #03A9F4, 299px 678px #4CAF50, 1193px 115px #FF9800, 918px 290px #CDDC39, 1447px 1422px #FFEB3B, 91px 1273px #9C27B0, 108px 223px #FFEB3B, 146px 754px #00BCD4, 461px 1446px #FF5722, 1004px 391px #673AB7, 1529px 516px #F44336, 1206px 845px #CDDC39, 347px 583px #009688, 1102px 1332px #F44336, 709px 1756px #00BCD4, 1972px 248px #FFF, 1669px 1344px #FF5722, 1132px 406px #F44336, 320px 1076px #CDDC39, 126px 943px #FFEB3B, 263px 604px #FF5722, 1546px 692px #F44336;
    animation: animStar 150s linear infinite;
}

.star1
{
    content: " ";
    position: absolute;
    width: 4px;
    height: 4px;
    background: transparent;
    box-shadow: 571px 173px #00BCD4, 1732px 143px #00BCD4, 1745px 454px #FF5722, 234px 784px #00BCD4, 1793px 1123px #FF9800, 1076px 504px #03A9F4, 633px 601px #FF5722, 350px 630px #FFEB3B, 1164px 782px #00BCD4, 76px 690px #3F51B5, 1825px 701px #CDDC39, 1646px 578px #FFEB3B, 544px 293px #2196F3, 445px 1061px #673AB7, 928px 47px #00BCD4, 168px 1410px #8BC34A, 777px 782px #9C27B0, 1235px 1941px #9C27B0, 104px 1690px #8BC34A, 1167px 1338px #E91E63, 345px 1652px #009688, 1682px 1196px #F44336, 1995px 494px #8BC34A, 428px 798px #FF5722, 340px 1623px #F44336, 605px 349px #9C27B0, 1339px 1344px #673AB7, 1102px 1745px #3F51B5, 1592px 1676px #2196F3, 419px 1024px #FF9800, 630px 1033px #4CAF50, 1995px 1644px #00BCD4, 1092px 712px #9C27B0, 1355px 606px #F44336, 622px 1881px #CDDC39, 1481px 621px #9E9E9E, 19px 1348px #8BC34A, 864px 1780px #E91E63, 442px 1136px #2196F3, 67px 712px #FF5722, 89px 1406px #F44336, 275px 321px #009688, 592px 630px #E91E63, 1012px 1690px #9C27B0, 1749px 23px #673AB7, 94px 1542px #FFEB3B, 1201px 1657px #3F51B5, 1505px 692px #2196F3, 1799px 601px #03A9F4, 656px 811px #00BCD4, 701px 597px #00BCD4, 1202px 46px #FF5722, 890px 569px #FF5722, 1613px 813px #2196F3, 223px 252px #FF9800, 983px 1093px #F44336, 726px 1029px #FFC107, 1764px 778px #CDDC39, 622px 1643px #F44336, 174px 1559px #673AB7, 212px 517px #00BCD4, 340px 505px #FFF, 1700px 39px #FFF, 1768px 516px #F44336, 849px 391px #FF9800, 228px 1824px #FFF, 1119px 1680px #FFC107, 812px 1480px #3F51B5, 1438px 1585px #CDDC39, 137px 1397px #FFF, 1080px 456px #673AB7, 1208px 1437px #03A9F4, 857px 281px #F44336, 1254px 1306px #CDDC39, 987px 990px #4CAF50, 1655px 911px #00BCD4, 1102px 1216px #FF5722, 1807px 1044px #FFF, 660px 435px #03A9F4, 299px 678px #4CAF50, 1193px 115px #FF9800, 918px 290px #CDDC39, 1447px 1422px #FFEB3B, 91px 1273px #9C27B0, 108px 223px #FFEB3B, 146px 754px #00BCD4, 461px 1446px #FF5722, 1004px 391px #673AB7, 1529px 516px #F44336, 1206px 845px #CDDC39, 347px 583px #009688, 1102px 1332px #F44336, 709px 1756px #00BCD4, 1972px 248px #FFF, 1669px 1344px #FF5722, 1132px 406px #F44336, 320px 1076px #CDDC39, 126px 943px #FFEB3B, 263px 604px #FF5722, 1546px 692px #F44336;
    animation: animStar 10s linear infinite;
}

.star2
{
    content: " ";
    position: absolute;
    width: 2px;
    height: 2px;
    background: transparent;
    box-shadow: 571px 173px #00BCD4, 1732px 143px #00BCD4, 1745px 454px #FF5722, 234px 784px #00BCD4, 1793px 1123px #FF9800, 1076px 504px #03A9F4, 633px 601px #FF5722, 350px 630px #FFEB3B, 1164px 782px #00BCD4, 76px 690px #3F51B5, 1825px 701px #CDDC39, 1646px 578px #FFEB3B, 544px 293px #2196F3, 445px 1061px #673AB7, 928px 47px #00BCD4, 168px 1410px #8BC34A, 777px 782px #9C27B0, 1235px 1941px #9C27B0, 104px 1690px #8BC34A, 1167px 1338px #E91E63, 345px 1652px #009688, 1682px 1196px #F44336, 1995px 494px #8BC34A, 428px 798px #FF5722, 340px 1623px #F44336, 605px 349px #9C27B0, 1339px 1344px #673AB7, 1102px 1745px #3F51B5, 1592px 1676px #2196F3, 419px 1024px #FF9800, 630px 1033px #4CAF50, 1995px 1644px #00BCD4, 1092px 712px #9C27B0, 1355px 606px #F44336, 622px 1881px #CDDC39, 1481px 621px #9E9E9E, 19px 1348px #8BC34A, 864px 1780px #E91E63, 442px 1136px #2196F3, 67px 712px #FF5722, 89px 1406px #F44336, 275px 321px #009688, 592px 630px #E91E63, 1012px 1690px #9C27B0, 1749px 23px #673AB7, 94px 1542px #FFEB3B, 1201px 1657px #3F51B5, 1505px 692px #2196F3, 1799px 601px #03A9F4, 656px 811px #00BCD4, 701px 597px #00BCD4, 1202px 46px #FF5722, 890px 569px #FF5722, 1613px 813px #2196F3, 223px 252px #FF9800, 983px 1093px #F44336, 726px 1029px #FFC107, 1764px 778px #CDDC39, 622px 1643px #F44336, 174px 1559px #673AB7, 212px 517px #00BCD4, 340px 505px #FFF, 1700px 39px #FFF, 1768px 516px #F44336, 849px 391px #FF9800, 228px 1824px #FFF, 1119px 1680px #FFC107, 812px 1480px #3F51B5, 1438px 1585px #CDDC39, 137px 1397px #FFF, 1080px 456px #673AB7, 1208px 1437px #03A9F4, 857px 281px #F44336, 1254px 1306px #CDDC39, 987px 990px #4CAF50, 1655px 911px #00BCD4, 1102px 1216px #FF5722, 1807px 1044px #FFF, 660px 435px #03A9F4, 299px 678px #4CAF50, 1193px 115px #FF9800, 918px 290px #CDDC39, 1447px 1422px #FFEB3B, 91px 1273px #9C27B0, 108px 223px #FFEB3B, 146px 754px #00BCD4, 461px 1446px #FF5722, 1004px 391px #673AB7, 1529px 516px #F44336, 1206px 845px #CDDC39, 347px 583px #009688, 1102px 1332px #F44336, 709px 1756px #00BCD4, 1972px 248px #FFF, 1669px 1344px #FF5722, 1132px 406px #F44336, 320px 1076px #CDDC39, 126px 943px #FFEB3B, 263px 604px #FF5722, 1546px 692px #F44336;
    animation: animStar 50s linear infinite;
}

.star3
{
    content: " ";
    position: absolute;
    width: 1px;
    height: 1px;
    background: transparent;
    box-shadow: 571px 173px #00BCD4, 1732px 143px #00BCD4, 1745px 454px #FF5722, 234px 784px #00BCD4, 1793px 1123px #FF9800, 1076px 504px #03A9F4, 633px 601px #FF5722, 350px 630px #FFEB3B, 1164px 782px #00BCD4, 76px 690px #3F51B5, 1825px 701px #CDDC39, 1646px 578px #FFEB3B, 544px 293px #2196F3, 445px 1061px #673AB7, 928px 47px #00BCD4, 168px 1410px #8BC34A, 777px 782px #9C27B0, 1235px 1941px #9C27B0, 104px 1690px #8BC34A, 1167px 1338px #E91E63, 345px 1652px #009688, 1682px 1196px #F44336, 1995px 494px #8BC34A, 428px 798px #FF5722, 340px 1623px #F44336, 605px 349px #9C27B0, 1339px 1344px #673AB7, 1102px 1745px #3F51B5, 1592px 1676px #2196F3, 419px 1024px #FF9800, 630px 1033px #4CAF50, 1995px 1644px #00BCD4, 1092px 712px #9C27B0, 1355px 606px #F44336, 622px 1881px #CDDC39, 1481px 621px #9E9E9E, 19px 1348px #8BC34A, 864px 1780px #E91E63, 442px 1136px #2196F3, 67px 712px #FF5722, 89px 1406px #F44336, 275px 321px #009688, 592px 630px #E91E63, 1012px 1690px #9C27B0, 1749px 23px #673AB7, 94px 1542px #FFEB3B, 1201px 1657px #3F51B5, 1505px 692px #2196F3, 1799px 601px #03A9F4, 656px 811px #00BCD4, 701px 597px #00BCD4, 1202px 46px #FF5722, 890px 569px #FF5722, 1613px 813px #2196F3, 223px 252px #FF9800, 983px 1093px #F44336, 726px 1029px #FFC107, 1764px 778px #CDDC39, 622px 1643px #F44336, 174px 1559px #673AB7, 212px 517px #00BCD4, 340px 505px #FFF, 1700px 39px #FFF, 1768px 516px #F44336, 849px 391px #FF9800, 228px 1824px #FFF, 1119px 1680px #FFC107, 812px 1480px #3F51B5, 1438px 1585px #CDDC39, 137px 1397px #FFF, 1080px 456px #673AB7, 1208px 1437px #03A9F4, 857px 281px #F44336, 1254px 1306px #CDDC39, 987px 990px #4CAF50, 1655px 911px #00BCD4, 1102px 1216px #FF5722, 1807px 1044px #FFF, 660px 435px #03A9F4, 299px 678px #4CAF50, 1193px 115px #FF9800, 918px 290px #CDDC39, 1447px 1422px #FFEB3B, 91px 1273px #9C27B0, 108px 223px #FFEB3B, 146px 754px #00BCD4, 461px 1446px #FF5722, 1004px 391px #673AB7, 1529px 516px #F44336, 1206px 845px #CDDC39, 347px 583px #009688, 1102px 1332px #F44336, 709px 1756px #00BCD4, 1972px 248px #FFF, 1669px 1344px #FF5722, 1132px 406px #F44336, 320px 1076px #CDDC39, 126px 943px #FFEB3B, 263px 604px #FF5722, 1546px 692px #F44336;
    animation: animStar 80s linear infinite;
}

@keyframes animStar
{
    0% {
        transform: translateY(0px);
    }
    100% {
        transform: translateY(-2000px);
    }
}



</style>
<route lang="yaml">
    meta:
        layout: auth
</route>