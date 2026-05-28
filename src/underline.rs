const UNDERLINE_CHAR: char = '\u{0332}';
const DOUBLE_UNDERLINE_CHAR: char = '\u{0333}';

pub fn process(text: &str, double: bool) -> String {
    let underline_char = if double {
        DOUBLE_UNDERLINE_CHAR
    } else {
        UNDERLINE_CHAR
    };
    let mut result = String::new();
    for c in text.chars() {
        result.push(c);
        result.push(underline_char);
    }
    result
}

pub fn single_underline(text: &str) -> String {
    process(text, false)
}

pub fn double_underline(text: &str) -> String {
    process(text, true)
}
