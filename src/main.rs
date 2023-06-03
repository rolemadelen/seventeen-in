#![allow(dead_code)]

struct Seventeen;

impl Seventeen {
    fn get_all(&self) -> Vec<(&str, &str)> {
        vec![
            ("en", "seventeen"),
            ("ko", "열일곱"),
            ("ko", "십칠"),
            ("ja", "十七"),
            ("fr", "dix-sept"),
        ]
    }

    fn seventeen(&self) -> u32 {
        17
    }

    fn in_english(&self) -> String {
        String::from("seventeen")
    }

    fn in_korean(&self) -> String {
        String::from("열일곱")
    }

    fn in_korean_2(&self) -> String {
        String::from("십칠")
    }

    fn in_japanese(&self) -> String {
        String::from("十七")
    }

    fn in_japanese_hiragana(&self) -> String {
        String::from("じゅナナ")
    }

    fn in_japanese_katakana(&self) -> String {
        String::from("ジュナナ")
    }

    fn in_roman_numeral(&self) -> String {
        String::from("XVII")
    }

    fn in_french(&self) -> String {
        String::from("dix-sept")
    }

    fn in_decimal(&self) -> String {
        String::from("17")
    }

    fn in_binary(&self) -> String {
        format!("{:b}", 17)
    }

    fn in_hex(&self) -> String {
        format!("{:x}", 17)
    }

    fn in_oct(&self) -> String {
        format!("{:o}", 17)
    }

    fn chant(&self) -> String {
        String::from("Heroes of the morning light! Hey!")
    }

    fn is_it(&self, x: i32) -> bool {
        if x == 17 { true } else { false }
    }

    fn is_it_str(&self, s: &str) -> bool {
        let input = self.get_all();
        for (_, value) in input {
            if value == s {
                return true;
            }
        }

        false
    }
}

fn main() {
    let seventeen = Seventeen;

    let all_languages = seventeen.get_all();
    println!("Number 17 in different languages: {:?}", all_languages);

    let seventeen_value = seventeen.seventeen();
    println!("The value of 17 is: {}", seventeen_value);

    let seventeen_english = seventeen.in_english();
    println!("17 in English: {}", seventeen_english);

    let is_seventeen = seventeen.is_it(18);
    println!("Is '18' a 17? {}", is_seventeen);

    let is_seventeen_in_korean = seventeen.is_it_str("열일곱");
    println!("Is '열일곱' equivalent to 17 in any language? {}", is_seventeen_in_korean);
}
