use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{CInt, persistence::OffsetPointer};

/// An index mapping rows in a table to their location in the data file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RowIdIndex {
    entries: HashMap<CInt, OffsetPointer>,
}

impl RowIdIndex {
    /// Create a new, empty index.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Insert a new entry into the index.
    pub fn insert(&mut self, row_id: CInt, pointer: OffsetPointer) {
        self.entries.insert(row_id, pointer);
    }

    /// Get the pointer for a given row ID, if it exists.
    pub fn get(&self, row_id: &CInt) -> Option<&OffsetPointer> {
        self.entries.get(row_id)
    }

    /// Remove an entry from the index (change the pointer to null)
    pub fn remove(&mut self, row_id: &CInt) {
        self.insert(row_id.clone(), OffsetPointer::null());
    }

    /// Get all entries in the index as a sorted vector of (row_id, pointer) tuples
    /// sorted by row_id. Primarily intended for persistence.
    pub fn get_entries(&self) -> Vec<(CInt, OffsetPointer)> {
        let mut res = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<(CInt, OffsetPointer)>>();
        res.sort();
        res
    }

    /// Get the number of entries in the index.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
