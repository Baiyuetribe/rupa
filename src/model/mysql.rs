use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_default::serde_default;

#[serde_default]
#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "mysqls")] // 数据库
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i64, //
	pub name: String,     // 数据库名
	pub username: String, // 用户名
	pub password: String, // 密码
	pub info: String,     // 备注信息
	pub permit: String,   // 权限，all或指定ip
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
