# Kana CLI: plan

A command-line app, with its interface in French, for learning hiragana and later katakana. It is written in Rust, so building it is also a way to learn the language.

---

## 1. What the research says, and what the app does about it

| Evidence | What the app does |
|---|---|
| **Retrieval practice** (recalling an answer rather than rereading it) and **distributed practice** are the two techniques that Dunlosky et al. (2013) rated "high utility". A meta-analysis of 29 studies found spaced retrieval beat massed retrieval with g = 0.74. | Every exercise asks you to *type* the answer, with no multiple choice. Reviews are scheduled at growing intervals (spaced repetition). |
| The best spacing pattern is still debated: some studies favour expanding intervals and others the opposite. Schedulers that model each card's memory, such as **FSRS**, predict forgetting better than SM-2 or fixed boxes on large public review datasets, and FSRS is now Anki's default. | Started with Leitner boxes (Steps 3–8), then moved to FSRS (Step 9): each card gets its own interval, set so you review it when recall drops to about 90%. |
| **Interleaving** helps people tell apart categories that look alike (Brunmair & Richter 2019, 59 studies, g = 0.42, strongest with visual material). | Once a set of characters is known, mix the look-alikes in the same session: さ/ち, ぬ/め, ね/れ/わ, る/ろ, は/ほ, and later シ/ツ, ソ/ン. |
| **Image mnemonics** have mixed evidence for kana. They help motivation and pass rates more than raw recall. | Show an optional French mnemonic, but only *after* a wrong answer. It is a support, not the method. |
| Learning a little at a time works best, and immediate feedback helps. | New characters arrive in groups of 5 (one gojūon row). The next group opens only when the current one is stable. Each answer is marked right or wrong straight away. |

**What this means for your daily practice:** 10–15 minutes a day, every day. Do the app's reviews first, then write each new character by hand on paper (the app can't check handwriting).

---

## 2. Scope of the first version

- 46 basic hiragana, then dakuten (が, ぱ…) and combined sounds (きゃ…).
- Mode **kana → romaji**: you see a character and type its romaji.
- Mode **romaji → kana**: you see romaji and recall the kana. You check yourself, then press `o`/`n` (oui/non).
- Scheduling: Leitner boxes at first (0, 1, 3, 7 and 21 days), then FSRS-5 from Step 9.
- Progress saved to a JSON file.
- Katakana comes later. It's the same engine with a second data table.

---

## 3. Steps

Each step names the Rust concepts it teaches.

**Convention:** code (identifiers, file names, comments) is in English. Only what the user sees (messages, prompts) is in French.

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

### Step 2: A minimal quiz ✅
- **Done:** an endless random quiz in French. `trim()` removes the `\n` that `read_line` keeps, `romaji.contains(&answer)` accepts every spelling, and `Ok(0) => break` exits cleanly at end of input (Ctrl+D).
- Lessons learned: `read_line` keeps the newline; `match` arms are checked in order, so `Ok(0)` goes before `Ok(_)`.
- Loop: pick a random kana, read a line from stdin, compare the answer, print "Correct !" or "Raté, c'était …".
- **Rust:** `std::io`, `String` vs `&str`, `match`, `loop`. Crate: `rand`.

### Step 3: Leitner boxes ✅
- **Done:** [src/card.rs](src/card.rs) has `Card { id, box_number, next_study }`, `enum Outcome { Correct, Wrong }`, and `new` / `is_due` / `answer`, with delays of 0/1/3/7/21 days. [src/main.rs](src/main.rs) builds a deck from group 1, asks only due cards, and ends with "Session terminée !". `q` quits.
- Lessons learned: don't call a type `Result`, because it hides the standard one; the last expression is the return value; using indices (`cards[x]`) avoids borrow-checker trouble.
- A right answer moves the card up one box. A wrong answer sends it back to box 1.
- A session shows the cards due today first, then the new ones.
- **Rust:** `enum`, `impl`, `Vec`, `Option`, sorting and filtering with iterators. Crate: `chrono`.

### Step 4: Saving progress ✅
- **Done:** [src/storage.rs](src/storage.rs) has `load` / `save` using `?`, and `Card` derives `Serialize`/`Deserialize`. `main` loads the deck, or builds a new one on first launch, with a `match` used as an expression, and saves after every answer.
- Lessons learned: `?` = "return the error, otherwise unwrap the value", and it needs a function that returns `Result`; `Box<dyn Error>` accepts any error; `match` produces a value; `if let` handles the one case you care about.
- Write the cards to and read them from `~/.kana/progression.json`.
- **Rust:** `Result` and `?`, error handling. Crates: `serde`, `serde_json`, `dirs`.

### Step 5: Gradual unlocking and interleaving ✅
- **Done:** `card::unlock_next_group` adds the next group when the current one is all in box 3+, and runs at startup, so a fresh deck starts with group 1. `kana::CONFUSABLES` + `look_alikes` add unlocked look-alikes to the session, using a `HashSet` of extras and a `HashSet` of asked cards so it can't ping-pong. A look-alike answered right keeps its schedule; a wrong one goes back to box 1. 7 unit tests in `card.rs`.
- Also: `storage::load` returns an empty deck when the file is missing, and `main` refuses to start if the file is corrupt, rather than overwriting it.
- Unlock the next group of 5 when every card in the current group is in box 3 or higher.
- Add a `CONFUSABLES` table. When one of these characters comes up, put its look-alikes into the same session.
- **Rust:** `HashMap`, closures, unit tests (`#[cfg(test)]`, `cargo test`).

