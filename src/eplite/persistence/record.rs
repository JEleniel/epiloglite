use serde::{Deserialize, Serialize};

use crate::{DataType, Varint};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub total_bytes: Varint,
    pub column_types: Vec<DataType>,
}
