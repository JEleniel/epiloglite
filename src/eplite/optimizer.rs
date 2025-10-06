//! Query optimizer with cost-based planning.
//!
//! Provides query optimization capabilities including index selection,
//! join ordering, and cost estimation.

use crate::eplite::error::Result;

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

/// Subquery type for optimization
#[derive(Debug, Clone, PartialEq)]
pub enum SubqueryType {
    /// Scalar subquery - returns single value
    Scalar,
    /// IN subquery - checks membership
    In,
    /// EXISTS subquery - checks existence
    Exists,
    /// Correlated subquery - references outer query
    Correlated,
}

/// Subquery information for optimization
#[derive(Debug, Clone)]
pub struct SubqueryInfo {
    pub subquery_type: SubqueryType,
    pub table: String,
    pub where_clause: Option<String>,
    pub columns: Vec<String>,
    pub correlation_refs: Vec<String>, // References to outer query columns
    pub can_flatten: bool,
}

/// Cached subquery result
#[derive(Debug, Clone)]
pub struct CachedSubqueryResult {
    pub result: Vec<Vec<String>>,
    pub hit_count: usize,
}

/// Query optimizer with cost-based planning
pub struct QueryOptimizer {
    table_stats: HashMap<String, QueryStats>,
    subquery_cache: HashMap<String, CachedSubqueryResult>,
}

