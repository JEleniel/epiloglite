use flagset::{FlagSet, flags};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

/// Trait describing a persistable record stored in a `Container`.
///
/// Implementations must provide an optional  128-bit `record_id` which uniquely
/// identifies the record within a single `Container`. The `record_id`
/// value of `None` is reserved to mean "unassigned" and callers (for
/// example `Container::create`) expect newly-created records to start
/// with an id of `None` and receive Some(id) when stored.
///
/// Records also expose a mutable set of `RecordFlags` which indicate
/// transient state such as `New`, `Modified`, or `Deleted`. These flags
/// are used by higher-level persistence/commit logic and may be modified
/// by containers or storage layers.
pub trait Record: Clone + std::fmt::Debug + serde::Serialize + DeserializeOwned {
    /// Return the numeric identifier for this record. New records must return `None`. The record
    /// id is assigned when the record is inserted into a container.
    fn record_id(&self) -> Option<u128>;

    /// Set the numeric identifier for this record. This becomes the record's permanent id.
    /// Will fail if the record already has an assigned id.
    fn try_set_record_id(&mut self, id: u128) -> Result<(), RecordError>;

    /// Return an immutable reference to the record's flags.
    fn flags(&self) -> &FlagSet<RecordFlags>;

    /// Reset all flags to `RecordFlags::None`
    fn clear_flags(&mut self);

    /// Sets the record's Modified flag
    fn set_flag_modified(&mut self);

    /// Set the record's Deleted flag
    fn set_flag_deleted(&mut self);
}

flags! {
    /// Bitflags describing the transient state of a `Record` instance.
    ///
    /// - `New`: record was created in-memory and not yet persisted.
    /// - `Modified`: record has been changed since it was loaded or
    ///   last persisted.
    /// - `Deleted`: record has been marked for deletion.
    #[derive(Default, Serialize, Deserialize)]
    pub enum RecordFlags: u8 {
        /// No flags set.
        None = 0,
        /// Record was created in memory and has not been persisted yet.
        #[default]
        New = 1 << 0,
        /// Record has been modified since it was loaded or persisted.
        Modified = 1 << 1,
        /// Record has been marked for deletion.
        Deleted = 1 << 2,
    }
}

/// Errors that can occur when manipulating `Record` instances.
#[derive(Debug, Clone, Error)]
pub enum RecordError {
    /// Attempted to set a record ID on a record that already has an assigned ID.
    #[error("Record ID is already assigned")]
    RecordIdAlreadyAssigned,
}
