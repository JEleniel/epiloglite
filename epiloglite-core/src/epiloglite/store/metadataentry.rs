use serde::{Deserialize, Serialize};

use crate::DataType;

/// Represents a metadata entry in the database, a.k.a. a *_def
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MetadataEntry {
    pub name: String,
    pub data_type: DataType,
}
