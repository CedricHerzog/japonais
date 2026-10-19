use clap::ValueEnum;

pub struct Kana {
    pub kana: &'static str,
    pub romaji: &'static [&'static str],
    pub group: u8,
    pub mnemonic: &'static str, // French, matches the illustrated card
}

pub const HIRAGANA: &[Kana] = &[
    // group 1: voyelles
    Kana {
        kana: "あ",
        romaji: &["a"],
        group: 1,
        mnemonic: "Un avion fait un looping et laisse une boucle de fumée ; les passagers crient « Aaah ! » (a)",
    },
    Kana {
        kana: "い",
        romaji: &["i"],
        group: 1,
        mnemonic: "Deux iris aux longues tiges, un grand et un petit : « i-ris » (i) (り a le trait de droite bien plus long.)",
    },
    Kana {
        kana: "う",
        romaji: &["u"],
        group: 1,
        mnemonic: "Un surfeur saute au-dessus d'une vague qui s'enroule : « Ouh ! » (u)",
    },
    Kana {
        kana: "え",
        romaji: &["e"],
        group: 1,
        mnemonic: "Un éclair zigzague du nuage jusqu'au sol : « É-clair ! » (e)",
    },
    Kana {
        kana: "お",
        romaji: &["o"],
        group: 1,
        mnemonic: "Une autruche passe le cou par-dessus la barrière, son gros ventre derrière : « Oh ! » (o)",
    },
    // group 2: k
    Kana {
        kana: "か",
        romaji: &["ka"],
        group: 2,
        mnemonic: "Une catapulte lance son boulet sur le château : « Ka-boum ! » (ka)",
    },
    Kana {
        kana: "き",
        romaji: &["ki"],
        group: 2,
        mnemonic: "Un kimono suspendu à sa perche : les manches, la ceinture obi, le col croisé et le pan du bas : « ki-mono » (ki) (さ n'a qu'une barre.)",
    },
    Kana {
        kana: "く",
        romaji: &["ku"],
        group: 2,
        mnemonic: "Main sur la hanche, il pointe le coude : « kou-de » (ku)",
    },
    Kana {
        kana: "け",
        romaji: &["ke"],
        group: 2,
        mnemonic: "La broche d'un kebab et son grand couteau : « ké-bab » (ke)",
    },
    Kana {
        kana: "こ",
        romaji: &["ko"],
        group: 2,
        mnemonic: "Un crocodile ouvre grand ses deux mâchoires : « cro-co » (ko) (Avec un tronc vertical devant, c'est に : ni.)",
    },
    // group 3: s
    Kana {
        kana: "さ",
        romaji: &["sa"],
        group: 3,
        mnemonic: "Une sardine vient mordre à l'hameçon sous la surface : « sar-dine » (sa) (ち ressemble à un 5.)",
    },
    Kana {
        kana: "し",
        romaji: &["shi", "si"],
        group: 3,
        mnemonic: "Un piment chili tout rouge, recourbé au bout ; ça pique : « chi ! » (shi/si)",
    },
    Kana {
        kana: "す",
        romaji: &["su"],
        group: 3,
        mnemonic: "Une souris descend le long d'une corde, sa longue queue pendante : « sou-ris » (su)",
    },
    Kana {
        kana: "せ",
        romaji: &["se"],
        group: 3,
        mnemonic: "Des serviettes sèchent sur le séchoir à linge : « sè-che » (se)",
    },
    Kana {
        kana: "そ",
        romaji: &["so"],
        group: 3,
        mnemonic: "Un chat roulé en boule fait un somme sous un grand « Z » : « so-mme » (so)",
    },
    // group 4: t
    Kana {
        kana: "た",
        romaji: &["ta"],
        group: 4,
        mnemonic: "Deux baguettes croisées frappent un tambour : « ta-ta-ta ! » (ta)",
    },
    Kana {
        kana: "ち",
        romaji: &["chi", "ti"],
        group: 4,
        mnemonic: "Un litchi bien rond pend à sa branche : « li-tchi » (chi/ti) (Ventre à droite comme un 5 ; さ s'ouvre comme un « c ».)",
    },
    Kana {
        kana: "つ",
        romaji: &["tsu", "tu"],
        group: 4,
        mnemonic: "Un croissant de lune ; la lune se dit « tsuki » en japonais : tsu (tsu/tu)",
    },
    Kana {
        kana: "て",
        romaji: &["te"],
        group: 4,
        mnemonic: "La cabine d'un téléphérique pend à son câble : « té-lé » (te)",
    },
    Kana {
        kana: "と",
        romaji: &["to"],
        group: 4,
        mnemonic: "Une taupe creuse son tunnel sous une carotte : « taupe » (to)",
    },
    // group 5: n
    Kana {
        kana: "な",
        romaji: &["na"],
        group: 5,
        mnemonic: "Un épouvantail garde un gros navet dans le potager : « na-vet » (na)",
    },
    Kana {
        kana: "に",
        romaji: &["ni"],
        group: 5,
        mnemonic: "Un nid plein d'oisillons contre le tronc d'un arbre : « nid » (ni) (Sans le tronc vertical, c'est こ : ko.)",
    },
    Kana {
        kana: "ぬ",
        romaji: &["nu"],
        group: 5,
        mnemonic: "Un ruban bien noué sur un cadeau, avec une petite boucle au bout : « nou-é » (nu) (め n'a pas de boucle.)",
    },
    Kana {
        kana: "ね",
        romaji: &["ne"],
        group: 5,
        mnemonic: "Nessie, le monstre du Loch Ness, sort son long cou et sa bosse du lac : « Né-ssie » (ne) (れ et わ n'ont pas de boucle.)",
    },
    Kana {
        kana: "の",
        romaji: &["no"],
        group: 5,
        mnemonic: "La coquille enroulée d'un nautile : « nau-tile » (no)",
    },
    // group 6: h
    Kana {
        kana: "は",
        romaji: &["ha"],
        group: 6,
        mnemonic: "Une ligne de pêche plonge dans l'étang, l'hameçon en boucle au bout : un poisson s'écrie « Ha ! » (ha) (ほ a un toit en plus.)",
    },
    Kana {
        kana: "ひ",
        romaji: &["hi"],
        group: 6,
        mnemonic: "Un hibou se blottit au creux de son nid tout rond : « hi »bou (hi)",
    },
    Kana {
        kana: "ふ",
        romaji: &["fu", "hu"],
        group: 6,
        mnemonic: "Un feu de camp s'allume d'un coup : « Fouf ! » (fu/hu)",
    },
    Kana {
        kana: "へ",
        romaji: &["he"],
        group: 6,
        mnemonic: "Un hérisson fait le gros dos : « Hé, pas touche ! » (he)",
    },
    Kana {
        kana: "ほ",
        romaji: &["ho"],
        group: 6,
        mnemonic: "Un scout hisse le drapeau entre les deux barres : « Oh hisse ! » (ho) (は n'a qu'une barre, pas de toit.)",
    },
    // group 7: m
    Kana {
        kana: "ま",
        romaji: &["ma"],
        group: 7,
        mnemonic: "Le grand mât d'un voilier, avec ses deux vergues et sa voile : « ma »t (ma)",
    },
    Kana {
        kana: "み",
        romaji: &["mi"],
        group: 7,
        mnemonic: "Un chaton joue avec une pelote de laine qui se déroule : « Miaou ! » (mi)",
    },
    Kana {
        kana: "む",
        romaji: &["mu"],
        group: 7,
        mnemonic: "Un moulin à vent sur sa colline, les ailes en croix : « mou »lin (mu)",
    },
    Kana {
        kana: "め",
        romaji: &["me"],
        group: 7,
        mnemonic: "Une méduse ondule, ses tentacules emmêlés : « mé »duse (me) (ぬ a une boucle au bout.)",
    },
    Kana {
        kana: "も",
        romaji: &["mo"],
        group: 7,
        mnemonic: "Il ne reste de la morue que la tête et les arêtes : « mo »rue (mo)",
    },
    // group 8: y
    Kana {
        kana: "や",
        romaji: &["ya"],
        group: 8,
        mnemonic: "Un « ya »cht file au vent, le mât penché, la voile gonflée et le fanion au sommet (ya)",
    },
    Kana {
        kana: "ゆ",
        romaji: &["yu"],
        group: 8,
        mnemonic: "Une yourte ronde, son mât central dépasse du toit et fume : « you »rte (yu)",
    },
    Kana {
        kana: "よ",
        romaji: &["yo"],
        group: 8,
        mnemonic: "Un yogi assis en tailleur, le dos bien droit, tend le bras : « yo »ga (yo)",
    },
    // group 9: r
    Kana {
        kana: "ら",
        romaji: &["ra"],
        group: 9,
        mnemonic: "Un rat dodu dresse la queue devant un fromage : « ra » ! (ra)",
    },
    Kana {
        kana: "り",
        romaji: &["ri"],
        group: 9,
        mnemonic: "Deux baguettes plongent dans un bol de riz, celle de droite plus longue : « ri » (ri) (い a deux traits presque égaux.)",
    },
    Kana {
        kana: "る",
        romaji: &["ru"],
        group: 9,
        mnemonic: "Une gymnaste fait tourbillonner son ruban, qui finit par une petite boucle : « ru »ban (ru) (ろ n'a pas de boucle.)",
    },
    Kana {
        kana: "れ",
        romaji: &["re"],
        group: 9,
        mnemonic: "Un renne tend sa patte arrière vers l'extérieur : « re »nne (re) (わ revient en rond.)",
    },
    Kana {
        kana: "ろ",
        romaji: &["ro"],
        group: 9,
        mnemonic: "Bien calé, le ventre tout rond, un gourmand lâche un rot : « Rôô ! » (ro) (る a une boucle en plus.)",
    },
    // group 10: w + n
    Kana {
        kana: "わ",
        romaji: &["wa"],
        group: 10,
        mnemonic: "Un chien assis, le dos tout rond, aboie : « Ouah ! » (wa) (れ tend la patte vers l'extérieur, ね finit par une boucle.)",
    },
    Kana {
        kana: "を",
        romaji: &["wo", "o"],
        group: 10,
        mnemonic: "Un skateur s'envole sur sa planche : « Wo-ho ! » (wo/o)",
    },
    Kana {
        kana: "ん",
        romaji: &["n", "nn"],
        group: 10,
        mnemonic: "Un gourmand aspire une longue nouille au-dessus du bol : « Nnn ! » (n/nn)",
    },
];

