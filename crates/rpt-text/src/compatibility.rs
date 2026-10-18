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

/// Arabic-script spans for the optional legacy discretionary-ligature policy.
/// Inherited combining marks/join controls stay with an Arabic span. Common
/// punctuation/spaces and other scripts are excluded, and a marks-only run is
/// never enabled. This preserves the original UTF-8 text and cluster boundaries.
pub fn arabic_ligature_ranges(text: &str) -> Vec<Range<usize>> {
    use unicode_script::{Script, UnicodeScript};
    let mut ranges = Vec::new();
    let mut start = None;
    let mut has_arabic = false;
    for (offset, character) in text
        .char_indices()
        .chain(std::iter::once((text.len(), ' ')))
    {
        let script = character.script();
        if script == Script::Arabic {
            start.get_or_insert(offset);
            has_arabic = true;
        } else if script == Script::Inherited && start.is_some() {
            // Keep following marks/join controls, but do not take a preceding
            // Latin/Hebrew combining mark merely because Arabic follows it.
        } else {
            if let Some(from) = start.take() {
                if has_arabic {
                    ranges.push(from..offset);
                }
            }
            has_arabic = false;
        }
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arabic_masks_keep_marks_and_joiners_without_enabling_other_scripts() {
        let text = "ff 中文 اا ا\u{200d}ا ا\u{64e}ا שָ\u{301} ff";
        let ranges = arabic_ligature_ranges(text);
        assert_eq!(
            ranges.iter().map(|r| &text[r.clone()]).collect::<Vec<_>>(),
            ["اا", "ا\u{200d}ا", "ا\u{64e}ا"]
        );
        assert!(arabic_ligature_ranges("ff 中文 שָ\u{301} \u{301}").is_empty());
        assert_eq!(arabic_ligature_ranges("اا"), vec![0..4]);
        let adjacent = "a\u{301}اا";
        assert_eq!(arabic_ligature_ranges(adjacent), vec![3..7]);
        assert_eq!(&adjacent[arabic_ligature_ranges(adjacent)[0].clone()], "اا");
    }

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
