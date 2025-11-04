use flagset::{FlagSet, flags};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait Record: Clone + std::fmt::Debug + serde::Serialize + DeserializeOwned {
    fn record_id(&self) -> u128;
    fn set_record_id(&mut self, id: u128);
    fn flags(&self) -> &FlagSet<RecordFlags>;
    fn flags_mut(&mut self) -> &mut FlagSet<RecordFlags>;
}

flags! {
    #[derive(Serialize, Deserialize)]
    pub enum RecordFlags: u8 {
        None = 0,
        New = 1 << 0,
        Modified = 1 << 1,
        Deleted = 1 << 2,
    }
}
