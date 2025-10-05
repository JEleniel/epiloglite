use flagset::{flags, FlagSet};

#[cfg(feature = "std")]
use std::io;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::fmt;

/// Type alias for I/O errors
#[cfg(feature = "std")]
pub type IoError = io::Error;

#[cfg(not(feature = "std"))]
pub type IoError = i32;

#[cfg(feature = "std")]
pub trait File: fmt::Debug {
    fn close(&mut self) -> Result<(), IoError>;
    fn read(&mut self, offset: u64) -> Result<Vec<u8>, IoError>;
    fn write(&mut self, data: &Vec<u8>, offset: u64) -> Result<(), IoError>;
    fn truncate(&mut self, offset: u64) -> Result<(), IoError>;
    fn sync(
        &mut self,
        flags: FlagSet<SynchronizationType>,
    ) -> Result<(), IoError>;
    fn file_size(&mut self) -> Result<u64, IoError>;
    fn lock(&mut self, lock_type: LockType) -> Result<(), IoError>;
    fn unlock(&mut self, unlock_type: UnlockType) -> Result<(), IoError>;
    fn check_reserved_lock(&mut self) -> Result<u64, IoError>;
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
