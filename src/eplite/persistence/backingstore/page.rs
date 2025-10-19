use epiloglite_core::{
    CIntError, PageFlags, RecordFlags, RecordTrait, SerializeError, calculate_crc, serialized_size,
    try_from_slice,
};
use flagset::FlagSet;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::eplite::{
    FREE_PAGE_CONTAINER_ID,
    persistence::{
        FREE_PAGE_BACK_GUARD, FREE_PAGE_FRONT_GUARD,
        backingstore::{SlotFlags, SlotIndexEntry},
    },
};
use crate::try_into_vec;
use crate::{CInt, eplite::persistence::backingstore::PageHeader};

/// Represents a page in EpilogLite, containing a header and a list of entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// The header of the page.
    header: PageHeader,
    /// The slot index for the page
    slot_index: Vec<SlotIndexEntry>,
    /// The data in the page.
    data: Vec<u8>,
    /// CRC32 checksum of the page (excluding the crc itself).
    crc: u32,
}

impl Page {
    /// Create a new page
    pub fn new(
        page_id: CInt,
        container_id: CInt,
        page_size: usize,
        flags: FlagSet<PageFlags>,
    ) -> Result<Self, PageError> {
        let header: PageHeader = PageHeader::new(page_size, page_id, container_id, flags);

        let mut new_page = Page {
            header,
            slot_index: Vec::new(),
            data: Vec::new(),
            crc: 0,
        };

        new_page.crc = calculate_crc(&new_page)?;
        Ok(new_page)
    }

    /// Get a new free page filled with zeroes and guard bytes
    pub fn new_free_page(page_id: CInt, page_size: usize) -> Result<Self, PageError> {
        let mut zero = Page::new(
            page_id,
            FREE_PAGE_CONTAINER_ID.into(),
            page_size,
            PageFlags::DIRTY & PageFlags::FREE,
        )?;
        zero = zero.free_page()?;
        Ok(zero)
    }

    /// Get the page ID
    pub fn page_id(&self) -> CInt {
        self.header.page_id.clone()
    }

    /// Get the container ID
    pub fn container_id(&self) -> CInt {
        self.header.container_id.clone()
    }

    /// Get the page flags
    pub fn flags(&self) -> FlagSet<PageFlags> {
        self.header.flags
    }

    /// Get the next page ID
    pub fn next_page_id(&self) -> CInt {
        self.header.next_page_id.clone()
    }

    /// Set the next page ID
    pub fn set_next_page_id(&mut self, next_page_id: CInt) {
        self.header.next_page_id = next_page_id;
        self.header.flags |= PageFlags::DIRTY;
    }

    /// Free the page
    pub fn free_page(mut self) -> Result<Self, PageError> {
        self.header.container_id = FREE_PAGE_CONTAINER_ID.into();
        self.header.flags = PageFlags::FREE | PageFlags::DIRTY;
        self.header
            .subtract_bytes_used(serialized_size(&self.data)?);
        self.data.clear();
        let fill_len = self.header.page_size() - self.header.header_size() - 4;
        self.data.extend_from_slice(&vec![0; fill_len]);
        self.data[..4].copy_from_slice(&FREE_PAGE_FRONT_GUARD.to_be_bytes());
        let self_len = self.data.len();
        self.data[self_len - 4..].copy_from_slice(&FREE_PAGE_BACK_GUARD.to_be_bytes());
        self.header.add_bytes_used(fill_len);
        if self.header.bytes_used() != self.header.page_size() {
            return Err(PageError::FreePageAllocationFailed(
                self.header.bytes_used(),
                self.header.page_size(),
            ));
        }
        Ok(self)
    }

    /// Get the current page usage in bytes
    pub fn bytes_used(&self) -> usize {
        self.header.bytes_used()
    }

    /// Is the page free?
    pub fn is_free_page(&self) -> Result<bool, PageError> {
        Ok(self.header.flags.contains(PageFlags::FREE)
            && u32::from_be_bytes(self.data[0..4].try_into().unwrap()) == FREE_PAGE_FRONT_GUARD
            && u32::from_be_bytes(self.data[self.data.len() - 4..].try_into().unwrap())
                == FREE_PAGE_BACK_GUARD)
    }

    /// Is the page dirty?
    pub fn is_dirty(&self) -> bool {
        self.header.flags.contains(PageFlags::DIRTY)
    }

    /// Set page Dirty flag
    pub fn set_dirty(&mut self) {
        self.header.flags |= PageFlags::DIRTY;
    }

    /// Set page Clean flag
    pub fn set_clean(&mut self) {
        self.header.flags ^= PageFlags::DIRTY;
    }

