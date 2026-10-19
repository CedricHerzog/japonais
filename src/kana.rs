use clap::ValueEnum;

pub struct Kana {
    pub kana: &'static str,
    pub romaji: &'static [&'static str],
    pub group: u8,
    pub mnemonic: Option<&'static str>, // shown in French after a mistake
}

pub const HIRAGANA: &[Kana] = &[
    // group 1: voyelles
    Kana {
        kana: "あ",
        romaji: &["a"],
        group: 1,
        mnemonic: Some("Un « a » minuscule caché à droite, barré d'une croix."),
    },
    Kana {
        kana: "い",
        romaji: &["i"],
        group: 1,
        mnemonic: Some(
            "Deux traits presque égaux, comme deux « i ». (り a le trait de droite bien plus long.)",
        ),
    },
    Kana {
        kana: "う",
        romaji: &["u"],
        group: 1,
        mnemonic: Some("Un « u » couché sur le côté, avec une petite tête."),
    },
    Kana {
        kana: "え",
        romaji: &["e"],
        group: 1,
        mnemonic: Some("Un danseur qui fait le grand écart en criant « Eh ! »"),
    },
    Kana {
        kana: "お",
        romaji: &["o"],
        group: 1,
        mnemonic: Some("Un « o » à droite, une croix à gauche : « Oh ! »"),
    },
    // group 2: k
    Kana {
        kana: "か",
        romaji: &["ka"],
        group: 2,
        mnemonic: None,
    },
    Kana {
        kana: "き",
        romaji: &["ki"],
        group: 2,
        mnemonic: Some("Une clé (« key ») avec deux dents : ki. (さ n'a qu'une barre.)"),
    },
    Kana {
        kana: "く",
        romaji: &["ku"],
        group: 2,
        mnemonic: Some("Un bec ouvert qui fait « coucou » : ku."),
    },
    Kana {
        kana: "け",
        romaji: &["ke"],
        group: 2,
        mnemonic: None,
    },
    Kana {
        kana: "こ",
        romaji: &["ko"],
        group: 2,
        mnemonic: Some(
            "Deux petits traits face à face : ko. (Avec un trait vertical devant, c'est に : ni.)",
        ),
    },
    // group 3: s
    Kana {
        kana: "さ",
        romaji: &["sa"],
        group: 3,
        mnemonic: Some(
            "Une barre au-dessus d'un « c » ouvert à droite : sa. (ち ressemble à un 5.)",
        ),
    },
    Kana {
        kana: "し",
        romaji: &["shi", "si"],
        group: 3,
        mnemonic: Some("Une trace de ski qui remonte : shi."),
    },
    Kana {
        kana: "す",
        romaji: &["su"],
        group: 3,
        mnemonic: Some("Une corde avec une boucle, suspendue : su."),
    },
    Kana {
        kana: "せ",
        romaji: &["se"],
        group: 3,
        mnemonic: None,
    },
    Kana {
        kana: "そ",
        romaji: &["so"],
        group: 3,
        mnemonic: None,
    },
    // group 4: t
    Kana {
        kana: "た",
        romaji: &["ta"],
        group: 4,
        mnemonic: Some("On y lit un « t » suivi d'un « a » : ta."),
    },
    Kana {
        kana: "ち",
        romaji: &["chi", "ti"],
        group: 4,
        mnemonic: Some(
            "Ressemble au chiffre 5, le ventre à droite : chi. (さ s'ouvre comme un « c ».)",
        ),
    },
    Kana {
        kana: "つ",
        romaji: &["tsu", "tu"],
        group: 4,
        mnemonic: Some("Une vague qui déferle : tsunami, tsu."),
    },
    Kana {
        kana: "て",
        romaji: &["te"],
        group: 4,
        mnemonic: Some("Un bras tendu : « tiens ! », te."),
    },
    Kana {
        kana: "と",
        romaji: &["to"],
        group: 4,
        mnemonic: Some("Un orteil (« toe ») avec une écharde : to."),
    },
    // group 5: n
    Kana {
        kana: "な",
        romaji: &["na"],
        group: 5,
        mnemonic: None,
    },
    Kana {
        kana: "に",
        romaji: &["ni"],
        group: 5,
        mnemonic: Some("Un trait vertical suivi de こ : ni."),
    },
    Kana {
        kana: "ぬ",
        romaji: &["nu"],
        group: 5,
        mnemonic: Some(
            "Des nouilles emmêlées avec une boucle au bout : nu. (め n'a pas de boucle.)",
        ),
    },
    Kana {
        kana: "ね",
        romaji: &["ne"],
        group: 5,
        mnemonic: Some(
            "Un chat (« neko ») qui enroule sa queue : ne. (れ et わ n'ont pas de boucle.)",
        ),
    },
    Kana {
        kana: "の",
        romaji: &["no"],
        group: 5,
        mnemonic: Some("Un panneau d'interdiction : no !"),
    },
    // group 6: h
    Kana {
        kana: "は",
        romaji: &["ha"],
        group: 6,
        mnemonic: Some("Un trait, puis une croix avec une boucle : ha. (ほ a un toit en plus.)"),
    },
    Kana {
        kana: "ひ",
        romaji: &["hi"],
        group: 6,
        mnemonic: Some("Un grand sourire : hi hi !"),
    },
    Kana {
        kana: "ふ",
        romaji: &["fu", "hu"],
        group: 6,
        mnemonic: Some("Le mont Fuji entouré de nuages : fu."),
    },
    Kana {
        kana: "へ",
        romaji: &["he"],
        group: 6,
        mnemonic: Some("Une petite colline : « hé, je l'ai grimpée ! », he."),
    },
    Kana {
        kana: "ほ",
        romaji: &["ho"],
        group: 6,
        mnemonic: Some("Comme は, avec un toit en plus : ho."),
    },
    // group 7: m
    Kana {
        kana: "ま",
        romaji: &["ma"],
        group: 7,
        mnemonic: None,
    },
    Kana {
        kana: "み",
        romaji: &["mi"],
        group: 7,
        mnemonic: None,
    },
    Kana {
        kana: "む",
        romaji: &["mu"],
        group: 7,
        mnemonic: None,
    },
    Kana {
        kana: "め",
        romaji: &["me"],
        group: 7,
        mnemonic: Some("Un œil (« me » en japonais), sans boucle au bout. (ぬ a une boucle.)"),
    },
    Kana {
        kana: "も",
        romaji: &["mo"],
        group: 7,
        mnemonic: None,
    },
    // group 8: y
    Kana {
        kana: "や",
        romaji: &["ya"],
        group: 8,
        mnemonic: Some("Un yak avec ses cornes : ya."),
    },
    Kana {
        kana: "ゆ",
        romaji: &["yu"],
        group: 8,
        mnemonic: None,
    },
    Kana {
        kana: "よ",
        romaji: &["yo"],
        group: 8,
        mnemonic: Some("Un yo-yo au bout de sa ficelle : yo."),
    },
    // group 9: r
    Kana {
        kana: "ら",
        romaji: &["ra"],
        group: 9,
        mnemonic: None,
    },
    Kana {
        kana: "り",
        romaji: &["ri"],
        group: 9,
        mnemonic: Some(
            "Deux traits, celui de droite bien plus long : ri. (い a deux traits presque égaux.)",
        ),
    },
    Kana {
        kana: "る",
        romaji: &["ru"],
        group: 9,
        mnemonic: Some("Une route qui finit en rond-point : ru. (ろ n'a pas de boucle.)"),
    },
    Kana {
        kana: "れ",
        romaji: &["re"],
        group: 9,
        mnemonic: Some("La fin part vers l'extérieur, comme une jambe : re. (わ revient en rond.)"),
    },
    Kana {
        kana: "ろ",
        romaji: &["ro"],
        group: 9,
        mnemonic: Some("Comme る, mais sans boucle : ro."),
    },
    // group 10: w + n
    Kana {
        kana: "わ",
        romaji: &["wa"],
        group: 10,
        mnemonic: Some(
            "La fin revient en rond : wa. (れ part vers l'extérieur, ね finit par une boucle.)",
        ),
    },
    Kana {
        kana: "を",
        romaji: &["wo", "o"],
        group: 10,
        mnemonic: None,
    },
    Kana {
        kana: "ん",
        romaji: &["n", "nn"],
        group: 10,
        mnemonic: Some("Un « n » minuscule écrit à la va-vite : n."),
    },
];

