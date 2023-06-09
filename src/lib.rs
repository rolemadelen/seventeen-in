pub struct Seventeen;

impl Seventeen {
    pub fn number(&self) -> u32 {
        17
    }

    pub fn string(&self) -> String {
        String::from("seventeen")
    }

    pub fn all_languages(&self) -> Vec<(String, String)> {
        vec![
            (String::from("en"), String::from("seventeen")),
            (String::from("ko"), String::from("열일곱")),
            (String::from("ko"), String::from("십칠")),
            (String::from("ja"), String::from("十七")),
            (String::from("fr"), String::from("dix-sept")),
        ]
    }

    pub fn english(&self) -> Vec<String> {
        vec![String::from("seventeen")]
    }

    pub fn korean(&self) -> Vec<String> {
        vec![
            String::from("열일곱"),
            String::from("십칠")
        ]
    }

    pub fn japanese(&self) -> Vec<String> {
        vec![
            String::from("十七"), 
            String::from("じゅナナ"),
            String::from("ジュナナ")
        ]
    }

    pub fn japanese_kanji(&self) -> Vec<String> {
        vec![String::from("十七")]
    }

    pub fn japanese_hiragana(&self) -> Vec<String> {
        vec![String::from("じゅナナ")]
    }

    pub fn japanese_katakana(&self) -> Vec<String> {
        vec![String::from("ジュナナ")]
    }

    pub fn roman_numeral(&self) -> Vec<String> {
        vec![String::from("XVII")]
    }

    pub fn french(&self) -> Vec<String> {
        vec![String::from("dix-sept")]
    }

    pub fn decimal(&self) -> Vec<String> {
        vec![String::from("17")]
    }

    pub fn binary(&self) -> Vec<String> {
        vec![format!("{:b}", 17)]
    }

    pub fn hex(&self) -> Vec<String> {
        vec![format!("{:x}", 17)]
    }

    pub fn oct(&self) -> Vec<String> {
        vec![format!("{:o}", 17)]
    }

    pub fn is_seventeen_number(&self, x: i32) -> Result<bool, String> {
        if x == 17 {
            Ok(true)
        } else {
            Err(String::from("{x} is not 17"))
        }
    }

    pub fn is_seventeen_text(&self, s: &String) -> Result<bool, String> {
        let input = &self.all_languages();
        for (_, value) in input {
            if value == s {
                return Ok(true);
            }
        }

        Err(String::from("'{s}' is not 17."))
    }
}

