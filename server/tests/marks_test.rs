use std::fs;
use tempfile::tempdir;
use text_marker::marks;

#[test]
fn load_missing_file_returns_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("marks.json");
    assert_eq!(marks::load(&path).unwrap(), Vec::<String>::new());
}

#[test]
fn toggle_adds_then_removes() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("marks.json");
    marks::toggle(&path, "foo").unwrap();
    assert_eq!(marks::load(&path).unwrap(), vec!["foo".to_string()]);
    marks::toggle(&path, "foo").unwrap();
    assert_eq!(marks::load(&path).unwrap(), Vec::<String>::new());
}

#[test]
fn toggle_empty_text_is_noop() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("marks.json");
    marks::toggle(&path, "").unwrap();
    assert_eq!(marks::load(&path).unwrap(), Vec::<String>::new());
}

#[test]
fn clear_empties_marks() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("marks.json");
    marks::toggle(&path, "a").unwrap();
    marks::toggle(&path, "b").unwrap();
    marks::clear(&path).unwrap();
    assert_eq!(marks::load(&path).unwrap(), Vec::<String>::new());
}

#[test]
fn load_malformed_file_errors() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("marks.json");
    fs::write(&path, "not json").unwrap();
    assert!(marks::load(&path).is_err());
}

#[test]
fn path_resolution_prefers_env() {
    let dir = tempdir().unwrap();
    let custom = dir.path().join("custom.json");
    std::env::set_var("TEXT_MARKER_MARKS_PATH", &custom);
    assert_eq!(marks::marks_path().unwrap(), custom);
    std::env::remove_var("TEXT_MARKER_MARKS_PATH");
}
