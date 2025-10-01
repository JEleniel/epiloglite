use flagset::{flags, FlagSet};
use std::io;

pub trait File: std::fmt::Debug {
    fn close(&mut self) -> Result<(), io::Error>;
    fn read(&mut self, offset: u64) -> Result<Vec<u8>, io::Error>;
    fn write(&mut self, data: &Vec<u8>, offset: u64) -> Result<(), io::Error>;
    fn truncate(&mut self, offset: u64) -> Result<(), io::Error>;
    fn sync(
        &mut self,
        flags: FlagSet<SynchronizationType>,
    ) -> Result<(), io::Error>;
    fn file_size(&mut self) -> Result<u64, io::Error>;
    fn lock(&mut self, lock_type: LockType) -> Result<(), io::Error>;
    fn unlock(&mut self, unlock_type: UnlockType) -> Result<(), io::Error>;
    fn check_reserved_lock(&mut self) -> Result<u64, io::Error>;
}

flags! {
    pub enum SynchronizationType: u64 {
         SqliteSyncNormal  =      0x00002,
         SqliteSyncFull       =   0x00003,
         SqliteSyncDataonly   =   0x00010,
    }
}

pub enum LockType {
    Shared,
    Reserved,
    Pending,
    Exclusive,
}

pub enum UnlockType {
    None,
    Shared,
}
