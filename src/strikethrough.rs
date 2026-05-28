const STRIKETHROUGH_CHAR: char = '\u{0336}';

pub fn process(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(c);
        result.push(STRIKETHROUGH_CHAR);
    }
    result
}
