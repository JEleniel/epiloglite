use epiloglite_core::CInt;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection<T> {
    pub collection_id: CInt,
    pub name: String,
    pub records: Vec<T>,
}

impl<T> Collection<T>
where
    T: Clone + std::fmt::Debug + Serialize + DeserializeOwned,
{
    pub fn new(collection_id: CInt, name: &str) -> Self {
        Self {
            collection_id,
            name: name.to_string(),
            records: Vec::new(),
        }
    }
}
