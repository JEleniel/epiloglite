//! Database connection and management
use std::io;
use thiserror::Error;

#[cfg(feature = "std")]
use std::path::PathBuf;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

/// A database instance with automatic connection pooling
pub struct Database {
    file_path: PathBuf,
}

impl Database {
    /// Open or create a database
    #[cfg(feature = "std")]
    pub fn open(path: PathBuf, create: bool) -> Result<Self, DatabaseError> {
        use std::fs;

        if !fs::exists(path.parent().unwrap()) {
            return Err(DatabaseError::FolderNotFound(
                path.parent().unwrap().to_str().unwrap().to_string(),
            ));
        }

        if fs::exists(path) {
            todo!();
        } else if create {
            create(path)
        } else {
            Err(DatabaseError::FileNotFound(path.to_str().to_string()));
        };
    }

    fn create(path: PathBuf) -> Self {}

    /// Get the database file path
    pub fn path(&self) -> PathBuf {
        self.file_path.clone()
    }

    /// Close the database connection
    pub fn close(mut self) -> Result<()> {
        todo!();
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DatabaseError {
    #[error("Folder not found {0}")]
    FolderNotFound(String),
    #[error("File not found {0}")]
    FileNotFound(String),
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    #[error("IO error {0:?}")]
    IOError(io::Error),
}
