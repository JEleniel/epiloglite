use epiloglite_core::{Cu128, serialized_size};
use flagset::FlagSet;
use serde::{Deserialize, Serialize};

use flagset::flags;

// Page flags live next to the PageHeader type so callers see the flags in the
// same module as the primary struct that owns them.
flags! {
    /// Flags set on pages managed by the storage layer.
    ///
    /// - `Dirty`: page has been modified in memory and needs to be flushed.
    /// - `Free`: page is available for allocation.
    /// - `Full`: page is at capacity for the allocation strategy.
    #[derive(Serialize, Deserialize)]
    pub enum PageFlags: u8 {
        /// No flags set.
        None = 0,
        /// Page has been modified and needs to be written to backing storage.
        Dirty = 1 << 0,
        /// Page is free and available for reuse.
        Free = 1 << 1,
        /// Page is full according to the page allocation policy.
        Full = 1 << 2,
    }
}

/// The header used by all pages in EpilogLite.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageHeader {
    /// The ID of the page.
    pub page_id: Cu128,
    /// Container to which this page belongs.
    pub container_id: Cu128,
    /// Flags for the page.
    pub flags: FlagSet<PageFlags>,
    /// Pointer to the next overflow page, if any.
    pub next_page_id: Cu128,
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
        page_id: Cu128,
        container_id: Cu128,
        flags: FlagSet<PageFlags>,
    ) -> Self {
        let mut header = PageHeader {
            page_id: Cu128::from(0u128),
            container_id,
            flags,
            next_page_id: Cu128::from(0u128),
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
        let header_size = serialized_size(self).unwrap();
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
