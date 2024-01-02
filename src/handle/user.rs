use crate::config;
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

// 获取验证码
pub async fn make_chaptcha() -> impl IntoResponse {
	let res = config::make_chaptcha();
	Json(json!({"status":"success","uuid":res.0,"data":res.1,}))
}

// 获取已存在的项目cag，返回一个列表
pub async fn get_cag(headers: HeaderMap) -> impl IntoResponse {
	if !jwt::check_admin_jwt(&headers) {
		return Json(json!({"status":400,"msg":"404"}));
	}
	// let db = config::get_db().await;
	// let datas = model::project::Entity::find().all(db).await.unwrap_or(Vec::new());
	// // let datas = datas.iter().map(|x| x.tag.clone()).collect::<Vec<_>>();
	// let datas = datas.iter().map(|x| &x.cag).collect::<Vec<_>>(); // 不变引用，相比clone无消耗，也无需额外处理
	Json(json!({"status":200,"msg":"ok"}))
}
