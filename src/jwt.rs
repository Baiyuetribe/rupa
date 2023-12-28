use axum::http::HeaderMap;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};

use crate::config;
// 单纯验证token是否有效
pub fn check_admin_jwt(token: &str) -> bool {
	let decoding_key = jwt::DecodingKey::from_secret(config::ADMIN_JWT_SECRET);

	// // 7.2.0体积小，速度快，8.0~9.0体积增加0.8Mb,速度略慢
	let validation = jwt::Validation {
		validate_exp: false,  // 关闭验证exp--开发状态
		..Default::default()  // 其余默认为hs256
	};

	// match jwt::decode::<Claims>(token, &decoding_key, &validation) { // 空的Claims，编译体积一样
	match jwt::decode::<serde_json::Value>(token, &decoding_key, &validation) {
		Ok(_) => true,
		Err(err) => {
			#[cfg(debug_assertions)]
			{
				log!("{:?}", err); // 这段代码只在 dev 模式下执行; 常见错误：MissingRequiredClaim("exp")、InvalidSignature
			}
			false
		}
	}
}
