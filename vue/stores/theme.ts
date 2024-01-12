import { defineStore } from "pinia";
import type { GlobalTheme, GlobalThemeOverrides } from "naive-ui";
import { darkTheme } from "naive-ui"; // 引入naive-ui的dark主题
export const useAppStore = defineStore("app", {
	state() {
		return {
			curTheme: null as GlobalTheme | null,
			curThemeOverrides: null as GlobalThemeOverrides | null, // 定义全局自定义样式
			lightThemeOverrides: {
				common: {
					primaryColor: "#6851ff",
				},
				Layout: {},
				Menu: {
					fontSize: "1rem",
					itemHeight: "3rem",
				},
			} as GlobalThemeOverrides,
			darkThemeOverrides: {
				common: {
					primaryColor: "#6851ff",
				},
				Menu: {
					fontSize: "1rem",
					itemHeight: "3rem",
				},
			} as GlobalThemeOverrides,
		};
	},
	getters: {
		theme(): GlobalTheme | null {
			return this.curTheme;
		},
		themeOverrides(): GlobalThemeOverrides {
			return this.curThemeOverrides || this.lightThemeOverrides;
		},
	},
	actions: {
		setTheme(theme: GlobalTheme | null) {
			this.curTheme = theme;
		},
		setThemeOverrides(theme: GlobalThemeOverrides) {
			// 同步变更覆盖的主题
			this.curThemeOverrides = theme;
		},
		initTheme() {
			// 初始话主题
			let theme = window.matchMedia("(prefers-color-scheme: dark)").matches
				? "dark"
				: "light"; // 自动主题
			if (theme === "dark") {
				this.setTheme(darkTheme);
				this.setThemeOverrides(this.darkThemeOverrides);
			}
		},
		changeTheme() {
			// 切换相反主题
			if (this.curTheme === null) {
				this.setTheme(darkTheme);
				this.setThemeOverrides(this.darkThemeOverrides);
			} else {
				this.setTheme(null);
				this.setThemeOverrides(this.lightThemeOverrides);
			}
		},
	},
});

// export default useAppStore;
