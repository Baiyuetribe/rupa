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

async fn wip_index() -> &'static str {
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
		.route("/", get(handle::vue::index)) // 前端静态资源
		.route("/favicon.ico", get(handle::vue::favicon)) // 图标
		.route("/assets/:path", get(handle::vue::assets)) // 前端静态资源
		.route("/api/captcha", get(handle::user::make_chaptcha)) // 验证码
		.route("/api/login", get(wip_index)) // 登录
		// 管理入口
		// .nest("/api/v2", admin())
		.route("/api/v2/dashboard", get(wip_index)) // 仪表盘
		.route("/api/v2/website", get(wip_index)) // 网站
		.route("/api/v2/sql", get(wip_index)) // 数据库
		.route("/api/v2/file", get(wip_index)) // 文件管理
		.route("/api/v2/log", get(wip_index)) // 日志
		.route("/api/v2/port", get(wip_index)) // 端口
		.route("/api/v2/safe", get(wip_index)) // 安全
		.route("/api/v2/app", get(wip_index)) // 应用
		.route("/api/v2/monitor", get(wip_index)) // 监控
		.route("/api/v2/cron", get(wip_index)) // 定时任务
		.route("/api/v2/setting", get(wip_index)) // 面板设置
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
