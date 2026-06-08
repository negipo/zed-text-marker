use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn run(marks_path: &std::path::Path, args: &[&str]) {
    let status = Command::new(env!("CARGO_BIN_EXE_text-marker"))
        .args(args)
        .env("TEXT_MARKER_MARKS_PATH", marks_path)
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn toggle_and_clear_via_cli() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("marks.json");

    run(&path, &["toggle", "hello"]);
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("hello"));

    run(&path, &["toggle", "hello"]);
    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.contains("hello"));

    run(&path, &["toggle", "a"]);
    run(&path, &["clear"]);
    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.contains("\"a\""));
}
