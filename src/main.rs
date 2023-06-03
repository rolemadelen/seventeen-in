struct Seventeen;

impl Seventeen {
    fn get_all(&self) -> Vec<(&str, &str)> {
        vec![
            ("en", "seventeen"),
            ("ko", "열일곱 십칠"),
            ("ja", "十七"),
            ("fr", "dix-sept"),
        ]
    }

    fn in_english(&self) -> String {
        String::from("seventeen")
    }

    fn in_korean(&self) -> String {
        String::from("열일곱 십칠")
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

    fn seventeen(&self) -> u32 {
        17
    }

    fn chant(&self) -> String {
        String::from("Heroes of the morning light! Hey!")
    }

    fn check_number(&self, x: i32) -> bool {
        if x == 17 { true } else { false }
    }

    fn check_str(&self, s: &str) -> bool {
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

    println!("{}", seventeen.seventeen());
    println!("{}", seventeen.in_english());
    println!("{}", seventeen.in_korean());
    println!("{}", seventeen.in_japanese());
    println!("{}", seventeen.in_japanese_hiragana());
    println!("{}", seventeen.in_japanese_katakana());
    println!("{}", seventeen.in_french());
    println!("{}", seventeen.in_roman_numeral());
    println!("{:?}", seventeen.get_all());
    println!("{}", seventeen.chant());

    println!("{}", seventeen.check_number(17));
    println!("{}", seventeen.check_number(18));
    println!("{}", seventeen.check_str("seventeen"));
    println!("{}", seventeen.check_str("열일곱"));
    println!("{}", seventeen.check_str("eighteen"));
}
