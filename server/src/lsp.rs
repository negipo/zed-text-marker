use std::collections::HashMap;
use std::path::PathBuf;

use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::marks;
use crate::matcher::find_ranges;

/// テキストとマーク語から診断を計算する純関数。
pub fn diagnostics_for(text: &str, words: &[String]) -> Vec<Diagnostic> {
    words
        .iter()
        .flat_map(|word| {
            find_ranges(text, std::slice::from_ref(word))
                .into_iter()
                .map(|range| Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("text-marker".to_string()),
                    message: format!("marked: {word}"),
                    ..Default::default()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub struct Backend {
    pub client: Client,
    docs: Mutex<HashMap<Url, String>>,
    marks: Mutex<Vec<String>>,
    marks_path: PathBuf,
}

impl Backend {
    pub fn new(client: Client, marks_path: PathBuf) -> Self {
        let initial = marks::load(&marks_path).unwrap_or_default();
        Self {
            client,
            docs: Mutex::new(HashMap::new()),
            marks: Mutex::new(initial),
            marks_path,
        }
    }

    async fn publish(&self, uri: &Url) {
        let docs = self.docs.lock().await;
        let marks = self.marks.lock().await;
        if let Some(text) = docs.get(uri) {
            let diags = diagnostics_for(text, &marks);
            self.client.publish_diagnostics(uri.clone(), diags, None).await;
        }
    }

    async fn publish_all(&self) {
        let uris: Vec<Url> = {
            let docs = self.docs.lock().await;
            docs.keys().cloned().collect()
        };
        for uri in uris {
            self.publish(&uri).await;
        }
    }

    /// notify監視で呼ばれる: marks.jsonを再読込し全ドキュメント再診断。
    pub async fn reload_marks(&self) {
        if let Ok(loaded) = marks::load(&self.marks_path) {
            *self.marks.lock().await = loaded;
        }
        self.publish_all().await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> RpcResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> RpcResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        self.docs.lock().await.insert(uri.clone(), params.text_document.text);
        self.publish(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().last() {
            self.docs.lock().await.insert(uri.clone(), change.text);
        }
        self.publish(&uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.docs.lock().await.remove(&uri);
        self.client.publish_diagnostics(uri, vec![], None).await;
    }
}
