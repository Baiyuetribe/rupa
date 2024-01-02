use crate::config;
use crate::utils;
use std::sync::Arc;

// rust里没有cron，只能自己实现了，欢迎pr
pub async fn init() {
	// 浏览次数更新 - 每10秒更新一次
	tokio::spawn(async move {
		let mut interval = tokio::time::interval(std::time::Duration::from_secs(10)); // 线上10秒
		loop {
			interval.tick().await;
			tokio::spawn(async move {
				// core::refresh_view().await;
			});
		}
	});
}
