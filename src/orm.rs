use crate::env;
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
	time::Duration,
};

static INS: Lazy<Mutex<HashMap<String, Arc<DatabaseConnection>>>> =
	Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn init(key: Option<&str>, config: &env::Database) -> Result<()> {
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

	let ins = Arc::new(Database::connect(opt).await?);
	let mut ins_mutex = INS.lock().unwrap();
	let key = key.unwrap_or("default").to_string();
	ins_mutex.insert(key, ins);
	Ok(())
}

pub async fn get(key: Option<&str>) -> Result<Arc<DatabaseConnection>> {
	let key = key.unwrap_or("default");
	let ins_mutex = INS.lock().unwrap();
	let ins = ins_mutex.get(key).cloned().ok_or(anyhow!(format!("not found : `{}`", key)));
	ins
}
