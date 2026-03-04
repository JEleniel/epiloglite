use flagset::FlagSet;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::num::TryFromIntError;
use thiserror::Error;

use crate::{PageFlags, PageHeader};

/// Represents a page in EpilogLite, containing a header and a list of entries.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page {
    /// The header of the page.
    header: PageHeader,
    /// The data in the page.
    data: Vec<u8>,
    /// CRC32 checksum of the page (excluding the crc itself).
    crc: u32,
}

impl Page {
    /// Create a new page
    pub fn new(
        page_id: u128,
        container_id: u128,
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
    pub fn new_free_page(
        page_id: epiloglite_core::Cu128,
        page_size: usize,
    ) -> Result<Self, PageError> {
        let mut zero = Page::new(
            page_id,
            FREE_PAGE_CONTAINER_ID.into(),
            page_size,
            PageFlags::Dirty & PageFlags::Free,
        )?;
        zero = zero.free_page()?;
        Ok(zero)
    }

    /// Get the page ID
    pub fn page_id(&self) -> epiloglite_core::Cu128 {
        self.header.page_id.clone()
    }

    /// Get the container ID
    pub fn container_id(&self) -> epiloglite_core::Cu128 {
        self.header.container_id.clone()
    }

    /// Get the page flags
    pub fn flags(&self) -> FlagSet<PageFlags> {
        self.header.flags
    }

    /// Get the next page ID
    pub fn next_page_id(&self) -> epiloglite_core::Cu128 {
        self.header.next_page_id.clone()
    }

    /// Set the next page ID
    pub fn set_next_page_id(&mut self, next_page_id: epiloglite_core::Cu128) {
        self.header.next_page_id = next_page_id;
        self.header.flags |= PageFlags::Dirty;
    }

    /// Free the page
    pub fn free_page(mut self) -> Result<Self, PageError> {
        self.header.container_id = FREE_PAGE_CONTAINER_ID.into();
        self.header.flags = PageFlags::Free | PageFlags::Dirty;
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
        Ok(self.header.flags.contains(PageFlags::Free)
            && u32::from_be_bytes(self.data[0..4].try_into().unwrap()) == FREE_PAGE_FRONT_GUARD
            && u32::from_be_bytes(self.data[self.data.len() - 4..].try_into().unwrap())
                == FREE_PAGE_BACK_GUARD)
    }

    /// Is the page dirty?
    pub fn is_dirty(&self) -> bool {
        self.header.flags.contains(PageFlags::Dirty)
    }

    /// Set page Dirty flag
    pub fn set_dirty(&mut self) {
        self.header.flags |= PageFlags::Dirty;
    }

    /// Set page Clean flag
    pub fn set_clean(&mut self) {
        self.header.flags ^= PageFlags::Dirty;
    }

    /// Get a reference to the entries in the page
    pub fn entries<T>(&self) -> Result<Vec<T>, PageError>
    where
        T: epiloglite_core::Record
            + std::fmt::Debug
            + Clone
            + serde::ser::Serialize
            + serde::de::DeserializeOwned,
    {
        let mut res: Vec<T> = Vec::new();
        for slot in &self.slot_index {
            if let SlotIndexEntryType::Used { offset, length, .. } = &slot.slot_index_entry_type {
                let offset_usize = usize::try_from(offset.clone())?;
                let length_usize = usize::try_from(length.clone())?;
                // bounds check (exclude crc at end)
                if offset_usize
                    .checked_add(length_usize)
                    .map_or(true, |end| end > self.data.len().saturating_sub(4))
                {
                    return Err(PageError::InvalidSlot(offset_usize, length_usize));
                }
                let rec: T = try_from_slice(&self.data[offset_usize..offset_usize + length_usize])?;
                if !rec.flags().contains(epiloglite_core::RecordFlags::Deleted) {
                    res.push(rec);
                }
            }
        }
        Ok(res)
    }

    /// Get a clone of the record at the specified ID
    pub fn get_record<T>(self, record_id: epiloglite_core::Cu128) -> Result<T, PageError>
    where
        T: std::fmt::Debug + Clone + serde::ser::Serialize + serde::de::DeserializeOwned,
    {
        if let Some(slot) = self
            .slot_index
            .into_iter()
            .find(|s| matches!(s.slot_index_entry_type, SlotIndexEntryType::Used { record_id: ref rid, .. } if *rid == record_id))
        {
            if let SlotIndexEntryType::Used { offset, length, .. } = &slot.slot_index_entry_type {
                let offset_usize = usize::try_from(offset.clone())?;
                let length_usize = usize::try_from(length.clone())?;
                if offset_usize.checked_add(length_usize).map_or(true, |end| end > self.data.len().saturating_sub(4)) {
                    return Err(PageError::InvalidSlot(offset_usize, length_usize));
                }
                let rec: T = try_from_slice(&self.data[offset_usize..offset_usize + length_usize])?;
                return Ok(rec);
            }
        }
        Err(PageError::RecordNotFound(record_id.into()))
    }

    /// Add an entry to the page
    pub fn write_record<T>(&mut self, entry: T) -> Result<(), PageError>
    where
        T: epiloglite_core::Record
            + std::fmt::Debug
            + Clone
            + serde::ser::Serialize
            + serde::de::DeserializeOwned,
    {
        let entry_bytes = try_into_vec(&entry)?;
        let entry_size = entry_bytes.len();

        // Try to reuse a free slot
        for slot in &mut self.slot_index {
            if let SlotIndexEntryType::Free { offset, length } = &slot.slot_index_entry_type {
                let slot_len: usize = usize::try_from(length.clone())?;
                if slot_len >= entry_size {
                    let offset_usize = usize::try_from(offset.clone())?;
                    self.data[offset_usize..offset_usize + entry_size]
                        .copy_from_slice(&entry_bytes);
                    slot.slot_index_entry_type = SlotIndexEntryType::Used {
                        record_id: epiloglite_core::Cu128::from(entry.record_id()),
                        offset: offset.clone(),
                        length: (entry_size as u128).into(),
                    };
                    self.header.flags |= PageFlags::Dirty;
                    return Ok(());
                }
            }
        }

        // No reusable slot; check space
        if self.header.flags.contains(PageFlags::Full) {
            return Err(PageError::PageFull);
        }
        if self.header.free_space() < entry_size {
            self.header.flags |= PageFlags::Full;
            return Err(PageError::PageFull);
        }

        let start = self.data.len();
        self.data.extend_from_slice(&entry_bytes);
        self.header.add_bytes_used(entry_size);
        let new_slot = SlotIndexEntry {
            slot_index_entry_type: SlotIndexEntryType::Used {
                record_id: epiloglite_core::Cu128::from(entry.record_id()),
                offset: (start as u128).into(),
                length: (entry_size as u128).into(),
            },
        };
        self.slot_index.push(new_slot);
        self.header.flags |= PageFlags::Dirty;
        Ok(())
    }

    /// Remove an entry from the page by record_id
    pub fn remove_entry(&mut self, record_id: epiloglite_core::Cu128) -> Result<(), PageError> {
        if let Some(slot) = self
            .slot_index
            .iter_mut()
            .find(|s| matches!(s.slot_index_entry_type, SlotIndexEntryType::Used { record_id: ref rid, .. } if *rid == record_id))
        {
            if let SlotIndexEntryType::Used { offset, length, .. } = &slot.slot_index_entry_type {
                let offset_usize = usize::try_from(offset.clone())?;
                let length_usize = usize::try_from(length.clone())?;
                if offset_usize.checked_add(length_usize).map_or(true, |end| end > self.data.len().saturating_sub(4)) {
                    return Err(PageError::InvalidSlot(offset_usize, length_usize));
                }
                slot.slot_index_entry_type = SlotIndexEntryType::Free { offset: offset.clone(), length: length.clone() };
                self.header.flags |= PageFlags::Dirty;
                return Ok(());
            }
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
    #[error("Record not found: {0:?}")]
    RecordNotFound(epiloglite_core::Cu128),
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
    SlotDecodeError(#[from] TryFromIntError),
    /// CInt decode error
    #[error("Cu128 decode error: {0:?}")]
    Cu128DecodeError(epiloglite_core::CIntError),
}

impl From<epiloglite_core::CIntError> for PageError {
    fn from(e: epiloglite_core::CIntError) -> Self {
        PageError::Cu128DecodeError(e)
    }
}
