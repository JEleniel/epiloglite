#[derive(Debug, Clone, PartialEq)]
pub enum CollatingFunction {
    BINARY,
    NOCASE,
    UNICODENOCASE,
    RTRIM,
}

impl CollatingFunction {
    pub fn collate(&self, a: &str, b: &str) -> std::cmp::Ordering {
        match self {
            CollatingFunction::BINARY => a.cmp(b),
            CollatingFunction::NOCASE => a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()),
            CollatingFunction::UNICODENOCASE => a.to_lowercase().cmp(&b.to_lowercase()), // Placeholder for actual Unicode case-insensitive comparison
            CollatingFunction::RTRIM => a.trim_end().cmp(b.trim_end()),
        }
    }
}

impl Default for CollatingFunction {
    fn default() -> Self {
        CollatingFunction::BINARY
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_collate_binary() {
        use super::CollatingFunction;
        let collator = CollatingFunction::BINARY;
        assert_eq!(collator.collate("abc", "abc"), std::cmp::Ordering::Equal);
        assert_eq!(collator.collate("abc", "abd"), std::cmp::Ordering::Less);
        assert_eq!(collator.collate("abd", "abc"), std::cmp::Ordering::Greater);
        assert_eq!(collator.collate("abc", "ABC"), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_collate_nocase() {
        use super::CollatingFunction;
        let collator = CollatingFunction::NOCASE;
        assert_eq!(collator.collate("abc", "ABC"), std::cmp::Ordering::Equal);
        assert_eq!(collator.collate("abc", "abd"), std::cmp::Ordering::Less);
        assert_eq!(collator.collate("abd", "abc"), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_collate_rtrim() {
        use super::CollatingFunction;
        let collator = CollatingFunction::RTRIM;
        assert_eq!(collator.collate("abc   ", "abc"), std::cmp::Ordering::Equal);
        assert_eq!(collator.collate("abc", "abc   "), std::cmp::Ordering::Equal);
        assert_eq!(collator.collate("abc ", "abd "), std::cmp::Ordering::Less);
        assert_eq!(
            collator.collate("abd ", "abc "),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_collate_unicodenocase() {
        use super::CollatingFunction;
        let collator = CollatingFunction::UNICODENOCASE;
        assert_eq!(collator.collate("Ù", "ù"), std::cmp::Ordering::Equal);
        assert_eq!(collator.collate("abc", "ABD"), std::cmp::Ordering::Less);
        assert_eq!(collator.collate("ABD", "abc"), std::cmp::Ordering::Greater);
    }
}
