use crate::indexer::indexer::*;
use log::debug;
use std::default::Default;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, async_trait};

pub struct Backend {
	client: Client,
	root_uri: Arc<RwLock<Option<Url>>>,
}

impl Backend {
	pub fn new(client: Client) -> Self {
		Backend {
			client,
			root_uri: Default::default(),
		}
	}
}

impl Backend {
	async fn on_change(&self, string: String, uri: String) -> Vec<Diagnostic> {
		Vec::new()
	}
}

#[async_trait]
impl LanguageServer for Backend {
	async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
		debug!("root_uri: {:?}", params.root_uri);

		let mut w = self.root_uri.write().unwrap();
		*w = params.root_uri.clone();

		Ok(InitializeResult {
			capabilities: ServerCapabilities {
				text_document_sync: Some(TextDocumentSyncCapability::Kind(
					TextDocumentSyncKind::FULL,
				)),
				completion_provider: Some(CompletionOptions {
					resolve_provider: Some(false),
					trigger_characters: Some(vec![".".to_string()]),
					work_done_progress_options: Default::default(),
					all_commit_characters: None,
					completion_item: None,
				}),
				workspace: Some(WorkspaceServerCapabilities {
					workspace_folders: Some(WorkspaceFoldersServerCapabilities {
						supported: Some(true),
						change_notifications: Some(OneOf::Left(true)),
					}),
					file_operations: None,
				}),
				definition_provider: Some(OneOf::Left(true)),
				..ServerCapabilities::default()
			},
			server_info: Some(ServerInfo {
				name: String::from("verilogd"),
				version: Some(String::from("0.1.0")),
			}),
		})
	}

	async fn initialized(&self, _: InitializedParams) {
		self.client
			.log_message(MessageType::INFO, "server initialized")
			.await;

		let uri = self.root_uri.read().unwrap().clone();
		indexer_index_workspace(Path::new(uri.unwrap().path())).await;
	}

	async fn shutdown(&self) -> Result<()> {
		Ok(())
	}

	async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
		debug! {"did_change_workspace_folders"};
	}

	async fn did_open(&self, params: DidOpenTextDocumentParams) {
		debug!(
			"did_open {} {}",
			params.text_document.uri.to_string(),
			params.text_document.text
		);
	}

	async fn did_change(&self, params: DidChangeTextDocumentParams) {
		debug!(
			"did_change {} {}",
			params.text_document.uri.to_string(),
			params.content_changes[0].text
		);
	}

	async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
		let uri = params.text_document_position.text_document.uri;
		let position = params.text_document_position.position;

		debug!("completion {}", uri);

		let completions = || -> Option<Vec<CompletionItem>> {
			let mut ret = Vec::new();

			Some(ret)
		}();
		Ok(completions.map(CompletionResponse::Array))
	}
}
