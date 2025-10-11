use regex::Regex;

pub trait Identifier {
    fn is_valid_identifier(&self) -> bool;
}

impl Identifier for String {
    fn is_valid_identifier(&self) -> bool {
        {
            let regex_identifier: Regex = Regex::new("^[A-Za-z][A-Za-z0-9_]*$").unwrap();
            regex_identifier.is_match(&self)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_identifiers() {
        use super::Identifier;
        let valid_identifiers = vec![
            "a",
            "A",
            "abc",
            "ABC",
            "a1",
            "A1",
            "a_b_c",
            "A_B_C",
            "a1_b2_c3",
            "A1_B2_C3",
            "a_b_c_123",
            "A_B_C_123",
        ];
        for identifier in valid_identifiers {
            assert!(identifier.to_string().is_valid_identifier());
        }
    }

    #[test]
    fn test_invalid_identifiers() {
        use super::Identifier;
        let invalid_identifiers = vec![
            "", "1", "_", "1a", "_a", "a-b-c", "a b c", "a.b.c", "a,b,c", "a@b#c$", "a!b%c^",
            "a&b*c(", "a)b+c=", "a{b}c[", "]a|b\\c;", ":\"'<>?", "/`~",
        ];
        for identifier in invalid_identifiers {
            println!("Testing invalid identifier: {}", identifier);
            assert!(!identifier.to_string().is_valid_identifier());
        }
    }
}
