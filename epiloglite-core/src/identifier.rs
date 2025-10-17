use regex::Regex;
use std::cell::RefCell;

/// Trait for validating identifier strings according to EpilogLite rules.
pub trait Identifier {
    /// Returns true if the string is a valid identifier (starts with a letter, contains only letters, digits, or underscores).
    fn is_valid_identifier(&self) -> bool;
}

impl Identifier for String {
    /// Checks if the string is a valid identifier.
    fn is_valid_identifier(&self) -> bool {
        thread_local! {
            static IDENT_REGEX: RefCell<Regex> = RefCell::new(Regex::new(r"^[A-Za-z][A-Za-z0-9_]*$").unwrap());
        }
        IDENT_REGEX.with(|regex| regex.borrow().is_match(self))
    }
}

#[cfg(test)]
#[path = "tests/identifier_tests.rs"]
mod identifier_tests;
