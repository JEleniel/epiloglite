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
    pub page_id: u128,
    /// Container to which this page belongs.
    pub container_id: Option<u128>,
    /// Flags for the page.
    pub flags: FlagSet<PageFlags>,
    /// Pointer to the next overflow page, if any.
    pub next_page_id: Option<u128>,
    /// The checksum of the page header.
    pub crc: u32,
}
