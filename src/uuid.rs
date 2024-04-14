use anyhow::{anyhow, ensure, Result};
use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use snowflaked::sync::Generator;
use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
	time::Duration,
};

#[derive(Debug)]
pub struct UUID(Generator);

impl UUID {
	pub fn take(&self) -> u64 {
		self.0.generate()
	}
}

static INS: Lazy<Mutex<HashMap<String, Arc<UUID>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn init(key: Option<&str>, seq: u16) -> Result<()> {
	ensure!(seq < 1024, "seq range : 0 - 1023");
	let ins = Arc::new(UUID(Generator::new(seq)));
	let mut ins_mutex = INS.lock().unwrap();
	let key = key.unwrap_or("default").to_string();
	ins_mutex.insert(key, ins);
	Ok(())
}

pub async fn get(key: Option<&str>) -> Result<Arc<UUID>> {
	let key = key.unwrap_or("default");
	let ins_mutex = INS.lock().unwrap();
	let ins = ins_mutex.get(key).cloned().ok_or(anyhow!(format!("not found : `{}`", key)));
	ins
}
