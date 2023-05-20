use std::sync::Arc;

use protols::parser::{tokenize, ParseResult, Parser};
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
    DidSaveTextDocumentParams, InitializedParams, MessageType, TextDocumentSyncCapability,
    TextDocumentSyncKind,
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
    pub parse_result: Option<ParseResult>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            parse_result: None,
        }
    }

    pub async fn did_save(&mut self, params: DidSaveTextDocumentParams) {
        let path = params.text_document.uri.path();
        let file = tokio::fs::read_to_string(path).await.unwrap();
        let tokens = tokenize(&file);
        let parser = Parser::new(tokens);
        let ast = parser.parse(&file);
        self.parse_result.replace(ast);
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ProtoLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        log::info!("initialize");

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

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        log::info!("completion");
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem {
                label: "JavaScript".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                data: Some(1.into()),
                ..Default::default()
            },
            CompletionItem {
                label: "TypeScript".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                data: Some(1.into()),
                ..Default::default()
            },
        ])))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
