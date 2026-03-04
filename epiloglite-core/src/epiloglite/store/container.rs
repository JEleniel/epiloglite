//! Primary container for storing and managing records.
//!
//! This module provides a lightweight generic container for a set of T where T: Record.
//!
//! The `Container` struct acts like the "table" in a relational database,
//! or a collection of "Nodes" in a graph database. It is backed by a paging
//! cache which is further backed by persistent storage (if not in memory).
//!
//! Key details:
//! - The row_id None is treated as unassigned; the first assigned row id is 0.
//! - Because of the semantics of Record and Container, writing the entire container
//! sequentially will automatically compress the page store.
//! - Freed row ids are recycled via a lowest first free-list.

use crate::{Record, RecordFlags};
use serde::{Serialize, de::DeserializeOwned};
use std::any::Any;
use std::collections::{BTreeSet, HashMap};
use thiserror::Error;

// Private sealing module prevents downstream crates from implementing
// `ContainerAny`. Only types in this module/crate can implement the sealed
// trait, therefore only `Container<T>` (implemented below) can implement
// `ContainerAny`.
mod private {
    pub trait Sealed {}
}

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

/// Object-safe facade for containers so they can be stored as trait objects.
///
/// This trait is intentionally minimal: it exposes `as_any`/`as_any_mut` so
/// callers can downcast back to the concrete `Container<T>` when needed.
pub trait ContainerAny: private::Sealed {
    /// Return a reference as `dyn Any` for downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Return a mutable reference as `dyn Any` for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Return the number of live (non-deleted) items in the container.
    fn len(&self) -> usize;

    /// Return true if the container contains no live (non-deleted) items.
    fn is_empty(&self) -> bool;

    /// Return the TypeId of the concrete container's record type.
    fn type_id(&self) -> std::any::TypeId;

    /// Return true if a row id exists and is not deleted.
    fn has_row(&self, row_id: u128) -> bool;

    /// Delete a row by id. Returns the same errors as `Container::delete`.
    fn delete_row(&mut self, row_id: u128) -> Result<(), ContainerError>;

    /// Retrieve a cloned record boxed as `Box<dyn Any>` so callers can downcast.
    fn retrieve_any(&self, row_id: u128) -> Option<Box<dyn Any>>;

    /// Upsert a record provided as a boxed `Any`. Returns the stored record
    /// boxed as `Any` on success.
    fn upsert_any(&mut self, record: Box<dyn Any>) -> Result<Box<dyn Any>, ContainerError>;
}

impl<T> ContainerAny for Container<T>
where
    T: Record + Serialize + DeserializeOwned + Clone + std::fmt::Debug + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.items
            .iter()
            .filter(|(_, v)| !v.flags().contains(RecordFlags::Deleted))
            .count()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn has_row(&self, row_id: u128) -> bool {
        self.items
            .get(&row_id)
            .map(|r| !r.flags().contains(RecordFlags::Deleted))
            .unwrap_or(false)
    }

    fn delete_row(&mut self, row_id: u128) -> Result<(), ContainerError> {
        if let Some(r) = self.items.get_mut(&row_id) {
            r.set_flag_deleted();
            self.free_row_id_list.insert(row_id);
            Ok(())
        } else {
            Err(ContainerError::RowNotFound(row_id))
        }
    }

    fn retrieve_any(&self, row_id: u128) -> Option<Box<dyn Any>> {
        self.items.get(&row_id).and_then(|r| {
            if r.flags().contains(RecordFlags::Deleted) {
                None
            } else {
                Some(Box::new(r.clone()) as Box<dyn Any>)
            }
        })
    }

    fn upsert_any(&mut self, record: Box<dyn Any>) -> Result<Box<dyn Any>, ContainerError> {
        // Attempt to downcast the incoming boxed any to the concrete record type.
        let maybe = record.downcast::<T>();
        match maybe {
            Ok(boxed) => {
                let mut rec = *boxed;
                if let Some(row_id) = rec.record_id() {
                    rec.set_flag_modified();
                    self.items.insert(row_id, rec.clone());
                    Ok(Box::new(rec) as Box<dyn Any>)
                } else {
                    let new_row_id = self.get_next_row_id()?;
                    rec.try_set_record_id(new_row_id)
                        .map_err(|_| ContainerError::RowAlreadyExists(new_row_id))?;
                    rec.set_flag_modified();
                    let ret = rec.clone();
                    self.items.insert(new_row_id, rec);
                    Ok(Box::new(ret) as Box<dyn Any>)
                }
            }
            Err(_) => Err(ContainerError::TypeMismatch),
        }
    }
}

