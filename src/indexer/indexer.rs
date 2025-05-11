use log::debug;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::Path;

async fn indexer_index_sv_file(sv_file_path: &str) {
	debug!("start indexer on SystemVerilog source {}", sv_file_path);
}

async fn indexer_index_filelist(filelist_path: &Path) {
	debug!("start indexer on filelist {}", filelist_path.display());

	let file = File::open(filelist_path).unwrap();
	let reader = BufReader::new(file);

	for line in reader.lines() {
		indexer_index_sv_file(&line.unwrap()).await;
	}
}

pub async fn indexer_index_workspace(workspace_root: &Path) {
	debug!("start indexer on workspace {}", workspace_root.display());

	for filename in workspace_root.read_dir().unwrap() {
		let filepath = filename.unwrap().path();
		if let Some(extension) = filepath.extension() {
			if extension == "f" {
				indexer_index_filelist(&filepath).await;
			}
		}
	}
}
