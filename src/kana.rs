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
        mnemonic: "Un avion fait un looping et laisse une boucle de fumée : « Aaah ! » (a)",
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
        mnemonic: "Deux iris aux longues tiges, un grand et un petit : « i-ris » (i) (り a le trait de droite bien plus long.)",
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
        mnemonic: "Un surfeur sur la crête d'une vague qui s'enroule : « Ouh ! » (u)",
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
        mnemonic: "Un éclair zigzague du nuage jusqu'au sol : « É-clair ! » (e)",
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
        mnemonic: "Une autruche dresse le cou, gros ventre rond et plume sur la queue : « Oh ! » (o)",
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
        mnemonic: "Une catapulte lance son boulet : « Ka-boum ! » (ka)",
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
        mnemonic: "Un kimono suspendu : les manches, la ceinture obi, le col croisé et le pan du bas : « ki-mono » (ki) (さ n'a qu'une barre.)",
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
        mnemonic: "Main sur la hanche, il pointe le coude : « kou-de » (ku)",
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
        mnemonic: "La broche d'un kebab et son grand couteau : « ké-bab » (ke)",
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
        mnemonic: "Un crocodile ouvre grand ses deux mâchoires : « cro-co » (ko) (Avec un tronc vertical devant, c'est に : ni.)",
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
        mnemonic: "Une sardine vient mordre à l'hameçon sous la surface : « sar-dine » (sa) (ち ressemble à un 5.)",
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
        mnemonic: "Un piment chili tout rouge, recourbé au bout ; ça pique : « chi ! » (shi/si)",
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
        mnemonic: "Une souris descend le long d'une corde, sa longue queue pendante : « sou-ris » (su)",
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
        mnemonic: "Des serviettes sèchent sur le séchoir à linge : « sè-che » (se)",
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
        mnemonic: "Un chat roulé en boule fait un somme sous un grand « Z » : « so-mme » (so)",
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
        mnemonic: "Deux baguettes croisées frappent un tambour : « ta-ta-ta ! » (ta)",
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
        mnemonic: "Un litchi bien rond pend à sa branche : « li-tchi » (chi/ti) (Ventre à droite comme un 5 ; さ s'ouvre comme un « c ».)",
        example: Example {
            word: "ちず",
            romaji: "chizu",
            fr: "carte",
        },
    },
    Kana {
        kana: "つ",
        romaji: &["tsu", "tu"],
        group: 4,
        mnemonic: "Un croissant de lune ; la lune se dit « tsuki » en japonais : tsu (tsu/tu)",
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
        mnemonic: "La cabine d'un téléphérique pend à son câble : « té-lé » (te)",
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
        mnemonic: "Une taupe creuse son tunnel sous une carotte : « taupe » (to)",
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
        mnemonic: "Un épouvantail garde un gros navet dans le potager : « na-vet » (na)",
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
        mnemonic: "Un nid plein d'oisillons contre le tronc d'un arbre : « nid » (ni) (Sans le tronc vertical, c'est こ : ko.)",
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
        mnemonic: "Un ruban bien noué sur un cadeau, avec une petite boucle au bout : « nou-é » (nu) (め n'a pas de boucle.)",
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
        mnemonic: "Nessie, le monstre du Loch Ness, sort son long cou et sa bosse du lac : « Né-ssie » (ne) (れ et わ n'ont pas de boucle.)",
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
        mnemonic: "La coquille enroulée d'un nautile : « nau-tile » (no)",
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
        mnemonic: "Une ligne de pêche plonge dans l'eau, l'hameçon en boucle au bout : un poisson s'écrie « Ha ! » (ha) (ほ a un toit en plus.)",
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
        mnemonic: "Un hibou se blottit au creux de son nid tout rond : « hi »bou (hi)",
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
        mnemonic: "Un feu de camp s'allume d'un coup : « Fouf ! » (fu/hu)",
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
        mnemonic: "Un hérisson fait le gros dos : « Hé, pas touche ! » (he)",
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
        mnemonic: "On tire la corde pour hisser le drapeau étoilé entre les deux barres : « Oh hisse ! » (ho) (は n'a qu'une barre, pas de toit.)",
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
        mnemonic: "Un chaton joue avec une pelote de laine qui se déroule : « Miaou ! » (mi)",
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
        mnemonic: "Un moulin à vent, ses ailes en croix et sa porte ronde : « mou »lin (mu)",
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
        mnemonic: "Un « ya »cht file au vent, le mât penché, la voile gonflée et le fanion au sommet (ya)",
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
        mnemonic: "Un rat dodu dresse la queue : « ra » ! (ra)",
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
        mnemonic: "Deux baguettes plongent dans un bol de riz, celle de droite plus longue : « ri » (ri) (い a deux traits presque égaux.)",
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
        mnemonic: "Une main fait tourbillonner un ruban de gymnastique, qui finit par une petite boucle : « ru »ban (ru) (ろ n'a pas de boucle.)",
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
        mnemonic: "Un chien assis, le dos tout rond, aboie : « Ouah ! » (wa) (れ tend la patte vers l'extérieur, ね finit par une boucle.)",
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
        mnemonic: "Un « wo »k fait sauter les nouilles au-dessus des flammes (wo/o)",
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
        mnemonic: "Une longue nouille pend des baguettes au-dessus du bol : « Nnn », c'est bon ! (n/nn)",
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
        mnemonic: "Un enfant dévale le grand toboggan en criant « Aaah ! » (a)",
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
        mnemonic: "Un palmier pousse sur une petite île : le trait penché est sa grande palme, le trait droit son tronc. « Île » (i)",
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
        mnemonic: "Un hibou sur sa branche, une petite plume dressée sur la tête, hulule « Hou ! » (u) (ワ n'a pas cette plume sur la tête.)",
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
        mnemonic: "Un éléphant de cirque tient en équilibre sur son piédestal : « éléphant » (e)",
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
        mnemonic: "Un voilier avec son mât, sa vergue et sa voile en triangle : « Ohé, bateau ! » (o)",
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
        mnemonic: "Une canne à pêche : le fil plonge dans l'eau et finit en hameçon, un poisson approche. « Canne » (ka)",
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
        mnemonic: "Au ski, un poteau planté dans la neige porte deux panneaux de piste : « ski » (ki)",
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
        mnemonic: "Un oiseau ouvre grand le bec pour chanter « Coucou ! » (ku) (ケ a une barre qui dépasse, タ un trait en plus dedans.)",
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
        mnemonic: "Au port, le ponton du quai dépasse au-dessus de l'eau : « quai » (ke) (ク n'a pas de barre qui dépasse.)",
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
        mnemonic: "Un cochon dans son enclos bien fermé, ouvert seulement à gauche : « cochon » (ko) (ユ a la barre du bas qui dépasse.)",
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
        mnemonic: "Un mouton saute la barrière du pré : « Saute ! » (sa)",
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
        mnemonic: "Des baguettes remontent un sushi depuis le bas ; deux gouttes de sauce soja tombent à gauche : « sushi » (shi/si) (ツ a ses petits traits en haut.)",
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
        mnemonic: "Un sumo, bras tendus et jambes bien écartées : « sumo » (su) (ヌ a un trait qui le traverse.)",
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
        mnemonic: "Au puits, un seau pend au crochet de la potence : « seau » (se)",
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
        mnemonic: "Une bouteille de soda se renverse : le jet descend depuis le haut, une goutte gicle à côté. « Soda » (so) (ン remonte depuis le bas.)",
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
        mnemonic: "Une part de tarte soulevée sur sa pelle ; le petit trait en plus ferme la part : « tarte » (ta) (ク n'a pas ce trait.)",
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
        mnemonic: "Un piment chili pend à sa branche, et sa queue traverse la branche : « chili » (chi/ti) (テ a deux barres séparées, rien ne les traverse.)",
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
        mnemonic: "Un tsunami : la grande vague retombe depuis le haut, deux paquets d'eau volent côte à côte au-dessus. « Tsunami » (tsu/tu) (シ a ses petits traits à gauche.)",
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
        mnemonic: "Une antenne de télé sur le toit : deux barres séparées et un mât qui descend. « Télé » (te) (チ a un trait qui traverse la barre.)",
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
        mnemonic: "Un plant de tomate : la tige droite et une branche d'où pend une tomate. « Tomate » (to)",
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
        mnemonic: "Un katana : la poignée en haut, la garde en travers et la lame courbe en dessous. « Katana » (na)",
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
        mnemonic: "Deux branches l'une sur l'autre : un nid de poussins sur celle du bas, la maman sur celle du haut. « Nid » (ni)",
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
        mnemonic: "Une longue nouille soulevée du bol, traversée par une baguette : « nouille » (nu) (ス n'a pas de trait qui le traverse.)",
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
        mnemonic: "Un petit arbre couvert de neige, sa branche plie sous le poids : « neige » (ne)",
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
        mnemonic: "La nuit de Noël, le traîneau descend du ciel en laissant une traînée dorée : « Noël » (no)",
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
        mnemonic: "D'un coup de hache, la bûche se fend en deux moitiés qui s'écartent : « Ha ! » (ha)",
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
        mnemonic: "Un hibou s'est perché sur la branche : hi-bou ! (hi)",
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
        mnemonic: "Le dompteur fait claquer son fouet devant le lion : « Fou-et ! » (fu/hu)",
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
        mnemonic: "Au sommet de la montagne, le randonneur crie « Hé ! » et l'écho lui répond. (he)",
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
        mnemonic: "Bras écartés, l'épouvantail chasse les corbeaux : « Ho ! Ho ! » (ho)",
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
        mnemonic: "Un cornet surmonté d'une boule à la mangue : « Ma glace ! » (ma)",
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
        mnemonic: "Le chat a laissé trois griffures sur le canapé : « Miaou ! » (mi)",
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
        mnemonic: "Sous ce grand nez pointu pousse une belle moustache : « Mou-stache ! » (mu)",
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
        mnemonic: "Le mouton saute par-dessus la barrière en croix : « Mêêê ! » (me)",
        example: Example {
            word: "メロン",
            romaji: "meron",
            fr: "melon",
        },
    },
    Kana {
        kana: "モ",
        romaji: &["mo"],
        group: 7,
        mnemonic: "Il ne reste que la tête, les arêtes et la queue de la morue : mo ! (mo)",
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
        mnemonic: "Le grand trait est le mât, les voiles se gonflent : un beau yacht ! (ya)",
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
        mnemonic: "Agrippé à la poignée de sa luge aux longs patins qui dépassent, l'enfant file : « You-pi ! » (yu) (ユ a la barre du bas qui dépasse, コ non.)",
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
        mnemonic: "Sur l'étagère, on range ses yo-yos : yo ! (yo)",
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
        mnemonic: "Un rat assis au bord de l'étagère lorgne le fromage, sa longue queue pend jusqu'en bas : ra ! (ra)",
        example: Example {
            word: "ラジオ",
            romaji: "rajio",
            fr: "radio",
        },
    },
    Kana {
        kana: "リ",
        romaji: &["ri"],
        group: 9,
        mnemonic: "Deux rideaux à la fenêtre, un court et un long : ri-deaux ! (ri)",
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
        mnemonic: "Un coureur file à grandes enjambées dans la rue, les deux traits sont ses jambes : ru ! (ru)",
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
        mnemonic: "Allongé sur son transat, il rêve d'une île au soleil : rê-ve ! (re)",
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
        mnemonic: "Ce robot a une tête toute carrée : ro-bot ! (ro)",
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
        mnemonic: "Le wagonnet arrive en haut des montagnes russes et bascule dans la descente : « Waaah ! » (wa) (ウ a un petit trait sur la tête, ワ non.)",
        example: Example {
            word: "ワイン",
            romaji: "wain",
            fr: "vin",
        },
    },
    Kana {
        kana: "ヲ",
        romaji: &["wo", "o"],
        group: 10,
        mnemonic: "Au bas du toboggan, l'enfant plonge et ressort de l'eau : « Oh ! » (wo/o)",
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
        mnemonic: "L'avion décolle et monte vers la droite en vrombissant : « Nnnnn ! » (n/nn) (ン monte, ソ descend.)",
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
