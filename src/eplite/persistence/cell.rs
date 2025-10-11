use serde::{Deserialize, Serialize};

use crate::Varint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    TableLeaf {
        total_bytes: Varint,
        row_id: Varint,
        payload: Vec<u8>,
        overflow_page: Option<u32>,
    },
    TableInterior {
        left_child_page: u32,
        row_id: Varint,
    },
    IndexLeaf {
        total_bytes: Varint,
        payload: Vec<u8>,
        overflow_page: Option<u32>,
    },
    IndexInterior {
        left_child_page: u32,
        total_bytes: Varint,
        payload: Vec<u8>,
        overflow_page: Option<u32>,
    },
}
