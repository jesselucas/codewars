//mod preloaded;
//use preloaded::MORSE_CODE;
use std::collections::HashMap;
use std::str::Split;

fn create_code_hashmap() -> HashMap<&'static str, &'static str> {
    [
        (".-", "A"),
        ("-...", "B"),
        ("-.-.", "C"),
        ("-..", "D"),
        (".", "E"),
        ("..-.", "F"),
        ("--.", "G"),
        ("....", "H"),
        ("..", "I"),
        (".---", "J"),
        ("-.-", "K"),
        (".-..", "L"),
        ("--", "M"),
        ("-.", "N"),
        ("---", "O"),
        (".--.", "P"),
        ("--.-", "Q"),
        (".-.", "R"),
        ("...", "S"),
        ("-", "T"),
        ("..-", "U"),
        ("...-", "V"),
        (".--", "W"),
        ("-..-", "X"),
        ("-.--", "Y"),
        ("--..", "Z"),
    ]
    .iter()
    .cloned()
    .collect()
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn decode_morse_orig(encoded: &str) -> String {
	let MORSE_CODE = create_code_hashmap();

    let encoded = encoded.trim();
    if encoded.is_empty() {
        return "".to_string();
    }

    let mut result: String = String::new();
    let codes: Split<&str> = encoded.split(" ");
    let mut previous_space = false;
    for code in codes {
        if code.is_empty() && !previous_space {
            result.push_str(" ");
            previous_space = true;
        } else {
            previous_space = false;
        }

        match MORSE_CODE.get(&code) {
            Some(l) => result.push_str(l),
            None => continue,
        };
    }

    result
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn decode_morse(encoded: &str) -> String {
	let MORSE_CODE = create_code_hashmap();

    let mut result: String = String::new();
    let codes: Split<&str> = encoded.split(" ");
    for code in codes {
        match MORSE_CODE.get(&code) {
            Some(l) => result.push_str(l),
            None => result.push(' '),
        };
    }

	result.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(decode_morse("  .... . -.--   .--- ..- -.. .  "), "HEY JUDE");
    }

    #[test]
    fn test_space() {
        assert_eq!(decode_morse(" "), "");
    }
}
