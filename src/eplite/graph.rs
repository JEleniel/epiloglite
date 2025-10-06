/// Graph data structures for EpilogLite
///
/// Provides native graph table type, node/edge representation,
/// traversal queries, and path-finding algorithms

use crate::eplite::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[cfg(not(feature = "std"))]
use alloc::{
	collections::{BTreeMap, BTreeSet, VecDeque},
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};

/// Node ID type
pub type NodeId = u64;

/// Edge ID type
pub type EdgeId = u64;

/// Node in a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
	/// Unique node identifier
	pub id: NodeId,
	
	/// Node label/type
	pub label: String,
	
	/// Node properties (key-value pairs)
	pub properties: BTreeMap<String, String>,
}

impl Node {
	/// Create a new node
	pub fn new(id: NodeId, label: String) -> Self {
		Node {
			id,
			label,
			properties: BTreeMap::new(),
		}
	}
	
	/// Set a property on the node
	pub fn set_property(&mut self, key: String, value: String) {
		self.properties.insert(key, value);
	}
	
	/// Get a property from the node
	pub fn get_property(&self, key: &str) -> Option<&String> {
		self.properties.get(key)
	}
}

/// Edge in a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
	/// Unique edge identifier
	pub id: EdgeId,
	
	/// Source node ID
	pub from_node: NodeId,
	
	/// Target node ID
	pub to_node: NodeId,
	
	/// Edge label/type
	pub label: String,
	
	/// Edge weight (for weighted graphs)
	pub weight: f64,
	
	/// Edge properties (key-value pairs)
	pub properties: BTreeMap<String, String>,
}

impl Edge {
	/// Create a new edge
	pub fn new(id: EdgeId, from_node: NodeId, to_node: NodeId, label: String) -> Self {
		Edge {
			id,
			from_node,
			to_node,
			label,
			weight: 1.0,
			properties: BTreeMap::new(),
		}
	}
	
	/// Create a new weighted edge
	pub fn new_weighted(
		id: EdgeId,
		from_node: NodeId,
		to_node: NodeId,
		label: String,
		weight: f64,
	) -> Self {
		Edge {
			id,
			from_node,
			to_node,
			label,
			weight,
			properties: BTreeMap::new(),
		}
	}
	
	/// Set a property on the edge
	pub fn set_property(&mut self, key: String, value: String) {
		self.properties.insert(key, value);
	}
	
	/// Get a property from the edge
	pub fn get_property(&self, key: &str) -> Option<&String> {
		self.properties.get(key)
	}
}

/// Graph table structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
	/// Graph name
	pub name: String,
	
	/// Nodes in the graph
	nodes: BTreeMap<NodeId, Node>,
	
	/// Edges in the graph
	edges: BTreeMap<EdgeId, Edge>,
	
	/// Adjacency list for fast traversal (node_id -> list of outgoing edges)
	adjacency_list: BTreeMap<NodeId, Vec<EdgeId>>,
	
	/// Reverse adjacency list for incoming edges (node_id -> list of incoming edges)
	reverse_adjacency_list: BTreeMap<NodeId, Vec<EdgeId>>,
	
	/// Next available node ID
	next_node_id: NodeId,
	
	/// Next available edge ID
	next_edge_id: EdgeId,
}

impl Graph {
	/// Create a new graph
	pub fn new(name: String) -> Self {
		Graph {
			name,
			nodes: BTreeMap::new(),
			edges: BTreeMap::new(),
			adjacency_list: BTreeMap::new(),
			reverse_adjacency_list: BTreeMap::new(),
			next_node_id: 1,
			next_edge_id: 1,
		}
	}
	
	/// Add a node to the graph
	pub fn add_node(&mut self, label: String) -> NodeId {
		let node_id = self.next_node_id;
		self.next_node_id += 1;
		
		let node = Node::new(node_id, label);
		self.nodes.insert(node_id, node);
		self.adjacency_list.insert(node_id, Vec::new());
		self.reverse_adjacency_list.insert(node_id, Vec::new());
		
		node_id
	}
	
