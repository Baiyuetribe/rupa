import type { App } from "vue";
import moshaToast from "mosha-vue-toastify";
import "mosha-vue-toastify/dist/style.css";

export default (app: App) => app.use(moshaToast);
