use serde::{Deserialize, Serialize};

use crate::{CInt, DataType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub total_bytes: CInt,
    pub column_types: Vec<DataType>,
}
