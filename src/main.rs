#![allow(dead_code)]

use seventeen::Seventeen;

fn main() {
    let seventeen = Seventeen;

    println!("Number: {}", seventeen.number());
    println!("String: {}", seventeen.string());
    println!("All Languages: {:?}", seventeen.all_languages());
    println!("English: {:?}", seventeen.english());
    println!("Korean: {:?}", seventeen.korean());
    println!("Japanese: {:?}", seventeen.japanese());
    println!("Kanji: {:?}", seventeen.japanese_kanji());
    println!("Hiragana: {:?}", seventeen.japanese_hiragana());
    println!("Katakana: {:?}", seventeen.japanese_katakana());
    println!("Roman Numeral: {:?}", seventeen.roman_numeral());
    println!("Base  2: {:?}", seventeen.binary());
    println!("Base  8: {:?}", seventeen.oct());
    println!("Base 10: {:?}", seventeen.decimal());
    println!("Base 16: {:?}", seventeen.hex());

    let str = String::from("dix-sept");
    if let Ok(_) = seventeen.is_seventeen_text(&str) {
        println!("Yes, '{str}' is seventeen.");
    } else {
        println!("Sorry, '{str}' is not seventeen.");
    }

    let str = String::from("XVII");
    if let Ok(_) = seventeen.is_seventeen_text(&str) {
        println!("Yes, '{str}' is seventeen.");
    } else {
        println!("Sorry, '{str}' is not seventeen.");
    }
}