    /// Get a reference to the entries in the page
    pub fn entries<T>(&self) -> Result<Vec<T>, PageError>
    where
        T: RecordTrait
            + std::fmt::Debug
            + Clone
            + serde::ser::Serialize
            + serde::de::DeserializeOwned,
    {
        let mut res: Vec<T> = Vec::new();
        for slot in &self.slot_index {
            if slot.flags.contains(SlotFlags::Active) {
                let offset: usize = slot.offset.clone().try_into()?;
                let new_record: T = try_from_slice(&self.data[offset..])?;
                if !new_record.flags().contains(RecordFlags::DELETED) {
                    res.push(new_record);
                }
            }
        }
        Ok(res)
    }

    /// Get a clone of the record at the specified ID
    pub fn get_record<T>(self, record_id: CInt) -> Result<T, PageError>
    where
        T: std::fmt::Debug + Clone + serde::ser::Serialize + serde::de::DeserializeOwned,
    {
        if let Some(slot) = self
            .slot_index
            .into_iter()
            .find(|r| r.record_id == record_id)
        {
            let offset: usize = slot.offset.clone().try_into()?;
            let length: usize = slot.length.clone().try_into()?;
            if &offset + &length > (self.data.len() - 4).into() {
                return Err(PageError::InvalidSlot(offset, length));
            }
            let offset: usize = slot.offset.clone().try_into()?;
            let length: usize = slot.length.clone().try_into()?;
            let record: T = try_from_slice(&self.data[offset..offset + length])?;
            Ok(record)
        } else {
            Err(PageError::RecordNotFound(record_id))
        }
    }

    /// Add an entry to the page
    pub fn write_record<T>(&mut self, entry: T) -> Result<(), PageError>
    where
        T: RecordTrait
            + std::fmt::Debug
            + Clone
            + serde::ser::Serialize
            + serde::de::DeserializeOwned,
    {
        let entry_bytes = try_into_vec(&entry).unwrap();
        let entry_size = entry_bytes.len();

        for slot in &mut self.slot_index {
            if slot.flags.contains(SlotFlags::Free) {
                let slot_len: usize = slot.length.clone().try_into()?;
                if slot_len >= entry_size {
                    // Reuse this slot
                    let offset: usize = slot.offset.clone().try_into()?;

                    self.data[offset..offset + entry_size].copy_from_slice(&entry_bytes);
                    slot.length = entry_size.into();
                    slot.record_id = entry.record_id();
                    slot.flags ^= SlotFlags::Free;
                    slot.flags |= SlotFlags::Active;
                    slot.flags |= SlotFlags::Dirty;
                    return Ok(());
                }
            }
        }

        if self.header.flags.contains(PageFlags::FULL) {
            return Err(PageError::PageFull);
        }

        if self.header.free_space() < entry_size {
            self.header.flags |= PageFlags::FULL;
            return Err(PageError::PageFull);
        }
        self.data.extend(entry_bytes);
        self.header.add_bytes_used(entry_size);
        let new_slot = SlotIndexEntry {
            flags: SlotFlags::Active | SlotFlags::Dirty,
            record_id: entry.record_id(),
            offset: (self.data.len() - entry_size).into(),
            length: entry_size.into(),
        };
        self.slot_index.push(new_slot);
        self.header.flags |= PageFlags::DIRTY;
        Ok(())
    }

    /// Remove an entry from the page by record_id
    pub fn remove_entry(&mut self, record_id: CInt) -> Result<(), PageError> {
        if let Some(slot) = self
            .slot_index
            .iter_mut()
            .find(|r| r.record_id == record_id)
        {
            let offset: usize = slot.offset.clone().try_into()?;
            let length: usize = slot.length.clone().try_into()?;
            // safer bounds check to avoid underflow / overflow
            if offset
                .checked_add(length)
                .map_or(true, |end| end > self.data.len().saturating_sub(4))
            {
                return Err(PageError::InvalidSlot(offset, length));
            }
            // remove Active, add Free and Dirty
            slot.flags ^= SlotFlags::Active;
            slot.flags |= SlotFlags::Free | SlotFlags::Dirty;
            self.header.flags |= PageFlags::DIRTY;
            return Ok(());
        }
        Err(PageError::RecordNotFound(record_id))
    }
}

/// Errors that can occur when working with pages.
#[derive(Debug, Error)]
pub enum PageError {
    /// Index out of bounds.
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
    /// Record not found.
    #[error("Record not found: {0}")]
    RecordNotFound(CInt),
    /// Invalid slot
    #[error("Invalid slot: {0}, length: {1}")]
    InvalidSlot(usize, usize),
    /// Page is full.
    #[error("Page is full")]
    PageFull,
    /// Serialization or deserialization error.
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] SerializeError),
    #[error("Free page allocation failed, expected {0} bytes, got {1} bytes")]
    FreePageAllocationFailed(usize, usize),
    /// Slot decode error
    #[error("Slot decode error: {0:?}")]
    SlotDecodeError(#[from] CIntError),
}
