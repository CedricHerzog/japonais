use clap::ValueEnum;

pub struct Kana {
    pub kana: &'static str,
    pub romaji: &'static [&'static str],
    pub group: u8,
    pub mnemonic: &'static str, // French, matches the illustrated card
    pub example: Example,
}

/// A real Japanese word that starts with the kana, as bonus vocabulary.
pub struct Example {
    pub word: &'static str,
    pub romaji: &'static str,
    pub fr: &'static str, // French translation
}

pub const HIRAGANA: &[Kana] = &[
    // group 1: voyelles
    Kana {
        kana: "あ",
        romaji: &["a"],
        group: 1,
        mnemonic: "Un avion qui fait un looping : « a »vion (a)",
        example: Example {
            word: "あめ",
            romaji: "ame",
            fr: "pluie",
        },
    },
    Kana {
        kana: "い",
        romaji: &["i"],
        group: 1,
        mnemonic: "Deux iris à longues tiges : « i »ris (i) (り a le trait de droite bien plus long.)",
        example: Example {
            word: "いぬ",
            romaji: "inu",
            fr: "chien",
        },
    },
    Kana {
        kana: "う",
        romaji: &["u"],
        group: 1,
        mnemonic: "Une grosse vague qui s'enroule, la mer en japonais : « u »mi (u)",
        example: Example {
            word: "うみ",
            romaji: "umi",
            fr: "mer",
        },
    },
    Kana {
        kana: "え",
        romaji: &["e"],
        group: 1,
        mnemonic: "Un éclair qui zigzague sous un nuage : « é »clair (e)",
        example: Example {
            word: "えき",
            romaji: "eki",
            fr: "gare",
        },
    },
    Kana {
        kana: "お",
        romaji: &["o"],
        group: 1,
        mnemonic: "Une autruche au long cou et au gros ventre rond : « au »truche (o)",
        example: Example {
            word: "おちゃ",
            romaji: "ocha",
            fr: "thé",
        },
    },
    // group 2: k
    Kana {
        kana: "か",
        romaji: &["ka"],
        group: 2,
        mnemonic: "Une catapulte, bras crochu et boulet lancé : « ca »tapulte (ka)",
        example: Example {
            word: "かさ",
            romaji: "kasa",
            fr: "parapluie",
        },
    },
    Kana {
        kana: "き",
        romaji: &["ki"],
        group: 2,
        mnemonic: "Un kimono suspendu, manches, obi et col croisé : « ki »mono (ki) (さ n'a qu'une barre.)",
        example: Example {
            word: "きもの",
            romaji: "kimono",
            fr: "kimono",
        },
    },
    Kana {
        kana: "く",
        romaji: &["ku"],
        group: 2,
        mnemonic: "Un bras plié, main sur la hanche : « cou »de (ku)",
        example: Example {
            word: "くつ",
            romaji: "kutsu",
            fr: "chaussure",
        },
    },
    Kana {
        kana: "け",
        romaji: &["ke"],
        group: 2,
        mnemonic: "La broche d'un kebab et son grand couteau : « ke »bab (ke)",
        example: Example {
            word: "けむり",
            romaji: "kemuri",
            fr: "fumée",
        },
    },
    Kana {
        kana: "こ",
        romaji: &["ko"],
        group: 2,
        mnemonic: "Un coquillage entrouvert avec une perle : « co »quillage (ko) (Avec un trait vertical devant, c'est に : ni.)",
        example: Example {
            word: "こども",
            romaji: "kodomo",
            fr: "enfant",
        },
    },
    // group 3: s
    Kana {
        kana: "さ",
        romaji: &["sa"],
        group: 3,
        mnemonic: "Une sardine qui mord à l'hameçon : « sa »rdine (sa) (ち ressemble à un 5.)",
        example: Example {
            word: "さかな",
            romaji: "sakana",
            fr: "poisson",
        },
    },
    Kana {
        kana: "し",
        romaji: &["shi", "si"],
        group: 3,
        mnemonic: "Un piment rouge recourbé au bout : « chi »li (shi/si)",
        example: Example {
            word: "しお",
            romaji: "shio",
            fr: "sel",
        },
    },
    Kana {
        kana: "す",
        romaji: &["su"],
        group: 3,
        mnemonic: "Une souris sur une corde, sa longue queue pendante : « sou »ris (su)",
        example: Example {
            word: "すし",
            romaji: "sushi",
            fr: "sushi",
        },
    },
    Kana {
        kana: "せ",
        romaji: &["se"],
        group: 3,
        mnemonic: "Des serviettes qui sèchent sur un séchoir : « se »rviettes (se)",
        example: Example {
            word: "せなか",
            romaji: "senaka",
            fr: "dos",
        },
    },
    Kana {
        kana: "そ",
        romaji: &["so"],
        group: 3,
        mnemonic: "Un chat roulé en boule qui dort sous un grand « Z » : « so »mmeil (so)",
        example: Example {
            word: "そら",
            romaji: "sora",
            fr: "ciel",
        },
    },
    // group 4: t
    Kana {
        kana: "た",
        romaji: &["ta"],
        group: 4,
        mnemonic: "Deux baguettes croisées et un tambour : « ta »mbour (ta)",
        example: Example {
            word: "たいこ",
            romaji: "taiko",
            fr: "tambour",
        },
    },
    Kana {
        kana: "ち",
        romaji: &["chi", "ti"],
        group: 4,
        mnemonic: "Un chihuahua assis, grande oreille et dos rond : « chi »huahua (chi/ti) (Ventre à droite comme un 5 ; さ s'ouvre comme un « c ».)",
        example: Example {
            word: "ちいさい",
            romaji: "chiisai",
            fr: "petit",
        },
    },
    Kana {
        kana: "つ",
        romaji: &["tsu", "tu"],
        group: 4,
        mnemonic: "Un croissant de lune, la lune en japonais : « tsu »ki (tsu/tu)",
        example: Example {
            word: "つき",
            romaji: "tsuki",
            fr: "lune",
        },
    },
    Kana {
        kana: "て",
        romaji: &["te"],
        group: 4,
        mnemonic: "La cabine d'un téléphérique pendue à son câble : « té »léphérique (te)",
        example: Example {
            word: "てがみ",
            romaji: "tegami",
            fr: "lettre",
        },
    },
    Kana {
        kana: "と",
        romaji: &["to"],
        group: 4,
        mnemonic: "Une taupe au museau pointu qui gratte la terre : « tau »pe (to)",
        example: Example {
            word: "とり",
            romaji: "tori",
            fr: "oiseau",
        },
    },
    // group 5: n
    Kana {
        kana: "な",
        romaji: &["na"],
        group: 5,
        mnemonic: "Un navet, son bulbe rond et ses feuilles : « na »vet (na)",
        example: Example {
            word: "なす",
            romaji: "nasu",
            fr: "aubergine",
        },
    },
    Kana {
        kana: "に",
        romaji: &["ni"],
        group: 5,
        mnemonic: "Un nid d'oisillons contre le tronc d'un arbre : « ni »d (ni) (Sans le trait vertical, c'est こ : ko.)",
        example: Example {
            word: "にわ",
            romaji: "niwa",
            fr: "jardin",
        },
    },
    Kana {
        kana: "ぬ",
        romaji: &["nu"],
        group: 5,
        mnemonic: "Des nouilles emmêlées qui débordent d'un bol : « nou »illes (nu) (め n'a pas de boucle.)",
        example: Example {
            word: "ぬいぐるみ",
            romaji: "nuigurumi",
            fr: "peluche",
        },
    },
    Kana {
        kana: "ね",
        romaji: &["ne"],
        group: 5,
        mnemonic: "Le monstre du Loch Ness, long cou et grosse bosse : « Ne »ssie (ne) (れ et わ n'ont pas de boucle.)",
        example: Example {
            word: "ねこ",
            romaji: "neko",
            fr: "chat",
        },
    },
    Kana {
        kana: "の",
        romaji: &["no"],
        group: 5,
        mnemonic: "La coquille enroulée d'un nautile : « nau »tile (no)",
        example: Example {
            word: "のり",
            romaji: "nori",
            fr: "algue nori",
        },
    },
    // group 6: h
    Kana {
        kana: "は",
        romaji: &["ha"],
        group: 6,
        mnemonic: "Un hamac accroché entre deux poteaux, un dormeur allongé dedans : « ha »mac (ha) (ほ a un toit en plus.)",
        example: Example {
            word: "はな",
            romaji: "hana",
            fr: "fleur",
        },
    },
    Kana {
        kana: "ひ",
        romaji: &["hi"],
        group: 6,
        mnemonic: "Un hibou blotti au creux de son nid tout rond : « hi »bou (hi)",
        example: Example {
            word: "ひよこ",
            romaji: "hiyoko",
            fr: "poussin",
        },
    },
    Kana {
        kana: "ふ",
        romaji: &["fu", "hu"],
        group: 6,
        mnemonic: "Une fourmi vue de face, gros abdomen et pattes écartées : « fou »rmi (fu/hu)",
        example: Example {
            word: "ふゆ",
            romaji: "fuyu",
            fr: "hiver",
        },
    },
    Kana {
        kana: "へ",
        romaji: &["he"],
        group: 6,
        mnemonic: "Un hérisson fait le gros dos, piquants dressés : « hé »risson (he)",
        example: Example {
            word: "へや",
            romaji: "heya",
            fr: "chambre",
        },
    },
    Kana {
        kana: "ほ",
        romaji: &["ho"],
        group: 6,
        mnemonic: "Une horloge comtoise, son balancier rond au bout de la tige : « ho »rloge (ho) (は n'a qu'une barre, pas de toit.)",
        example: Example {
            word: "ほし",
            romaji: "hoshi",
            fr: "étoile",
        },
    },
    // group 7: m
    Kana {
        kana: "ま",
        romaji: &["ma"],
        group: 7,
        mnemonic: "Le grand mât d'un voilier, avec ses deux vergues et ses voiles : « ma »t (ma)",
        example: Example {
            word: "まど",
            romaji: "mado",
            fr: "fenêtre",
        },
    },
    Kana {
        kana: "み",
        romaji: &["mi"],
        group: 7,
        mnemonic: "Un minou allongé, la tête dans la boucle, lève la patte : « mi »nou (mi)",
        example: Example {
            word: "みみ",
            romaji: "mimi",
            fr: "oreille",
        },
    },
    Kana {
        kana: "む",
        romaji: &["mu"],
        group: 7,
        mnemonic: "Un moulin à vent, ses quatre ailes en croix et sa porte ronde : « mou »lin (mu)",
        example: Example {
            word: "むし",
            romaji: "mushi",
            fr: "insecte",
        },
    },
    Kana {
        kana: "め",
        romaji: &["me"],
        group: 7,
        mnemonic: "Une méduse ondule, ses tentacules emmêlés : « mé »duse (me) (ぬ a une boucle au bout.)",
        example: Example {
            word: "めがね",
            romaji: "megane",
            fr: "lunettes",
        },
    },
    Kana {
        kana: "も",
        romaji: &["mo"],
        group: 7,
        mnemonic: "Il ne reste de la morue que la tête et les arêtes : « mo »rue (mo)",
        example: Example {
            word: "もも",
            romaji: "momo",
            fr: "pêche",
        },
    },
    // group 8: y
    Kana {
        kana: "や",
        romaji: &["ya"],
        group: 8,
        mnemonic: "Un yacht file au vent, le mât penché, la voile gonflée et le fanion au sommet : « ya »cht (ya)",
        example: Example {
            word: "やま",
            romaji: "yama",
            fr: "montagne",
        },
    },
    Kana {
        kana: "ゆ",
        romaji: &["yu"],
        group: 8,
        mnemonic: "Une yourte ronde, son mât central dépasse du toit et fume : « you »rte (yu)",
        example: Example {
            word: "ゆき",
            romaji: "yuki",
            fr: "neige",
        },
    },
    Kana {
        kana: "よ",
        romaji: &["yo"],
        group: 8,
        mnemonic: "Un yogi assis en tailleur, le dos bien droit, tend le bras : « yo »ga (yo)",
        example: Example {
            word: "よる",
            romaji: "yoru",
            fr: "nuit",
        },
    },
    // group 9: r
    Kana {
        kana: "ら",
        romaji: &["ra"],
        group: 9,
        mnemonic: "Un rat dodu, le dos rond, dresse la queue : « ra »t (ra)",
        example: Example {
            word: "らくだ",
            romaji: "rakuda",
            fr: "chameau",
        },
    },
    Kana {
        kana: "り",
        romaji: &["ri"],
        group: 9,
        mnemonic: "Deux baguettes plongent dans un bol de riz, celle de droite plus longue : « ri »z (ri) (い a deux traits presque égaux.)",
        example: Example {
            word: "りんご",
            romaji: "ringo",
            fr: "pomme",
        },
    },
    Kana {
        kana: "る",
        romaji: &["ru"],
        group: 9,
        mnemonic: "Un ruban de gymnastique tourbillonne et finit par une petite boucle : « ru »ban (ru) (ろ n'a pas de boucle.)",
        example: Example {
            word: "るす",
            romaji: "rusu",
            fr: "absence (personne à la maison)",
        },
    },
    Kana {
        kana: "れ",
        romaji: &["re"],
        group: 9,
        mnemonic: "Un renne tend sa patte arrière vers l'extérieur : « re »nne (re) (わ revient en rond.)",
        example: Example {
            word: "れいぞうこ",
            romaji: "reizōko",
            fr: "réfrigérateur",
        },
    },
    Kana {
        kana: "ろ",
        romaji: &["ro"],
        group: 9,
        mnemonic: "Un roitelet tout rond dresse sa queue bien haut : « ro »itelet (ro) (る a une boucle en plus.)",
        example: Example {
            word: "ろうそく",
            romaji: "rōsoku",
            fr: "bougie",
        },
    },
    // group 10: w + n
    Kana {
        kana: "わ",
        romaji: &["wa"],
        group: 10,
        mnemonic: "Un wallaby assis sur sa grosse cuisse ronde : « wa »llaby (wa) (れ tend la patte vers l'extérieur, ね finit par une boucle.)",
        example: Example {
            word: "わんわん",
            romaji: "wanwan",
            fr: "ouaf ouaf (toutou)",
        },
    },
    Kana {
        kana: "を",
        romaji: &["wo", "o"],
        group: 10,
        mnemonic: "Un wok fait sauter les nouilles au-dessus des flammes : « wo »k (wo/o)",
        example: Example {
            word: "を",
            romaji: "o",
            fr: "particule du complément d'objet",
        },
    },
    Kana {
        kana: "ん",
        romaji: &["n", "nn"],
        group: 10,
        mnemonic: "Une longue nouille pend des baguettes au-dessus du bol : « n »ouille (n/nn)",
        example: Example {
            word: "うどん",
            romaji: "udon",
            fr: "udon (grosses nouilles)",
        },
    },
];

