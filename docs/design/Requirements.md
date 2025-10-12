# EpilogLite: Features and Requirements

## Core Data Types

* The storage engine must support all Rust primitive types (u8–u128, i8–i128, f32/f64, bool, char, String) without data loss.
* Nullable columns should be supported using either a leader byte per value or a bitmap for boolean columns.
* A CompressedInt type must provide efficient storage for u64 and u128 values and support additional Rust numeric types and optional value indicators.
* Exact numeric storage for use cases such as currency must be supported using fixed-point or Decimal types.
* Strings must be stored as UTF-8 to match Rust's internal representation, with future compression options available.
* Numeric vector columns (Vec<T>) must be stored efficiently as byte sequences, with type information maintained for indexing and queries.

## Tables and Relationships

* Full ANSI SQL compliance must be provided for standard table operations.
* Multi-Table Foreign Keys (MTFKs) must allow a column to reference multiple tables using a composite identifier.
* Boolean columns must be stored as bitmaps, with optional compression for dense or repetitive patterns.
* Nullable columns must be supported across all tables.

## Graph Support

* The engine must support a Node-Edge graph model.
* Nodes and edges should be stored as tables with adjacency pointers.
* Adjacency lists may be optionally compressed using delta encoding for monotonic sequences.

## NoSQL Containers

* Embedded NoSQL containers, such as JSON or BSON-like columns, must be supported.
* Optional secondary indexing should be available for queries on NoSQL data.

## Indexing

* Index types must include B+Tree, Hash, R-Tree, Inverted/Gin, Column Index, and Graph Index.
* Indexes must be type-aware to support correct comparisons and query semantics.
* Indices should leverage CompressedInt and optional compression techniques for storage efficiency.

## Compression and Encoding

* CompressedInt must serve as the primary compression mechanism for numeric values and optional indicators.
* Delta encoding should be used for sequences that exhibit monotonicity.
* Run-Length Encoding (RLE) may be used for sparse or repetitive sequences and bitmaps.
* Bit-packing should generally be avoided except in scenarios with highly repetitive small enums.

## Transactions and ACID Compliance

* Full ACID compliance must be maintained for all RDBMS operations.
* The engine must support atomicity, consistency, isolation, and durability guarantees.
* Transaction mechanisms must allow for efficient high-TPS operation.
* Write-Ahead Logging (WAL) or equivalent techniques must be used to ensure durability and crash recovery.

## Vector Column Support

* Variable-length and numeric vector columns must be supported as binary blobs.
* Sparse vectors may be optionally compressed using delta encoding or RLE.
* Indices must maintain type information to allow correct query operations.

## Performance Considerations

* CompressedInt encoding must minimize storage overhead for numeric values.
* Nullable value representation must balance simplicity and efficiency.
* Compression techniques must be applied judiciously to maintain performance while reducing storage.
* The system must remain lightweight and performant to be suitable for application-local usage rather than as a full server-grade database.

## Design Decisions Summary

* Nullable columns: leader byte per value or bitmap for booleans.
* Integer and enum columns: CompressedInt; avoid additional RLE or bit-packing except for sparse vectors.
* Boolean columns: bitmaps, optional RLE.
* Vector columns: offset arrays plus compressed blob, optional delta encoding or RLE for sparse sequences.
* Real numbers requiring exactness: fixed-point or Decimal types.
* Multi-Table Foreign Keys: composite identifiers using CompressedInt.
* Transactions: ACID-compliant with WAL or equivalent durability mechanisms.
