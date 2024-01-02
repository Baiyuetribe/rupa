use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;

#[serde_default]
#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "webs")] // 网站
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i64, //
	pub name: String,        // 网站名
	pub path: String,        // 路径
	pub description: String, // 备注描述
	pub created: i64,        // 创建时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
