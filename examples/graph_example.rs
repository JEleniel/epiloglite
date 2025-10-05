/// Example demonstrating graph data structures in EpilogLite
/// 
/// This example shows how to:
/// - Create a graph
/// - Add nodes and edges
/// - Query paths between nodes
/// - Use graph traversal algorithms

use epiloglite::eplite::graph::{Graph, GraphManager};
use epiloglite::eplite::error::Result;

fn main() -> Result<()> {
	println!("=== EpilogLite Graph Data Structures Example ===\n");
	
	// Create a new graph manager
	let mut manager = GraphManager::new();
	
	// Create a social network graph
	manager.create_graph("social".to_string())?;
	println!("✓ Created graph 'social'");
	
	// Get the graph
	let graph = manager.get_graph_mut("social").unwrap();
	
	// Add nodes (people)
	let alice = graph.add_node("Person".to_string());
	let bob = graph.add_node("Person".to_string());
	let charlie = graph.add_node("Person".to_string());
	let diana = graph.add_node("Person".to_string());
	let eve = graph.add_node("Person".to_string());
	
	println!("✓ Added 5 nodes (people)");
	
	// Add properties to nodes
	if let Some(node) = graph.get_node_mut(alice) {
		node.set_property("name".to_string(), "Alice".to_string());
		node.set_property("age".to_string(), "30".to_string());
	}
	
	if let Some(node) = graph.get_node_mut(bob) {
		node.set_property("name".to_string(), "Bob".to_string());
		node.set_property("age".to_string(), "28".to_string());
	}
	
	if let Some(node) = graph.get_node_mut(charlie) {
		node.set_property("name".to_string(), "Charlie".to_string());
		node.set_property("age".to_string(), "35".to_string());
	}
	
	if let Some(node) = graph.get_node_mut(diana) {
		node.set_property("name".to_string(), "Diana".to_string());
		node.set_property("age".to_string(), "32".to_string());
	}
	
	if let Some(node) = graph.get_node_mut(eve) {
		node.set_property("name".to_string(), "Eve".to_string());
		node.set_property("age".to_string(), "29".to_string());
	}
	
	println!("✓ Set properties on all nodes");
	
	// Add edges (relationships)
	graph.add_edge(alice, bob, "KNOWS".to_string())?;
	graph.add_edge(alice, charlie, "KNOWS".to_string())?;
	graph.add_edge(bob, diana, "KNOWS".to_string())?;
	graph.add_edge(charlie, diana, "KNOWS".to_string())?;
	graph.add_edge(diana, eve, "KNOWS".to_string())?;
	
	println!("✓ Added {} edges (relationships)", graph.edge_count());
	
	println!("\n--- Graph Statistics ---");
	println!("Nodes: {}", graph.node_count());
	println!("Edges: {}", graph.edge_count());
	
	// Query neighbors
	println!("\n--- Neighbors of Alice ---");
	let alice_neighbors = graph.get_neighbors(alice);
	for &neighbor_id in &alice_neighbors {
		if let Some(node) = graph.get_node(neighbor_id) {
			if let Some(name) = node.get_property("name") {
				println!("  - {}", name);
			}
		}
	}
	
	// Breadth-First Search from Alice
	println!("\n--- BFS Traversal from Alice ---");
	let bfs_result = graph.bfs(alice)?;
	for node_id in bfs_result {
		if let Some(node) = graph.get_node(node_id) {
			if let Some(name) = node.get_property("name") {
				println!("  {}: {}", node_id, name);
			}
		}
	}
	
	// Depth-First Search from Alice
	println!("\n--- DFS Traversal from Alice ---");
	let dfs_result = graph.dfs(alice)?;
	for node_id in dfs_result {
		if let Some(node) = graph.get_node(node_id) {
			if let Some(name) = node.get_property("name") {
				println!("  {}: {}", node_id, name);
			}
		}
	}
	
	// Find shortest path
	println!("\n--- Shortest Path from Alice to Eve ---");
	let path = graph.shortest_path(alice, eve)?;
	if let Some(path_nodes) = path {
		print!("Path: ");
		for (i, node_id) in path_nodes.iter().enumerate() {
			if let Some(node) = graph.get_node(*node_id) {
				if let Some(name) = node.get_property("name") {
					if i > 0 {
						print!(" -> ");
					}
					print!("{}", name);
				}
			}
		}
		println!("\nPath length: {} hops", path_nodes.len() - 1);
	} else {
		println!("No path found");
	}
	
	// Find all paths
	println!("\n--- All Paths from Alice to Diana (max depth 5) ---");
	let all_paths = graph.find_all_paths(alice, diana, 5)?;
	println!("Found {} path(s):", all_paths.len());
	for (i, path) in all_paths.iter().enumerate() {
		print!("  Path {}: ", i + 1);
		for (j, node_id) in path.iter().enumerate() {
			if let Some(node) = graph.get_node(*node_id) {
				if let Some(name) = node.get_property("name") {
					if j > 0 {
						print!(" -> ");
					}
					print!("{}", name);
				}
			}
		}
		println!();
	}
	
	// Create a weighted graph for distance example
	println!("\n--- Weighted Graph Example ---");
	manager.create_graph("cities".to_string())?;
	let cities = manager.get_graph_mut("cities").unwrap();
	
	let la = cities.add_node("City".to_string());
	let vegas = cities.add_node("City".to_string());
	let phoenix = cities.add_node("City".to_string());
	let dallas = cities.add_node("City".to_string());
	
	if let Some(node) = cities.get_node_mut(la) {
		node.set_property("name".to_string(), "Los Angeles".to_string());
	}
	if let Some(node) = cities.get_node_mut(vegas) {
		node.set_property("name".to_string(), "Las Vegas".to_string());
	}
	if let Some(node) = cities.get_node_mut(phoenix) {
		node.set_property("name".to_string(), "Phoenix".to_string());
	}
	if let Some(node) = cities.get_node_mut(dallas) {
		node.set_property("name".to_string(), "Dallas".to_string());
	}
	
	// Add weighted edges (distances in miles)
	cities.add_weighted_edge(la, vegas, "ROAD".to_string(), 270.0)?;
	cities.add_weighted_edge(la, phoenix, "ROAD".to_string(), 373.0)?;
	cities.add_weighted_edge(vegas, phoenix, "ROAD".to_string(), 297.0)?;
	cities.add_weighted_edge(phoenix, dallas, "ROAD".to_string(), 887.0)?;
	cities.add_weighted_edge(vegas, dallas, "ROAD".to_string(), 1234.0)?;
	
	println!("✓ Created cities graph with weighted edges (distances)");
	
	// Find shortest path by distance
	println!("\n--- Shortest Route from LA to Dallas ---");
	let route = cities.shortest_path(la, dallas)?;
	if let Some(route_nodes) = route {
		print!("Route: ");
		for (i, node_id) in route_nodes.iter().enumerate() {
			if let Some(node) = cities.get_node(*node_id) {
				if let Some(name) = node.get_property("name") {
					if i > 0 {
						print!(" -> ");
					}
					print!("{}", name);
				}
			}
		}
		println!();
	}
	
	println!("\n✓ Graph example completed successfully!");
	
	Ok(())
}