	/// Add a node with a specific ID
	pub fn add_node_with_id(&mut self, node_id: NodeId, label: String) -> Result<()> {
		if self.nodes.contains_key(&node_id) {
			return Err(Error::Constraint(format!(
				"Node with ID {} already exists in graph '{}'",
				node_id, self.name
			)));
		}
		
		let node = Node::new(node_id, label);
		self.nodes.insert(node_id, node);
		self.adjacency_list.insert(node_id, Vec::new());
		self.reverse_adjacency_list.insert(node_id, Vec::new());
		
		if node_id >= self.next_node_id {
			self.next_node_id = node_id + 1;
		}
		
		Ok(())
	}
	
	/// Get a node by ID
	pub fn get_node(&self, node_id: NodeId) -> Option<&Node> {
		self.nodes.get(&node_id)
	}
	
	/// Get a mutable node by ID
	pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
		self.nodes.get_mut(&node_id)
	}
	
	/// Remove a node from the graph
	pub fn remove_node(&mut self, node_id: NodeId) -> Result<()> {
		if !self.nodes.contains_key(&node_id) {
			return Err(Error::NotFound(format!(
				"Node with ID {} not found in graph '{}'",
				node_id, self.name
			)));
		}
		
		// Remove all edges connected to this node
		let outgoing_edges = self.adjacency_list.get(&node_id).cloned().unwrap_or_default();
		let incoming_edges = self
			.reverse_adjacency_list
			.get(&node_id)
			.cloned()
			.unwrap_or_default();
		
		for edge_id in outgoing_edges.iter().chain(incoming_edges.iter()) {
			self.edges.remove(edge_id);
		}
		
		// Clean up adjacency lists
		self.adjacency_list.remove(&node_id);
		self.reverse_adjacency_list.remove(&node_id);
		
		// Remove node
		self.nodes.remove(&node_id);
		
		Ok(())
	}
	
	/// Add an edge to the graph
	pub fn add_edge(&mut self, from_node: NodeId, to_node: NodeId, label: String) -> Result<EdgeId> {
		// Verify both nodes exist
		if !self.nodes.contains_key(&from_node) {
			return Err(Error::NotFound(format!(
				"Source node {} not found in graph '{}'",
				from_node, self.name
			)));
		}
		if !self.nodes.contains_key(&to_node) {
			return Err(Error::NotFound(format!(
				"Target node {} not found in graph '{}'",
				to_node, self.name
			)));
		}
		
		let edge_id = self.next_edge_id;
		self.next_edge_id += 1;
		
		let edge = Edge::new(edge_id, from_node, to_node, label);
		self.edges.insert(edge_id, edge);
		
		// Update adjacency lists
		self.adjacency_list
			.get_mut(&from_node)
			.unwrap()
			.push(edge_id);
		self.reverse_adjacency_list
			.get_mut(&to_node)
			.unwrap()
			.push(edge_id);
		
		Ok(edge_id)
	}
	
	/// Add a weighted edge to the graph
	pub fn add_weighted_edge(
		&mut self,
		from_node: NodeId,
		to_node: NodeId,
		label: String,
		weight: f64,
	) -> Result<EdgeId> {
		// Verify both nodes exist
		if !self.nodes.contains_key(&from_node) {
			return Err(Error::NotFound(format!(
				"Source node {} not found in graph '{}'",
				from_node, self.name
			)));
		}
		if !self.nodes.contains_key(&to_node) {
			return Err(Error::NotFound(format!(
				"Target node {} not found in graph '{}'",
				to_node, self.name
			)));
		}
		
		let edge_id = self.next_edge_id;
		self.next_edge_id += 1;
		
		let edge = Edge::new_weighted(edge_id, from_node, to_node, label, weight);
		self.edges.insert(edge_id, edge);
		
		// Update adjacency lists
		self.adjacency_list
			.get_mut(&from_node)
			.unwrap()
			.push(edge_id);
		self.reverse_adjacency_list
			.get_mut(&to_node)
			.unwrap()
			.push(edge_id);
		
		Ok(edge_id)
	}
	
	/// Get an edge by ID
	pub fn get_edge(&self, edge_id: EdgeId) -> Option<&Edge> {
		self.edges.get(&edge_id)
	}
	
	/// Get a mutable edge by ID
	pub fn get_edge_mut(&mut self, edge_id: EdgeId) -> Option<&mut Edge> {
		self.edges.get_mut(&edge_id)
	}
	
	/// Remove an edge from the graph
	pub fn remove_edge(&mut self, edge_id: EdgeId) -> Result<()> {
		let edge = self.edges.get(&edge_id).ok_or_else(|| {
			Error::NotFound(format!(
				"Edge with ID {} not found in graph '{}'",
				edge_id, self.name
			))
		})?;
		
		let from_node = edge.from_node;
		let to_node = edge.to_node;
		
		// Remove from adjacency lists
		if let Some(edges) = self.adjacency_list.get_mut(&from_node) {
			edges.retain(|&id| id != edge_id);
		}
		if let Some(edges) = self.reverse_adjacency_list.get_mut(&to_node) {
			edges.retain(|&id| id != edge_id);
		}
		
		// Remove edge
		self.edges.remove(&edge_id);
		
		Ok(())
	}
	
	/// Get all nodes in the graph
	pub fn get_all_nodes(&self) -> Vec<&Node> {
		self.nodes.values().collect()
	}
	
	/// Get all edges in the graph
	pub fn get_all_edges(&self) -> Vec<&Edge> {
		self.edges.values().collect()
	}
	
	/// Get outgoing edges from a node
	pub fn get_outgoing_edges(&self, node_id: NodeId) -> Vec<&Edge> {
		self.adjacency_list
			.get(&node_id)
			.map(|edge_ids| edge_ids.iter().filter_map(|id| self.edges.get(id)).collect())
			.unwrap_or_default()
	}
	
	/// Get incoming edges to a node
	pub fn get_incoming_edges(&self, node_id: NodeId) -> Vec<&Edge> {
		self.reverse_adjacency_list
			.get(&node_id)
			.map(|edge_ids| edge_ids.iter().filter_map(|id| self.edges.get(id)).collect())
			.unwrap_or_default()
	}
	
	/// Get neighbors of a node (nodes connected by outgoing edges)
	pub fn get_neighbors(&self, node_id: NodeId) -> Vec<NodeId> {
		self.adjacency_list
			.get(&node_id)
			.map(|edge_ids| {
				edge_ids
					.iter()
					.filter_map(|id| self.edges.get(id).map(|e| e.to_node))
					.collect()
			})
			.unwrap_or_default()
	}
	
	/// Count nodes in the graph
	pub fn node_count(&self) -> usize {
		self.nodes.len()
	}
	
	/// Count edges in the graph
	pub fn edge_count(&self) -> usize {
		self.edges.len()
	}
	
	/// Breadth-First Search traversal from a starting node
	pub fn bfs(&self, start_node: NodeId) -> Result<Vec<NodeId>> {
		if !self.nodes.contains_key(&start_node) {
			return Err(Error::NotFound(format!(
				"Start node {} not found in graph '{}'",
				start_node, self.name
			)));
		}
		
		let mut visited = BTreeSet::new();
		let mut queue = VecDeque::new();
		let mut result = Vec::new();
		
		queue.push_back(start_node);
		visited.insert(start_node);
		
		while let Some(node_id) = queue.pop_front() {
			result.push(node_id);
			
			for neighbor in self.get_neighbors(node_id) {
				if !visited.contains(&neighbor) {
					visited.insert(neighbor);
					queue.push_back(neighbor);
				}
			}
		}
		
		Ok(result)
	}
	
	/// Depth-First Search traversal from a starting node
	pub fn dfs(&self, start_node: NodeId) -> Result<Vec<NodeId>> {
		if !self.nodes.contains_key(&start_node) {
			return Err(Error::NotFound(format!(
				"Start node {} not found in graph '{}'",
				start_node, self.name
			)));
		}
		
		let mut visited = BTreeSet::new();
		let mut result = Vec::new();
		
		self.dfs_recursive(start_node, &mut visited, &mut result);
		
		Ok(result)
	}
	
	/// Helper function for DFS
	fn dfs_recursive(&self, node_id: NodeId, visited: &mut BTreeSet<NodeId>, result: &mut Vec<NodeId>) {
		visited.insert(node_id);
		result.push(node_id);
		
		for neighbor in self.get_neighbors(node_id) {
			if !visited.contains(&neighbor) {
				self.dfs_recursive(neighbor, visited, result);
			}
		}
	}
	
	/// Find shortest path between two nodes using Dijkstra's algorithm
	pub fn shortest_path(&self, start_node: NodeId, end_node: NodeId) -> Result<Option<Vec<NodeId>>> {
		if !self.nodes.contains_key(&start_node) {
			return Err(Error::NotFound(format!(
				"Start node {} not found in graph '{}'",
				start_node, self.name
			)));
		}
		if !self.nodes.contains_key(&end_node) {
			return Err(Error::NotFound(format!(
				"End node {} not found in graph '{}'",
				end_node, self.name
			)));
		}
		
		// Initialize distances and predecessors
		let mut distances: BTreeMap<NodeId, f64> = BTreeMap::new();
		let mut predecessors: BTreeMap<NodeId, NodeId> = BTreeMap::new();
		let mut unvisited: BTreeSet<NodeId> = self.nodes.keys().copied().collect();
		
		// Initialize all distances to infinity except start node
		for &node_id in self.nodes.keys() {
			distances.insert(node_id, f64::INFINITY);
		}
		distances.insert(start_node, 0.0);
		
		while !unvisited.is_empty() {
			// Find unvisited node with minimum distance
			let current = match unvisited
				.iter()
				.min_by(|a, b| {
					distances
						.get(a)
						.unwrap_or(&f64::INFINITY)
						.partial_cmp(distances.get(b).unwrap_or(&f64::INFINITY))
						.unwrap()
				})
				.copied()
			{
				Some(node) => node,
				None => break,
			};
			
			// If we reached the end node, we can stop
			if current == end_node {
				break;
			}
			
			// If the current node is unreachable, we can stop
			if distances.get(&current).copied().unwrap_or(f64::INFINITY) == f64::INFINITY {
				break;
			}
			
			unvisited.remove(&current);
			
			// Update distances to neighbors
			let outgoing_edges = self.get_outgoing_edges(current);
			for edge in outgoing_edges {
				let neighbor = edge.to_node;
				if !unvisited.contains(&neighbor) {
					continue;
				}
				
				let current_distance = distances.get(&current).copied().unwrap_or(f64::INFINITY);
				let new_distance = current_distance + edge.weight;
				let neighbor_distance = distances.get(&neighbor).copied().unwrap_or(f64::INFINITY);
				
				if new_distance < neighbor_distance {
					distances.insert(neighbor, new_distance);
					predecessors.insert(neighbor, current);
				}
			}
		}
		
		// Reconstruct path
		if !predecessors.contains_key(&end_node) && start_node != end_node {
			// No path found
			return Ok(None);
		}
		
		let mut path = Vec::new();
		let mut current = end_node;
		path.push(current);
		
		while current != start_node {
			match predecessors.get(&current) {
				Some(&pred) => {
					path.push(pred);
					current = pred;
				}
				None => break,
			}
		}
		
		path.reverse();
		
		if path[0] == start_node {
			Ok(Some(path))
		} else {
			Ok(None)
		}
	}
	
	/// Find all paths between two nodes (limited depth to avoid infinite loops)
	pub fn find_all_paths(
		&self,
		start_node: NodeId,
		end_node: NodeId,
		max_depth: usize,
	) -> Result<Vec<Vec<NodeId>>> {
		if !self.nodes.contains_key(&start_node) {
			return Err(Error::NotFound(format!(
				"Start node {} not found in graph '{}'",
				start_node, self.name
			)));
		}
		if !self.nodes.contains_key(&end_node) {
			return Err(Error::NotFound(format!(
				"End node {} not found in graph '{}'",
				end_node, self.name
			)));
		}
		
		let mut paths = Vec::new();
		let mut current_path = Vec::new();
		let mut visited = BTreeSet::new();
		
		self.find_paths_recursive(
			start_node,
			end_node,
			&mut current_path,
			&mut visited,
			&mut paths,
			max_depth,
		);
		
		Ok(paths)
	}
	
	/// Helper function for finding all paths
	fn find_paths_recursive(
		&self,
		current: NodeId,
		end: NodeId,
		current_path: &mut Vec<NodeId>,
		visited: &mut BTreeSet<NodeId>,
		all_paths: &mut Vec<Vec<NodeId>>,
		max_depth: usize,
	) {
		if current_path.len() >= max_depth {
			return;
		}
		
		current_path.push(current);
		visited.insert(current);
		
		if current == end {
			all_paths.push(current_path.clone());
		} else {
			for neighbor in self.get_neighbors(current) {
				if !visited.contains(&neighbor) {
					self.find_paths_recursive(
						neighbor,
						end,
						current_path,
						visited,
						all_paths,
						max_depth,
					);
				}
			}
		}
		
		current_path.pop();
		visited.remove(&current);
	}
}

