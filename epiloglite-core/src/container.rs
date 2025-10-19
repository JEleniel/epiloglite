// use std::{collections::HashMap, sync::{Arc, Mutex}};

// use serde::{Deserialize, Serialize, de::DeserializeOwned};
// use thiserror::Error;

// use crate::{CInt, FieldMetadata, SerializeError};

// /// A container for storing records of type T. Acts similar to tables in a database.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Container<T> {
//     container_id: CInt,
//     name: String,
//     metadata: Vec<FieldMetadata>,
//     #[serde(skip)]
//     data: HashMap<CInt, T>,
// }

// impl<T> Container<T>
// where
//     T: std::fmt::Debug + Clone + Serialize + DeserializeOwned,
// {
//     /// Create a new container instance
//     fn new(
//         container_id: CInt,
//         name: &str,
//         metadata: Vec<FieldMetadata>,
//     ) -> Self {
//         Self {
//             container_id,
//             name: name.to_string(),
//             metadata,
//             data: HashMap::new(),
//         }
//     }

//     /// Get metadata about the contained struct's fields
//     fn metadata() -> &'static [FieldMetadata] {
//         T::field_metadata()
//     }

//     /// Get the last row ID
//     fn last_row_id(&self) -> CInt {
//         self.last_row_id
//     }

//     /// Get metadata about the contained struct's fields
//     fn metadata() -> &'static [FieldMetadata] {
//         T::field_metadata()
//     }

//     /// Get the container ID
//     fn container_id(&self) -> CInt {
//         self.container_id
//     }

//     /// Get the container name
//     fn name(&self) -> &str {
//         &self.name
//     }

//     /// Get all records in the container
//     fn records(&self) -> &Vec<T> {
//         &self.data
//     }

//     fn upsert_record(&mut self, record: T) -> Result<CInt, ContainerError> {
//         if record.id==0 {
//             let new_id = ;
//             record.set_id(new_id);
//             self.data.insert(new_id, record);
//             Ok(new_id)
//         } else {
//             let record_id = record.id();
//             if self.data.contains_key(&record_id) {
//                 self.data.insert(record_id, record);
//                 Ok(record_id)
//             } else {
//                 Err(ContainerError::RecordNotFound(record_id as usize))
//             }
//         }
//     }
//     fn delete_record(&mut self, record_id: usize) -> Result<(), ContainerError>;
// }

// #[derive(Debug, Error)]
// pub enum ContainerError {
//     /// Container is full.
//     #[error("Container is full")]
//     ContainerFull,
//     /// Serialization or deserialization error.
//     #[error("Serialization/Deserialization error: {0}")]
//     SerializationError(#[from] SerializeError),
//     /// Record not found.
//     #[error("Record not found: {0}")]
//     RecordNotFound(usize),
// }
