use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::marks;

const BUNDLED_TASKS: &str = include_str!("../../../assets/tasks.json");

/// 同梱の tasks 定義(JSON配列)を返す。
pub fn bundled_tasks() -> &'static str {
    BUNDLED_TASKS
}

/// 既存の tasks.json に同梱タスクを冪等にマージした JSON 文字列を返す。
/// 同じ label の既存エントリは差し替える。既存が壊れていればエラー。
pub fn merge_tasks(existing: Option<&str>, ours_json: &str) -> Result<String> {
    let ours: Vec<Value> =
        serde_json::from_str(ours_json).context("同梱タスクJSONをパースできません")?;
    let our_labels: Vec<&str> = ours.iter().filter_map(label_of).collect();

    let mut merged: Vec<Value> = match existing {
        Some(s) if !s.trim().is_empty() => {
            serde_json::from_str(s).context("既存の tasks.json をパースできません")?
        }
        _ => Vec::new(),
    };

    merged.retain(|t| match label_of(t) {
        Some(l) => !our_labels.contains(&l),
        None => true,
    });
    merged.extend(ours);

    Ok(serde_json::to_string_pretty(&merged)?)
}

fn label_of(task: &Value) -> Option<&str> {
    task.get("label").and_then(|l| l.as_str())
}

fn tasks_path() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("TEXT_MARKER_TASKS_PATH") {
        return Ok(PathBuf::from(p));
    }
    let home = dirs::home_dir().context("ホームディレクトリを解決できません")?;
    Ok(home.join(".config/zed/tasks.json"))
}

pub fn run() -> Result<()> {
    let path = tasks_path()?;
    let existing = if path.exists() {
        Some(std::fs::read_to_string(&path).with_context(|| {
            format!("既存の tasks.json を読めません: {}", path.display())
        })?)
    } else {
        None
    };

    let merged = merge_tasks(existing.as_deref(), BUNDLED_TASKS)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("ディレクトリを作成できません: {}", parent.display()))?;
    }
    std::fs::write(&path, merged)
        .with_context(|| format!("tasks.json を書けません: {}", path.display()))?;

    let marks_path = marks::marks_path()?;
    if let Some(parent) = marks_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("ディレクトリを作成できません: {}", parent.display()))?;
    }

    println!("Merged text-marker tasks into {}", path.display());
    println!();
    println!("Remaining manual steps:");
    println!("  1. Install the dev extension in Zed: run `zed: install dev extension`");
    println!("     from the command palette and select the `extension/` directory.");
    println!("  2. Add the bindings from assets/keymap.json to your Zed keymap.");
    println!("     In vim normal mode `m h` marks the word under the cursor and");
    println!("     `m shift-h` clears all marks; in visual mode `m h` marks the selection.");
    Ok(())
}
