use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;

#[serde_default]
#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "logs")] // 存放监控日志
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i64, //
	pub key: String,   // 键
	pub value: String, // 值
	pub created: i64,  // 创建时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
