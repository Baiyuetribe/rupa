import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
// 自动加载vue等API
import AutoImport from "unplugin-auto-import/vite";
import { dirResolver, DirResolverHelper } from "vite-auto-import-resolvers";
// 组件全自动按需引入
import Components from "unplugin-vue-components/vite"; // 组件文件夹无法自动引入
import {
	NaiveUiResolver,
	VueUseComponentsResolver,
} from "unplugin-vue-components/resolvers";
// 下面两个为组件优化；虽然 Vite 可以智能地检测动态依赖关系，但它的按需自然有时会使复杂项目的启动变得非常缓慢。
import OptimizationPersist from "vite-plugin-optimize-persist";
import PkgConfig from "vite-plugin-package-config";
// 页面路由自动引入
import Pages from "vite-plugin-pages";
// Layout 全自动引入
import Layouts from "vite-plugin-vue-layouts"; // 依赖插件

// import Inspect from 'vite-plugin-inspect'

// https://vitejs.dev/config/
export default defineConfig({
	build: {
		outDir: "../.vue",
		emptyOutDir: true,
	},
	server: {
		proxy: {
			"/api": {
				target: "http://127.0.0.1:3399",
				changeOrigin: true,
			},
		},
	},
	plugins: [
		vue(),
		DirResolverHelper(),
		AutoImport({
			dirs: ["./composables/**/*.ts", "./http/*.js"],
			dts: "./stores/auto-imports.d.ts",
			injectAtEnd: false,
			vueTemplate: true,
			imports: [
				"vue",
				"pinia",
				{
					"vue-router": [
						"createRouter",
						"createWebHistory",
						"useRouter",
						"useRoute",
					],
				},
			],
		}),
		Components({
			dirs: ["components"], // 指定组件文件夹
			resolvers: [NaiveUiResolver(), VueUseComponentsResolver()],
			dts: "./stores/components.d.ts",
		}),
		// 插件缓存，非常有效果，除首次加载外，速度极快
		PkgConfig(),
		OptimizationPersist(),
		// 文件路由
		Pages({
			dirs: "pages",
		}),
		// Layout布局
		Layouts({ layoutsDirs: "layouts", defaultLayout: "default" }),

		// Inspect(),
	],
});
