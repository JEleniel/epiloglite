//! Types used for journaling operations.
//!
//! The journal records a sequence of operations (transaction boundaries,
//! schema changes, and CRUD operations) which can be replayed for recovery
//! or inspection.

use serde::{Deserialize, Serialize};

use crate::Cu128;
use crate::Metadata;

/// The type of a journal entry.
///
/// Each variant represents a discrete operation recorded in the write-ahead
/// journal. Variants carry the data necessary to replay or undo the
/// operation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum JournalEntryType {
    /// Marks the start of a transaction.
    ///
    /// Contains the transaction identifier used to correlate subsequent
    /// commit/rollback entries and per-transaction operations.
    BeginTransaction {
        /// Transaction identifier.
        transaction_id: Cu128,
    },

    /// Marks a successful end of a transaction.
    ///
    /// Includes the transaction identifier so consumers can match commits to
    /// their begin record.
    CommitTransaction {
        /// Transaction identifier.
        transaction_id: Cu128,
    },

    /// Marks a rolled-back transaction.
    ///
    /// Includes the transaction identifier so consumers can match rollbacks
    /// to their begin record.
    RollbackTransaction {
        /// Transaction identifier.
        transaction_id: Cu128,
    },

    /// Add a new collection (schema-level change).
    ///
    /// Carries the collection id and its metadata necessary to recreate the
    /// collection during replay.
    AddCollection {
        /// Unique collection identifier.
        collection_id: Cu128,
        /// Collection metadata (schema, options, etc.).
        metadata: Metadata,
    },

    /// Remove an existing collection.
    ///
    /// Includes the id and metadata so the removal can be validated or
    /// reversed if required by a recovery tool.
    RemoveCollection {
        /// Unique collection identifier.
        collection_id: Cu128,
        /// Collection metadata at the time of removal.
        metadata: Metadata,
    },

    /// Add an index to a collection.
    ///
    /// Contains the collection id, index id, and the list of fields that make
    /// up the index.
    AddIndex {
        /// Collection identifier the index belongs to.
        collection_id: Cu128,
        /// Index identifier.
        index_id: Cu128,
        /// Ordered list of field names that the index covers.
        fields: Vec<String>,
    },

    /// Remove an index from a collection.
    ///
    /// Contains the collection id, index id and fields for clarity during
    /// replay and auditing.
    RemoveIndex {
        /// Collection identifier the index belongs to.
        collection_id: Cu128,
        /// Index identifier.
        index_id: Cu128,
        /// Ordered list of field names that the index covered.
        fields: Vec<String>,
    },

    /// Create (insert) a new record within a collection.
    ///
    /// `data` contains the serialized record payload.
    Create {
        /// Collection where the record will be stored.
        collection_id: Cu128,
        /// Record identifier.
        record_id: Cu128,
        /// Serialized record bytes.
        data: Vec<u8>,
    },

    /// Update an existing record in a collection.
    ///
    /// `data` contains the new serialized payload for the record.
    Update {
        /// Collection where the record exists.
        collection_id: Cu128,
        /// Record identifier.
        record_id: Cu128,
        /// Serialized record bytes (new value).
        data: Vec<u8>,
    },

    /// Delete a record from a collection.
    ///
    /// `data` may contain the previous value or additional metadata needed
    /// for undo/compaction. Consumers should treat it as an opaque byte blob.
    Delete {
        /// Collection where the record exists.
        collection_id: Cu128,
        /// Record identifier.
        record_id: Cu128,
        /// Serialized bytes associated with the deletion (opaque payload).
        data: Vec<u8>,
    },
}

/// An entry in the journal, pairing a timestamp with an operation.
///
/// The journal records when an operation was observed and the operation
/// itself. Note that the journal is not strictly append-only: the system's
/// maintenance process may remove stale entries (for example, entries which
/// have been verified/applied and are no longer needed for recovery or
/// auditing).
///
/// The `timestamp` uses `chrono::DateTime<chrono::Utc>` to carry a timezone-
/// aware instant. Consumers should interpret the timestamp in UTC.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct JournalEntry {
    /// Creation timestamp for the entry.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// The operation recorded by this journal entry.
    pub entry_type: JournalEntryType,
}
