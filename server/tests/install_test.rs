use std::fs;
use std::process::Command;

use serde_json::Value;
use tempfile::tempdir;
use text_marker::commands::install::{bundled_tasks, merge_tasks};

fn labels(json: &str) -> Vec<String> {
    let arr: Vec<Value> = serde_json::from_str(json).unwrap();
    arr.iter()
        .filter_map(|t| t.get("label").and_then(|l| l.as_str()).map(String::from))
        .collect()
}

#[test]
fn merge_into_empty_adds_both_tasks() {
    let merged = merge_tasks(None, bundled_tasks()).unwrap();
    let ls = labels(&merged);
    assert!(ls.contains(&"text-marker: toggle".to_string()));
    assert!(ls.contains(&"text-marker: clear".to_string()));
    assert_eq!(ls.len(), 2);
}

#[test]
fn merge_preserves_unrelated_tasks() {
    let existing = r#"[{"label": "build", "command": "cargo build"}]"#;
    let merged = merge_tasks(Some(existing), bundled_tasks()).unwrap();
    let ls = labels(&merged);
    assert!(ls.contains(&"build".to_string()));
    assert_eq!(ls.len(), 3);
}

#[test]
fn merge_is_idempotent() {
    let once = merge_tasks(None, bundled_tasks()).unwrap();
    let twice = merge_tasks(Some(&once), bundled_tasks()).unwrap();
    assert_eq!(labels(&twice).len(), 2);
}

#[test]
fn merge_malformed_existing_errors() {
    assert!(merge_tasks(Some("not json"), bundled_tasks()).is_err());
}

#[test]
fn merge_empty_existing_treated_as_empty() {
    let merged = merge_tasks(Some("   "), bundled_tasks()).unwrap();
    assert_eq!(labels(&merged).len(), 2);
}

#[test]
fn install_command_writes_tasks_file() {
    let dir = tempdir().unwrap();
    let tasks_path = dir.path().join("tasks.json");
    let marks_path = dir.path().join("text-marker/marks.json");

    let status = Command::new(env!("CARGO_BIN_EXE_text-marker"))
        .arg("install")
        .env("TEXT_MARKER_TASKS_PATH", &tasks_path)
        .env("TEXT_MARKER_MARKS_PATH", &marks_path)
        .status()
        .unwrap();
    assert!(status.success());

    let content = fs::read_to_string(&tasks_path).unwrap();
    assert!(content.contains("text-marker: toggle"));
    assert!(content.contains("text-marker: clear"));
    // marks ディレクトリが用意される
    assert!(marks_path.parent().unwrap().exists());
}