pub const KATAKANA: &[Kana] = &[
    // group 1: voyelles
    Kana {
        kana: "ア",
        romaji: &["a"],
        group: 1,
        mnemonic: "Un enfant dévale le grand toboggan en criant « Aaah ! » (a)",
    },
    Kana {
        kana: "イ",
        romaji: &["i"],
        group: 1,
        mnemonic: "Un palmier pousse sur une petite île : le trait penché est sa grande palme, le trait droit son tronc. « Île » (i)",
    },
    Kana {
        kana: "ウ",
        romaji: &["u"],
        group: 1,
        mnemonic: "Un hibou sur sa branche, une petite plume dressée sur la tête, hulule « Hou ! » (u) (ワ n'a pas cette plume sur la tête.)",
    },
    Kana {
        kana: "エ",
        romaji: &["e"],
        group: 1,
        mnemonic: "Un éléphant de cirque tient en équilibre sur son piédestal : « éléphant » (e)",
    },
    Kana {
        kana: "オ",
        romaji: &["o"],
        group: 1,
        mnemonic: "Un voilier avec son mât, sa vergue et sa voile en triangle : « Ohé, bateau ! » (o)",
    },
    // group 2: k
    Kana {
        kana: "カ",
        romaji: &["ka"],
        group: 2,
        mnemonic: "Une canne à pêche posée sur la berge : le fil plonge dans l'eau et finit en hameçon. « Canne » (ka)",
    },
    Kana {
        kana: "キ",
        romaji: &["ki"],
        group: 2,
        mnemonic: "Au ski, un poteau planté dans la neige porte deux panneaux de piste : « ski » (ki)",
    },
    Kana {
        kana: "ク",
        romaji: &["ku"],
        group: 2,
        mnemonic: "Un oiseau ouvre grand le bec pour chanter « Coucou ! » (ku) (ケ a une barre qui dépasse, タ un trait en plus dedans.)",
    },
    Kana {
        kana: "ケ",
        romaji: &["ke"],
        group: 2,
        mnemonic: "Au port, le ponton du quai dépasse au-dessus de l'eau : « quai » (ke) (ク n'a pas de barre qui dépasse.)",
    },
    Kana {
        kana: "コ",
        romaji: &["ko"],
        group: 2,
        mnemonic: "Un cochon dans son enclos bien fermé, ouvert seulement à gauche : « cochon » (ko) (ユ a la barre du bas qui dépasse.)",
    },
    // group 3: s
    Kana {
        kana: "サ",
        romaji: &["sa"],
        group: 3,
        mnemonic: "Un mouton saute la barrière du pré : « Saute ! » (sa)",
    },
    Kana {
        kana: "シ",
        romaji: &["shi", "si"],
        group: 3,
        mnemonic: "Des baguettes remontent un sushi depuis le bas ; deux gouttes de sauce soja tombent à gauche : « sushi » (shi/si) (ツ a ses petits traits en haut.)",
    },
    Kana {
        kana: "ス",
        romaji: &["su"],
        group: 3,
        mnemonic: "Un sumo, bras tendus et jambes bien écartées : « sumo » (su) (ヌ a un trait qui le traverse.)",
    },
    Kana {
        kana: "セ",
        romaji: &["se"],
        group: 3,
        mnemonic: "Au puits, un seau pend au crochet de la potence : « seau » (se)",
    },
    Kana {
        kana: "ソ",
        romaji: &["so"],
        group: 3,
        mnemonic: "Une bouteille de soda se renverse : le jet descend depuis le haut, une goutte gicle à côté. « Soda » (so) (ン remonte depuis le bas.)",
    },
    // group 4: t
    Kana {
        kana: "タ",
        romaji: &["ta"],
        group: 4,
        mnemonic: "Une part de tarte soulevée sur sa pelle ; le petit trait en plus ferme la part : « tarte » (ta) (ク n'a pas ce trait.)",
    },
    Kana {
        kana: "チ",
        romaji: &["chi", "ti"],
        group: 4,
        mnemonic: "Un piment chili pend à sa branche, et sa queue traverse la branche : « chili » (chi/ti) (テ a deux barres séparées, rien ne les traverse.)",
    },
    Kana {
        kana: "ツ",
        romaji: &["tsu", "tu"],
        group: 4,
        mnemonic: "Un tsunami : la grande vague retombe depuis le haut, deux paquets d'eau volent côte à côte au-dessus. « Tsunami » (tsu/tu) (シ a ses petits traits à gauche.)",
    },
    Kana {
        kana: "テ",
        romaji: &["te"],
        group: 4,
        mnemonic: "Une antenne de télé sur le toit : deux barres séparées et un mât qui descend. « Télé » (te) (チ a un trait qui traverse la barre.)",
    },
    Kana {
        kana: "ト",
        romaji: &["to"],
        group: 4,
        mnemonic: "Un plant de tomate : la tige droite et une branche d'où pend une tomate. « Tomate » (to)",
    },
    // group 5: n
    Kana {
        kana: "ナ",
        romaji: &["na"],
        group: 5,
        mnemonic: "Un katana : la poignée en haut, la garde en travers et la lame courbe en dessous. « Katana » (na)",
    },
    Kana {
        kana: "ニ",
        romaji: &["ni"],
        group: 5,
        mnemonic: "Deux branches l'une sur l'autre, avec un nid sur celle du bas : « nid » (ni)",
    },
    Kana {
        kana: "ヌ",
        romaji: &["nu"],
        group: 5,
        mnemonic: "Une longue nouille soulevée du bol, traversée par une baguette : « nouille » (nu) (ス n'a pas de trait qui le traverse.)",
    },
    Kana {
        kana: "ネ",
        romaji: &["ne"],
        group: 5,
        mnemonic: "Un petit arbre sous la neige, sa branche plie sous le poids : « neige » (ne)",
    },
    Kana {
        kana: "ノ",
        romaji: &["no"],
        group: 5,
        mnemonic: "La nuit de Noël, le traîneau descend du ciel en laissant une traînée dorée : « Noël » (no)",
    },
    // group 6: h
    Kana {
        kana: "ハ",
        romaji: &["ha"],
        group: 6,
        mnemonic: "D'un coup de hache, la bûche se fend en deux moitiés qui s'écartent : « Ha ! » (ha)",
    },
    Kana {
        kana: "ヒ",
        romaji: &["hi"],
        group: 6,
        mnemonic: "Un hibou s'est perché sur la branche de l'arbre mort : hi-bou ! (hi)",
    },
    Kana {
        kana: "フ",
        romaji: &["fu", "hu"],
        group: 6,
        mnemonic: "Le dompteur fait claquer son fouet devant le lion : « Fou-et ! » (fu/hu)",
    },
    Kana {
        kana: "ヘ",
        romaji: &["he"],
        group: 6,
        mnemonic: "Au sommet de la montagne, le randonneur crie « Hé ! » et l'écho lui répond. (he)",
    },
    Kana {
        kana: "ホ",
        romaji: &["ho"],
        group: 6,
        mnemonic: "Bras écartés, l'épouvantail chasse les corbeaux : « Ho ! Ho ! » (ho)",
    },
    // group 7: m
    Kana {
        kana: "マ",
        romaji: &["ma"],
        group: 7,
        mnemonic: "L'enfant tend son cornet surmonté d'une boule à la fraise : « Ma glace ! » (ma)",
    },
    Kana {
        kana: "ミ",
        romaji: &["mi"],
        group: 7,
        mnemonic: "Le chat a laissé trois griffures sur le canapé : « Miaou ! » (mi)",
    },
    Kana {
        kana: "ム",
        romaji: &["mu"],
        group: 7,
        mnemonic: "Sous ce grand nez pointu pousse une belle moustache : « Mou-stache ! » (mu)",
    },
    Kana {
        kana: "メ",
        romaji: &["me"],
        group: 7,
        mnemonic: "Le mouton saute par-dessus la barrière en croix : « Mêêê ! » (me)",
    },
    Kana {
        kana: "モ",
        romaji: &["mo"],
        group: 7,
        mnemonic: "Dans l'assiette, il ne reste que la tête et les arêtes de la morue : mo ! (mo)",
    },
    // group 8: y
    Kana {
        kana: "ヤ",
        romaji: &["ya"],
        group: 8,
        mnemonic: "Penché sous sa voile, le véliplanchiste fonce sur la vague : « Yaaa ! » (ya)",
    },
    Kana {
        kana: "ユ",
        romaji: &["yu"],
        group: 8,
        mnemonic: "Agrippé à la poignée de sa luge aux longs patins qui dépassent, l'enfant file dans la neige : « You-pi ! » (yu) (ユ a la barre du bas qui dépasse, コ non.)",
    },
    Kana {
        kana: "ヨ",
        romaji: &["yo"],
        group: 8,
        mnemonic: "Sur l'étagère fixée au mur, on range ses yo-yos : yo ! (yo)",
    },
    // group 9: r
    Kana {
        kana: "ラ",
        romaji: &["ra"],
        group: 9,
        mnemonic: "Un rat assis sur l'étagère lorgne le fromage, sa longue queue pend jusqu'en bas : ra ! (ra)",
    },
    Kana {
        kana: "リ",
        romaji: &["ri"],
        group: 9,
        mnemonic: "Le vent gonfle les deux rideaux de la fenêtre, un court et un long : ri-deaux ! (ri)",
    },
    Kana {
        kana: "ル",
        romaji: &["ru"],
        group: 9,
        mnemonic: "Un coureur file à grandes enjambées dans la rue : ru ! (ru)",
    },
    Kana {
        kana: "レ",
        romaji: &["re"],
        group: 9,
        mnemonic: "Allongé sur son transat, il rêve d'une île au soleil : rê-ve ! (re)",
    },
    Kana {
        kana: "ロ",
        romaji: &["ro"],
        group: 9,
        mnemonic: "Ce robot a une tête toute carrée : ro-bot ! (ro)",
    },
    // group 10: w + n
    Kana {
        kana: "ワ",
        romaji: &["wa"],
        group: 10,
        mnemonic: "Le wagonnet arrive en haut des montagnes russes et bascule dans la descente : « Waaah ! » (wa) (ウ a un petit trait sur la tête, ワ non.)",
    },
    Kana {
        kana: "ヲ",
        romaji: &["wo", "o"],
        group: 10,
        mnemonic: "Du haut du toboggan, l'enfant glisse et plonge dans l'eau : « Oh ! » (wo/o)",
    },
    Kana {
        kana: "ン",
        romaji: &["n", "nn"],
        group: 10,
        mnemonic: "Clin d'œil et sourire en coin qui remonte vers la droite : « Hmm ! » (n/nn) (ン remonte, ソ descend.)",
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
#[derive(Clone, Copy, PartialEq, ValueEnum)]
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
    fn every_look_alike_exists() {
        let all: Vec<&Kana> = HIRAGANA.iter().chain(KATAKANA).collect();
        for set in CONFUSABLES {
            for &kana in *set {
                assert!(all.iter().any(|k| k.kana == kana), "{kana}");
            }
        }
    }

    #[test]
    fn every_mnemonic_names_the_sound() {
        for k in HIRAGANA.iter().chain(KATAKANA) {
            let sound = format!("({})", k.romaji.join("/"));
            assert!(k.mnemonic.contains(&sound), "{}: {}", k.kana, k.mnemonic);
        }
    }
}