### Step 6: Command-line interface ✅
- **Done:** the binary is named `kana`. [src/main.rs](src/main.rs) parses `revision` (the default, `--mode normal|inverse`), `stats` (a per-box bar chart) and `reset` (asks for confirmation) with clap derive. The review loop moved to [src/session.rs](src/session.rs), which returns a `Summary` that implements `Display` by hand. Answers are lowercased, so `KA` is accepted.
- `--script katakana` is left for Step 8.
- Subcommands: `kana revision`, `kana stats`, `kana reset`, with options `--mode inverse` and `--script katakana`.
- Write all messages in French. Show a short summary at the end of each session (score, cards that went up a box, next review).
- **Rust:** traits and derive. Crate: `clap` (derive).

### Step 7: Mnemonics and polish ✅
- **Done:** `Kana.mnemonic: Option<&'static str>`, filled with a French mnemonic for 33 kana (every look-alike included) and `None` for the rest; it's shown after a mistake in both modes. Colours via `colored`'s `Colorize` trait (it also respects `NO_COLOR`). A new `Outcome::Fast` moves a card up 2 boxes when the answer takes under 3 s (`Instant` / `Duration`), in normal mode only. 8 unit tests.
- Add a `mnemonic: Option<&'static str>` field (the text itself in French), shown after a mistake.
- Colours with the `colored` crate. Time each answer: a slow right answer moves up one box, a fast one moves up two.
- **Rust:** `Option` in depth, `std::time::Instant`.

### Step 8: Katakana ✅
- **Done:** [src/kana.rs](src/kana.rs) has `KATAKANA` (46 kana, same groups and romaji as `HIRAGANA`) and `enum Script { Hiragana, Katakana }` with `table()` / `name()`. `CONFUSABLES` gains シ/ツ, ソ/ン, ク/ケ/タ, ス/ヌ, コ/ユ, チ/テ, ウ/ワ, each with a French mnemonic. `unlock_next_group`, `session::run` and `storage` take the table or the `Script` instead of hard-coding `HIRAGANA`. A global `--script katakana` works with every subcommand; katakana progress lives in `~/.kana/katakana.json`, hiragana keeps `progression.json`. 3 new tests check the tables.
- Lessons learned: an enum with methods (`match self`) is often enough where you'd reach for a trait; passing `&[Kana]` instead of naming a global makes a function reusable; `#[arg(global = true)]` lets an option go after any subcommand.
- Add a `KATAKANA` table and a `Script` enum. Reuse the whole engine as is.
- **Rust:** generics or traits, and refactoring.

### Step 9: FSRS instead of Leitner ✅
- **Done:** [src/fsrs.rs](src/fsrs.rs) implements FSRS-5 by hand with its 19 default weights. A `Memory { stability, difficulty }` comes from `Memory::initial(rating)`, is updated by `next(rating, elapsed_days)` (separate same-day, lapse and success formulas), and `interval_days` turns stability into the next review date at 90% retention. `Card` now holds `memory: Option<Memory>` and `last_review` instead of `box_number`. Wrong → Again (due again today), Correct → Good, Fast → Easy. A group unlocks when every card reaches 5 days of stability, so it takes a second good review or a fast first answer, as box 3 did. `storage` converts old Leitner files on load (`#[serde(untagged)]` enum + `From`). `kana stats` shows cards by stability. 20 tests in total.
- Lessons learned: `f64` maths (`powf`, `exp`, `clamp`); `impl From<A> for B` gives you `B::from(a)` and `.into()`; with `#[serde(untagged)]` variants are tried in order, and missing `Option` fields deserialize as `None`, so the stricter variant must come first; `#[allow(dead_code)]` documents something kept on purpose.
- **Rust:** `f64`, the `From` trait, serde enums, data migration.

---

## 4. Suggested project layout

```
kana/
├── Cargo.toml
└── src/
    ├── main.rs        # CLI (clap)
    ├── kana.rs        # data tables
    ├── card.rs        # Card, review outcomes, unlocking
    ├── fsrs.rs        # FSRS scheduling maths
    ├── session.rs     # review loop
    └── storage.rs     # JSON save/load
```

## 5. Possible next steps
- Tune FSRS weights to your own review history (needs a review log).
- A TUI (`ratatui`) or a GUI, then kanji.

---

### Sources
- Dunlosky et al. (2013), *Improving Students' Learning With Effective Learning Techniques*. Psychological Science in the Public Interest.
- Spaced vs massed retrieval meta-analysis summary: https://tipsforteachers.substack.com/p/research-bite-53-a-meta-analytic
- Spaced practice overview: https://inspire.acu.edu.au/articles/spaced-practice
- Brunmair & Richter (2019), *Similarity matters: A meta-analysis of interleaved learning*: https://www.psychologie.uni-wuerzburg.de/fileadmin/06020400/2019/Brunmair_Richter_in_press__2019_META-ANALYSIS_OF_INTERLEAVED_LEARNING.pdf
- Mnemonics for Japanese script (JALT Journal): https://jalt-publications.org/jj/articles/2624-using-mnemonics-facilitate-learning-japanese-script-characters
- Picture mnemonics without a picture-superiority effect: https://www.atlantis-press.com/article/125911420.pdf
