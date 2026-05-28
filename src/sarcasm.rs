pub fn process(text: &str) -> String {
    let mut result = String::new();
    let mut upcase = true;
    for c in text.chars() {
        if c.is_alphabetic() {
            if upcase {
                for upper in c.to_uppercase() {
                    result.push(upper);
                }
            } else {
                for lower in c.to_lowercase() {
                    result.push(lower);
                }
            }
            upcase = !upcase;
        } else {
            result.push(c);
        }
    }
    result
}
