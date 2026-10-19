//! The illustrated mnemonic cards (SVG), embedded in the binary, in table order.

use crate::kana::Script;

/// `cards!("hiragana": "a", "i")` → `&[("a", include_bytes!(".../hiragana/a.svg")), ...]`
macro_rules! cards {
    ($script:literal: $($romaji:literal),* $(,)?) => {
        &[$(($romaji, include_bytes!(concat!("../assets/cards/", $script, "/", $romaji, ".svg")))),*]
    };
}

const HIRAGANA_IMAGES: &[(&str, &[u8])] = cards!("hiragana":
    "a", "i", "u", "e", "o", "ka", "ki", "ku", "ke", "ko", "sa", "shi", "su", "se", "so",
    "ta", "chi", "tsu", "te", "to", "na", "ni", "nu", "ne", "no", "ha", "hi", "fu", "he", "ho",
    "ma", "mi", "mu", "me", "mo", "ya", "yu", "yo", "ra", "ri", "ru", "re", "ro", "wa", "wo", "n",
);

const KATAKANA_IMAGES: &[(&str, &[u8])] = cards!("katakana":
    "a", "i", "u", "e", "o", "ka", "ki", "ku", "ke", "ko", "sa", "shi", "su", "se", "so",
    "ta", "chi", "tsu", "te", "to", "na", "ni", "nu", "ne", "no", "ha", "hi", "fu", "he", "ho",
    "ma", "mi", "mu", "me", "mo", "ya", "yu", "yo", "ra", "ri", "ru", "re", "ro", "wa", "wo", "n",
);

fn images(script: Script) -> &'static [(&'static str, &'static [u8])] {
    match script {
        Script::Hiragana => HIRAGANA_IMAGES,
        Script::Katakana => KATAKANA_IMAGES,
    }
}

/// The SVG card of kana `id` (an index into the script's table).
pub fn card(script: Script, id: usize) -> &'static [u8] {
    images(script)[id].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_card_per_kana_in_table_order() {
        for script in [Script::Hiragana, Script::Katakana] {
            let images = images(script);
            assert_eq!(images.len(), script.table().len());
            for (k, (romaji, svg)) in script.table().iter().zip(images) {
                assert_eq!(k.romaji[0], *romaji);
                let svg = std::str::from_utf8(svg).unwrap();
                assert!(svg.contains("viewBox=\"0 0 400 400\""), "{romaji}");
                assert_eq!(svg.matches("id=\"glyph\"").count(), 1, "{romaji}");
            }
        }
    }
}
