use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;

#[serde_default]
#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "likes")] // 热门点赞量统计
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i64, //  键id == ?
	pub pid: i64,     // 文章ID
	pub num: i64,     // 点赞数量
	pub created: i64, // 创建时间-- 用来删除过期数据等
}
// 后面两个必须定义，否则报错DeriveEntityModel
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
