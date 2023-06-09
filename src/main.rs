use seventeen::Seventeen;

fn main() {
    let seventeen = Seventeen::new();
    println!("{}", seventeen.in_english());
    println!("{}", seventeen.in_korean());
    println!("{}", seventeen.in_japanese());
    println!("{}", seventeen.in_japanese_hiragana());
    println!("{}", seventeen.in_japanese_katakana());
    println!("{}", seventeen.in_french());
}
