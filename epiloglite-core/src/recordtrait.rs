use flagset::{FlagSet, flags};

use crate::CInt;

pub trait RecordTrait {
    fn record_id(&self) -> CInt;
    fn flags(&self) -> FlagSet<RecordFlags>;
}

flags! {
    pub enum RecordFlags: u8 {
        NONE = 0,
        DELETED = 1 << 0,
        DIRTY = 1 << 1,
        NEW = 1 << 2,
        SYNCED = 1 << 3,
    }
}
