use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Translation {
    language: String,
    word: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Seventeen {
    seventeen: Vec<Translation>
}

impl Seventeen {
    fn find_language(&self, language: &str) -> &String {
        &self.seventeen.iter().find(|&s| *s.language == language.to_string()).unwrap().word
    }

    pub fn new() -> Self {
        let input_path = std::env::args().nth(1).unwrap();
        let translation = std::fs::read_to_string(&input_path).unwrap();
        serde_json::from_str::<Seventeen>(&translation).unwrap()
    }

    pub fn in_english(&self) -> &String {
        self.find_language("English")
    }

    pub fn in_korean(&self) -> &String {
        self.find_language("Korean")
    }
    
    pub fn in_japanese(&self) -> &String {
        self.find_language("Japanese")
    }
    
    pub fn in_japanese_hiragana(&self) -> &String {
        self.find_language("Japanese_Hiragana")
    }
    
    pub fn in_japanese_katakana(&self) -> &String {
        self.find_language("Japanese_Katakana")
    }
    
    pub fn in_french(&self) -> &String {
        self.find_language("French")
    }
}