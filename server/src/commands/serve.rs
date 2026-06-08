use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tower_lsp::{LspService, Server};

use crate::lsp::Backend;
use crate::marks;

pub fn run() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(serve())
}

async fn serve() -> Result<()> {
    let marks_path = marks::marks_path()?;
    if let Some(parent) = marks_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let (service, socket) = LspService::new(|client| Arc::new(Backend::new(client, marks_path.clone())));
    let backend = service.inner().clone();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    let watch_dir = marks_path.parent().map(|p| p.to_path_buf());
    let _watcher = spawn_watcher(watch_dir, tx)?;

    tokio::spawn(async move {
        while rx.recv().await.is_some() {
            tokio::time::sleep(Duration::from_millis(50)).await;
            backend.reload_marks().await;
        }
    });

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}

fn spawn_watcher(
    dir: Option<std::path::PathBuf>,
    tx: tokio::sync::mpsc::UnboundedSender<()>,
) -> Result<Option<RecommendedWatcher>> {
    let Some(dir) = dir else { return Ok(None) };
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if res.is_ok() {
            let _ = tx.send(());
        }
    })?;
    watcher.watch(&dir, RecursiveMode::NonRecursive)?;
    Ok(Some(watcher))
}
