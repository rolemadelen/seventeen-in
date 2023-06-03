# Seventeen

The "Seventeen" program provides functionality related to the number 17 in various languages and formats.

## Features

The "Seventeen" struct provides the following methods:

- `get_all()`: Returns a vector of tuples containing the number 17 in different languages.
- `seventeen()`: Returns the value 17 as an unsigned 32-bit integer.
- `in_english()`: Returns the string representation of 17 in English.
- `in_korean()`: Returns the string representation of 17 in Korean.
- `in_japanese()`: Returns the string representation of 17 in Japanese.
- `in_japanese_hiragana()`: Returns the string representation of 17 in Japanese Hiragana script.
- `in_japanese_katakana()`: Returns the string representation of 17 in Japanese Katakana script.
- `in_roman_numeral()`: Returns the string representation of 17 in Roman numerals.
- `in_french()`: Returns the string representation of 17 in French.
- `in_decimal()`: Returns the string representation of 17 in decimal format.
- `in_binary()`: Returns the string representation of 17 in binary format.
- `in_hex()`: Returns the string representation of 17 in hexadecimal format.
- `in_oct()`: Returns the string representation of 17 in octal format.
- `chant()`: Returns a chant associated with the number 17.
- `is_it(x: i32)`: Checks if the given number is equal to 17 and returns a Result<i32, String>.
- `is_it_str(s: &str)`: Checks if the given string is equivalent to the string representation of 17 in any language and returns a Result<String, String>.

## Usage

```rust
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
```

## Contributions

Contributions to the "Seventeen" program are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This program is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
