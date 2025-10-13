use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::{
    CInt,
    persistence::{JournalEntry, OffsetPointer},
};

pub const FREE_PAGE_FRONT_GUARD: u32 = 0xDECAFACE;
pub const FREE_PAGE_BACK_GUARD: u32 = 0xECAFACED;

/// The header used by all non-free pages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageHeader {
    /// The type of the page
    pub page_type: PageType,
    /// Flags for the page
    pub flags: u8,
    /// The number of items in the page
    pub counter: CInt,
}

impl PageHeader {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(6);
        bytes.push(self.page_type.clone() as u8);
        bytes.push(self.flags);
        bytes.extend(&self.counter.bytes());
        bytes
    }
}

/// The footer used by all non-free pages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageFooter {
    /// CRC32 checksum of the page (excluding the footer)
    pub page_crc: u32,
    /// Pointer to the next overflow page, if any
    pub overflow_pointer: OffsetPointer,
}

impl PageFooter {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(12);
        bytes.extend(&self.page_crc.to_be_bytes());
        bytes.extend(&self.overflow_pointer.bytes());
        bytes
    }
}

/// A free page, which contains no useful data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreePage {
    /// A magic number to mark the start of a free page
    pub front_guard: u32,
    /// All zeroes to fill the rest of the page
    pub zeroes: Vec<u8>,
    /// A magic number to mark the end of a free page
    pub back_guard: u32,
}

impl FreePage {
    pub fn new(page_size: usize) -> Self {
        FreePage {
            front_guard: FREE_PAGE_FRONT_GUARD,
            zeroes: vec![0; page_size - 8],
            back_guard: FREE_PAGE_BACK_GUARD,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.zeroes.len());
        bytes.extend(&self.front_guard.to_be_bytes());
        bytes.extend(&self.zeroes);
        bytes.extend(&self.back_guard.to_be_bytes());
        bytes
    }
}

/// A journal page, which contains a list of journal entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalPage {
    /// The header of the page
    pub header: PageHeader,
    /// The journal entries in the page
    pub entries: Vec<JournalEntry>,
    /// The footer of the page
    pub footer: PageFooter,
}

impl JournalPage {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.header.bytes());
        for entry in &self.entries {
            bytes.extend(&entry.bytes());
        }
        bytes.extend(&self.footer.bytes());
        bytes
    }
}

/// A data page, which contains a list of data items of type `T`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPage<T> {
    /// The header of the page
    pub header: PageHeader,
    /// The data items in the page
    pub data: Vec<T>,
    /// The footer of the page
    pub footer: PageFooter,
}

/// The type of a page (free pages are untyped)
/// Zero is reserved as a invalid page type.
#[derive(Debug, Clone, FromRepr, Serialize, Deserialize, PartialEq)]
#[repr(u8)]
pub enum PageType {
    /// A Journal page
    Journal = 1,
    /// A data page
    Data = 2,
}
