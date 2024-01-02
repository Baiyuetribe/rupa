import { App } from "vue";
// @ts-ignore
import generatedRoutes from "virtual:generated-pages";
// @ts-ignore
import { setupLayouts } from "virtual:generated-layouts";
import { createRouter, createWebHashHistory } from "vue-router";
// createWebHistory跟go后端共用时，存在直接请求导致404的问题
const routes = setupLayouts(generatedRoutes);

export const router = createRouter({
	routes,
	history: createWebHashHistory(),
});
// 路由守卫
router.beforeEach(async (to, from, next) => {
	document.title = "RuPa管理面板";
	if (typeof to.name == "undefined") {
		// console.log("未知路由");
		next("/login");
	}
	// 判断是否登录
	if (to.path == "/admin" || to.path == "/login") {
		next();
	} else {
		if (localStorage.getItem("admin_token")) {
			next();
		} else {
			next("/login");
		}
	}
});
export default (app: App) => app.use(router);