pub const KATAKANA: &[Kana] = &[
    // group 1: voyelles
    Kana {
        kana: "ア",
        romaji: &["a"],
        group: 1,
        mnemonic: None,
    },
    Kana {
        kana: "イ",
        romaji: &["i"],
        group: 1,
        mnemonic: None,
    },
    Kana {
        kana: "ウ",
        romaji: &["u"],
        group: 1,
        mnemonic: Some("Comme う, avec un petit trait sur la tête : u. (ワ n'a pas ce trait.)"),
    },
    Kana {
        kana: "エ",
        romaji: &["e"],
        group: 1,
        mnemonic: None,
    },
    Kana {
        kana: "オ",
        romaji: &["o"],
        group: 1,
        mnemonic: None,
    },
    // group 2: k
    Kana {
        kana: "カ",
        romaji: &["ka"],
        group: 2,
        mnemonic: None,
    },
    Kana {
        kana: "キ",
        romaji: &["ki"],
        group: 2,
        mnemonic: None,
    },
    Kana {
        kana: "ク",
        romaji: &["ku"],
        group: 2,
        mnemonic: Some(
            "Comme く avec un petit toit : ku. (タ a un trait en plus dedans, ケ une barre qui dépasse.)",
        ),
    },
    Kana {
        kana: "ケ",
        romaji: &["ke"],
        group: 2,
        mnemonic: Some(
            "Une barre qui dépasse à droite, avec une jambe qui descend : ke. (ク n'a pas de barre.)",
        ),
    },
    Kana {
        kana: "コ",
        romaji: &["ko"],
        group: 2,
        mnemonic: Some(
            "Un carré ouvert à gauche, net et fermé : ko. (ユ a la barre du bas qui dépasse.)",
        ),
    },
    // group 3: s
    Kana {
        kana: "サ",
        romaji: &["sa"],
        group: 3,
        mnemonic: None,
    },
    Kana {
        kana: "シ",
        romaji: &["shi", "si"],
        group: 3,
        mnemonic: Some(
            "Les deux petits traits sont à gauche, l'un sous l'autre, et le grand remonte depuis le bas : shi. (ツ a ses petits traits en haut.)",
        ),
    },
    Kana {
        kana: "ス",
        romaji: &["su"],
        group: 3,
        mnemonic: Some("Un tabouret à deux pieds écartés : su. (ヌ a un trait qui le traverse.)"),
    },
    Kana {
        kana: "セ",
        romaji: &["se"],
        group: 3,
        mnemonic: None,
    },
    Kana {
        kana: "ソ",
        romaji: &["so"],
        group: 3,
        mnemonic: Some(
            "Le grand trait descend depuis le haut, comme une goutte qui tombe : so. (ン remonte.)",
        ),
    },
    // group 4: t
    Kana {
        kana: "タ",
        romaji: &["ta"],
        group: 4,
        mnemonic: Some("ク avec un petit trait ajouté à l'intérieur : ta."),
    },
    Kana {
        kana: "チ",
        romaji: &["chi", "ti"],
        group: 4,
        mnemonic: Some(
            "Ressemble à ち, et le trait du milieu traverse la barre : chi. (テ a deux barres séparées.)",
        ),
    },
    Kana {
        kana: "ツ",
        romaji: &["tsu", "tu"],
        group: 4,
        mnemonic: Some(
            "Les deux petits traits sont côte à côte en haut, et le grand descend depuis le haut : tsu. (シ a ses petits traits à gauche.)",
        ),
    },
    Kana {
        kana: "テ",
        romaji: &["te"],
        group: 4,
        mnemonic: Some(
            "Deux barres et un pied qui descend, comme une table : te. (チ a un trait qui traverse.)",
        ),
    },
    Kana {
        kana: "ト",
        romaji: &["to"],
        group: 4,
        mnemonic: None,
    },
    // group 5: n
    Kana {
        kana: "ナ",
        romaji: &["na"],
        group: 5,
        mnemonic: None,
    },
    Kana {
        kana: "ニ",
        romaji: &["ni"],
        group: 5,
        mnemonic: None,
    },
    Kana {
        kana: "ヌ",
        romaji: &["nu"],
        group: 5,
        mnemonic: Some(
            "Comme ス, mais traversé par un trait, comme une nouille piquée par une baguette : nu.",
        ),
    },
    Kana {
        kana: "ネ",
        romaji: &["ne"],
        group: 5,
        mnemonic: None,
    },
    Kana {
        kana: "ノ",
        romaji: &["no"],
        group: 5,
        mnemonic: None,
    },
    // group 6: h
    Kana {
        kana: "ハ",
        romaji: &["ha"],
        group: 6,
        mnemonic: None,
    },
    Kana {
        kana: "ヒ",
        romaji: &["hi"],
        group: 6,
        mnemonic: None,
    },
    Kana {
        kana: "フ",
        romaji: &["fu", "hu"],
        group: 6,
        mnemonic: None,
    },
    Kana {
        kana: "ヘ",
        romaji: &["he"],
        group: 6,
        mnemonic: None,
    },
    Kana {
        kana: "ホ",
        romaji: &["ho"],
        group: 6,
        mnemonic: None,
    },
    // group 7: m
    Kana {
        kana: "マ",
        romaji: &["ma"],
        group: 7,
        mnemonic: None,
    },
    Kana {
        kana: "ミ",
        romaji: &["mi"],
        group: 7,
        mnemonic: None,
    },
    Kana {
        kana: "ム",
        romaji: &["mu"],
        group: 7,
        mnemonic: None,
    },
    Kana {
        kana: "メ",
        romaji: &["me"],
        group: 7,
        mnemonic: None,
    },
    Kana {
        kana: "モ",
        romaji: &["mo"],
        group: 7,
        mnemonic: None,
    },
    // group 8: y
    Kana {
        kana: "ヤ",
        romaji: &["ya"],
        group: 8,
        mnemonic: None,
    },
    Kana {
        kana: "ユ",
        romaji: &["yu"],
        group: 8,
        mnemonic: Some("Un « U » couché dont la barre du bas dépasse : yu. (コ ne dépasse pas.)"),
    },
    Kana {
        kana: "ヨ",
        romaji: &["yo"],
        group: 8,
        mnemonic: None,
    },
    // group 9: r
    Kana {
        kana: "ラ",
        romaji: &["ra"],
        group: 9,
        mnemonic: None,
    },
    Kana {
        kana: "リ",
        romaji: &["ri"],
        group: 9,
        mnemonic: None,
    },
    Kana {
        kana: "ル",
        romaji: &["ru"],
        group: 9,
        mnemonic: None,
    },
    Kana {
        kana: "レ",
        romaji: &["re"],
        group: 9,
        mnemonic: None,
    },
    Kana {
        kana: "ロ",
        romaji: &["ro"],
        group: 9,
        mnemonic: None,
    },
    // group 10: w + n
    Kana {
        kana: "ワ",
        romaji: &["wa"],
        group: 10,
        mnemonic: Some("Comme ウ sans le petit trait du haut, et proche de わ : wa."),
    },
    Kana {
        kana: "ヲ",
        romaji: &["wo", "o"],
        group: 10,
        mnemonic: None,
    },
    Kana {
        kana: "ン",
        romaji: &["n", "nn"],
        group: 10,
        mnemonic: Some(
            "Le grand trait remonte depuis le bas, comme un sourire en coin : n. (ソ descend.)",
        ),
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
    &["シ", "ツ"],
    &["ソ", "ン"],
    &["ク", "ケ", "タ"],
    &["ス", "ヌ"],
    &["コ", "ユ"],
    &["チ", "テ"],
    &["ウ", "ワ"],
];

/// Which syllabary to study. Each one has its own table and its own deck.
#[derive(Clone, Copy, ValueEnum)]
pub enum Script {
    Hiragana,
    Katakana,
}

impl Script {
    pub fn table(self) -> &'static [Kana] {
        match self {
            Script::Hiragana => HIRAGANA,
            Script::Katakana => KATAKANA,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Script::Hiragana => "hiragana",
            Script::Katakana => "katakana",
        }
    }
}

