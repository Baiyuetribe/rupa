use axum::http::HeaderMap;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};

use crate::config;

pub fn check_admin_jwt(headers: &axum::http::HeaderMap) -> bool {
	let token = headers
		.get(axum::http::header::AUTHORIZATION)
		.map(|value| value.to_str().unwrap_or("").trim_start_matches("Bearer "))
		.unwrap_or(""); // 对于一个option选项，用map展开，确保安全
	if token.is_empty() {
		// 如果是纯验证，还可以跟check_jwt一起使用 ||
		return false;
	}
	let decoding_key = jwt::DecodingKey::from_secret(config::ADMIN_JWT_SECRET);
	let validation = jwt::Validation::new(jwt::Algorithm::HS256);

	match jwt::decode::<serde_json::Value>(token, &decoding_key, &validation) {
		Ok(v) => true,
		Err(_) => false,
	}
}
