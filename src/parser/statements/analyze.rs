use crate::parser::literals::Level2Name;

pub enum AnalyzeStatement {
    Schema(String),
    Level2Object(Level2Name),
}