/// Graph manager for a database
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphManager {
	graphs: BTreeMap<String, Graph>,
}

impl GraphManager {
	/// Create a new graph manager
	pub fn new() -> Self {
		GraphManager {
			graphs: BTreeMap::new(),
		}
	}
	
	/// Create a new graph
	pub fn create_graph(&mut self, name: String) -> Result<()> {
		if self.graphs.contains_key(&name) {
			return Err(Error::Constraint(format!("Graph '{}' already exists", name)));
		}
		
		let graph = Graph::new(name.clone());
		self.graphs.insert(name, graph);
		
		Ok(())
	}
	
	/// Get a graph by name
	pub fn get_graph(&self, name: &str) -> Option<&Graph> {
		self.graphs.get(name)
	}
	
	/// Get a mutable graph by name
	pub fn get_graph_mut(&mut self, name: &str) -> Option<&mut Graph> {
		self.graphs.get_mut(name)
	}
	
	/// Drop a graph
	pub fn drop_graph(&mut self, name: &str) -> Result<()> {
		if self.graphs.remove(name).is_none() {
			return Err(Error::NotFound(format!("Graph '{}' not found", name)));
		}
		Ok(())
	}
	
	/// List all graphs
	pub fn list_graphs(&self) -> Vec<String> {
		self.graphs.keys().cloned().collect()
	}
}

