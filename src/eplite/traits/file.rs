use flagset::{flags, FlagSet};
use std::io;

pub trait File: std::fmt::Debug {
    fn close(self: &mut Self) -> Result<(), io::Error>;
    fn read(self: &mut Self, offset: u64) -> Result<Vec<u8>, io::Error>;
    fn write(self: &mut Self, data: &Vec<u8>, offset: u64) -> Result<(), io::Error>;
    fn truncate(self: &mut Self, offset: u64) -> Result<(), io::Error>;
    fn sync(
        self: &mut Self,
        flags: FlagSet<SynchronizationType>,
    ) -> Result<(), io::Error>;
    fn file_size(self: &mut Self) -> Result<u64, io::Error>;
    fn lock(self: &mut Self, lock_type: LockType) -> Result<(), io::Error>;
    fn unlock(self: &mut Self, unlock_type: UnlockType) -> Result<(), io::Error>;
    fn check_reserved_lock(self: &mut Self) -> Result<u64, io::Error>;
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
