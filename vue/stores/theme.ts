import { defineStore } from "pinia";
import type { GlobalTheme, GlobalThemeOverrides } from "naive-ui";

const useAppStore = defineStore("app", {
	state() {
		return {
			curTheme: null as GlobalTheme | null,
			lightTheme: {
				common: {
					baseColor: "#6851ff",
					primaryColor: "#6851ff",
					primaryColorSuppl: "#316c72",
					primaryColorHover: "#316c72",
					successColorHover: "#316c72",
					successColorSuppl: "#316c72",
				},
				Layout: {
					baseColor: "#316c72",
					primaryColor: "#316c72",
					primaryColorSuppl: "#316c72",
					primaryColorHover: "#316c72",
					successColorHover: "#316c72",
					successColorSuppl: "#316c72",
					siderColor: "#316c72",
					color: "#316c72",
				},
			} as GlobalThemeOverrides,
			darkTheme: {
				common: {},
			} as GlobalThemeOverrides,
		};
	},
	getters: {
		theme(): GlobalTheme | null {
			return this.curTheme;
		},
		lightTheme(): GlobalThemeOverrides {
			return this.lightTheme;
		},
		darkTheme(): GlobalThemeOverrides {
			return this.darkTheme;
		},
	},
	actions: {
		setTheme(theme: GlobalTheme | null) {
			this.curTheme = theme;
		},
		setLightTheme(theme: GlobalThemeOverrides) {
			this.lightTheme = theme;
		},
		setDarkTheme(theme: GlobalThemeOverrides) {
			this.darkTheme = theme;
		},
	},
});

export default useAppStore;
