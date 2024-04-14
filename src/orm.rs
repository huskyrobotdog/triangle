use crate::env;
use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn init(config: &env::Database) -> Result<DatabaseConnection> {
	let mut opt = ConnectOptions::new(format!(
		"mysql://{}:{}@{}:{}/{}{}",
		config.username,
		config.password,
		config.host,
		config.port,
		config.database,
		match &config.url {
			Some(url) => format!("?{}", url),
			None => "".into(),
		}
	));
	opt.sqlx_logging(config.show_sql);
	opt.sqlx_logging_level(log::LevelFilter::Info);
	if let Some(slow_sql_millis) = config.slow_sql_millis {
		opt.sqlx_slow_statements_logging_settings(
			log::LevelFilter::Warn,
			Duration::from_millis(slow_sql_millis),
		);
	}
	if let Some(idle_timeout) = config.idle_timeout {
		opt.idle_timeout(Duration::from_secs(idle_timeout));
	}
	opt.min_connections(config.min_conn);
	opt.max_connections(config.max_conn);
	opt.sqlx_logging(config.show_sql);

	let db = Database::connect(opt).await?;
	Ok(db)
}
