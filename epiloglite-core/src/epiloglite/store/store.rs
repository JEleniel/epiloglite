//! The developer facing surface of an EpilogLite store.
use crate::ContainerAny;
use crate::{Container, Cu128, Record};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::fmt::Debug;

/// In-memory representation of the entire store.
///
/// `Store` holds heterogeneous `Container` instances keyed by an id.
pub struct Store {
    /// Application ID, for use by applications
    application_id: Cu128,
    /// Migration version, for use by applications
    migration_version: Cu128,

    /// Mapping of container id -> boxed, heterogeneous container.
    ///
    /// We store containers as trait objects implementing `ContainerAny` so the
    /// map can hold `Container<T>` for many different `T` types. Callers can
    /// downcast back to the concrete `Container<T>` when they need access via
    /// the `as_any` / `as_any_mut` helpers on `ContainerAny`.
    pub containers: HashMap<u128, Box<dyn ContainerAny>>,

    /// Mapping of container_id to the metadata for that container.-
    pub metadata_index: HashMap<u128, MetadataEntry>,
}

impl Store {
    /// Insert a typed container into the store under `id`.
    pub fn insert_container<T>(&mut self, id: u128, container: Container<T>)
    where
        T: Record + Serialize + DeserializeOwned + Clone + Debug + 'static,
    {
        self.containers.insert(id, Box::new(container));
    }

    /// Get a shared reference to a typed container by id, if it exists and
    /// the type matches.
    pub fn get_container<T>(&self, id: u128) -> Option<&Container<T>>
    where
        T: Record + Serialize + DeserializeOwned + Clone + Debug + 'static,
    {
        self.containers
            .get(&id)
            .and_then(|b| b.as_any().downcast_ref::<Container<T>>())
    }

    /// Get an exclusive reference to a typed container by id, if it exists
    /// and the type matches.
    pub fn get_container_mut<T>(&mut self, id: u128) -> Option<&mut Container<T>>
    where
        T: Record + Serialize + DeserializeOwned + Clone + Debug + 'static,
    {
        self.containers
            .get_mut(&id)
            .and_then(|b| b.as_any_mut().downcast_mut::<Container<T>>())
    }
}
