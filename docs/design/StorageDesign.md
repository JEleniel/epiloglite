# EpilogLite Storage Design

## Overview

EpilogLite is a lightweight, embedded data storage engine designed for **primarily local use**, with an optional **HTTPS REST API** server for remote access. While originally built for testing, the REST API has been extended for production readiness.

At its core, EpilogLite emphasizes **robustness, consistency, and minimalism**, with **journaling as the central recovery and integrity mechanism**. Every operation — including record updates, page writes, and metadata changes — passes through the journaling system, enabling forward or backward replay for crash recovery.

### Key Architectural Features

- **Local-first, optionally remote:** Operates locally by default; REST API allows remote or distributed access.
- **Journaling as foundation:** Tracks all before/after record states for durability and recovery.
- **Strong identity guarantees:** Each store includes a 128-bit **Application ID** and 128-bit **Migration Version** for version integrity.
- **Unified container model:** Record indices, metadata, and even the database header are stored as collections with consistent handling.
- **Copy-on-Write updates:** Records are written to a new page slot; old slots are marked free and indices updated atomically.
- **Dirty tracking:** Both **record-level** and **page-level** flags enable efficient incremental flushing.
- **Maintenance thread:** Handles journal replay, compression, cleanup, checkpointing, and commit.

### Idiomatic Rust Design

- **Builder pattern:** All complex objects use the builder pattern for clean, validated construction.
- **Functional record operations:** Record updates return modified copies rather than mutating in place.

## Data Encoding and Serialization

### Compressed Integer (`CInt`)

- Encodes integers from **u16–u128** using 1–17 bytes.
- Reduces physical storage and improves I/O throughput.
- Mandatory and handled transparently by the engine.

### String Compression

- Optional, transparent compression for `String` fields (algorithm TBD).

### Struct Serialization

- Records serialized with **Serde**.
- Primitives encoded directly; complex/nested fields handled recursively.

### Nested Collections and Vectors

- `Vec<primitive>` stored inline with vector indexing.
- `Vec<Struct>` stored as secondary collections, automatically mapping `1:1`, `1:*`, and `*:*` relationships.
- Engine manages indexing, loading, and write-through semantics for secondary collections.

## Page and Index Layout

- All IDs are **u128**: `collection_id`, `record_id`, `page_id`, `page_offset`.
- `collection_id` is a hash of the struct name (hash type TBD) and serves as the metadata row ID.
- `record_id` assigned sequentially; deleted IDs recycled via a pool.
- Each page contains:

    + **Header:** `page_id`, `container_id`, `flags`, `next_page_id`.
    + **SlotIndex:** `(record_id, offset)` pairs for each record.
    + **Data area:** `Vec<u8>` holding serialized records.
    + **CRC32** for integrity.
- Page 0: `record_id 1` (database header). Page 1: `record_id 2` (duplicate header).
- **Record_id index**: `(collection_id, record_id, OffsetPointer)`.
- **Page_size** is 2^9–2^63 bytes; `u64` limit due to `Vec<u8>` in-memory representation.

## Internal Accounting Collections

Persisted like normal collections unless specified:

1. **Record Index** — `(collection_id, record_id, OffsetPointer)` for all records.
2. **Free Page List** — `Vec<u128>` of available freed pages; always contains one free page at the end unless allocation limit reached.
3. **Dirty Page List** — **not persisted**, used to track pages that require flushing.
4. **Journal** — tracks all changes, including transaction boundaries; can be replayed forward or rolled back.

## Integration and Metadata

- `#[record]` procedural macro ensures `record_id` and `flags` fields and implements `Record` trait.
- Metadata is serialized internally for version-consistent deserialization.
- Migration handled via `From`/`Into` conversions; no direct table/schema mutation required.

## Maintenance and Background Systems

- Journal scanning and cleanup
- Page compression and space recovery
- Flushing and committing dirty pages
- Journal forward/backward recovery
- Metadata and collection validation

## Stakeholders

| Role            | Interest                                                    |
| --------------- | ----------------------------------------------------------- |
| **Developers**  | Implement and extend core engine, journal, and API features |
| **Designers**   | Define schema and metadata evolution using the record macro |
| **Users**       | Interact via the REST API or local API bindings             |
| **Maintainers** | Ensure ongoing stability, performance, and compatibility    |

---

## Small API Contract (Inputs / Outputs / Errors)

- Inputs: record operations (create, read, update, delete) and maintenance commands (checkpoint, compact, replay).
- Outputs: persisted pages, journal entries, and explicit operation acknowledgements (OK, Err).
- Errors: I/O failures, CRC mismatches, migration/version mismatches, and allocation exhaustion.

### Success criteria

- All acknowledged writes appear in the journal and can be replayed to reconstruct state.
- Checkpoints create a consistent on-disk snapshot with an empty or truncated journal.

## Edge Cases and Notes

- Crash during write: journal should allow forward or backward recovery.
- Partially written page: CRC32 protects integrity; page is invalidated and replay will re-apply from journal.
- Migration version mismatch: engine refuses to open store unless migration path exists.
- Very large pages: limited by `Vec<u8>` in-memory; avoid relying on >u64 memory assumptions.
- Concurrent access: local-only mode expects single-writer; remote REST API requires additional coordination and optimistic locking.

## Next Steps

1. Link this doc to `docs/design/01_Overview.md` and to `docs/api/README.md`.
2. Add cross-reference comments in `epiloglite-core/src/epiloglite.rs` and `src/eplite/database.rs` pointing to this design.
3. Implement unit tests for `CInt` encoding/decoding and page CRC validation.
4. Choose string compression algorithm (LZ4 / Zstd) and add an opt-in feature flag.
5. Define `collection_id` hash function and document the format.

---

## Appendix: Quick glossary

- Application ID: 128-bit identifier set at store creation.
- Migration Version: 128-bit version tag used for migration compatibility.
- OffsetPointer: compact pointer describing page id + offset within page.
- CInt: compressed integer encoding used across headers and indices.
