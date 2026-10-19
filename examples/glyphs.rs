//! Extracts each kana's outline from the Klee One font as an SVG path,
//! for the illustrated cards. Run with `cargo run --example glyphs`.
//! Writes assets/glyphs/{script}/{romaji}.path, in a 400×400 box.

use std::fmt::Write as _;
use std::fs;

#[path = "../src/kana.rs"]
#[allow(dead_code)]
mod kana;

use kana::{HIRAGANA, KATAKANA, Kana};

const FONT: &[u8] = include_bytes!("../assets/fonts/KleeOne-Kana.ttf");
const SIZE: f32 = 400.0;
const MARGIN: f32 = 50.0; // the em square fills 50..350

/// Collects outline commands as an SVG path, mapping font units to the card box.
struct SvgPath {
    d: String,
    scale: f32,
    top: f32, // font y at the top of the em square
}

impl SvgPath {
    fn x(&self, x: f32) -> f32 {
        MARGIN + x * self.scale
    }
    fn y(&self, y: f32) -> f32 {
        MARGIN + (self.top - y) * self.scale // font y goes up, SVG y goes down
    }
}

impl ttf_parser::OutlineBuilder for SvgPath {
    fn move_to(&mut self, x: f32, y: f32) {
        let (x, y) = (self.x(x), self.y(y));
        write!(self.d, "M{x:.1} {y:.1}").unwrap();
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let (x, y) = (self.x(x), self.y(y));
        write!(self.d, "L{x:.1} {y:.1}").unwrap();
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (x1, y1, x, y) = (self.x(x1), self.y(y1), self.x(x), self.y(y));
        write!(self.d, "Q{x1:.1} {y1:.1} {x:.1} {y:.1}").unwrap();
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (x1, y1, x2, y2) = (self.x(x1), self.y(y1), self.x(x2), self.y(y2));
        let (x, y) = (self.x(x), self.y(y));
        write!(self.d, "C{x1:.1} {y1:.1} {x2:.1} {y2:.1} {x:.1} {y:.1}").unwrap();
    }
    fn close(&mut self) {
        self.d.push('Z');
    }
}

fn main() {
    let face = ttf_parser::Face::parse(FONT, 0).expect("bad font");
    let em = face.units_per_em() as f32;
    // Centre the em square vertically on the ascender/descender range.
    let top = (face.ascender() as f32 + face.descender() as f32 + em) / 2.0;

    for (script, table) in [("hiragana", HIRAGANA), ("katakana", KATAKANA)] {
        let dir = format!("assets/glyphs/{script}");
        fs::create_dir_all(&dir).unwrap();
        for k in table {
            let path = outline(&face, k, em, top);
            fs::write(format!("{dir}/{}.path", k.romaji[0]), path).unwrap();
        }
        println!("{script} : {} glyphes", table.len());
    }
}

fn outline(face: &ttf_parser::Face, k: &Kana, em: f32, top: f32) -> String {
    let c = k.kana.chars().next().unwrap();
    let id = face.glyph_index(c).expect("kana missing from font");
    let mut builder = SvgPath {
        d: String::new(),
        scale: (SIZE - 2.0 * MARGIN) / em,
        top,
    };
    face.outline_glyph(id, &mut builder);
    builder.d
}
