# Seventeen

The "Seventeen" program provides functionality related to the number 17 in various languages and formats.

## Features

The "Seventeen" struct provides the following methods:

- `all_languages()`: Returns a vector of tuples containing the number 17 in different languages.
- `seventeen()`: Returns the value 17 as an unsigned 32-bit integer.
- `english()`: Returns the string representation of 17 in English.
- `korean()`: Returns the string representation of 17 in Korean.
- `japanese()`: Returns the string representation of 17 in Japanese.
- `japanese_kanji()`: Returns the string representation of 17 in Japanese Kanji.
- `japanese_hiragana()`: Returns the string representation of 17 in Japanese Hiragana.
- `japanese_katakana()`: Returns the string representation of 17 in Japanese Katakana.
- `french()`: Returns the string representation of 17 in French.
- `roman_numeral()`: Returns the string representation of 17 in Roman numerals.
- `decimal()`: Returns the string representation of 17 in decimal format.
- `binary()`: Returns the string representation of 17 in binary format.
- `hex()`: Returns the string representation of 17 in hexadecimal format.
- `oct()`: Returns the string representation of 17 in octal format.
- `is_seventeen_number(x: i32)`: Checks if the given number is equal to 17 and returns a Result<bool, String>.
- `is_seventeen_text(s: &str)`: Checks if the given string is equivalent to the string representation of 17 in any language and returns a Result<bool, String>.

## Usage

```rust
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
}

```

## Contributions

Contributions to the "Seventeen" program are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This program is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
