use serde::{Deserialize, Serialize};

mod logging;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Configuration {}