impl Default for GraphManager {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_create_node() {
		let node = Node::new(1, "Person".to_string());
		assert_eq!(node.id, 1);
		assert_eq!(node.label, "Person");
		assert_eq!(node.properties.len(), 0);
	}
	
	#[test]
	fn test_node_properties() {
		let mut node = Node::new(1, "Person".to_string());
		node.set_property("name".to_string(), "Alice".to_string());
		node.set_property("age".to_string(), "30".to_string());
		
		assert_eq!(node.get_property("name"), Some(&"Alice".to_string()));
		assert_eq!(node.get_property("age"), Some(&"30".to_string()));
		assert_eq!(node.get_property("email"), None);
	}
	
	#[test]
	fn test_create_edge() {
		let edge = Edge::new(1, 1, 2, "KNOWS".to_string());
		assert_eq!(edge.id, 1);
		assert_eq!(edge.from_node, 1);
		assert_eq!(edge.to_node, 2);
		assert_eq!(edge.label, "KNOWS");
		assert_eq!(edge.weight, 1.0);
	}
	
	#[test]
	fn test_create_weighted_edge() {
		let edge = Edge::new_weighted(1, 1, 2, "DISTANCE".to_string(), 5.5);
		assert_eq!(edge.id, 1);
		assert_eq!(edge.weight, 5.5);
	}
	
