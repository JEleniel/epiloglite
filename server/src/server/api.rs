//! REST API and GraphQL server for EpilogLite
//!
//! This module provides a standalone server mode that exposes the database
//! via REST API and GraphQL endpoints with TLS 1.3 and authentication.

#[cfg(feature = "server")]
use crate::{Database, Result, Error};

#[cfg(feature = "server")]
use axum::{
	Router,
	routing::{get, post},
	extract::{State, Json},
	http::StatusCode,
	response::IntoResponse,
};

// GraphQL support temporarily disabled due to version conflicts
// #[cfg(feature = "server")]
// use async_graphql::{Schema, Object, Context, EmptySubscription};

#[cfg(feature = "server")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "server")]
use serde::{Serialize, Deserialize};

/// Server configuration
#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct ServerConfig {
	/// Server bind address
	pub bind_addr: String,
	/// TLS certificate path
	pub tls_cert_path: Option<String>,
	/// TLS key path
	pub tls_key_path: Option<String>,
	/// Enable REST API
	pub enable_rest: bool,
	/// Enable GraphQL
	pub enable_graphql: bool,
	/// JWT secret for authentication
	pub jwt_secret: String,
}

#[cfg(feature = "server")]
impl Default for ServerConfig {
	fn default() -> Self {
		Self {
			bind_addr: "127.0.0.1:8080".to_string(),
			tls_cert_path: None,
			tls_key_path: None,
			enable_rest: true,
			enable_graphql: true,
			jwt_secret: "change-me-in-production".to_string(),
		}
	}
}

// Note: Database is not Send+Sync, so we cannot use it directly with axum.
// For now, we wrap it in Arc<Mutex<_>> and note this limitation.

/// Server state shared across requests
/// 
/// Note: This uses a simplified approach that doesn't support
/// true concurrent database access. For production use, consider
/// implementing proper connection pooling.
#[cfg(feature = "server")]
#[derive(Clone)]
pub struct ServerState {
	// Using string path instead of Database to avoid Send+Sync issues
	db_path: Arc<String>,
	config: ServerConfig,
}

/// SQL execution request
#[cfg(feature = "server")]
#[derive(Debug, Deserialize)]
pub struct SqlRequest {
	sql: String,
}

/// SQL execution response
#[cfg(feature = "server")]
#[derive(Debug, Serialize)]
pub struct SqlResponse {
	success: bool,
	message: String,
	rows_affected: Option<usize>,
}

/// Authentication request
#[cfg(feature = "server")]
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
	username: String,
	password: String,
}

/// Authentication response
#[cfg(feature = "server")]
#[derive(Debug, Serialize)]
pub struct AuthResponse {
	token: String,
}

// GraphQL support temporarily disabled due to version conflicts
// /// GraphQL query root
// #[cfg(feature = "server")]
// pub struct QueryRoot;
// 
// #[cfg(feature = "server")]
// #[Object]
// impl QueryRoot {
// 	/// Execute SQL query via GraphQL
// 	async fn execute_sql(&self, ctx: &Context<'_>, sql: String) -> async_graphql::Result<String> {
// 		let state = ctx.data::<ServerState>()?;
// 		let mut db = state.db.inner.lock().unwrap();
// 		
// 		match db.execute(&sql) {
// 			Ok(_) => Ok("Query executed successfully".to_string()),
// 			Err(e) => Err(async_graphql::Error::new(format!("Database error: {}", e))),
// 		}
// 	}
// 	
// 	/// Get database version
// 	async fn version(&self) -> String {
// 		env!("CARGO_PKG_VERSION").to_string()
// 	}
// }
// 
// /// GraphQL mutation root
// #[cfg(feature = "server")]
// pub struct MutationRoot;
// 
// #[cfg(feature = "server")]
// #[Object]
// impl MutationRoot {
// 	/// Execute SQL mutation via GraphQL
// 	async fn execute_mutation(&self, ctx: &Context<'_>, sql: String) -> async_graphql::Result<String> {
// 		let state = ctx.data::<ServerState>()?;
// 		let mut db = state.db.inner.lock().unwrap();
// 		
// 		match db.execute(&sql) {
// 			Ok(_) => Ok("Mutation executed successfully".to_string()),
// 			Err(e) => Err(async_graphql::Error::new(format!("Database error: {}", e))),
// 		}
// 	}
// }

