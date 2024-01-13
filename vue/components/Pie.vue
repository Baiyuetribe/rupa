<template lang="pug">
n-flex(justify="center" align="center" vertical)
    div(:id="id" ref="pie" style="width: 150px; height: 150px;margin-top:-1rem")
    n-text(depth="3" style="margin-top:-1rem") {{ props.footer }}
</template>
<script lang="ts" setup>
import * as echarts from 'echarts';
const appStore = useAppStore();
const props = defineProps({
    id: {
        type: String,
        default: 'pieChartId',
    },
    percent: {
        type: String,
        default: '0',
    },
    name: {
        type: String,
        default: '', // 名称：cpu\内存
    },
    footer: {
        type: String,
        default: '',
    },
});

const initChart = () => {
    let myChart = echarts?.getInstanceByDom(document.getElementById(props.id) as HTMLElement);
    if (myChart === null || myChart === undefined) {
        myChart = echarts.init(document.getElementById(props.id) as HTMLElement);
    }
    const option = {
        title: [
            {
                text: `{t|${props.percent} %}`,
                textStyle: {
                    rich: {
                        t: {
                            fontSize: '22',
                        },
                    },
                    color: '#0f0f0f',
                    lineHeight: 25,
                    // fontSize: 20,
                    fontWeight: 500,
                },
                left: '49%',
                top: '32%',
                subtext: props.name,
                subtextStyle: {
                    color: '#646A73',
                    fontSize: 14,
                },
                textAlign: 'center',
            },
        ],
        polar: {
            radius: ['71%', '80%'],
            center: ['50%', '50%'],
        },
        angleAxis: {
            max: 100,
            show: false,
        },
        radiusAxis: {
            type: 'category',
            show: true,
            axisLabel: {
                show: false,
            },
            axisLine: {
                show: false,
            },
            axisTick: {
                show: false,
            },
        },
        series: [
            {
                type: 'bar',
                roundCap: true,
                barWidth: 30,
                showBackground: true,
                coordinateSystem: 'polar',
                backgroundStyle: {
                    color: 'rgba(0, 94, 235, 0.05)',
                },
                color: [appStore.themeOverrides.common?.primaryColorHover], // 色彩要求略浅
                label: {
                    show: false,
                },
                data: [props.percent],
            },
        ],
    };
    // 渲染数据
    myChart.setOption(option, true);
}

function changeChartSize() {
    echarts.getInstanceByDom(document.getElementById(props.id) as HTMLElement)?.resize();
}

watch(
    () => props.option,
    (val) => {
        if (val) {
            nextTick(() => {
                initChart();
            });
        }
    },
);

onMounted(() => {
    nextTick(() => {
        initChart();
        window.addEventListener('resize', changeChartSize);
    });
});

onBeforeUnmount(() => {
    echarts.getInstanceByDom(document.getElementById(props.id) as HTMLElement).dispose();
    window.removeEventListener('resize', changeChartSize);
});
</script>
<style lang="scss" scoped></style>
