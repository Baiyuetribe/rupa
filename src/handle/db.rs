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
use sea_orm::ModelTrait;
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
pub struct SqlInput {
	page: u64, // 页码
}

// 数据库列表获取
pub async fn get_sql(headers: HeaderMap, Query(input): Query<SqlInput>) -> impl IntoResponse {
	if !jwt::check_admin_jwt(&headers) {
		return Json(json!({"status":300,"msg":"权限不足",}));
	}
	let db = config::get_db().await;
	let total = model::mysql::Entity::find().count(db).await.unwrap_or(0);
	let data = model::mysql::Entity::find()
		.order_by_desc(model::mysql::Column::Id)
		.paginate(db, 10)
		.fetch_page(input.page - 1)
		.await
		.unwrap_or(vec![]);
	Json(json!({"status":200,"data":data,"total":total,}))
}

#[serde_default]
#[derive(Debug, Deserialize)]
pub struct DbInput {
	id: i64,         // 主键
	method: String,  // 改权限、删除、改密码
	content: String, // 对应的内容
}

// 数据库创建、删除、修改，重启，备份，恢复
pub async fn post_sql(headers: HeaderMap, Json(input): Json<DbInput>) -> impl IntoResponse {
	if !jwt::check_admin_jwt(&headers) {
		return Json(json!({"status":300,"msg":"权限不足",}));
	}
	match input.method.as_str() {
		"del" => {
			let db = config::get_db().await;
			let res = model::mysql::Entity::find_by_id(input.id).one(db).await.unwrap_or(None);
			match res {
				None => return Json(json!({"status":300,"msg":"数据库不存在",})),
				Some(v) => {
					v.delete(db).await;
				}
			}
		}
		"changepwd" => {
			let db = config::get_db().await;
			let res = model::mysql::Entity::find_by_id(input.id).one(db).await.unwrap_or(None);
			match res {
				None => return Json(json!({"status":300,"msg":"数据库不存在",})),
				Some(mut v) => {
					// 此处还需要执行mysql相关命令
					let mut sql_data: model::mysql::ActiveModel = v.into();
					sql_data.password = Set(input.content);
					sql_data.update(db).await;
				}
			}
		}
		_ => {}
	}
	Json(json!({"status":200,"msg":"ok",}))
}
