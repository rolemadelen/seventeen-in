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

    fn is_it(&self, x: i32) -> Result<i32, String> {
        if x == 17 {
            Ok(x)
        } else {
            Err(String::from("{x} is not 17"))
        }
    }

    fn is_it_str(&self, s: &str) -> Result<String, String> {
        let input = self.get_all();
        for (_, value) in input {
            if value == s {
                return Ok(value.to_string());
            }
        }

        Err(String::from("'{s}' is not 17."))
    }
}

fn main() {
    let seventeen = Seventeen;
    println!("{}", seventeen.in_english());
    println!("{}", seventeen.in_korean());
    println!("{}", seventeen.in_korean_2());
    println!("{}", seventeen.in_japanese());
    println!("{}", seventeen.in_japanese_hiragana());
    println!("{}", seventeen.in_japanese_katakana());
    println!("{}", seventeen.in_french());
    println!("{}", seventeen.in_roman_numeral());
    println!("{}", seventeen.in_binary());
    println!("{}", seventeen.in_oct());
    println!("{}", seventeen.in_decimal());
    println!("{}", seventeen.in_hex());
    println!("{:?}", seventeen.get_all());

    let is_it_17 = seventeen.is_it(17);
    if let Ok(x) = is_it_17 {
        println!("Yes this number is 17.");
    } else {
        println!("No this number is not 17.");
    }
}
