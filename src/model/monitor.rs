use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;

#[serde_default]
#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "monitors")] // 秒级监控日志
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i64, //
	// 负载相关
	pub cpu: f64,   // cpu 负载
	pub cpu1: f64,  // cpu 1分钟负载
	pub cpu5: f64,  // cpu 2分钟负载
	pub cpu15: f64, // cpu 15分钟负载
	// 内存占用
	pub mem: f64, // 内存占用
	// 磁盘占用
	pub disk: f64, // 磁盘占用
	// 网络相关
	pub net_in: f64,  // 网络流入
	pub net_out: f64, // 网络流出
	// 系统相关
	pub top5: String, // 前5进程统计：名称+cpu占用+内存占用
	// 时间相关
	pub created: i64, // 创建时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