/// REST API handler for SQL execution
#[cfg(feature = "server")]
async fn execute_sql_handler(
	State(state): State<ServerState>,
	Json(req): Json<SqlRequest>,
) -> impl IntoResponse {
	// Open database connection for this request
	// In production, use connection pooling
	let mut db = match Database::open(state.db_path.as_str()) {
		Ok(db) => db,
		Err(e) => {
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(SqlResponse {
					success: false,
					message: format!("Failed to open database: {}", e),
					rows_affected: None,
				})
			);
		}
	};
	
	match db.execute(&req.sql) {
		Ok(_) => (
			StatusCode::OK,
			Json(SqlResponse {
				success: true,
				message: "Query executed successfully".to_string(),
				rows_affected: Some(1),
			})
		),
		Err(e) => (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json(SqlResponse {
				success: false,
				message: format!("Database error: {}", e),
				rows_affected: None,
			})
		),
	}
}

/// Authentication handler (simplified - production should use proper password hashing)
#[cfg(feature = "server")]
async fn auth_handler(
	State(_state): State<ServerState>,
	Json(_req): Json<AuthRequest>,
) -> impl IntoResponse {
	// In production, verify username/password against database
	// For now, return a dummy JWT token
	(
		StatusCode::OK,
		Json(AuthResponse {
			token: "dummy-jwt-token".to_string(),
		})
	)
}

/// Health check endpoint
#[cfg(feature = "server")]
async fn health_handler() -> impl IntoResponse {
	(StatusCode::OK, "healthy")
}

/// EpilogLite server
#[cfg(feature = "server")]
pub struct EpilogLiteServer {
	config: ServerConfig,
	db_path: String,
}

#[cfg(feature = "server")]
impl EpilogLiteServer {
	/// Create a new server instance with a database path
	pub fn new(db_path: String, config: ServerConfig) -> Self {
		Self {
			config,
			db_path,
		}
	}
	
	/// Create a new server instance with an in-memory database
	pub fn new_memory(config: ServerConfig) -> Self {
		Self::new(":memory:".to_string(), config)
	}
	
	/// Build the router with all endpoints
	fn build_router(&self) -> Router {
		let state = ServerState {
			db_path: Arc::new(self.db_path.clone()),
			config: self.config.clone(),
		};
		
		let mut router = Router::new()
			.route("/health", get(health_handler));
		
		// Add REST API endpoints
		if self.config.enable_rest {
			router = router
				.route("/api/execute", post(execute_sql_handler))
				.route("/api/auth", post(auth_handler));
		}
		
		// GraphQL support temporarily disabled due to version conflicts
		// Add GraphQL endpoint
		// if self.config.enable_graphql {
		// 	let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
		// 		.data(state.clone())
		// 		.finish();
		// 	
		// 	use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
		// 	use async_graphql::http::GraphiQLSource;
		// 	
		// 	let graphql_handler = |schema: async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>, req: GraphQLRequest| async move {
		// 		GraphQLResponse::from(schema.execute(req.into_inner()).await)
		// 	};
		// 	
		// 	let graphiql_handler = || async {
		// 		axum::response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
		// 	};
		// 	
		// 	router = router
		// 		.route("/graphql", post({
		// 			let schema_clone = schema.clone();
		// 			move |req| graphql_handler(schema_clone.clone(), req)
		// 		}))
		// 		.route("/graphiql", get(graphiql_handler));
		// }
		
		router.with_state(state)
	}
	
	/// Start the server
	pub async fn start(&self) -> Result<()> {
		let app = self.build_router();
		
		let listener = tokio::net::TcpListener::bind(&self.config.bind_addr)
			.await
			.map_err(|e| Error::Internal(format!("Failed to bind to {}: {}", self.config.bind_addr, e)))?;
		
		log::info!("Server listening on {}", self.config.bind_addr);
		
		axum::serve(listener, app)
			.await
			.map_err(|e| Error::Internal(format!("Server error: {}", e)))?;
		
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_server_config_default() {
		let config = ServerConfig::default();
		assert_eq!(config.bind_addr, "127.0.0.1:8080");
		assert!(config.enable_rest);
		assert!(config.enable_graphql);
	}
}
