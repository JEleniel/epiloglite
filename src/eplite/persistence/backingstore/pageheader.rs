use epiloglite_core::{CInt, PageFlags, serialized_size};
use flagset::FlagSet;
use serde::{Deserialize, Serialize};

/// The header used by all pages in EpilogLite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageHeader {
    /// The ID of the page.
    pub page_id: CInt,
    /// Container to which this page belongs.
    pub container_id: CInt,
    /// Flags for the page.
    pub flags: FlagSet<PageFlags>,
    /// Pointer to the next overflow page, if any.
    pub next_page_id: CInt,
    /// Size of the page header in bytes.
    #[serde(skip)]
    header_size: usize,
    /// Maximum size of the page in bytes, including the header and footer.
    #[serde(skip)]
    page_size: usize,
    /// Current page size in bytes
    #[serde(skip)]
    bytes_used: usize,
}

impl PageHeader {
    /// Create a new PageHeader
    pub fn new(
        page_size: usize,
        page_id: CInt,
        container_id: CInt,
        flags: FlagSet<PageFlags>,
    ) -> Self {
        let mut header = PageHeader {
            page_id: 0.into(),
            container_id,
            flags,
            next_page_id: 0.into(),
            header_size: 0,
            page_size,
            bytes_used: 0,
        };
        header.header_size = serialized_size(&header).unwrap();
        header.bytes_used = header.header_size + 4; // Initial bytes used is header size + CRC
        header
    }

    /// Get the bytes used
    pub fn bytes_used(&self) -> usize {
        self.bytes_used
    }

    /// Set the bytes used
    pub fn add_bytes_used(&mut self, used_bytes: usize) {
        self.bytes_used += used_bytes;
        let header_size = serialized_size(&self).unwrap();
        if header_size != self.header_size {
            self.bytes_used += header_size - self.header_size;
            self.header_size = header_size;
        }
    }

    /// Subtract bytes used
    pub fn subtract_bytes_used(&mut self, used_bytes: usize) {
        if used_bytes > self.bytes_used {
            self.bytes_used = 0;
            return;
        }
        self.bytes_used -= used_bytes;
    }

    /// Get the page size
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    /// Get the header size
    pub fn header_size(&self) -> usize {
        self.header_size
    }

    /// Get free space available in the page
    pub fn free_space(&self) -> usize {
        self.page_size - self.bytes_used
    }
}
