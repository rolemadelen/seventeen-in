use seventeen::Seventeen;

fn main() {
    let seventeen = Seventeen;

    println!("Number: {}", seventeen.in_number());
    println!("String: {}", seventeen.in_string());
    println!("All Languages: {:?}", seventeen.in_all_languages());
    println!("English: {:?}", seventeen.in_english());
    println!("Korean: {:?}", seventeen.in_korean());
    println!("Japanese: {:?}", seventeen.in_japanese());
    println!("Kanji: {:?}", seventeen.in_japanese_kanji());
    println!("Hiragana: {:?}", seventeen.in_japanese_hiragana());
    println!("Katakana: {:?}", seventeen.in_japanese_katakana());
    println!("Roman Numeral: {:?}", seventeen.in_roman_numeral());
    println!("Base  2: {:?}", seventeen.in_binary());
    println!("Base  8: {:?}", seventeen.in_oct());
    println!("Base 10: {:?}", seventeen.in_decimal());
    println!("Base 16: {:?}", seventeen.in_hex());

    let str = String::from("dix-sept");
    if let Ok(_) = seventeen.is_text_seventeen(&str) {
        println!("Yes, '{str}' is seventeen.");
    } else {
        println!("Sorry, '{str}' is not seventeen.");
    }

    let str = String::from("XVII");
    if let Ok(_) = seventeen.is_text_seventeen(&str) {
        println!("Yes, '{str}' is seventeen.");
    } else {
        println!("Sorry, '{str}' is not seventeen.");
    }
}
