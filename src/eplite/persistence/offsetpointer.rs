use serde::{Deserialize, Serialize};

use crate::CInt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OffsetPointer {
    pub page_number: CInt,
    pub offset: CInt,
}
