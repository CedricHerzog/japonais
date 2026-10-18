# Kana CLI: plan

A command-line app, with its interface in French, for learning hiragana and later katakana. It is written in Rust, so building it is also a way to learn the language.

---

## 1. What the research says, and what the app does about it

| Evidence | What the app does |
|---|---|
| **Retrieval practice** (recalling an answer rather than rereading it) and **distributed practice** are the two techniques that Dunlosky et al. (2013) rated "high utility". A meta-analysis of 29 studies found spaced retrieval beat massed retrieval with g = 0.74. | Every exercise asks you to *type* the answer, with no multiple choice. Reviews are scheduled at growing intervals (spaced repetition). |
| The best spacing pattern is still debated: some studies favour expanding intervals and others the opposite. | Use a simple expanding schedule (Leitner boxes). Don't try to optimise it. |
| **Interleaving** helps people tell apart categories that look alike (Brunmair & Richter 2019, 59 studies, g = 0.42, strongest with visual material). | Once a set of characters is known, mix the look-alikes in the same session: さ/ち, ぬ/め, ね/れ/わ, る/ろ, は/ほ, and later シ/ツ, ソ/ン. |
| **Image mnemonics** have mixed evidence for kana. They help motivation and pass rates more than raw recall. | Show an optional French mnemonic, but only *after* a wrong answer. It is a support, not the method. |
| Learning a little at a time works best, and immediate feedback helps. | New characters arrive in groups of 5 (one gojūon row). The next group opens only when the current one is stable. Each answer is marked right or wrong straight away. |

**What this means for your daily practice:** 10–15 minutes a day, every day. Do the app's reviews first, then write each new character by hand on paper (the app can't check handwriting).

---

## 2. Scope of the first version

- 46 basic hiragana, then dakuten (が, ぱ…) and combined sounds (きゃ…).
- Mode **kana → romaji**: you see a character and type its romaji.
- Mode **romaji → kana**: you see romaji and recall the kana. You check yourself, then press `o`/`n` (oui/non).
- Leitner scheduling with 5 boxes and intervals of 0, 1, 3, 7 and 21 days.
- Progress saved to a JSON file.
- Katakana comes later. It's the same engine with a second data table.

---

## 3. Steps

Each step names the Rust concepts it teaches.

### Step 0: Setup ✅
- Install Rust with `rustup`, then run `cargo new kana`.
- Read chapters 1–3 of *The Rust Book* (doc.rust-lang.org/book).
- **Rust:** cargo, `fn main`, `let`/`mut`, basic types.

### Step 1: Character data ✅
- **Done:** [src/kana.rs](src/kana.rs) holds `pub struct Kana { kana, romaji: &'static [&'static str], group: u8 }` and `HIRAGANA`, with 46 kana in 10 groups. [src/main.rs](src/main.rs) declares `mod kana;` and lists the whole table. The build has no warnings.
- Lessons learned: `'x'` is a `char` and `"x"` is a `&str`; `&[...]` makes a slice; each field needs its own `pub`; `mod` goes in `main.rs`.
- Create `src/kana.rs` with `struct Kana { kana: &'static str, romaji: &'static [&'static str], group: u8 }` and a `const HIRAGANA: &[Kana]` table.
- Accept the alternate romaji spellings: し = shi/si, ち = chi/ti, つ = tsu/tu, ふ = fu/hu.
- **Rust:** `struct`, slices, modules (`mod`), `&'static str`.

### Step 2: A minimal quiz
- Loop: pick a random kana, read a line from stdin, compare the answer, print "Correct !" or "Raté, c'était …".
- **Rust:** `std::io`, `String` vs `&str`, `match`, `loop`. Crate: `rand`.

### Step 3: Leitner boxes
- `struct Carte { id, boite: u8, prochaine_revision: NaiveDate }`.
- A right answer moves the card up one box. A wrong answer sends it back to box 1.
- A session shows the cards due today first, then the new ones.
- **Rust:** `enum`, `impl`, `Vec`, `Option`, sorting and filtering with iterators. Crate: `chrono`.

### Step 4: Saving progress
- Write the cards to and read them from `~/.kana/progression.json`.
- **Rust:** `Result` and `?`, error handling. Crates: `serde`, `serde_json`, `dirs`.

### Step 5: Gradual unlocking and interleaving
- Unlock the next group of 5 when every card in the current group is in box 3 or higher.
- Add a `CONFUSABLES` table. When one of these characters comes up, put its look-alikes into the same session.
- **Rust:** `HashMap`, closures, unit tests (`#[cfg(test)]`, `cargo test`).

### Step 6: Command-line interface
- Subcommands: `kana revision`, `kana stats`, `kana reset`, with options `--mode inverse` and `--script katakana`.
- Write all messages in French. Show a short summary at the end of each session (score, cards that went up a box, next review).
- **Rust:** traits and derive. Crate: `clap` (derive).

### Step 7: Mnemonics and polish
- Add a `mnemonique: Option<&'static str>` field in French, shown after a mistake.
- Colours with the `colored` crate. Time each answer: a slow right answer moves up one box, a fast one moves up two.
- **Rust:** `Option` in depth, `std::time::Instant`.

### Step 8: Katakana
- Add a `KATAKANA` table and a `Script` enum. Reuse the whole engine as is.
- **Rust:** generics or traits, and refactoring.

---

## 4. Suggested project layout

```
kana/
├── Cargo.toml
└── src/
    ├── main.rs        # CLI (clap)
    ├── kana.rs        # data tables
    ├── carte.rs       # Carte + Leitner logic
    ├── session.rs     # review loop
    └── stockage.rs    # JSON save/load
```

## 5. Possible next steps
- Replace Leitner with SM-2 or FSRS, which schedule reviews more precisely.
- A TUI (`ratatui`) or a GUI, then kanji.

---

### Sources
- Dunlosky et al. (2013), *Improving Students' Learning With Effective Learning Techniques*. Psychological Science in the Public Interest.
- Spaced vs massed retrieval meta-analysis summary: https://tipsforteachers.substack.com/p/research-bite-53-a-meta-analytic
- Spaced practice overview: https://inspire.acu.edu.au/articles/spaced-practice
- Brunmair & Richter (2019), *Similarity matters: A meta-analysis of interleaved learning*: https://www.psychologie.uni-wuerzburg.de/fileadmin/06020400/2019/Brunmair_Richter_in_press__2019_META-ANALYSIS_OF_INTERLEAVED_LEARNING.pdf
- Mnemonics for Japanese script (JALT Journal): https://jalt-publications.org/jj/articles/2624-using-mnemonics-facilitate-learning-japanese-script-characters
- Picture mnemonics without a picture-superiority effect: https://www.atlantis-press.com/article/125911420.pdf
