// use anyhow::Result as anyres;
use crate::db;
use crate::handle;
use axum::{
	error_handling::HandleErrorLayer,
	extract::{DefaultBodyLimit, Path},
	handler::HandlerWithoutStateExt, // 用于普通函数转server
	http::{Method, StatusCode},
	response::IntoResponse,
	routing::{get, post},
	Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	net::SocketAddr,
	sync::{Arc, Mutex, PoisonError, RwLock},
	time::Duration,
};

use tower::{BoxError, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::services::{ServeDir, ServeFile}; // 跨域

async fn vip_index() -> &'static str {
	"Hello, World!"
}
async fn handler_404() -> (StatusCode, &'static str) {
	(StatusCode::NOT_FOUND, "Not found")
}

pub fn init() -> Router {
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "Not found")
	}
	let service = handle_404.into_service();

	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers(Any)
		.allow_origin(Any); // 解决跨域问题

	let app: Router = Router::new()
		.route("/", get(handle::vue::embed_vue)) // 前端静态资源
		.route("/api/captcha", get(handle::user::make_chaptcha)) // 验证码
		.route("/api/login", get(vip_index)) // 登录
		// 管理入口
		// .nest("/api/v2", admin())
		.route("/api/v2/dashboard", get(vip_index)) // 仪表盘
		.route("/api/v2/website", get(vip_index)) // 网站
		.route("/api/v2/sql", get(vip_index)) // 数据库
		.route("/api/v2/file", get(vip_index)) // 文件管理
		.route("/api/v2/log", get(vip_index)) // 日志
		.route("/api/v2/port", get(vip_index)) // 端口
		.route("/api/v2/safe", get(vip_index)) // 安全
		.route("/api/v2/app", get(vip_index)) // 应用
		.route("/api/v2/monitor", get(vip_index)) // 监控
		.route("/api/v2/cron", get(vip_index)) // 定时任务
		.route("/api/v2/setting", get(vip_index)) // 面板设置
		.fallback(handler_404) // 未定义的，返回404
		.layer(
			ServiceBuilder::new()
				.layer(HandleErrorLayer::new(|error: BoxError| async move {
					if error.is::<tower::timeout::error::Elapsed>() {
						Ok(StatusCode::REQUEST_TIMEOUT)
					} else {
						log!("Unhandled internal error: {error}");
						Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}")))
					}
				}))
				.timeout(Duration::from_secs(10))
				// .layer(TraceLayer::new_for_http())
				.into_inner(),
		)
		.layer(cors);
	app
}
