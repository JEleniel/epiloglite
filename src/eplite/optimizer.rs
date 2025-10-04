//! Query optimizer with cost-based planning.
//!
//! Provides query optimization capabilities including index selection,
//! join ordering, and cost estimation.

use crate::eplite::error::{Error, Result};

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{
	collections::BTreeMap as HashMap,
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};

/// Statistics for a table
#[derive(Debug, Clone)]
pub struct QueryStats {
	pub row_count: usize,
	pub avg_row_size: usize,
}

/// Query plan with estimated cost
#[derive(Debug, Clone)]
pub struct QueryPlan {
	pub steps: Vec<String>,
	pub estimated_cost: f64,
}

/// Query optimizer with cost-based planning
pub struct QueryOptimizer {
	table_stats: HashMap<String, QueryStats>,
}

impl QueryOptimizer {
	/// Create a new query optimizer
	pub fn new() -> Self {
		Self {
			table_stats: HashMap::new(),
		}
	}

	/// Add statistics for a table
	pub fn add_stats(&mut self, table: &str, stats: QueryStats) {
		self.table_stats.insert(table.to_string(), stats);
	}

	/// Estimate cost of a full table scan
	fn estimate_full_scan_cost(&self, table: &str) -> f64 {
		if let Some(stats) = self.table_stats.get(table) {
			(stats.row_count * stats.avg_row_size) as f64
		} else {
			1000.0 // Default cost estimate
		}
	}

	/// Estimate cost of an index scan
	fn estimate_index_scan_cost(&self, table: &str, _index: &str) -> f64 {
		if let Some(stats) = self.table_stats.get(table) {
			// Index scan: log(n) lookups
			(stats.row_count as f64).log2() * 10.0
		} else {
			100.0
		}
	}

	/// Optimize a SELECT query
	pub fn optimize_select(
		&self,
		tables: &[&str],
		where_clause: Option<&str>,
		available_indexes: &[String],
	) -> Result<QueryPlan> {
		let mut steps = Vec::new();
		let mut cost = 0.0;

		for table in tables {
			// Check if we can use an index
			let can_use_index = where_clause.is_some()
				&& available_indexes
					.iter()
					.any(|idx| idx.contains(table) && where_clause.unwrap().contains("="));

			if can_use_index {
				steps.push(format!("Index scan on {}", table));
				cost += self.estimate_index_scan_cost(table, "idx");
			} else {
				steps.push(format!("Full scan on {}", table));
				cost += self.estimate_full_scan_cost(table);
			}
		}

		if where_clause.is_some() {
			steps.push("Apply WHERE filter".to_string());
			cost += 10.0;
		}

		Ok(QueryPlan {
			steps,
			estimated_cost: cost,
		})
	}

	/// Optimize JOIN operations
	pub fn optimize_join(
		&self,
		tables: Vec<&str>,
		_join_conditions: Vec<(&str, &str, &str, &str)>,
	) -> Result<QueryPlan> {
		let mut steps = Vec::new();
		let mut cost = 0.0;

		// Simple join ordering: smaller table first
		let mut sorted_tables = tables.clone();
		sorted_tables.sort_by_key(|t| {
			self.table_stats
				.get(*t)
				.map(|s| s.row_count)
				.unwrap_or(1000)
		});

		for (i, table) in sorted_tables.iter().enumerate() {
			if i == 0 {
				steps.push(format!("Scan {}", table));
				cost += self.estimate_full_scan_cost(table);
			} else {
				steps.push(format!("Join with {}", table));
				// Nested loop join cost
				let prev_rows = sorted_tables[..i]
					.iter()
					.map(|t| {
						self.table_stats
							.get(*t)
							.map(|s| s.row_count)
							.unwrap_or(100)
					})
					.product::<usize>();
				let curr_rows = self
					.table_stats
					.get(*table)
					.map(|s| s.row_count)
					.unwrap_or(100);
				cost += (prev_rows * curr_rows) as f64 * 0.1;
			}
		}

		Ok(QueryPlan {
			steps,
			estimated_cost: cost,
		})
	}
}

impl Default for QueryOptimizer {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cost_estimation() {
		let optimizer = QueryOptimizer::new();
		let cost = optimizer.estimate_full_scan_cost("users");
		assert!(cost > 0.0);
	}

	#[test]
	fn test_index_selection() {
		let mut optimizer = QueryOptimizer::new();
		optimizer.add_stats(
			"users",
			QueryStats {
				row_count: 1000,
				avg_row_size: 100,
			},
		);

		let plan = optimizer
			.optimize_select(&["users"], Some("id = 1"), &["idx_users_id".to_string()])
			.unwrap();

		assert!(plan.steps.contains(&"Index scan on users".to_string()));
		assert!(plan.estimated_cost < 1000.0);
	}

	#[test]
	fn test_join_order() {
		let mut optimizer = QueryOptimizer::new();
		optimizer.add_stats(
			"users",
			QueryStats {
				row_count: 100,
				avg_row_size: 50,
			},
		);
		optimizer.add_stats(
			"orders",
			QueryStats {
				row_count: 1000,
				avg_row_size: 30,
			},
		);

		let plan = optimizer
			.optimize_join(
				vec!["users", "orders"],
				vec![("users", "id", "orders", "user_id")],
			)
			.unwrap();

		// Smaller table (users) should be scanned first
		assert!(plan.steps[0].contains("users"));
	}

	#[test]
	fn test_plan_generation() {
		let optimizer = QueryOptimizer::new();
		let plan = optimizer
			.optimize_select(&["products"], Some("price > 50"), &[])
			.unwrap();

		assert!(!plan.steps.is_empty());
		assert!(plan.estimated_cost > 0.0);
	}
}
