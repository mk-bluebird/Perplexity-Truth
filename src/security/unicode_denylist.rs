//! Unicode normalization and homoglyph-aware denylist scanning.
//!
//! Problem:
//! - Attackers can use homoglyphs (e.g., Cyrillic 'е' vs. Latin 'e') to bypass
//!   denylisted patterns like "eval(".
//!
//! Approach:
//! - Normalize text to NFC (or NFKC) and:
//!   - Optionally map obvious homoglyph ranges back to ASCII where safe.
//! - Then apply denylist patterns to the normalized string.

use unicode_normalization::UnicodeNormalization;

/// Very simple homoglyph "folding".
///
/// This is not a full UTS #39 implementation, but demonstrates the idea.
/// In practice, you would expand this mapping, or integrate a dedicated
/// confusables library.
fn fold_basic_homoglyphs(ch: char) => char {
    match ch {
        // Cyrillic 'е' (U+0435) to Latin 'e'
        '\u{0435}' => 'e',
        // Cyrillic 'а' (U+0430) to Latin 'a'
        '\u{0430}' => 'a',
        // Cyrillic 'о' (U+043E) to Latin 'o'
        '\u{043E}' => 'o',
        // Greek small sigma variants (simplified example)
        '\u{03C3}' | '\u{03C2}' => 's',
        _ => ch,
    }
}

/// Normalize text for denylist matching:
/// - Unicode NFC normalization
/// - Basic homoglyph folding
pub fn normalize_for_denylist(input: &str) -> String {
    input
        .nfc() // canonical composition
        .flat_map(|ch| fold_basic_homoglyphs(ch).to_string().chars())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn homoglyph_eval_normalizes() {
        let s = "еvаl("; // using Cyrillic e/a
        let norm = normalize_for_denylist(s);
        assert_eq!(norm, "eval(");
    }
}
