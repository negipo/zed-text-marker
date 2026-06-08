use text_marker::lsp::diagnostics_for;
use tower_lsp::lsp_types::DiagnosticSeverity;

#[test]
fn diagnostics_cover_each_match() {
    let text = "foo bar foo";
    let diags = diagnostics_for(text, &["foo".to_string()]);
    assert_eq!(diags.len(), 2);
    assert_eq!(diags[0].severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(diags[0].source.as_deref(), Some("text-marker"));
    assert!(diags[0].message.contains("foo"));
}

#[test]
fn diagnostics_empty_when_no_marks() {
    let diags = diagnostics_for("foo", &[]);
    assert!(diags.is_empty());
}