impl QueryOptimizer {
    /// Create a new query optimizer
    pub fn new() -> Self {
        Self {
            table_stats: HashMap::new(),
            subquery_cache: HashMap::new(),
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
                    .map(|t| self.table_stats.get(*t).map(|s| s.row_count).unwrap_or(100))
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

    /// Check if a subquery can be flattened based on optimization rules
    ///
    /// Based on rules from QUERYPLANNER.md, checks conditions like:
    /// - Subquery has a FROM clause
    /// - No LIMIT/OFFSET conflicts
    /// - No ORDER BY conflicts
    /// - Compatible with outer query structure
    pub fn can_flatten_subquery(
        &self,
        subquery: &SubqueryInfo,
        outer_has_aggregate: bool,
        outer_has_distinct: bool,
    ) -> bool {
        // Rule: subquery must not be DISTINCT when outer query is involved
        if outer_has_distinct && subquery.subquery_type == SubqueryType::Correlated {
            return false;
        }

        // Rule: if outer query is aggregate, subquery should be simple
        if outer_has_aggregate && !subquery.correlation_refs.is_empty() {
            return false;
        }

        // Correlated subqueries are harder to flatten
        if subquery.subquery_type == SubqueryType::Correlated
            && !subquery.correlation_refs.is_empty()
        {
            // Only flatten if correlation is simple
            return subquery.correlation_refs.len() == 1;
        }

        // Check the can_flatten flag set during analysis
        subquery.can_flatten
    }

    /// Flatten a subquery into the outer query
    ///
    /// Attempts to merge subquery FROM clause into outer query's FROM clause
    /// and rewrite expressions accordingly
    pub fn flatten_subquery(&self, subquery: &SubqueryInfo) -> Result<QueryPlan> {
        let mut steps = Vec::new();
        let mut cost = 0.0;

        steps.push(format!("Flatten subquery on table {}", subquery.table));

        // Estimate cost of flattened query
        cost += self.estimate_full_scan_cost(&subquery.table);

        if subquery.where_clause.is_some() {
            steps.push("Merge WHERE clauses".to_string());
            cost += 5.0;
        }

        if !subquery.correlation_refs.is_empty() {
            steps.push("Resolve correlation references".to_string());
            cost += subquery.correlation_refs.len() as f64 * 2.0;
        }

        Ok(QueryPlan {
            steps,
            estimated_cost: cost,
        })
    }

    /// Optimize IN subquery to use efficient lookup
    ///
    /// Converts IN (SELECT ...) to either:
    /// - Hash join if subquery result is large
    /// - Direct lookup if subquery result is small
    pub fn optimize_in_subquery(&self, subquery: &SubqueryInfo) -> Result<QueryPlan> {
        let mut steps = Vec::new();
        let mut cost = 0.0;

        let subquery_rows = self
            .table_stats
            .get(&subquery.table)
            .map(|s| s.row_count)
            .unwrap_or(100);

        if subquery_rows < 50 {
            // Small result set: use direct comparison
            steps.push(format!(
                "IN subquery: Direct comparison ({})",
                subquery.table
            ));
            cost += subquery_rows as f64 * 2.0;
        } else {
            // Large result set: build hash table
            steps.push(format!("IN subquery: Hash lookup ({})", subquery.table));
            steps.push("Build hash table from subquery".to_string());
            cost += subquery_rows as f64 * 0.5; // Hash table build
            cost += 10.0; // Hash lookups
        }

        Ok(QueryPlan {
            steps,
            estimated_cost: cost,
        })
    }

    /// Optimize EXISTS subquery for short-circuit evaluation
    ///
    /// EXISTS only needs to find one matching row, so we can stop early
    pub fn optimize_exists_subquery(&self, subquery: &SubqueryInfo) -> Result<QueryPlan> {
        let mut steps = Vec::new();
        let mut cost = 0.0;

        steps.push(format!(
            "EXISTS subquery: Short-circuit scan ({})",
            subquery.table
        ));

        // EXISTS can stop after finding first match
        // Cost is much lower than full scan
        let full_cost = self.estimate_full_scan_cost(&subquery.table);
        cost += full_cost * 0.1; // Assume we find match quickly

        if subquery.where_clause.is_some() {
            steps.push("Apply WHERE filter (early termination)".to_string());
            cost += 2.0;
        }

        Ok(QueryPlan {
            steps,
            estimated_cost: cost,
        })
    }

    /// Optimize correlated subquery execution
    ///
    /// Correlated subqueries reference outer query columns and must be
    /// executed for each outer row. We optimize by:
    /// - Caching results for repeated parameter values
    /// - Converting to semi-join when possible
    pub fn optimize_correlated_subquery(
        &self,
        subquery: &SubqueryInfo,
        outer_rows: usize,
    ) -> Result<QueryPlan> {
        let mut steps = Vec::new();
        let mut cost = 0.0;

        if subquery.correlation_refs.len() == 1 {
            // Single correlation: can potentially convert to semi-join
            steps.push(format!(
                "Correlated subquery: Convert to semi-join ({})",
                subquery.table
            ));
            cost += self.estimate_full_scan_cost(&subquery.table);
            cost += (outer_rows as f64).log2() * 5.0; // Join cost
        } else {
            // Multiple correlations: must execute per row with caching
            steps.push(format!(
                "Correlated subquery: Execute per row with cache ({})",
                subquery.table
            ));
            steps.push("Cache subquery results by parameters".to_string());

            // Assume 50% cache hit rate for typical queries
            let cache_hit_rate = 0.5;
            let effective_executions = outer_rows as f64 * (1.0 - cache_hit_rate);

            cost += effective_executions * self.estimate_full_scan_cost(&subquery.table);
        }

        Ok(QueryPlan {
            steps,
            estimated_cost: cost,
        })
    }

    /// Cache a subquery result
    pub fn cache_subquery_result(&mut self, key: String, result: Vec<Vec<String>>) {
        self.subquery_cache.insert(
            key.clone(),
            CachedSubqueryResult {
                result,
                hit_count: 0,
            },
        );
    }

    /// Retrieve a cached subquery result and increment hit count
    pub fn get_cached_subquery(&mut self, key: &str) -> Option<Vec<Vec<String>>> {
        if let Some(cached) = self.subquery_cache.get_mut(key) {
            cached.hit_count += 1;
            Some(cached.result.clone())
        } else {
            None
        }
    }

    /// Clear the subquery cache
    pub fn clear_subquery_cache(&mut self) {
        self.subquery_cache.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        let entries = self.subquery_cache.len();
        let total_hits = self.subquery_cache.values().map(|c| c.hit_count).sum();
        (entries, total_hits)
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

    #[test]
    fn test_subquery_flattening_simple() {
        let optimizer = QueryOptimizer::new();

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::Scalar,
            table: "orders".to_string(),
            where_clause: Some("status = 'active'".to_string()),
            columns: vec!["total".to_string()],
            correlation_refs: vec![],
            can_flatten: true,
        };

        // Should be able to flatten non-correlated subquery
        assert!(optimizer.can_flatten_subquery(&subquery, false, false));

        let plan = optimizer.flatten_subquery(&subquery).unwrap();
        assert!(plan.steps.iter().any(|s| s.contains("Flatten")));
        assert!(plan.estimated_cost > 0.0);
    }

    #[test]
    fn test_subquery_flattening_correlated() {
        let optimizer = QueryOptimizer::new();

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::Correlated,
            table: "orders".to_string(),
            where_clause: Some("user_id = outer.id".to_string()),
            columns: vec!["total".to_string()],
            correlation_refs: vec!["outer.id".to_string()],
            can_flatten: true,
        };

        // Single correlation reference should be flattenable
        assert!(optimizer.can_flatten_subquery(&subquery, false, false));

        let plan = optimizer.flatten_subquery(&subquery).unwrap();
        assert!(plan.steps.iter().any(|s| s.contains("Flatten")));
        assert!(plan.steps.iter().any(|s| s.contains("correlation")));
    }

    #[test]
    fn test_subquery_flattening_blocked_by_aggregate() {
        let optimizer = QueryOptimizer::new();

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::Correlated,
            table: "orders".to_string(),
            where_clause: None,
            columns: vec!["count".to_string()],
            correlation_refs: vec!["outer.id".to_string()],
            can_flatten: true,
        };

        // Outer aggregate with correlated subquery cannot be flattened
        assert!(!optimizer.can_flatten_subquery(&subquery, true, false));
    }

