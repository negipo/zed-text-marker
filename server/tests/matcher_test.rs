use text_marker::matcher::find_ranges;
use tower_lsp::lsp_types::{Position, Range};

fn r(sl: u32, sc: u32, el: u32, ec: u32) -> Range {
    Range { start: Position { line: sl, character: sc }, end: Position { line: el, character: ec } }
}

#[test]
fn matches_word_boundary_only() {
    let text = "foo foobar foo";
    let got = find_ranges(text, &["foo".to_string()]);
    assert_eq!(got, vec![r(0, 0, 0, 3), r(0, 11, 0, 14)]);
}

#[test]
fn matches_japanese_substring() {
    let text = "対話の対話";
    let got = find_ranges(text, &["対話".to_string()]);
    // 日本語は前後がASCII単語文字でないので両方ヒット。UTF-16で各文字1、対話=2。
    assert_eq!(got, vec![r(0, 0, 0, 2), r(0, 3, 0, 5)]);
}

#[test]
fn matches_across_lines() {
    let text = "alpha\nbeta foo\ngamma";
    let got = find_ranges(text, &["foo".to_string()]);
    assert_eq!(got, vec![r(1, 5, 1, 8)]);
}

#[test]
fn surrogate_pair_counts_as_two_utf16() {
    // 絵文字😀(U+1F600)はUTF-16でサロゲートペア=2。後ろのfooの桁がずれる。
    let text = "😀 foo";
    let got = find_ranges(text, &["foo".to_string()]);
    assert_eq!(got, vec![r(0, 3, 0, 6)]);
}

#[test]
fn empty_word_is_ignored() {
    let got = find_ranges("foo", &["".to_string()]);
    assert_eq!(got, Vec::<Range>::new());
}

#[test]
fn multiple_words() {
    let text = "foo bar";
    let mut got = find_ranges(text, &["foo".to_string(), "bar".to_string()]);
    got.sort_by_key(|r| r.start.character);
    assert_eq!(got, vec![r(0, 0, 0, 3), r(0, 4, 0, 7)]);
}