/// Kana that look like `kana`, without `kana` itself.
pub fn look_alikes(kana: &str) -> impl Iterator<Item = &'static str> {
    CONFUSABLES
        .iter()
        .filter(move |set| set.contains(&kana))
        .flat_map(|set| set.iter().copied())
        .filter(move |&other| other != kana)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_have_46_kana_in_10_groups() {
        for table in [HIRAGANA, KATAKANA] {
            assert_eq!(table.len(), 46);
            for group in 1..=10 {
                let n = table.iter().filter(|k| k.group == group).count();
                let expected = if group == 8 || group == 10 { 3 } else { 5 };
                assert_eq!(n, expected, "group {group}");
            }
        }
    }

    #[test]
    fn tables_line_up() {
        for (h, k) in HIRAGANA.iter().zip(KATAKANA) {
            assert_eq!(h.romaji, k.romaji);
            assert_eq!(h.group, k.group);
        }
    }

    #[test]
    fn every_look_alike_exists_and_has_a_mnemonic() {
        let all: Vec<&Kana> = HIRAGANA.iter().chain(KATAKANA).collect();
        for set in CONFUSABLES {
            for &kana in *set {
                let k = all.iter().find(|k| k.kana == kana);
                assert!(k.is_some_and(|k| k.mnemonic.is_some()), "{kana}");
            }
        }
    }
}