pub const KATAKANA: &[Kana] = &[
    // group 1: voyelles
    Kana {
        kana: "ア",
        romaji: &["a"],
        group: 1,
        mnemonic: "Une ancre : la barre en travers en haut, la longue tige et les pattes au bout : « a »ncre (a)",
        example: Example {
            word: "アイス",
            romaji: "aisu",
            fr: "glace",
        },
    },
    Kana {
        kana: "イ",
        romaji: &["i"],
        group: 1,
        mnemonic: "Un palmier sur une petite île, le trait penché est une grande palme : « î »le (i)",
        example: Example {
            word: "インク",
            romaji: "inku",
            fr: "encre",
        },
    },
    Kana {
        kana: "ウ",
        romaji: &["u"],
        group: 1,
        mnemonic: "Un ouistiti : le cadre est sa tête, le petit trait du haut sa touffe de poils : « ou »istiti (u) (ワ n'a pas cette touffe sur la tête.)",
        example: Example {
            word: "ウイスキー",
            romaji: "uisukī",
            fr: "whisky",
        },
    },
    Kana {
        kana: "エ",
        romaji: &["e"],
        group: 1,
        mnemonic: "Une étagère : deux planches reliées par un montant, avec des livres et une plante : « é »tagère (e)",
        example: Example {
            word: "エレベーター",
            romaji: "erebētā",
            fr: "ascenseur",
        },
    },
    Kana {
        kana: "オ",
        romaji: &["o"],
        group: 1,
        mnemonic: "Une ombrelle ouverte, le manche finit en crosse : « o »mbrelle (o)",
        example: Example {
            word: "オレンジ",
            romaji: "orenji",
            fr: "orange",
        },
    },
    // group 2: k
    Kana {
        kana: "カ",
        romaji: &["ka"],
        group: 2,
        mnemonic: "Une canne à pêche : le fil plonge dans l'eau et finit en hameçon : « ca »nne (ka)",
        example: Example {
            word: "カメラ",
            romaji: "kamera",
            fr: "appareil photo",
        },
    },
    Kana {
        kana: "キ",
        romaji: &["ki"],
        group: 2,
        mnemonic: "Un bonsaï : le tronc et deux étages de feuillage, « ki » veut dire arbre en japonais : « ki » (ki)",
        example: Example {
            word: "キウイ",
            romaji: "kiui",
            fr: "kiwi",
        },
    },
    Kana {
        kana: "ク",
        romaji: &["ku"],
        group: 2,
        mnemonic: "Un coucou ouvre grand le bec, et ce bec, c'est ク : « cou »cou (ku) (ケ a une barre qui dépasse, タ un trait en plus dedans.)",
        example: Example {
            word: "クッキー",
            romaji: "kukkī",
            fr: "biscuit",
        },
    },
    Kana {
        kana: "ケ",
        romaji: &["ke"],
        group: 2,
        mnemonic: "Un kendama, le bilboquet japonais : le long trait est le manche, la barre la traverse à coupelles, la bille pend à sa ficelle : « ke »ndama (ke) (ク n'a pas de barre qui dépasse.)",
        example: Example {
            word: "ケーキ",
            romaji: "kēki",
            fr: "gâteau",
        },
    },
    Kana {
        kana: "コ",
        romaji: &["ko"],
        group: 2,
        mnemonic: "Un coffre-fort ouvert, la porte pivote à gauche, des lingots dedans : « co »ffre (ko) (ユ a la barre du bas qui dépasse.)",
        example: Example {
            word: "コーヒー",
            romaji: "kōhī",
            fr: "café",
        },
    },
    // group 3: s
    Kana {
        kana: "サ",
        romaji: &["sa"],
        group: 3,
        mnemonic: "Un sac : le bord en haut, les deux anses qui dépassent : « sa »c (sa)",
        example: Example {
            word: "サラダ",
            romaji: "sarada",
            fr: "salade",
        },
    },
    Kana {
        kana: "シ",
        romaji: &["shi", "si"],
        group: 3,
        mnemonic: "La tête d'un chien de profil : deux petits traits pour les yeux, le grand trait pour la gueule qui sourit : « chi »en (shi/si) (ツ a ses petits traits en haut.)",
        example: Example {
            word: "シャワー",
            romaji: "shawā",
            fr: "douche",
        },
    },
    Kana {
        kana: "ス",
        romaji: &["su"],
        group: 3,
        mnemonic: "Un sumo, bras tendus et jambes bien écartées : « sou »mo (su) (ヌ a un trait qui le traverse.)",
        example: Example {
            word: "スープ",
            romaji: "sūpu",
            fr: "soupe",
        },
    },
    Kana {
        kana: "セ",
        romaji: &["se"],
        group: 3,
        mnemonic: "Un serpent dressé qui ondule jusqu'au sol en passant une branche : « ser »pent (se)",
        example: Example {
            word: "セーター",
            romaji: "sētā",
            fr: "pull",
        },
    },
    Kana {
        kana: "ソ",
        romaji: &["so"],
        group: 3,
        mnemonic: "Une bouteille de soda renversée, le jet descend depuis le haut : « so »da (so) (ン remonte depuis le bas.)",
        example: Example {
            word: "ソーダ",
            romaji: "sōda",
            fr: "soda",
        },
    },
    // group 4: t
    Kana {
        kana: "タ",
        romaji: &["ta"],
        group: 4,
        mnemonic: "Une part de tarte, le trait en plus ferme la part : « ta »rte (ta) (ク n'a pas ce trait.)",
        example: Example {
            word: "タルト",
            romaji: "taruto",
            fr: "tarte",
        },
    },
    Kana {
        kana: "チ",
        romaji: &["chi", "ti"],
        group: 4,
        mnemonic: "Un piment chili pend à sa branche, sa queue traverse la branche : « tchi »li (chi/ti) (テ a deux barres séparées, rien ne les traverse.)",
        example: Example {
            word: "チーズ",
            romaji: "chīzu",
            fr: "fromage",
        },
    },
    Kana {
        kana: "ツ",
        romaji: &["tsu", "tu"],
        group: 4,
        mnemonic: "Une grande vague retombe depuis le haut, deux paquets d'eau volent côte à côte : « tsu »nami (tsu/tu) (シ a ses petits traits à gauche.)",
        example: Example {
            word: "ツアー",
            romaji: "tsuā",
            fr: "voyage organisé",
        },
    },
    Kana {
        kana: "テ",
        romaji: &["te"],
        group: 4,
        mnemonic: "Une antenne de télé : deux barres séparées et un mât qui descend : « té »lé (te) (チ a un trait qui traverse la barre.)",
        example: Example {
            word: "テレビ",
            romaji: "terebi",
            fr: "télé",
        },
    },
    Kana {
        kana: "ト",
        romaji: &["to"],
        group: 4,
        mnemonic: "Un plant de tomate : la tige droite et une branche d'où pend une tomate : « to »mate (to)",
        example: Example {
            word: "トマト",
            romaji: "tomato",
            fr: "tomate",
        },
    },
    // group 5: n
    Kana {
        kana: "ナ",
        romaji: &["na"],
        group: 5,
        mnemonic: "Une aubergine (nasu en japonais) : la queue, la collerette en travers et le fruit courbe : « na »su (na)",
        example: Example {
            word: "ナイフ",
            romaji: "naifu",
            fr: "couteau",
        },
    },
    Kana {
        kana: "ニ",
        romaji: &["ni"],
        group: 5,
        mnemonic: "Deux branches l'une sur l'autre, un nid de poussins sur celle du bas : « ni »d (ni)",
        example: Example {
            word: "ニュース",
            romaji: "nyūsu",
            fr: "informations",
        },
    },
    Kana {
        kana: "ヌ",
        romaji: &["nu"],
        group: 5,
        mnemonic: "Une longue nouille soulevée du bol, traversée par une baguette : « nou »ille (nu) (ス n'a pas de trait qui le traverse.)",
        example: Example {
            word: "ヌードル",
            romaji: "nūdoru",
            fr: "nouilles",
        },
    },
    Kana {
        kana: "ネ",
        romaji: &["ne"],
        group: 5,
        mnemonic: "Un petit arbre couvert de neige, sa branche plie sous le poids : « nei »ge (ne)",
        example: Example {
            word: "ネクタイ",
            romaji: "nekutai",
            fr: "cravate",
        },
    },
    Kana {
        kana: "ノ",
        romaji: &["no"],
        group: 5,
        mnemonic: "Le traîneau du Père Noël descend du ciel en laissant une traînée dorée : « No »ël (no)",
        example: Example {
            word: "ノート",
            romaji: "nōto",
            fr: "cahier",
        },
    },
    // group 6: h
    Kana {
        kana: "ハ",
        romaji: &["ha"],
        group: 6,
        mnemonic: "Une hache fend la bûche en deux moitiés qui s'écartent : « ha »che (ha)",
        example: Example {
            word: "ハンマー",
            romaji: "hanmā",
            fr: "marteau",
        },
    },
    Kana {
        kana: "ヒ",
        romaji: &["hi"],
        group: 6,
        mnemonic: "Un hibou de profil, le dos droit, le petit trait en guise d'aile : « hi »bou (hi)",
        example: Example {
            word: "ヒーロー",
            romaji: "hīrō",
            fr: "héros",
        },
    },
    Kana {
        kana: "フ",
        romaji: &["fu", "hu"],
        group: 6,
        mnemonic: "Le fouet du dompteur claque devant le lion : « fou »et (fu/hu)",
        example: Example {
            word: "フォーク",
            romaji: "fōku",
            fr: "fourchette",
        },
    },
    Kana {
        kana: "ヘ",
        romaji: &["he"],
        group: 6,
        mnemonic: "Un héron en vol, ailes levées en pointe : « hé »ron (he)",
        example: Example {
            word: "ヘリコプター",
            romaji: "herikoputā",
            fr: "hélicoptère",
        },
    },
    Kana {
        kana: "ホ",
        romaji: &["ho"],
        group: 6,
        mnemonic: "Un homard vu de dessus, pinces au bout des bras, queue en bas : « ho »mard (ho)",
        example: Example {
            word: "ホテル",
            romaji: "hoteru",
            fr: "hôtel",
        },
    },
    // group 7: m
    Kana {
        kana: "マ",
        romaji: &["ma"],
        group: 7,
        mnemonic: "Un verre à cocktail en V, une olive sur sa pique : « ma »rtini (ma)",
        example: Example {
            word: "マンゴー",
            romaji: "mangō",
            fr: "mangue",
        },
    },
    Kana {
        kana: "ミ",
        romaji: &["mi"],
        group: 7,
        mnemonic: "Trois baguettes de mikado aux pointes colorées : « mi »kado (mi)",
        example: Example {
            word: "ミルク",
            romaji: "miruku",
            fr: "lait",
        },
    },
    Kana {
        kana: "ム",
        romaji: &["mu"],
        group: 7,
        mnemonic: "Un grand nez de profil et sa belle moustache : « mou »stache (mu)",
        example: Example {
            word: "ムース",
            romaji: "mūsu",
            fr: "mousse",
        },
    },
    Kana {
        kana: "メ",
        romaji: &["me"],
        group: 7,
        mnemonic: "Deux rubans croisés retiennent une médaille : « mé »daille (me)",
        example: Example {
            word: "メダル",
            romaji: "medaru",
            fr: "médaille",
        },
    },
    Kana {
        kana: "モ",
        romaji: &["mo"],
        group: 7,
        mnemonic: "La tête, les arêtes et la queue d'une morue : « mo »rue (mo)",
        example: Example {
            word: "モデル",
            romaji: "moderu",
            fr: "modèle",
        },
    },
    // group 8: y
    Kana {
        kana: "ヤ",
        romaji: &["ya"],
        group: 8,
        mnemonic: "Un voilier dont le grand trait est le mât : « ya »cht (ya)",
        example: Example {
            word: "ヤク",
            romaji: "yaku",
            fr: "yak",
        },
    },
    Kana {
        kana: "ユ",
        romaji: &["yu"],
        group: 8,
        mnemonic: "Un petit bateau à voile dont la coque dépasse devant et derrière : « you »you (yu) (ユ a la barre du bas qui dépasse, コ non.)",
        example: Example {
            word: "ユニフォーム",
            romaji: "yunifōmu",
            fr: "uniforme",
        },
    },
    Kana {
        kana: "ヨ",
        romaji: &["yo"],
        group: 8,
        mnemonic: "Une yole vue de dessus, trois rameurs tenant chacun sa rame : « yo »le (yo)",
        example: Example {
            word: "ヨーヨー",
            romaji: "yōyō",
            fr: "yo-yo",
        },
    },
    // group 9: r
    Kana {
        kana: "ラ",
        romaji: &["ra"],
        group: 9,
        mnemonic: "Des baguettes soulèvent une longue nouille au-dessus du bol : « ra »men (ra)",
        example: Example {
            word: "ラーメン",
            romaji: "rāmen",
            fr: "ramen, soupe de nouilles",
        },
    },
    Kana {
        kana: "リ",
        romaji: &["ri"],
        group: 9,
        mnemonic: "Deux rideaux à la fenêtre, un court noué d'un ruban et un long : « ri »deaux (ri)",
        example: Example {
            word: "リボン",
            romaji: "ribon",
            fr: "ruban",
        },
    },
    Kana {
        kana: "ル",
        romaji: &["ru"],
        group: 9,
        mnemonic: "Une route qui se sépare en deux, un panneau au carrefour et une voiture qui s'en va : « rou »te (ru)",
        example: Example {
            word: "ルート",
            romaji: "rūto",
            fr: "route, itinéraire",
        },
    },
    Kana {
        kana: "レ",
        romaji: &["re"],
        group: 9,
        mnemonic: "Un requin plonge, sa queue en V sort de l'eau : « re »quin (re)",
        example: Example {
            word: "レモン",
            romaji: "remon",
            fr: "citron",
        },
    },
    Kana {
        kana: "ロ",
        romaji: &["ro"],
        group: 9,
        mnemonic: "La tête toute carrée d'un robot : « ro »bot (ro)",
        example: Example {
            word: "ロボット",
            romaji: "robotto",
            fr: "robot",
        },
    },
    // group 10: w + n
    Kana {
        kana: "ワ",
        romaji: &["wa"],
        group: 10,
        mnemonic: "La gueule grande ouverte d'un crocodile, en japonais : « wa »ni (wa) (ウ a un petit trait sur la tête, ワ non.)",
        example: Example {
            word: "ワニ",
            romaji: "wani",
            fr: "crocodile",
        },
    },
    Kana {
        kana: "ヲ",
        romaji: &["wo", "o"],
        group: 10,
        mnemonic: "Une oreille et sa boucle au lobe : « o »reille (wo/o)",
        example: Example {
            word: "ヲ",
            romaji: "o",
            fr: "rare, comme を",
        },
    },
    Kana {
        kana: "ン",
        romaji: &["n", "nn"],
        group: 10,
        mnemonic: "Un nunchaku : deux bâtons reliés par une chaîne, le long lancé vers le haut : « n »unchaku (n/nn) (ン monte, ソ descend.)",
        example: Example {
            word: "パン",
            romaji: "pan",
            fr: "pain",
        },
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
    fn every_example_starts_with_its_kana() {
        for k in HIRAGANA.iter().chain(KATAKANA) {
            let word = k.example.word;
            // No word starts with ん; を and ヲ are only particles.
            let ok = match k.kana {
                "ん" | "ン" => word.ends_with(k.kana),
                _ => word.starts_with(k.kana),
            };
            assert!(ok, "{}: {word}", k.kana);
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
