use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Page {
    BTreePage {
        page_tpye: BTreePageType,
        free_offset: u16,
        num_cells: u16,
        cell_content_area: u16,
        fragmented_free_bytes: u8,
        right_most_pointer: Option<u32>,
        cell_pointers: Vec<u16>,
        cell_contents: Vec<Vec<u8>>,
        reserved: Vec<u8>,
    },
    FreelistTrunkPage {
        next_trunk_page: Option<u32>,
        num_leaf_pages: u32,
        leaf_pages: Vec<u32>,
    },
    FreelistLeafPage {
        bytes: Vec<u8>,
    },
    OverflowPage {
        next_page: u32,
        data: Vec<u8>,
    },
    PointerMapPage {
        pointer_maps: Vec<PointerMap>,
    },
    LockBytePage {
        bytes: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerMap {
    page_type: PointerMapType,
    page_number: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRepr)]
#[repr(u8)]
pub enum BTreePageType {
    InteriorIndex = 2,
    InteriorTable = 5,
    LeafIndex = 10,
    LeafTable = 13,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRepr)]
#[repr(u8)]
pub enum PointerMapType {
    BTreeRoot = 1,
    FreeList = 2,
    OverflowRoot = 3,
    OverflowPage = 4,
    BTreePage = 5,
}
