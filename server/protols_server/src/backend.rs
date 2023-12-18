use std::sync::Arc;

use protols::parser::Source;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
    DidSaveTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, InitializedParams,
    MessageType, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp::{
    lsp_types::{InitializeParams, InitializeResult, ServerCapabilities},
    Client, LanguageServer,
};

pub struct ProtoLanguageServer(Arc<RwLock<Backend>>);

impl ProtoLanguageServer {
    pub fn new(client: Client) -> Self {
        Self(Arc::new(RwLock::new(Backend::new(client))))
    }
}

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub source: Source,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            source: Source::new(),
        }
    }

    pub async fn did_save(&mut self, params: DidSaveTextDocumentParams) {
        let path = params.text_document.uri.path();
        log::debug!("did save {path}");
        let file = tokio::fs::read_to_string(path).await.unwrap();

        self.source.parse(path, &file);
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ProtoLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("initialize");
        dbg!(params);

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.0
            .read()
            .await
            .client
            .log_message(MessageType::INFO, "server initialized!")
            .await;

        log::info!("initialized");
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.0.write().await.did_save(params).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        log::debug!("completion");
        let line = params.text_document_position.position.line as usize;
        let column = params.text_document_position.position.character as usize;

        let items = self.0.read().await.source.completions(line, column);

        Ok(Some(CompletionResponse::Array(
            items
                .iter()
                .map(|item| CompletionItem {
                    label: item.to_string(),
                    kind: Some(CompletionItemKind::STRUCT),
                    ..Default::default()
                })
                .collect(),
        )))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        log::debug!("goto_definition");

        todo!()
    }

    async fn shutdown(&self) -> Result<()> {
        log::info!("shutdown");
        Ok(())
    }
}
