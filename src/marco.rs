// 统一输出日志
macro_rules! log {
    ($($arg:tt)*) => ({
        // println!("{}:{}: {}", file!(), line!(), format!($($arg)*)); // 简易，无时间戳
        let local: chrono::DateTime<chrono::Local> = chrono::Local::now(); // 获取当前的UTC时间
        let formatted_time = local.format("%Y/%m/%d %H:%M:%S").to_string();
        // println!("{} {}:{}: {}",formatted_time, file!(), line!(), format!($($arg)*)); // 带时间戳
        let formatted_time = colored::Colorize::green(formatted_time.as_str());
        println!("{} {}:{}: {}",formatted_time, file!(), line!(), format!($($arg)*)); // 带时间戳+彩色
    });
}

// 由于宏的特殊性，main.rs作为主要入口，必须添加如下代码：
// #[macro_use]
// mod marco;
// 才能在src子文件夹里调用该宏
