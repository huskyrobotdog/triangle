#![allow(unused)]

pub mod runtime;

pub use anyhow;
pub use tokio;

#[cfg(feature = "serde")]
pub use serde;
#[cfg(feature = "serde")]
pub use serde_urlencoded;

#[cfg(feature = "json")]
pub use serde_json;

#[cfg(feature = "chrono")]
pub use chrono;

#[cfg(feature = "env")]
pub use config;
#[cfg(feature = "env")]
pub mod env;

#[cfg(feature = "logger")]
pub use tracing;
#[cfg(feature = "logger")]
pub use tracing_appender;
#[cfg(feature = "logger")]
pub use tracing_subscriber;
#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "http")]
pub use reqwest;
#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "uuid")]
pub use snowflaked;
#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(feature = "crypto")]
pub use base64;
#[cfg(feature = "crypto")]
pub use ring;
#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "orm")]
pub use log;
#[cfg(feature = "orm")]
pub use sea_orm;
#[cfg(feature = "orm")]
pub mod orm;

#[cfg(feature = "web")]
pub mod web;
