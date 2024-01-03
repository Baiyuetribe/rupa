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
	Json(json!({"status":200,"uuid":res.0,"data":res.1,}))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub exp: usize, // 必须，
}

#[serde_default]
#[derive(Debug, Deserialize)]
pub struct LoginInput {
	name: String,     // 登录账号
	password: String, // 登录账号
	captcha: u32,     // 验证码
	uuid: String,     // 验证码标识
}
// 登录
pub async fn login(Json(input): Json<LoginInput>) -> impl IntoResponse {
	if input.name.is_empty() || input.password.is_empty() {
		return Json(json!({"status":400,"msg":"参数错误",}));
	}
	if !config::check_chaptcha(input.uuid.as_str(), input.captcha) {
		return Json(json!({"status":300,"msg":"验证码错误",}));
	}
	// 查询用户
	let db = config::get_db().await;
	let user = model::user::Entity::find().one(db).await.unwrap_or(None);
	let user = match user {
		None => return Json(json!({"status":400,"msg":"用户不存在",})),
		Some(user) => user,
	};
	if user.name != input.name || !utils::check_password(&input.password, &user.password) {
		return Json(json!({"status":400,"msg":"账号或密码错误",}));
	}
	let exp = chrono::Utc::now() + chrono::Duration::hours(4); // 设置有效期为7天
	let claims = Claims {
		exp: exp.timestamp() as usize, // May 2033 as UTC timestamp
	};
	// Create the authorization token
	let token = match jsonwebtoken::encode(
		&jsonwebtoken::Header::default(),
		&claims,
		&jsonwebtoken::EncodingKey::from_secret(config::ADMIN_JWT_SECRET),
	) {
		Ok(v) => v,
		Err(_) => {
			return Json(json!({"status":400,"msg":"token生成失败",}));
		}
	};
	return Json(json!({"status":200,"data":token}));
}

// 修改账号或密码登操作
pub async fn auth() -> impl IntoResponse {
	let res = config::make_chaptcha();
	Json(json!({"status":200,"uuid":res.0,"data":res.1,}))
}