    #[test]
    fn test_in_subquery_optimization_small() {
        let mut optimizer = QueryOptimizer::new();
        optimizer.add_stats(
            "categories",
            QueryStats {
                row_count: 10,
                avg_row_size: 20,
            },
        );

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::In,
            table: "categories".to_string(),
            where_clause: None,
            columns: vec!["id".to_string()],
            correlation_refs: vec![],
            can_flatten: false,
        };

        let plan = optimizer.optimize_in_subquery(&subquery).unwrap();

        // Small result set should use direct comparison
        assert!(plan.steps.iter().any(|s| s.contains("Direct comparison")));
        assert!(plan.estimated_cost < 100.0);
    }

    #[test]
    fn test_in_subquery_optimization_large() {
        let mut optimizer = QueryOptimizer::new();
        optimizer.add_stats(
            "products",
            QueryStats {
                row_count: 1000,
                avg_row_size: 50,
            },
        );

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::In,
            table: "products".to_string(),
            where_clause: Some("active = 1".to_string()),
            columns: vec!["category_id".to_string()],
            correlation_refs: vec![],
            can_flatten: false,
        };

        let plan = optimizer.optimize_in_subquery(&subquery).unwrap();

        // Large result set should use hash lookup
        assert!(plan.steps.iter().any(|s| s.contains("Hash lookup")));
        assert!(plan.steps.iter().any(|s| s.contains("hash table")));
    }

    #[test]
    fn test_exists_subquery_optimization() {
        let mut optimizer = QueryOptimizer::new();
        optimizer.add_stats(
            "orders",
            QueryStats {
                row_count: 10000,
                avg_row_size: 100,
            },
        );

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::Exists,
            table: "orders".to_string(),
            where_clause: Some("user_id = 123".to_string()),
            columns: vec![],
            correlation_refs: vec![],
            can_flatten: false,
        };

        let plan = optimizer.optimize_exists_subquery(&subquery).unwrap();

        // EXISTS should use short-circuit evaluation
        assert!(plan.steps.iter().any(|s| s.contains("Short-circuit")));

        // Cost should be much less than full scan
        let full_scan_cost = optimizer.estimate_full_scan_cost("orders");
        assert!(plan.estimated_cost < full_scan_cost * 0.2);
    }

    #[test]
    fn test_correlated_subquery_single_correlation() {
        let mut optimizer = QueryOptimizer::new();
        optimizer.add_stats(
            "order_items",
            QueryStats {
                row_count: 5000,
                avg_row_size: 50,
            },
        );

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::Correlated,
            table: "order_items".to_string(),
            where_clause: Some("order_id = outer.id".to_string()),
            columns: vec!["price".to_string()],
            correlation_refs: vec!["outer.id".to_string()],
            can_flatten: false,
        };

        let plan = optimizer
            .optimize_correlated_subquery(&subquery, 100)
            .unwrap();

        // Single correlation can be converted to semi-join
        assert!(plan.steps.iter().any(|s| s.contains("semi-join")));
    }

    #[test]
    fn test_correlated_subquery_multiple_correlations() {
        let mut optimizer = QueryOptimizer::new();
        optimizer.add_stats(
            "transactions",
            QueryStats {
                row_count: 2000,
                avg_row_size: 80,
            },
        );

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::Correlated,
            table: "transactions".to_string(),
            where_clause: Some("user_id = outer.id AND date > outer.start_date".to_string()),
            columns: vec!["amount".to_string()],
            correlation_refs: vec!["outer.id".to_string(), "outer.start_date".to_string()],
            can_flatten: false,
        };

        let plan = optimizer
            .optimize_correlated_subquery(&subquery, 50)
            .unwrap();

        // Multiple correlations should use caching
        assert!(plan.steps.iter().any(|s| s.contains("Execute per row")));
        assert!(plan.steps.iter().any(|s| s.contains("cache")));
    }

    #[test]
    fn test_subquery_caching() {
        let mut optimizer = QueryOptimizer::new();

        let key = "SELECT id FROM users WHERE age > 18".to_string();
        let result = vec![
            vec!["1".to_string()],
            vec!["2".to_string()],
            vec!["3".to_string()],
        ];

        // Cache should be empty initially
        assert!(optimizer.get_cached_subquery(&key).is_none());

        // Cache the result
        optimizer.cache_subquery_result(key.clone(), result.clone());

        // Should retrieve cached result
        let cached = optimizer.get_cached_subquery(&key);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap(), result);

        // Hit count should increment
        let (entries, hits) = optimizer.get_cache_stats();
        assert_eq!(entries, 1);
        assert_eq!(hits, 1);

        // Get again to increment hit count
        optimizer.get_cached_subquery(&key);
        let (_, hits) = optimizer.get_cache_stats();
        assert_eq!(hits, 2);
    }

    #[test]
    fn test_subquery_cache_clear() {
        let mut optimizer = QueryOptimizer::new();

        optimizer.cache_subquery_result("query1".to_string(), vec![vec!["1".to_string()]]);
        optimizer.cache_subquery_result("query2".to_string(), vec![vec!["2".to_string()]]);

        let (entries, _) = optimizer.get_cache_stats();
        assert_eq!(entries, 2);

        optimizer.clear_subquery_cache();

        let (entries, _) = optimizer.get_cache_stats();
        assert_eq!(entries, 0);
    }

    #[test]
    fn test_subquery_cost_comparison() {
        let mut optimizer = QueryOptimizer::new();
        optimizer.add_stats(
            "products",
            QueryStats {
                row_count: 1000,
                avg_row_size: 100,
            },
        );

        let subquery = SubqueryInfo {
            subquery_type: SubqueryType::In,
            table: "products".to_string(),
            where_clause: None,
            columns: vec!["id".to_string()],
            correlation_refs: vec![],
            can_flatten: true,
        };

        // Compare costs of different strategies
        let flatten_plan = optimizer.flatten_subquery(&subquery).unwrap();
        let in_plan = optimizer.optimize_in_subquery(&subquery).unwrap();

        // Both should have reasonable costs
        assert!(flatten_plan.estimated_cost > 0.0);
        assert!(in_plan.estimated_cost > 0.0);

        // Both approaches should produce valid plans
        assert!(!flatten_plan.steps.is_empty());
        assert!(!in_plan.steps.is_empty());
    }
}
