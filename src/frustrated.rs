pub fn process(text: &str) -> String {
    format!("{}.", text.split_whitespace().map(|w| w.to_uppercase()).collect::<Vec<_>>().join(". "))
}
