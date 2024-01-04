use crate::config;
use crate::core;
use crate::jwt;
use crate::model;
use crate::utils;
use axum::http::HeaderMap;
use axum::{
	extract::{Path, Query, State},
	response::IntoResponse,
	Json,
};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;
use serde_json::json;
use std::sync::Arc;
// 实时获得系统cup使用率

// 文件管理模块

#[serde_default]
#[derive(Debug, Deserialize)]
pub struct DirInput {
	path: String, // 路径
}
// 目录获取
pub async fn get_dir(Json(input): Json<DirInput>) -> impl IntoResponse {
	if input.path.is_empty() {
		return Json(json!({"status":400,"msg":"参数错误",}));
	}
	let data = utils::exec_cmd_with_output(format!("ls -lT {}", input.path).as_str());
	Json(json!({"status":200,"data":data,}))
}

// ls -l --time-style=long-iso . //-- linux
// -rw-r--r-- 1 root root  1949987 2023-10-26 09:49 backup0.sql
// -rw-r--r-- 1 root root      137 2023-04-27 06:12 Caddyfile
// ls -lT . // mac
// -rw-r--r--   1 baiyue  staff   95728  1  3 20:58:52 2024 Cargo.lock
// -rw-r--r--   1 baiyue  staff    2180  1  3 20:58:45 2024 Cargo.toml

// 计算目录大小
pub async fn dir_size() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}

// 压缩目录或文件，采用zip标准
pub async fn compress() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}

// 修改文件或目录名称
pub async fn rename() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}

// 修改权限
pub async fn chmod() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}

// 目录复制
pub async fn cp() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}

// 目录移动
pub async fn mv() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}

// 文件编辑，范围二进制文件内容，由前端vscode时间代码编辑器
pub async fn edit() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}
