use std::{
	net::SocketAddr,
	sync::{Arc, Mutex, PoisonError},
};

#[macro_use]
mod marco; // 特殊搭配，使得全局有效

mod handle; // 引入库--也就是子文件夹
mod model; // 引入库--也就是子文件夹

mod config;
mod cron; // 定时任务
mod db; // 数据库
mod jwt;
mod router; // 路由
mod utils; // 通用工具 // 配置文件 // jwt
use tokio::sync::{mpsc, Semaphore};
use tokio::time; // 休眠操作
use tokio::time::Duration;
#[tokio::main]
async fn main() {
	// 0. 初始化全局变量
	config::init_cache().await; // 基本上静态和动态变量，都在config里完成+ 数据库初始化

	// 1. 启动定时任务
	cron::init().await;
	// 2. 启动路由
	let app = router::init();

	let addr = if cfg!(target_os = "linux") { "0.0.0.0:3399" } else { "127.0.0.1:3399" };
	let listener = tokio::net::TcpListener::bind(addr).await.expect("端口被占用");
	println!("listening on http://{}", addr);
	axum::serve(listener, app).await.expect("服务启用失败");
}
