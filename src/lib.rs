pub struct Seventeen;

impl Seventeen {
    pub fn in_number(&self) -> u32 {
        17
    }

    pub fn in_string(&self) -> String {
        String::from("seventeen")
    }

    pub fn in_all_languages(&self) -> Vec<(String, String)> {
        vec![
            (String::from("en"), String::from("seventeen")),
            (String::from("ko"), String::from("열일곱")),
            (String::from("ko"), String::from("십칠")),
            (String::from("ja"), String::from("十七")),
            (String::from("fr"), String::from("dix-sept")),
        ]
    }

    pub fn in_english(&self) -> Vec<String> {
        vec![String::from("seventeen")]
    }

    pub fn in_korean(&self) -> Vec<String> {
        vec![
            String::from("열일곱"),
            String::from("십칠")
        ]
    }

    pub fn in_japanese(&self) -> Vec<String> {
        vec![
            String::from("十七"), 
            String::from("じゅナナ"),
            String::from("ジュナナ")
        ]
    }

    pub fn in_japanese_kanji(&self) -> Vec<String> {
        vec![String::from("十七")]
    }

    pub fn in_japanese_hiragana(&self) -> Vec<String> {
        vec![String::from("じゅナナ")]
    }

    pub fn in_japanese_katakana(&self) -> Vec<String> {
        vec![String::from("ジュナナ")]
    }

    pub fn in_roman_numeral(&self) -> Vec<String> {
        vec![String::from("XVII")]
    }

    pub fn in_french(&self) -> Vec<String> {
        vec![String::from("dix-sept")]
    }

    pub fn in_decimal(&self) -> Vec<String> {
        vec![String::from("17")]
    }

    pub fn in_binary(&self) -> Vec<String> {
        vec![format!("{:b}", 17)]
    }

    pub fn in_hex(&self) -> Vec<String> {
        vec![format!("{:x}", 17)]
    }

    pub fn in_oct(&self) -> Vec<String> {
        vec![format!("{:o}", 17)]
    }

    pub fn is_number_seventeen(&self, x: i32) -> Result<bool, String> {
        if x == 17 {
            Ok(true)
        } else {
            Err(String::from("{x} is not 17"))
        }
    }

    pub fn is_text_seventeen(&self, s: &String) -> Result<bool, String> {
        let input = &self.in_all_languages();
        for (_, value) in input {
            if value == s {
                return Ok(true);
            }
        }

        Err(String::from("'{s}' is not 17."))
    }
}

