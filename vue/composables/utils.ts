// const isDark = useDark()
// const toggleDark = useToggle(isDark)

// 格式化日期
export const formatDate = (dateString: string) => {
	const date = new Date(dateString);
	const year = date.getFullYear();
	const month = ("0" + (date.getMonth() + 1)).slice(-2);
	const day = ("0" + date.getDate()).slice(-2);
	const hours = ("0" + date.getHours()).slice(-2);
	const minutes = ("0" + date.getMinutes()).slice(-2);
	const seconds = ("0" + date.getSeconds()).slice(-2);
	return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
};

// 生成随机字符串
export const randSalt = (length: number) => {
	const chars =
		"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let salt = "";
	for (let i = 0; i < length; i++) {
		const index = Math.floor(Math.random() * chars.length);
		salt += chars.charAt(index);
	}
	return salt;
};
// 通知函数，独立的css放在全局进行引用
import { createToast as toast } from "mosha-vue-toastify";
// export const toast = createToast; // 对外调用 方法1
export { toast }; // 对外调用 方法2