// Implement the private sealing trait for Container<T> so it can implement
// `ContainerAny` but external crates cannot implement `ContainerAny`.
impl<T> private::Sealed for Container<T> where
    T: Record + Serialize + DeserializeOwned + Clone + std::fmt::Debug + 'static
{
}

impl<T> Container<T>
where
    T: Record + Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    /// Create an empty container.
    ///
    /// The initial free-list contains `0` so the first allocated id per container will be
    /// `0`.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            free_row_id_list: BTreeSet::from([0]),
        }
    }

    /// Get a reference to a record by its numeric id (external API uses u128).
    pub fn retrieve(&self, row_id: u128) -> Option<&T> {
        if let Some(record) = self.items.get(&row_id) {
            if record.flags().contains(RecordFlags::Deleted) {
                None
            } else {
                Some(record)
            }
        } else {
            None
        }
    }

    /// Remove a record and return it. When a row is deleted, its id is
    /// recycled back onto the free list so it can be reused by future
    /// allocations. The implementation also compacts the sequential tail of
    /// ids so we don't leak a growing next_row_id when high-numbered rows are
    /// removed.
    /// Delete a record by numeric id (external API uses u128).
    pub fn delete(&mut self, row_id: u128) -> Result<(), ContainerError> {
        if let Some(to_remove) = self.items.get_mut(&row_id) {
            to_remove.set_flag_deleted();
            self.free_row_id_list.insert(row_id);
            Ok(())
        } else {
            Err(ContainerError::RowNotFound(row_id))
        }
    }

    /// Upsert: insert if the record is unassigned (`None`), otherwise replace
    /// the existing record Returns the record with its assigned id.
    pub fn upsert(&mut self, mut record: T) -> Result<T, ContainerError> {
        if let Some(row_id) = record.record_id() {
            record.set_flag_modified();
            self.items.insert(row_id, record.clone());
            Ok(record)
        } else {
            let new_row_id = self.get_next_row_id()?;
            // Assign the id using the Record API; map any error into ContainerError.
            record
                .try_set_record_id(new_row_id)
                .map_err(|_| ContainerError::RowAlreadyExists(new_row_id))?;
            record.set_flag_modified();
            let ret = record.clone();
            self.items.insert(new_row_id, record);
            Ok(ret)
        }
    }

    /// Return the number of live stored items in the container.
    pub fn len(&self) -> usize {
        self.items
            .iter()
            .filter(|(_, v)| !v.flags().contains(RecordFlags::Deleted))
            .count()
    }

    /// Return true if the container contains no items.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_next_row_id(&mut self) -> Result<u128, ContainerError> {
        if let Some(&next_id) = self.free_row_id_list.iter().next() {
            self.free_row_id_list.remove(&next_id);
            // Ensure the free list always has the next id available.
            if self.free_row_id_list.is_empty() {
                // Find the current max key in the items map and add next id.
                let next = self
                    .items
                    .keys()
                    .max()
                    .copied()
                    .map_or(0u128, |max_id| max_id + 1);
                if next <= u128::MAX {
                    self.free_row_id_list.insert(next);
                } else {
                    return Err(ContainerError::OutOfRowIds);
                }
            }
            Ok(next_id)
        } else {
            Err(ContainerError::OutOfRowIds)
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

    /// Returned when attempting to upsert a record of the wrong concrete type
    /// into a container (downcast of Box<dyn Any> failed).
    #[error("Type mismatch when inserting/upserting record into container")]
    TypeMismatch,
}
