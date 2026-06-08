use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct MarksFile {
    marks: Vec<String>,
}

/// marks.json のパスを解決する。環境変数を優先し、なければ既定パス。
pub fn marks_path() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("TEXT_MARKER_MARKS_PATH") {
        return Ok(PathBuf::from(p));
    }
    let home = dirs::home_dir().context("ホームディレクトリを解決できません")?;
    Ok(home.join(".config/zed/text-marker/marks.json"))
}

/// marks.json を読む。ファイルがなければ空。壊れていればエラー。
pub fn load(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("marks.json を読めません: {}", path.display()))?;
    let parsed: MarksFile = serde_json::from_str(&content)
        .with_context(|| format!("marks.json が壊れています: {}", path.display()))?;
    Ok(parsed.marks)
}

/// 語のマークをトグルする。空文字列は何もしない。
pub fn toggle(path: &Path, text: &str) -> Result<()> {
    if text.is_empty() {
        return Ok(());
    }
    let mut marks = load(path)?;
    if let Some(pos) = marks.iter().position(|m| m == text) {
        marks.remove(pos);
    } else {
        marks.push(text.to_string());
    }
    write(path, &marks)
}

/// 全マークを消す。
pub fn clear(path: &Path) -> Result<()> {
    write(path, &[])
}

/// marks をアトミックに書き込む(temp + rename)。
fn write(path: &Path, marks: &[String]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("ディレクトリを作成できません: {}", parent.display()))?;
    }
    let file = MarksFile { marks: marks.to_vec() };
    let json = serde_json::to_string_pretty(&file)?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, json)
        .with_context(|| format!("一時ファイルを書けません: {}", tmp.display()))?;
    std::fs::rename(&tmp, path)
        .with_context(|| format!("リネームできません: {}", path.display()))?;
    Ok(())
}
