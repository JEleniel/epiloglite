//! Role-based permissions system for table-level access control.
//!
//! Provides RBAC (Role-Based Access Control) for database tables with
//! support for multiple roles and granular permissions.

use crate::eplite::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::fs;
#[cfg(feature = "std")]
use std::path::Path;

#[cfg(not(feature = "std"))]
use alloc::{
	collections::BTreeMap as HashMap,
	string::{String, ToString},
	vec,
	vec::Vec,
};

/// Permission types for database operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
	Select,
	Insert,
	Update,
	Delete,
	Create,
	Drop,
	Alter,
}

/// User roles with predefined permission sets
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
	/// Administrator with all permissions
	Admin,
	/// Can read and write but not create/drop tables
	ReadWrite,
	/// Can only read data
	ReadOnly,
	/// Custom role with specific permissions
	Custom(Vec<Permission>),
}

impl Role {
	/// Get permissions for this role
	pub fn permissions(&self) -> Vec<Permission> {
		match self {
			Role::Admin => vec![
				Permission::Select,
				Permission::Insert,
				Permission::Update,
				Permission::Delete,
				Permission::Create,
				Permission::Drop,
				Permission::Alter,
			],
			Role::ReadWrite => vec![
				Permission::Select,
				Permission::Insert,
				Permission::Update,
				Permission::Delete,
			],
			Role::ReadOnly => vec![Permission::Select],
			Role::Custom(perms) => perms.clone(),
		}
	}
}

/// Grant entry for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GrantEntry {
	user: String,
	table: String,
	role: Role,
}

/// Permissions manager for table-level access control
#[derive(Debug)]
pub struct PermissionManager {
	/// Map of (user, table) -> Role
	grants: HashMap<(String, String), Role>,
}

impl PermissionManager {
	/// Create a new permissions manager
	pub fn new() -> Self {
		Self {
			grants: HashMap::new(),
		}
	}

	/// Grant a role to a user for a table
	pub fn grant(&mut self, user: &str, table: &str, role: Role) -> Result<()> {
		self.grants
			.insert((user.to_string(), table.to_string()), role);
		Ok(())
	}

	/// Revoke all permissions for a user on a table
	pub fn revoke(&mut self, user: &str, table: &str) -> Result<()> {
		self.grants.remove(&(user.to_string(), table.to_string()));
		Ok(())
	}

	/// Check if a user has a specific permission on a table
	pub fn check_permission(&self, user: &str, table: &str, permission: Permission) -> bool {
		if let Some(role) = self.grants.get(&(user.to_string(), table.to_string())) {
			role.permissions().contains(&permission)
		} else {
			false
		}
	}

	/// Check if user has admin role on a table
	pub fn is_admin(&self, user: &str, table: &str) -> bool {
		if let Some(role) = self.grants.get(&(user.to_string(), table.to_string())) {
			matches!(role, Role::Admin)
		} else {
			false
		}
	}

	/// Save permissions to a file (for SQLite compatibility)
	#[cfg(feature = "std")]
	pub fn save_to_file(&self, path: &Path) -> Result<()> {
		// Convert to serializable format
		let entries: Vec<GrantEntry> = self
			.grants
			.iter()
			.map(|((user, table), role)| GrantEntry {
				user: user.clone(),
				table: table.clone(),
				role: role.clone(),
			})
			.collect();

		let json = serde_json::to_string_pretty(&entries)
			.map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
		fs::write(path, json)?;
		Ok(())
	}

	/// Load permissions from a file
	#[cfg(feature = "std")]
	pub fn load_from_file(path: &Path) -> Result<Self> {
		let json = fs::read_to_string(path)?;
		let entries: Vec<GrantEntry> = serde_json::from_str(&json)
			.map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

		let mut manager = Self::new();
		for entry in entries {
			manager.grants.insert((entry.user, entry.table), entry.role);
		}
		Ok(manager)
	}
}

impl Default for PermissionManager {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_role_creation() {
		let admin = Role::Admin;
		let readonly = Role::ReadOnly;
		let custom = Role::Custom(vec![Permission::Select, Permission::Insert]);

		assert_eq!(admin.permissions().len(), 7);
		assert_eq!(readonly.permissions().len(), 1);
		assert_eq!(custom.permissions().len(), 2);
	}

	#[test]
	fn test_grant_permission() {
		let mut mgr = PermissionManager::new();
		mgr.grant("alice", "users", Role::Admin).unwrap();

		assert!(mgr.check_permission("alice", "users", Permission::Select));
		assert!(mgr.check_permission("alice", "users", Permission::Delete));
	}

	#[test]
	fn test_check_permission() {
		let mut mgr = PermissionManager::new();
		mgr.grant("bob", "products", Role::ReadOnly).unwrap();

		assert!(mgr.check_permission("bob", "products", Permission::Select));
		assert!(!mgr.check_permission("bob", "products", Permission::Insert));
	}

	#[test]
	fn test_revoke_permission() {
		let mut mgr = PermissionManager::new();
		mgr.grant("charlie", "orders", Role::ReadWrite).unwrap();
		mgr.revoke("charlie", "orders").unwrap();

		assert!(!mgr.check_permission("charlie", "orders", Permission::Select));
	}

	#[test]
	fn test_admin_bypass() {
		let mut mgr = PermissionManager::new();
		mgr.grant("admin_user", "sensitive_data", Role::Admin)
			.unwrap();

		assert!(mgr.is_admin("admin_user", "sensitive_data"));
		assert!(mgr.check_permission("admin_user", "sensitive_data", Permission::Drop));
	}

	#[test]
	fn test_permission_file() {
		use std::path::PathBuf;
		let mut mgr = PermissionManager::new();
		mgr.grant("user1", "table1", Role::ReadWrite).unwrap();

		let path = PathBuf::from("/tmp/test_perms.json");
		mgr.save_to_file(&path).unwrap();

		let loaded = PermissionManager::load_from_file(&path).unwrap();
		assert!(loaded.check_permission("user1", "table1", Permission::Select));

		let _ = fs::remove_file(path);
	}
}
