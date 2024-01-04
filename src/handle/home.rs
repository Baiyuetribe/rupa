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
use sysinfo::{Components, Disks, Networks, System}; // 系统信息
													// 实时获得系统cup使用率

// 首页实时监控数据刷新
pub async fn refresh_home() -> impl IntoResponse {
	let data = core::status::get_system_data();
	Json(json!({"status":200,"data":data,}))
}
