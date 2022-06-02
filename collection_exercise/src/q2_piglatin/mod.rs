/// Convert string to pig latin.
///
/// E.g. "first" -> "irst-fay"
/// E.g. "apple" -> "apple-hey"
///
pub fn piglatin(word: &str) -> String {
    if word.chars().count() == 0 {
        panic!("Input should be a word");
    }
    let first_letter = &word[0..1];
    let is_vowel: bool = match first_letter {
        "a" | "e" | "i" | "o" | "u" => true,
        _ => false,
    };
    if is_vowel {
        format!("{}-hey", word)
    } else {
        let prefix = &word[1..];
        format!("{}-{}ay", prefix, first_letter)
    }
}
