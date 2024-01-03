use crate::model::*;
use sea_orm::sea_query::MySqlQueryBuilder;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_orm::{QuerySelect, *};
use std::time::Duration;

pub async fn create_tables(db: &DbConn) {
	create_table(db, user::Entity).await;
	create_table(db, app::Entity).await;
	create_table(db, cron::Entity).await;
	create_table(db, log::Entity).await;
	create_table(db, monitor::Entity).await;
	create_table(db, setting::Entity).await;
	create_table(db, web::Entity).await;
}

pub async fn init_db() -> DatabaseConnection {
	let mut opt = ConnectOptions::new("sqlite://rupa.db?mode=rwc".to_owned()); // sqlite://path/to/db.sqlite?mode=rwc
	opt.max_connections(200)
		.min_connections(5)
		.connect_timeout(Duration::from_secs(1))
		.acquire_timeout(Duration::from_secs(8))
		.idle_timeout(Duration::from_secs(8))
		.max_lifetime(Duration::from_secs(8));
	let db = Database::connect(opt).await.expect("数据库连接失败");
	create_tables(&db).await; // 对于ansyn必须要等待
	log!("数据库初始化完毕");
	return db;
}

// use crate::domain::*;
use sea_orm::{self, DbConn, EntityTrait, Schema};

// 创建一个统一函数
async fn create_table<E>(db: &DbConn, entity: E)
where
	E: EntityTrait,
{
	let db_mysql = db.get_database_backend();
	let schema = Schema::new(db_mysql);
	let stmt = db_mysql.build(schema.create_table_from_entity(entity).if_not_exists()); // 仅创建，存在时则忽略，缺点是模型改变后无效
	match db.execute(stmt).await {
		Ok(_) => (),
		Err(e) => log!("Error: {}", e),
	}
}
