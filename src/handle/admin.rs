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

// 获取已存在的项目cag，返回一个列表
pub async fn get_cag(headers: HeaderMap) -> impl IntoResponse {
	if !jwt::check_admin_jwt(&headers) {
		return Json(json!({"status":400,"msg":"404"}));
	}
	let db = config::get_db().await;
	let datas = model::project::Entity::find().all(db).await.unwrap_or(Vec::new());
	// let datas = datas.iter().map(|x| x.tag.clone()).collect::<Vec<_>>();
	let datas = datas.iter().map(|x| &x.cag).collect::<Vec<_>>(); // 不变引用，相比clone无消耗，也无需额外处理
	Json(json!({"status":200,"data":datas}))
}

#[serde_default]
#[derive(Debug, Serialize, Deserialize)]
pub struct CagInput {
	data: Vec<String>,
}
// 修改cagn内容，实现对的排序控制，并及时刷新该缓存
pub async fn post_cag(headers: HeaderMap, Json(mut input): Json<CagInput>) -> impl IntoResponse {
	if !jwt::check_admin_jwt(&headers) {
		return Json(json!({"status":400,"msg":"404"}));
	}
	// 更新cag排序缓存
	// 更新project缓存
	let db = config::get_db().await;
	let datas = model::project::Entity::find()
		.order_by_desc(model::project::Column::Order)
		.all(db)
		.await
		.unwrap_or(Vec::new());
	let res = config::PROJECT_CAG
		.iter()
		.map(|x| config::project_res {
			cag: x.to_string(),
			content: datas
				.iter()
				.filter(|y| &y.cag == x)
				.map(|y| config::project_sub_res {
					name: y.name.clone(),
					info: y.info.clone(),
					lang: y.lang.clone(),
					url: y.url.clone(),
				})
				.collect::<Vec<_>>(),
		})
		.collect::<Vec<_>>();
	let cache = Arc::clone(&config::PROJECT_CACHE);
	let mut cache = cache.write().await;
	*cache = res;
	Json(json!({"status":200,"msg":"更新cag和project排序成功"}))
}
