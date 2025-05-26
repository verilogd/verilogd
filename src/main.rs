pub mod completion;
pub mod indexer;
pub mod lang;
pub mod lsp;

use lsp::backend::Backend;

use log::debug;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::env;
use std::fs::File;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
	let stdin = tokio::io::stdin();
	let stdout = tokio::io::stdout();

	let _ = WriteLogger::init(
		LevelFilter::Debug,
		Config::default(),
		File::create("/tmp/verilogd.log").unwrap(),
	);

	debug!("Verilogd started");

	let (service, socket) = LspService::new(|client| Backend::new(client));
	Server::new(stdin, stdout, socket).serve(service).await;
}
