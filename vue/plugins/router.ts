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
	// next(); // 如果仅开发前端，可临时注释后面的
	if (typeof to.name == "undefined") {
		next("/login");
	}
	// 判断是否登录
	if (to.path == "/login") {
		next();
	} else {
		if (localStorage.getItem("rupa_token")) {
			if (to.path == "/") {
				next("/dashboard");
			} else {
				next();
			}
		} else {
			next("/login");
		}
	}
});
export default (app: App) => app.use(router);
