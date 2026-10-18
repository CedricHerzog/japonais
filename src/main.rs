mod kana;

fn main() {
    for k in kana::HIRAGANA {
        println!("{} {} {}", k.group, k.kana, k.romaji.join(" "))
    }
    println!("Total : {}", kana::HIRAGANA.len())
}
