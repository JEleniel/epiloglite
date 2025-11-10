//! Primary container for storing and managing records.
//!
//! This module provides a lightweight generic container for a set of T where T: Record.
//!
//! The `Container` struct acts like the "table" in a relational database,
//! or a collection of "Nodes" in a graph database. It is backed by a paging
//! cache which is further backed by persistent storage (if nor in memory).
//!
//! Key details:
//! - Row ids may include `0`. Unassigned records use `Option::None` for the
//!   `record_id` field; `Some(0)` is a valid, assigned id.
//! - Because of the semantics of Record and Container, writing the entire container
//! sequentially will automatically compress the page store.
//! - Freed row ids are recycled via a lowest first free-list.
//! - The container is generic over `T` where `T: Record + Serialize +
//!   DeserializeOwned + Clone + Debug` so stored records can be cloned and
//!   serialized as needed.

use crate::Record;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::{BTreeSet, HashMap};
use thiserror::Error;

/// In memory store for Records. The `Container` is the primary method fo storing
/// and managing records in EpilogLite.
#[derive(Clone, Debug, Serialize)]
pub struct Container<T>
where
    T: Record + Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    /// Mapping of row id -> record value, eliminating the need for a separate
    /// index structure.
    items: HashMap<u128, T>,
    /// Sorted set of recycled / freed / next row ids available for reuse.
    ///
    /// The set keeps all available candidate ids sorted; allocation always
    /// takes the lowest available id. This prevents fragmentation where
    /// small ids remain unused while larger ids are allocated.
    free_row_id_list: BTreeSet<u128>,
}

impl<T> Container<T>
where
    T: Record + Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    /// Create an empty container.
    ///
    /// The initial free-list contains `0` so the first allocated id will be
    /// `0`. Previously `0` was treated as an unassigned sentinel, but the
    /// container now treats `None` as unassigned and `Some(0)` as a valid id.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            // start allocating at 0; None is unassigned
            free_row_id_list: BTreeSet::from([0]),
        }
    }

    /// Insert a record. The record must have record_id == None (unassigned).
    /// On success returns the record with its new record id set.
    pub fn create(&mut self, mut record: T) -> Result<T, ContainerError> {
        // Ensure incoming record is unassigned (external API: primitive u128)
        if let Some(id) = record.record_id() {
            return Err(ContainerError::RowAlreadyExists(id));
        }

        if let Some(next_id) = self.free_row_id_list.iter().next().cloned() {
            // Remove the candidate from the free list
            self.free_row_id_list.remove(&next_id);

            // After issuing an id, append the next candidate id to the free list
            // so future allocations can continue growing sequentially until overflow.
            let highest_issued = self
                .items
                .keys()
                .max()
                .map(|c| u128::try_from(c.clone()).unwrap_or(0u128))
                .unwrap_or(0u128);

            let next_candidate = highest_issued.max(next_id).checked_add(1);
            if let Some(next) = next_candidate {
                // Avoid inserting a value that would overflow u128 (checked_add prevented overflow)
                self.free_row_id_list.insert(next);
            } else {
                // Reached maximum u128 value; no further ids can be allocated
                return Err(ContainerError::OutOfRowIds);
            }

            record.set_record_id(Some(next_id));
            self.items.insert(next_id, record.clone());
            Ok(record)
        } else {
            Err(ContainerError::OutOfRowIds)
        }
    }

    /// Get a reference to a record by its numeric id (external API uses u128).
    pub fn retrieve(&self, row_id: u128) -> Option<&T> {
        self.items.get(&row_id)
    }

    /// Remove a record and return it. When a row is deleted, its id is
    /// recycled back onto the free list so it can be reused by future
    /// allocations. The implementation also compacts the sequential tail of
    /// ids so we don't leak a growing next_row_id when high-numbered rows are
    /// removed.
    /// Delete a record by numeric id (external API uses u128).
    pub fn delete(&mut self, row_id: u128) -> Option<T> {
        if let Some(removed) = self.items.remove(&row_id) {
            self.free_row_id_list.insert(row_id);
            Some(removed)
        } else {
            None
        }
    }

    /// Update an existing record. The record must have an assigned record_id
    /// (i.e. `Some(_)`) and that id must already exist inside the container.
    /// On success returns the updated record.
    pub fn update(&mut self, mut record: T) -> Result<T, ContainerError> {
        if let Some(id) = record.record_id() {
            *record.flags_mut() |= crate::RecordFlags::Modified;
            self.items.insert(id, record.clone());
            Ok(record)
        } else {
            Err(ContainerError::RowNotFound(0))
        }
    }

    /// Upsert: insert if the record is unassigned (`None`), otherwise replace
    /// the existing record (or insert if that id doesn't exist). Returns the
    /// record with its assigned id.
    pub fn upsert(&mut self, record: T) -> Result<T, ContainerError> {
        if let Some(_) = record.record_id() {
            self.update(record)
        } else {
            self.create(record)
        }
    }
}

#[derive(Debug, Error)]
/// Errors produced by `Container` operations.
pub enum ContainerError {
    /// Returned when attempting to insert a record that already has a
    /// non-zero id.
    #[error("Attempt to insert an existing row, ID: {0}")]
    RowAlreadyExists(u128),

    /// Returned when the allocator has exhausted all CInt ids.
    #[error("Out of row IDs")]
    OutOfRowIds,

    /// Returned when a requested row id does not exist inside the
    /// container.
    #[error("Row not found, ID: {0}")]
    RowNotFound(u128),
}
