use phf::{phf_set, Set};

/// Strings that are thirteen.
pub static SEVENTEEN_STRINGS: Set<&'static str> = phf_set! {
    "sewentien",            // Afrikaans
    "shtatëmbëdhjetë",      // Albanian
    "عشر",                  // Arabic
    "տասն­յոթ",              // Armenian
    "сямнаццаць",            // Belarusian
    "седемнадесет",          // Bulgarian
    "disset",               // Catalan
    "十七",                   // Chinese | Japanese
    "sedmnáct",             // Czech
    "sytten",               // Danish | Norweign
    "zeventien",            // Dutch
    "seventeen",            // English
    "seitseteist",          // Estonian
    "seytjan",              // Faroese
    "هفده",                   // Farsi
    "seitsemäntoista",       // Finnish
    "sept",                 // French
    "ჩვიდმეტი",              // Georgian
    "siebzehn",             // German
    "δεκα­επτά",               // Greek
    "עשרה",                  // Hebrew
    "सत्रह",                  // Hindi
    "tizen­hét",             // Hungarian
    "sautján",              // Icelandic
    "belas",                // Indonesian
    "diciassette",          // Italian
    "じゅうなな",               // Japanese Hiragana
    "じゅうしち",               // Japanese Hiragana
    "じゅなな",                // Japanese Hiragana (Misspelled)
    "じゅしち",                // Japanese Hiragana (Misspelled)
    "ジュウナナ",              // Japanese Katakana
    "ジュウシチ",              // Japanese Katakana
    "ジュシチ",                // Japanese Katakana (Misspelled)
    "ジュナナ",                // Japanese Katakana (Misspelled)
    "십칠",                  // Korean
    "십7",                  // Korean
    "10칠",                  // Korean
    "열일곱",                // Korean
    "열7",                // Korean
    "10일곱",                // Korean
    "세븐틴",                // Korean
    "septiņpadsmit",        // Latvian
    "septyniolika",         // Lithuanian
    "седумнаесет",          // Macedonian
    "tujuh belas",          // Malay
    "siedemnaście",         // Polish
    "dezessete",            // Portuguese
    "şaptesprezece",        // Romanian
    "семнадцать",           // Russian
    "седамнаест",           // Serbian (Cyrillic)
    "sedamnaest",           // Serbian (Latin) | Croatian
    "sedemnásť",            // Slovak
    "sedemnajst",           // Slovenian
    "diecisiete",           // Spanish
    "sjutton",              // Swedish
    "பதினாறு",                 // Tamil
    "สิบ​เจ็ด",                // Thai
    "on yedi",              // Turkish
    "сімнадцять",           // Ukrainian
    "ун он ситтар",         // Uzbek (Cyrillic)
    "un on sittar",         // Uzbek (Latin)
    "mười bảy",             // Vietnamese

    // number 17
    "17",
    "i7",
    "I7",
    "|7",
    "!7",

    // password variation
    "33venteen",
    "s3v3nteen",
    "s3v3nt3en",
    "s3v3nt33n",

    // binary
    "0b10001",
    "10001d0",
    "10001",
    "i0001",
    "i000i",
    "io00i",
    "ioo0i",
    "ioooi",

    // Oct
    "0o21",
    "o21",

    // hex
    "0x11",
    "ii",
    "||",
    "ll",
};