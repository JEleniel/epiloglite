use regex::Regex;

pub trait Identifier {
    fn is_valid_identifier(&self) -> bool;
}

impl Identifier for String {
    fn is_valid_identifier(&self) -> bool {
        {
            let regex_identifier: Regex = Regex::new("[A-Za-z][A-Za-z0-9_]*").unwrap();
            regex_identifier.is_match(&self)
        }
    }
}
