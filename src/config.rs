use crate::db;
use crate::model;
use crate::utils;
use lazy_static::lazy_static;
use safe_cache::{async_cleanup_task, Cache};
use sea_orm::DatabaseConnection; // 专用于async函数运行
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, OnceCell, RwLock};

// config.rs
pub static ADMIN_JWT_SECRET: &[u8] = "rupa开发临时秘钥".as_bytes(); // 管理员，const用于编译期常量，static用于运行期常量

lazy_static! {
	// pub static ref PROJECT_CAG: Arc<Vec<String>> = Arc::new(Vec::new()); // 存放项目排序
	pub static ref DB: OnceCell<DatabaseConnection> = OnceCell::new(); // 由于async机制，无法使用lazy_static初始化，因此在init函数里进行初始化
	pub static ref CHAPTCHA_CACHE: Arc<Cache> = Arc::new(Cache::new()); // 存放验证码

}

pub async fn init_cache() {
	// 0. 初始化数据库
	DB.set(db::init_db().await);
	// 1. 验证码缓存初始化
	let cache = CHAPTCHA_CACHE.clone();
	async_cleanup_task(cache, 30).await; // 每30秒清理一次
	log!("初始化结束")
}

// 通用外接接口 -- 传递引用
pub async fn get_db() -> &'static DatabaseConnection {
	DB.get_or_init(db::init_db).await
}

pub fn make_chaptcha() -> (String, String) {
	let uuid = utils::get_uuid();
	let res = math_captcha::Captcha::new(170, 50);
	CHAPTCHA_CACHE.set(uuid.clone(), res.value, 10 * 60); // 存档为10分钟
	(uuid, res.base64_img)
}
