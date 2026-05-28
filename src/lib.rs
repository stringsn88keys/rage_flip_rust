pub mod flipper;
pub mod sarcasm;
pub mod strikethrough;
pub mod underline;
pub mod chaos;
pub mod text_substitution;
pub mod frustrated;
pub mod clipboard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip() {
        let result = flipper::flip("Hello");
        assert_eq!(result, "oʅʅǝH");
    }

    #[test]
    fn test_table_flip() {
        let result = flipper::table_flip("Hello");
        assert!(result.contains("╯"));
        assert!(result.contains("┻"));
    }

    #[test]
    fn test_sarcasm() {
        let result = sarcasm::process("Hello World");
        assert_eq!(result, "HeLlO wOrLd");
    }

    #[test]
    fn test_strikethrough() {
        let result = strikethrough::process("Hello");
        assert_eq!(result, "H̶e̶l̶l̶o̶");
    }

    #[test]
    fn test_underline() {
        let result = underline::single_underline("Hello");
        assert_eq!(result, "H̲e̲l̲l̲o̲");
    }

    #[test]
    fn test_double_underline() {
        let result = underline::double_underline("Hello");
        assert_eq!(result, "H̳e̳l̳l̳o̳");
    }

    #[test]
    fn test_chaos() {
        let result = chaos::process("Hello", Some(50));
        assert!(!result.is_empty());
    }

    #[test]
    fn test_frustrated() {
        let result = frustrated::process("Hello");
        assert_eq!(result, "HELLO.");
    }
}
