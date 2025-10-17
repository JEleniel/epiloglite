# Rust Database Row and Metadata Requirements

This document summarizes the design requirements for record management and metadata in a Rust-based, page-oriented database system.

## 1. Row Management

* Every persistable object must have a `record_id: usize` field, managed internally by the database.
* `record_id` should be mutable by the database during write operations.
* The `record_id` may remain private to the object; developers do not need to explicitly declare it.
* Row IDs provide a unique identifier per record and support efficient mapping in the master index.
* Updates to `record_id` must be atomic and consistent with journaling and index commits.

## 2. Metadata Management

* Each object must have introspectable metadata describing its fields.
* Metadata should include the **names** and **types** of each field.
* Field metadata must be accessible statically for building database schema collections.
* Metadata supports persistence, indexing, and dynamic query operations.
* Nested or complex Rust types (`Vec<T>`, `Map<K,V>`, JSON-like objects, nodes/edges) should be captured in the metadata.

---

## 3. Serialization

* Objects must implement `Serialize + DeserializeOwned`.
* Row serialization must include all fields, including the database-managed `record_id` if present.
* Metadata may guide serialization and deserialization.

---

## 4. Database Integration

* The database layer should handle conversion between active objects and serialized pages.
* The master index must track `(page_id, offset)` mappings for each record.
* Atomicity of record writes, index updates, and journal entries must be maintained.
* Free page management should support reuse and compaction without requiring developers to manage record IDs manually.
* The design should support generic Rust objects and maintain consistency across memory and on-disk storage.

---

**Summary:** Every persistable Rust object must have a database-managed `record_id` and accessible field metadata, while supporting serialization and integration into a page-oriented storage engine. This ensures consistency, atomicity, and dynamic schema introspection for complex Rust types.
