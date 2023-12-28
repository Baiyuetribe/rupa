// use anyhow::Result as anyres;
use crate::db;
use crate::handle;
use axum::{
	error_handling::HandleErrorLayer,
	extract::DefaultBodyLimit,
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

async fn index() -> &'static str {
	"Hello, World!"
}
async fn handler_404() -> (StatusCode, &'static str) {
	(StatusCode::NOT_FOUND, "Not found")
}

pub fn init() -> Router {
	// you can convert handler function to service
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "Not found")
	}
	// you can convert handler function to service
	let service = handle_404.into_service();

	// let static_dir = ServeDir::new(".vue").not_found_service(service);
	let cors = CorsLayer::new()
		// allow `GET` and `POST` when accessing the resource
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		// allow requests from any origin
		.allow_headers(Any)
		.allow_origin(Any); // 解决跨域问题

	let app: Router = Router::new()
		// .route("/foo", get(get_foo).post(post_foo))
		// .nest("/", static_dir.clone())
		// 文件服务器，绑定storage文件夹
		.route("/api/demo", get(index)) // 获取文章列表
		// .route("/api/upload", post(index)) // jwt拦截，然后上传文件
		.nest("/api/v4", admin()) // 其余处理
		.fallback(handler_404) // 未定义的，返回404
		// add a fallback service for handling routes to unknown paths
		// Add middleware to all routes
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
	// with(db)放最后
	app
}
