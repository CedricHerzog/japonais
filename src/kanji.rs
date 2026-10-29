use std::sync::LazyLock;

/// Kanji per group. A new school grade always starts a new group.
const GROUP_SIZE: usize = 10;

pub struct Kanji {
    pub kanji: &'static str,
    pub grade: u8, // 1–6 for primary school, 8 for secondary school
    pub on: Vec<&'static str>,
    pub kun: Vec<&'static str>,
    pub meanings: Vec<&'static str>, // French when KANJIDIC2 has it, else English
    pub group: u8,
}

/// The 2,136 Jōyō kanji, in school grade order. Built by `examples/kanji_data.rs`.
pub static KANJI: LazyLock<Vec<Kanji>> = LazyLock::new(|| parse(include_str!("../data/kanji.tsv")));

fn parse(tsv: &'static str) -> Vec<Kanji> {
    let list = |field: &'static str| -> Vec<&'static str> {
        field.split('|').filter(|s| !s.is_empty()).collect()
    };
    let mut table: Vec<Kanji> = Vec::new();
    let mut group = 0;
    let mut in_group = GROUP_SIZE;
    for line in tsv.lines().filter(|l| !l.starts_with('#')) {
        let fields: Vec<&'static str> = line.split('\t').collect();
        let [kanji, grade, on, kun, meanings] = fields[..] else {
            panic!("ligne invalide dans kanji.tsv : {line}");
        };
        let grade: u8 = grade.parse().unwrap();
        if in_group == GROUP_SIZE || table.last().is_some_and(|k| k.grade != grade) {
            group += 1;
            in_group = 0;
        }
        in_group += 1;
        table.push(Kanji {
            kanji,
            grade,
            on: list(on),
            kun: list(kun),
            meanings: list(meanings),
            group,
        });
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn table_has_the_2136_joyo_kanji() {
        assert_eq!(KANJI.len(), 2136);
        let unique: HashSet<&str> = KANJI.iter().map(|k| k.kanji).collect();
        assert_eq!(unique.len(), 2136);
        for (grade, expected) in [
            (1, 80),
            (2, 160),
            (3, 200),
            (4, 202),
            (5, 193),
            (6, 191),
            (8, 1110),
        ] {
            let n = KANJI.iter().filter(|k| k.grade == grade).count();
            assert_eq!(n, expected, "grade {grade}");
        }
    }

    #[test]
    fn every_kanji_has_a_meaning_and_a_reading() {
        for k in KANJI.iter() {
            assert!(!k.meanings.is_empty(), "{}", k.kanji);
            assert!(!k.on.is_empty() || !k.kun.is_empty(), "{}", k.kanji);
        }
    }

    #[test]
    fn groups_are_small_and_stay_in_one_grade() {
        let last = KANJI.last().unwrap().group;
        for group in 1..=last {
            let members: Vec<&Kanji> = KANJI.iter().filter(|k| k.group == group).collect();
            assert!((1..=GROUP_SIZE).contains(&members.len()), "group {group}");
            assert!(
                members.iter().all(|k| k.grade == members[0].grade),
                "group {group}"
            );
        }
    }
}
