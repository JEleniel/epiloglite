# Graph Data Structures in EpilogLite

## Overview

EpilogLite now includes native support for graph data structures, enabling efficient graph traversal, path finding, and graph analytics operations alongside traditional relational database functionality.

## Features

### Graph Data Model

- **Nodes**: Vertices in the graph with labels and properties
- **Edges**: Directed connections between nodes with labels, weights, and properties
- **Properties**: Key-value pairs on both nodes and edges
- **Adjacency Lists**: Efficient O(1) neighbor lookups using forward and reverse adjacency lists

### Graph Algorithms

- **Breadth-First Search (BFS)**: Level-order traversal from a starting node
- **Depth-First Search (DFS)**: Recursive depth-first exploration
- **Dijkstra's Shortest Path**: Finds minimum-weight path between two nodes
- **All Paths**: Discovers all paths between nodes with configurable max depth

### Query Syntax

EpilogLite extends SQL with graph-specific operations:

```sql
-- Create a graph
CREATE GRAPH social

-- Add nodes
ADD NODE TO social LABEL 'Person'
ADD NODE TO social LABEL 'Person' PROPERTIES (name = 'Alice', age = '30')

-- Add edges
ADD EDGE TO social FROM '1' TO '2' LABEL 'KNOWS'
ADD EDGE TO social FROM '1' TO '2' LABEL 'DISTANCE' WEIGHT '5.5'

-- Find paths
MATCH PATH IN social FROM '1' TO '5'
MATCH PATH IN social FROM '1' TO '5' USING SHORTEST
MATCH PATH IN social FROM '1' TO '5' USING BFS
MATCH PATH IN social FROM '1' TO '5' USING DFS
MATCH PATH IN social FROM '1' TO '5' USING ALL(10)

-- Drop a graph
DROP GRAPH social
```

## Programming API

### Creating and Managing Graphs

```rust
use epiloglite::eplite::graph::{GraphManager, Graph};
use epiloglite::eplite::error::Result;

fn main() -> Result<()> {
    // Create a graph manager
    let mut manager = GraphManager::new();
    
    // Create a new graph
    manager.create_graph("social".to_string())?;
    
    // Get the graph
    let graph = manager.get_graph_mut("social").unwrap();
    
    Ok(())
}
```

### Adding Nodes

```rust
// Add a node (returns auto-generated ID)
let alice = graph.add_node("Person".to_string());

// Add properties
if let Some(node) = graph.get_node_mut(alice) {
    node.set_property("name".to_string(), "Alice".to_string());
    node.set_property("age".to_string(), "30".to_string());
}
```

### Adding Edges

```rust
// Add an unweighted edge
graph.add_edge(alice, bob, "KNOWS".to_string())?;

// Add a weighted edge
graph.add_weighted_edge(
    los_angeles,
    phoenix,
    "ROAD".to_string(),
    373.0
)?;
```

### Querying the Graph

```rust
// Get neighbors
let neighbors = graph.get_neighbors(alice);

// BFS traversal
let bfs_result = graph.bfs(alice)?;

// DFS traversal
let dfs_result = graph.dfs(alice)?;

// Find shortest path
let path = graph.shortest_path(alice, eve)?;

// Find all paths (max depth 10)
let all_paths = graph.find_all_paths(alice, diana, 10)?;
```

## Architecture

### Data Structures

- **Graph**: Main graph container with nodes, edges, and adjacency lists
- **Node**: ID, label, and properties (BTreeMap)
- **Edge**: ID, from/to node IDs, label, weight, and properties
- **GraphManager**: Manages multiple graphs in a database

### Storage

Graphs use `BTreeMap` and `BTreeSet` for storage, providing:
- Deterministic ordering
- O(log n) operations
- no-std compatibility
- Efficient memory usage

### Adjacency Lists

Two adjacency lists are maintained for each graph:
- **Forward**: Maps node ID → list of outgoing edge IDs
- **Reverse**: Maps node ID → list of incoming edge IDs

This enables O(1) lookup of:
- Outgoing edges from a node
- Incoming edges to a node
- Direct neighbors

## Examples

See `examples/graph_example.rs` for a comprehensive demonstration including:
- Creating a social network graph
- Adding nodes with properties
- Creating relationships (edges)
- Graph traversal (BFS, DFS)
- Path finding (shortest path, all paths)
- Weighted graph example (city distances)

Run with:
```bash
cargo run --example graph_example
```

## Implementation Status

### Completed ✅
- Graph data structures
- Node and edge management
- Graph traversal algorithms
- Path finding algorithms
- SQL-like query syntax
- Parser integration
- Comprehensive tests (24 tests)
- Example code

### Future Work 📋
- Full database storage integration
- Graph persistence with pager system
- Mixed queries (graphs + relational tables)
- Additional graph algorithms (centrality, clustering)
- Graph-specific indexes
- Query optimization for graphs

## Testing

The graph module includes 15 unit tests covering:
- Node creation and properties
- Edge creation (weighted and unweighted)
- Graph operations (add, remove, query)
- Traversal algorithms (BFS, DFS)
- Path finding (shortest path, all paths, no path cases)
- Graph manager operations

Additionally, 9 parser tests verify the graph query syntax.

Run tests with:
```bash
cargo test --lib
```

## Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Add Node | O(log n) | O(1) |
| Add Edge | O(log n) | O(1) |
| Get Neighbors | O(1) + O(d) | O(d) |
| BFS | O(V + E) | O(V) |
| DFS | O(V + E) | O(V) |
| Dijkstra | O((V + E) log V) | O(V) |
| All Paths | O(V^k) | O(V) |

Where:
- V = number of nodes
- E = number of edges
- d = out-degree of a node
- k = max depth for all paths

## License

Graph support is part of EpilogLite and is licensed under LGPL-3.0-only.
