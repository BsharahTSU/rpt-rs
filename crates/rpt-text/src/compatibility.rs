//! Shared conservative shaping ranges for the legacy Crystal compatibility profile.
use std::ops::Range;

/// UTF-8 byte ranges containing Basic-Latin text whose pair kerning may be disabled.
/// Adjacent ASCII spaces/punctuation are included so a leading-space pair is covered.
/// Runs without ASCII letters, and every non-ASCII character, retain normal shaping.
/// This is deliberately narrower than all Latin scripts; it does not disable Arabic
/// joining, required substitutions, CJK positioning, or Latin ligatures.
pub fn latin_kerning_ranges(text: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = None;
    let mut has_letter = false;
    for (offset, character) in text
        .char_indices()
        .chain(std::iter::once((text.len(), '\u{80}')))
    {
        if character.is_ascii() {
            start.get_or_insert(offset);
            has_letter |= character.is_ascii_alphabetic();
        } else {
            if let Some(from) = start.take() {
                if has_letter {
                    ranges.push(from..offset);
                }
            }
            has_letter = false;
        }
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ranges_preserve_utf8_boundaries_and_non_latin_text() {
        let text = "中文 AV العربية Wr 42 成績";
        let ranges = latin_kerning_ranges(text);
        assert_eq!(
            ranges.iter().map(|r| &text[r.clone()]).collect::<Vec<_>>(),
            [" AV ", " Wr 42 "]
        );
        assert!(latin_kerning_ranges("中文 123 العربية").is_empty());
        assert!(latin_kerning_ranges("").is_empty());
        assert_eq!(latin_kerning_ranges(" AV"), vec![0..3]);
    }
}
