pub struct Kana {
    pub kana: &'static str,
    pub romaji: &'static [&'static str],
    pub group: u8,
}

pub const HIRAGANA: &[Kana] = &[
    // group 1: voyelles
    Kana {
        kana: "あ",
        romaji: &["a"],
        group: 1,
    },
    Kana {
        kana: "い",
        romaji: &["i"],
        group: 1,
    },
    Kana {
        kana: "う",
        romaji: &["u"],
        group: 1,
    },
    Kana {
        kana: "え",
        romaji: &["e"],
        group: 1,
    },
    Kana {
        kana: "お",
        romaji: &["o"],
        group: 1,
    },
    // group 2: k
    Kana {
        kana: "か",
        romaji: &["ka"],
        group: 2,
    },
    Kana {
        kana: "き",
        romaji: &["ki"],
        group: 2,
    },
    Kana {
        kana: "く",
        romaji: &["ku"],
        group: 2,
    },
    Kana {
        kana: "け",
        romaji: &["ke"],
        group: 2,
    },
    Kana {
        kana: "こ",
        romaji: &["ko"],
        group: 2,
    },
    // group 3: s
    Kana {
        kana: "さ",
        romaji: &["sa"],
        group: 3,
    },
    Kana {
        kana: "し",
        romaji: &["shi", "si"],
        group: 3,
    },
    Kana {
        kana: "す",
        romaji: &["su"],
        group: 3,
    },
    Kana {
        kana: "せ",
        romaji: &["se"],
        group: 3,
    },
    Kana {
        kana: "そ",
        romaji: &["so"],
        group: 3,
    },
    // group 4: t
    Kana {
        kana: "た",
        romaji: &["ta"],
        group: 4,
    },
    Kana {
        kana: "ち",
        romaji: &["chi", "ti"],
        group: 4,
    },
    Kana {
        kana: "つ",
        romaji: &["tsu", "tu"],
        group: 4,
    },
    Kana {
        kana: "て",
        romaji: &["te"],
        group: 4,
    },
    Kana {
        kana: "と",
        romaji: &["to"],
        group: 4,
    },
    // group 5: n
    Kana {
        kana: "な",
        romaji: &["na"],
        group: 5,
    },
    Kana {
        kana: "に",
        romaji: &["ni"],
        group: 5,
    },
    Kana {
        kana: "ぬ",
        romaji: &["nu"],
        group: 5,
    },
    Kana {
        kana: "ね",
        romaji: &["ne"],
        group: 5,
    },
    Kana {
        kana: "の",
        romaji: &["no"],
        group: 5,
    },
    // group 6: h
    Kana {
        kana: "は",
        romaji: &["ha"],
        group: 6,
    },
    Kana {
        kana: "ひ",
        romaji: &["hi"],
        group: 6,
    },
    Kana {
        kana: "ふ",
        romaji: &["fu", "hu"],
        group: 6,
    },
    Kana {
        kana: "へ",
        romaji: &["he"],
        group: 6,
    },
    Kana {
        kana: "ほ",
        romaji: &["ho"],
        group: 6,
    },
    // group 7: m
    Kana {
        kana: "ま",
        romaji: &["ma"],
        group: 7,
    },
    Kana {
        kana: "み",
        romaji: &["mi"],
        group: 7,
    },
    Kana {
        kana: "む",
        romaji: &["mu"],
        group: 7,
    },
    Kana {
        kana: "め",
        romaji: &["me"],
        group: 7,
    },
    Kana {
        kana: "も",
        romaji: &["mo"],
        group: 7,
    },
    // group 8: y
    Kana {
        kana: "や",
        romaji: &["ya"],
        group: 8,
    },
    Kana {
        kana: "ゆ",
        romaji: &["yu"],
        group: 8,
    },
    Kana {
        kana: "よ",
        romaji: &["yo"],
        group: 8,
    },
    // group 9: r
    Kana {
        kana: "ら",
        romaji: &["ra"],
        group: 9,
    },
    Kana {
        kana: "り",
        romaji: &["ri"],
        group: 9,
    },
    Kana {
        kana: "る",
        romaji: &["ru"],
        group: 9,
    },
    Kana {
        kana: "れ",
        romaji: &["re"],
        group: 9,
    },
    Kana {
        kana: "ろ",
        romaji: &["ro"],
        group: 9,
    },
    // group 10: w + n
    Kana {
        kana: "わ",
        romaji: &["wa"],
        group: 10,
    },
    Kana {
        kana: "を",
        romaji: &["wo", "o"],
        group: 10,
    },
    Kana {
        kana: "ん",
        romaji: &["n", "nn"],
        group: 10,
    },
];

/// Sets of kana that look alike, mixed into the same session.
pub const CONFUSABLES: &[&[&str]] = &[
    &["さ", "ち"],
    &["き", "さ"],
    &["ぬ", "め"],
    &["ね", "れ", "わ"],
    &["る", "ろ"],
    &["は", "ほ"],
    &["い", "り"],
    &["こ", "に"],
];

/// Kana that look like `kana`, without `kana` itself.
pub fn look_alikes(kana: &str) -> impl Iterator<Item = &'static str> {
    CONFUSABLES
        .iter()
        .filter(move |set| set.contains(&kana))
        .flat_map(|set| set.iter().copied())
        .filter(move |&other| other != kana)
}
