use std::future::Future;
use tokio::runtime::{Builder, Handle, Runtime};

pub fn init(thread_name: Option<&str>) -> Runtime {
	Builder::new_multi_thread()
		.thread_name(thread_name.unwrap_or("Triangle"))
		.enable_all()
		.build()
		.unwrap()
}

pub fn blocking<F: Future>(future: F) -> F::Output {
	tokio::task::block_in_place(move || Handle::current().block_on(future))
}
