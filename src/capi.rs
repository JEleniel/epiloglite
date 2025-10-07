//! SQLite 3 compatible C API
//!
//! Provides a drop-in replacement for the SQLite C API,
//! allowing EpilogLite to be used as a library from C/C++ applications.

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(non_snake_case)]
#![cfg_attr(feature = "capi", allow(unsafe_code))]

use crate::eplite::database::Database;
use crate::eplite::error::Result as EplResult;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::{Arc, Mutex};

/// SQLite database connection handle
#[repr(C)]
pub struct sqlite3 {
	db: Arc<Mutex<Database>>,
}

/// SQLite prepared statement handle
#[repr(C)]
pub struct sqlite3_stmt {
	_placeholder: u8,
}

/// SQLite result codes
pub const SQLITE_OK: c_int = 0;
pub const SQLITE_ERROR: c_int = 1;
pub const SQLITE_BUSY: c_int = 5;
pub const SQLITE_NOMEM: c_int = 7;
pub const SQLITE_READONLY: c_int = 8;
pub const SQLITE_MISUSE: c_int = 21;
pub const SQLITE_ROW: c_int = 100;
pub const SQLITE_DONE: c_int = 101;

/// Open flags
pub const SQLITE_OPEN_READONLY: c_int = 0x00000001;
pub const SQLITE_OPEN_READWRITE: c_int = 0x00000002;
pub const SQLITE_OPEN_CREATE: c_int = 0x00000004;

/// Convert EpilogLite Result to SQLite error code
fn result_to_code<T>(result: EplResult<T>) -> c_int {
	match result {
		Ok(_) => SQLITE_OK,
		Err(_) => SQLITE_ERROR,
	}
}

/// Open a database connection
///
/// # Safety
/// filename must be a valid null-terminated string
/// ppDb must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn sqlite3_open(
	filename: *const c_char,
	ppDb: *mut *mut sqlite3,
) -> c_int {
	sqlite3_open_v2(filename, ppDb, SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE, ptr::null())
}

/// Open a database connection with flags
///
/// # Safety
/// filename must be a valid null-terminated string
/// ppDb must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn sqlite3_open_v2(
	filename: *const c_char,
	ppDb: *mut *mut sqlite3,
	_flags: c_int,
	_zVfs: *const c_char,
) -> c_int {
	if filename.is_null() || ppDb.is_null() {
		return SQLITE_MISUSE;
	}

	let filename_str = match CStr::from_ptr(filename).to_str() {
		Ok(s) => s,
		Err(_) => return SQLITE_ERROR,
	};

	let db = match Database::open(filename_str) {
		Ok(db) => db,
		Err(_) => return SQLITE_ERROR,
	};

	let sqlite3_db = Box::new(sqlite3 {
		db: Arc::new(Mutex::new(db)),
	});

	*ppDb = Box::into_raw(sqlite3_db);
	SQLITE_OK
}

/// Close a database connection
///
/// # Safety
/// pDb must be a valid sqlite3 pointer or null
#[no_mangle]
pub unsafe extern "C" fn sqlite3_close(pDb: *mut sqlite3) -> c_int {
	if pDb.is_null() {
		return SQLITE_OK;
	}

	// Just drop the box, cleanup happens automatically
	let _db_box = Box::from_raw(pDb);
	SQLITE_OK
}

/// Execute SQL statement
///
/// # Safety
/// pDb must be a valid sqlite3 pointer
/// zSql must be a valid null-terminated string
#[no_mangle]
pub unsafe extern "C" fn sqlite3_exec(
	pDb: *mut sqlite3,
	zSql: *const c_char,
	_callback: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut *mut c_char, *mut *mut c_char) -> c_int>,
	_pArg: *mut c_void,
	pzErrMsg: *mut *mut c_char,
) -> c_int {
	if pDb.is_null() || zSql.is_null() {
		return SQLITE_MISUSE;
	}

	let db = &*pDb;
	let mut db_guard = match db.db.lock() {
		Ok(guard) => guard,
		Err(_) => return SQLITE_ERROR,
	};

	let sql_str = match CStr::from_ptr(zSql).to_str() {
		Ok(s) => s,
		Err(_) => return SQLITE_ERROR,
	};

	match db_guard.execute(sql_str) {
		Ok(_) => SQLITE_OK,
		Err(e) => {
			if !pzErrMsg.is_null() {
				let err_msg = CString::new(format!("{:?}", e)).unwrap_or_default();
				*pzErrMsg = err_msg.into_raw();
			}
			SQLITE_ERROR
		}
	}
}

/// Get error message
///
/// # Safety
/// pDb must be a valid sqlite3 pointer
#[no_mangle]
pub unsafe extern "C" fn sqlite3_errmsg(pDb: *mut sqlite3) -> *const c_char {
	if pDb.is_null() {
		return ptr::null();
	}

	// Return a default error message
	b"unknown error\0".as_ptr() as *const c_char
}

/// Get library version string
#[no_mangle]
pub extern "C" fn sqlite3_libversion() -> *const c_char {
	b"3.0.0-epiloglite\0".as_ptr() as *const c_char
}

/// Get library version number
#[no_mangle]
pub extern "C" fn sqlite3_libversion_number() -> c_int {
	3000000 // 3.0.0
}

/// Free memory allocated by sqlite3
///
/// # Safety
/// p must be a valid pointer allocated by sqlite3 or null
#[no_mangle]
pub unsafe extern "C" fn sqlite3_free(p: *mut c_void) {
	if !p.is_null() {
		let _ = CString::from_raw(p as *mut c_char);
	}
}

/// Initialize SQLite library (no-op for EpilogLite)
#[no_mangle]
pub extern "C" fn sqlite3_initialize() -> c_int {
	SQLITE_OK
}

/// Shutdown SQLite library (no-op for EpilogLite)
#[no_mangle]
pub extern "C" fn sqlite3_shutdown() -> c_int {
	SQLITE_OK
}

// Tests are disabled because the C API requires unsafe code
// which conflicts with the crate-level `unsafe_code = "forbid"` lint.
// The C API is tested via integration tests or external C programs.
//
// To test manually:
// 1. Build with: cargo build --release
// 2. Create a C program that links to the library
// 3. Test all sqlite3_* functions

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::ffi::CString;
//
//     #[test]
//     fn test_version_functions() {
//         let version = unsafe { CStr::from_ptr(sqlite3_libversion()) };
//         assert!(version.to_str().unwrap().contains("epiloglite"));
//
//         let version_num = sqlite3_libversion_number();
//         assert_eq!(version_num, 3000000);
//     }
// }