	#[test]
	fn test_create_graph() {
		let graph = Graph::new("social".to_string());
		assert_eq!(graph.name, "social");
		assert_eq!(graph.node_count(), 0);
		assert_eq!(graph.edge_count(), 0);
	}
	
	#[test]
	fn test_add_nodes() {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("Person".to_string());
		let node2 = graph.add_node("Person".to_string());
		
		assert_eq!(node1, 1);
		assert_eq!(node2, 2);
		assert_eq!(graph.node_count(), 2);
	}
	
	#[test]
	fn test_add_edge() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("Person".to_string());
		let node2 = graph.add_node("Person".to_string());
		
		let edge_id = graph.add_edge(node1, node2, "KNOWS".to_string())?;
		
		assert_eq!(edge_id, 1);
		assert_eq!(graph.edge_count(), 1);
		
		Ok(())
	}
	
	#[test]
	fn test_get_neighbors() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("Person".to_string());
		let node2 = graph.add_node("Person".to_string());
		let node3 = graph.add_node("Person".to_string());
		
		graph.add_edge(node1, node2, "KNOWS".to_string())?;
		graph.add_edge(node1, node3, "KNOWS".to_string())?;
		
		let neighbors = graph.get_neighbors(node1);
		assert_eq!(neighbors.len(), 2);
		assert!(neighbors.contains(&node2));
		assert!(neighbors.contains(&node3));
		
		Ok(())
	}
	
	#[test]
	fn test_bfs() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("A".to_string());
		let node2 = graph.add_node("B".to_string());
		let node3 = graph.add_node("C".to_string());
		let node4 = graph.add_node("D".to_string());
		
		graph.add_edge(node1, node2, "EDGE".to_string())?;
		graph.add_edge(node1, node3, "EDGE".to_string())?;
		graph.add_edge(node2, node4, "EDGE".to_string())?;
		
		let bfs_result = graph.bfs(node1)?;
		assert_eq!(bfs_result.len(), 4);
		assert_eq!(bfs_result[0], node1);
		
		Ok(())
	}
	
	#[test]
	fn test_dfs() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("A".to_string());
		let node2 = graph.add_node("B".to_string());
		let node3 = graph.add_node("C".to_string());
		let node4 = graph.add_node("D".to_string());
		
		graph.add_edge(node1, node2, "EDGE".to_string())?;
		graph.add_edge(node1, node3, "EDGE".to_string())?;
		graph.add_edge(node2, node4, "EDGE".to_string())?;
		
		let dfs_result = graph.dfs(node1)?;
		assert_eq!(dfs_result.len(), 4);
		assert_eq!(dfs_result[0], node1);
		
		Ok(())
	}
	
	#[test]
	fn test_shortest_path() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("A".to_string());
		let node2 = graph.add_node("B".to_string());
		let node3 = graph.add_node("C".to_string());
		let node4 = graph.add_node("D".to_string());
		
		graph.add_weighted_edge(node1, node2, "EDGE".to_string(), 1.0)?;
		graph.add_weighted_edge(node1, node3, "EDGE".to_string(), 4.0)?;
		graph.add_weighted_edge(node2, node3, "EDGE".to_string(), 2.0)?;
		graph.add_weighted_edge(node3, node4, "EDGE".to_string(), 1.0)?;
		
		let path = graph.shortest_path(node1, node4)?;
		assert!(path.is_some());
		let path = path.unwrap();
		assert_eq!(path[0], node1);
		assert_eq!(path[path.len() - 1], node4);
		
		Ok(())
	}
	
	#[test]
	fn test_shortest_path_no_path() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("A".to_string());
		let node2 = graph.add_node("B".to_string());
		
		let path = graph.shortest_path(node1, node2)?;
		assert!(path.is_none());
		
		Ok(())
	}
	
	#[test]
	fn test_remove_node() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("A".to_string());
		let node2 = graph.add_node("B".to_string());
		
		graph.add_edge(node1, node2, "EDGE".to_string())?;
		
		assert_eq!(graph.node_count(), 2);
		assert_eq!(graph.edge_count(), 1);
		
		graph.remove_node(node1)?;
		
		assert_eq!(graph.node_count(), 1);
		assert_eq!(graph.edge_count(), 0);
		
		Ok(())
	}
	
	#[test]
	fn test_graph_manager() -> Result<()> {
		let mut mgr = GraphManager::new();
		
		mgr.create_graph("social".to_string())?;
		mgr.create_graph("knowledge".to_string())?;
		
		assert_eq!(mgr.list_graphs().len(), 2);
		assert!(mgr.get_graph("social").is_some());
		assert!(mgr.get_graph("unknown").is_none());
		
		mgr.drop_graph("social")?;
		assert_eq!(mgr.list_graphs().len(), 1);
		
		Ok(())
	}
	
	#[test]
	fn test_find_all_paths() -> Result<()> {
		let mut graph = Graph::new("test".to_string());
		
		let node1 = graph.add_node("A".to_string());
		let node2 = graph.add_node("B".to_string());
		let node3 = graph.add_node("C".to_string());
		let node4 = graph.add_node("D".to_string());
		
		// Create multiple paths from node1 to node4
		graph.add_edge(node1, node2, "EDGE".to_string())?;
		graph.add_edge(node1, node3, "EDGE".to_string())?;
		graph.add_edge(node2, node4, "EDGE".to_string())?;
		graph.add_edge(node3, node4, "EDGE".to_string())?;
		
		let paths = graph.find_all_paths(node1, node4, 10)?;
		assert_eq!(paths.len(), 2);
		
		Ok(())
	}
}
