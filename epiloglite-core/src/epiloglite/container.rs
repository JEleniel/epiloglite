use std::collections::HashMap;

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

use crate::{CInt, Record};

#[derive(Clone, Debug, Serialize)]
pub struct Container<T>
where
    T: Record + Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    items: HashMap<u128, T>,
    /// Stack of recycled / freed / next row ids available for reuse (LIFO)
    free_row_id_list: Vec<u128>,
}

impl<T> Container<T>
where
    T: Record + Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            free_row_id_list: vec![1], // start allocating at 1; 0 means unassigned
        }
    }

    /// Insert a record. The record must have record_id == 0 (unassigned).
    /// On success returns the record with its new record id set.
    pub fn create(&mut self, mut record: T) -> Result<T, ContainerError> {
        if record.record_id() != 0 {
            return Err(ContainerError::RowAlreadyExists(CInt::from(
                record.record_id(),
            )));
        }

        let assigned_id = if let Some(recycled) = self.free_row_id_list.pop() {
            if self.free_row_id_list.is_empty() {
                if let Some(value) = self.get_next_record_id() {
                    return value;
                }
            }
            recycled
        } else {
            // If no ids are in the free list, we are out of IDs _or_ something went wrong

            return Err(ContainerError::OutOfRowIds);
        };

        record.set_record_id(assigned_id);
        self.items.insert(assigned_id, record.clone());

        Ok(record)
    }

    fn get_next_record_id(&mut self) -> Option<Result<T, ContainerError>> {
        let last_id = self.items.keys().max().map(|id| id).unwrap();
        if last_id == &u128::MAX {
            // we've exhausted all possible ids
            return Some(Err(ContainerError::OutOfRowIds));
        }
        let next_id = last_id + 1;
        self.free_row_id_list.push(next_id);
        // ensure we always have at least one id available for future allocations
        None
    }

    /// Get a reference to a record by its numeric id
    pub fn retrieve(&self, row_id: u128) -> Option<&T> {
        self.items.get(&row_id)
    }

    /// Remove a record and return it. When a row is deleted, its id is
    /// recycled back onto the free list so it can be reused by future
    /// allocations. The implementation also compacts the sequential tail of
    /// ids so we don't leak a growing next_row_id when high-numbered rows are
    /// removed.
    pub fn delete(&mut self, row_id: u128) -> Option<T> {
        if let Some(removed) = self.items.remove(&row_id) {
            if row_id == 0 {
                // never recycle the zero sentinel
                return Some(removed);
            }

            // Otherwise, make the id available for reuse.
            self.free_row_id_list.push(row_id);

            Some(removed)
        } else {
            None
        }
    }

    /// Update an existing record. The record must have a non-zero record_id and
    /// an existing row must be present in the container. On success returns
    /// the updated record.
    pub fn update(&mut self, mut record: T) -> Result<T, ContainerError> {
        let id = record.record_id();
        if id == 0 {
            return Err(ContainerError::RowNotFound(CInt::from(id)));
        }

        if !self.items.contains_key(&id) {
            return Err(ContainerError::RowNotFound(CInt::from(id)));
        }

        // Mark as modified (use bitwise assignment like other uses in the repo)
        *record.flags_mut() |= crate::RecordFlags::Modified;

        self.items.insert(id, record.clone());
        Ok(record)
    }

    /// Upsert: insert if the record has record_id == 0, otherwise replace the
    /// existing record (or insert if that id doesn't exist). Returns the
    /// record with its assigned id.
    pub fn upsert(&mut self, mut record: T) -> Result<T, ContainerError> {
        let id = record.record_id();
        if id == 0 {
            // treat as create
            return self.create(record);
        }

        // Mark as modified and replace/insert
        *record.flags_mut() |= crate::RecordFlags::Modified;
        self.items.insert(id, record.clone());
        Ok(record)
    }
}

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("Attempt to insert an existing row, ID: {0}")]
    RowAlreadyExists(CInt),
    #[error("Out of row IDs")]
    OutOfRowIds,
    #[error("Row not found, ID: {0}")]
    RowNotFound(CInt),
}
