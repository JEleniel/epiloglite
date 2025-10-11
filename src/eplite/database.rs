//! Database connection and management
use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// A database instance with automatic connection pooling
pub struct Database {
    file_path: PathBuf,
}

impl Database {
    /// Open or create a database
    pub fn open(path: &PathBuf, create: bool) -> Result<Self, DatabaseError> {
        use std::fs;

        if !fs::exists(path.parent().unwrap()).unwrap() {
            return Err(DatabaseError::FolderNotFound(
                path.parent().unwrap().to_str().unwrap().to_string(),
            ));
        }

        if fs::exists(path).unwrap() {
            todo!()
        } else if create {
            Self::create_database(path)
        } else {
            Err(DatabaseError::FileNotFound(
                path.to_str().unwrap().to_string(),
            ))
        }
    }

    fn create_database(path: &PathBuf) -> Result<Self, DatabaseError> {
        todo!()
    }

    /// Get the database file path
    pub fn path(&self) -> PathBuf {
        self.file_path.clone()
    }

    /// Close the database connection
    pub fn close(mut self) -> Result<(), DatabaseError> {
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
    IOError(String),
}
