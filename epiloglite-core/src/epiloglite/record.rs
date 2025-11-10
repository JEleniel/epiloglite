use flagset::{FlagSet, flags};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Trait describing a persistable record stored in a `Container`.
///
/// Implementations must provide a 128-bit `record_id` which uniquely
/// identifies the record within a single `Container`. The `record_id`
/// value of `0` is reserved to mean "unassigned" and callers (for
/// example `Container::create`) expect newly-created records to start
/// with an id of `0` and receive a non-zero id when stored.
///
/// Records also expose a mutable set of `RecordFlags` which indicate
/// transient state such as `New`, `Modified`, or `Deleted`. These flags
/// are used by higher-level persistence/commit logic and may be modified
/// by containers or storage layers.
pub trait Record: Clone + std::fmt::Debug + serde::Serialize + DeserializeOwned {
    /// Return the numeric identifier for this record. `0` means unassigned.
    /// Note: under the external API we expose `u128` primitives; `0` is used
    /// to indicate an unassigned record.
    fn record_id(&self) -> Option<u128>;
    /// Set the numeric identifier for this record. Pass `0` to mark the
    /// record as unassigned (available for allocation).
    fn set_record_id(&mut self, id: Option<u128>);
    /// Return an immutable reference to the record's flags.
    fn flags(&self) -> &FlagSet<RecordFlags>;
    /// Return a mutable reference to the record's flags so callers can
    /// mark the record as modified/new/deleted.
    fn flags_mut(&mut self) -> &mut FlagSet<RecordFlags>;
}

flags! {
    /// Bitflags describing the transient state of a `Record` instance.
    ///
    /// - `New`: record was created in-memory and not yet persisted.
    /// - `Modified`: record has been changed since it was loaded or
    ///   last persisted.
    /// - `Deleted`: record has been marked for deletion.
    #[derive(Serialize, Deserialize)]
    pub enum RecordFlags: u8 {
        /// No flags set.
        None = 0,
        /// Record was created in memory and has not been persisted yet.
        New = 1 << 0,
        /// Record has been modified since it was loaded or persisted.
        Modified = 1 << 1,
        /// Record has been marked for deletion.
        Deleted = 1 << 2,
    }
}
