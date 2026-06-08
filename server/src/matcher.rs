use tower_lsp::lsp_types::{Position, Range};

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// バイトオフセット列を LSP Position に変換する。character は UTF-16 コードユニット数。
fn offsets_to_positions(text: &str, targets: Vec<usize>) -> Vec<Position> {
    let mut order: Vec<usize> = (0..targets.len()).collect();
    order.sort_by_key(|&i| targets[i]);

    let mut result = vec![Position::default(); targets.len()];
    let mut line: u32 = 0;
    let mut col_utf16: u32 = 0;
    let mut byte: usize = 0;
    let mut ti = 0;
    let bytes_len = text.len();
    let mut chars = text.char_indices().peekable();

    loop {
        while ti < order.len() && targets[order[ti]] == byte {
            result[order[ti]] = Position { line, character: col_utf16 };
            ti += 1;
        }
        if ti >= order.len() {
            break;
        }
        match chars.next() {
            Some((_, ch)) => {
                byte += ch.len_utf8();
                if ch == '\n' {
                    line += 1;
                    col_utf16 = 0;
                } else {
                    col_utf16 += ch.len_utf16() as u32;
                }
            }
            None => {
                while ti < order.len() && targets[order[ti]] == bytes_len {
                    result[order[ti]] = Position { line, character: col_utf16 };
                    ti += 1;
                }
                break;
            }
        }
    }
    result
}

/// text 内の各語の単語境界マッチを Range で返す。出現順(開始バイト昇順)。
pub fn find_ranges(text: &str, words: &[String]) -> Vec<Range> {
    let bytes = text.as_bytes();
    let mut spans: Vec<(usize, usize)> = Vec::new();

    for word in words {
        if word.is_empty() {
            continue;
        }
        let wbytes = word.as_bytes();
        let mut start = 0;
        while let Some(rel) = find_subslice(&bytes[start..], wbytes) {
            let s = start + rel;
            let e = s + wbytes.len();
            let left_ok = s == 0 || !is_word_byte(bytes[s - 1]);
            let right_ok = e == bytes.len() || !is_word_byte(bytes[e]);
            if left_ok && right_ok {
                spans.push((s, e));
            }
            start = s + 1;
        }
    }

    spans.sort_by_key(|&(s, _)| s);

    let mut offsets = Vec::with_capacity(spans.len() * 2);
    for &(s, e) in &spans {
        offsets.push(s);
        offsets.push(e);
    }
    let positions = offsets_to_positions(text, offsets);

    spans
        .iter()
        .enumerate()
        .map(|(i, _)| Range { start: positions[i * 2], end: positions[i * 2 + 1] })
        .collect()
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}
