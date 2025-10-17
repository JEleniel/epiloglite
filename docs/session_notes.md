---

## Additional Details and Open Topics (2025-10-15)

### Journal Format and Recovery

- The journal format is currently sketched in the source code but not finalized.
- On recovery, transactions with a 'before' but no commit/rollback are rolled back to their prior state.
- Other actions are played forward if possible, or reverted if not.
- Maintenance processes will prune the journal; recovery/committer only need to mark actions as completed.

### Migration and Upgrades

- The file format includes an application ID and migration version for developer use.
- Collection structure migration is intended to be handled by loading the old structure, converting (e.g., via `into()`), then swapping and writing the new collection. Details are still to be worked out.

### Error Handling

- Follows Rust idioms: errors are returned as `Result<T, thiserror::Error>`.
- Recoverable errors are handled internally; unrecoverable errors bubble up to the library user.

### Configuration and Tuning

- Most parameters are self-tuning by default.
- The set of developer-tunable parameters is not finalized.

### Security and Integrity

- CRC32 checksums are used for tamper/corruption detection at multiple levels, but this is not a robust security feature.
- Encryption and access control are not planned for v1, but may be considered in the future.

---
# EpilogLite Documentation Session Notes

## Purpose

This document captures the details, insights, and decisions from the current documentation session regarding EpilogLite's file format, journaling, and collections/records design. It is intended as a living record to support future documentation, design reviews, and implementation alignment.

---

## Source Documents Reviewed

- `FileFormat.md`: Describes the on-disk file format, page layout, and key invariants.
- `JournalFirstCommitLastOperation.md`: Outlines the journaling architecture, page/table abstraction, and high-level database layout.
- `CollectionsRecords.md`: Summarizes requirements for row management, metadata, and serialization in the database.
- Current Rust source code (not modified in this session).

---

## Session Log

- [ ] Session started: 2025-10-15
- [ ] Initial review of key documents completed.
- [ ] This document created to capture ongoing notes and discussion.

---

## Next Steps

- Continue to extract, synthesize, and clarify technical details from the reviewed documents.
- Record any questions, ambiguities, or design decisions as they arise.
- Update this document as the session progresses.

---

_Please add further notes, summaries, or questions below as the session continues._

---

## Design Priorities and Clarifications (2025-10-15)

### 1. Recoverability as Default Priority

- The engine is designed for environments where interruption is common (e.g., mobile).
- Recoverability is the default top priority; other factors (performance, etc.) may be tuned by developers via feature flags or parameters.

### 2. Uniform Page and Collection Model

- All pages are data pages; all logical elements (indices, free page lists, etc.) are collections.
- Collections are handled identically, with full before/after journaling and atomic index pointer commits.

### 3. Collection IDs and Internal Structure

- Each collection is assigned a unique ID by the engine.
- Internal collections (metadata, master record ID index, journal, etc.) have reserved IDs and starting pages (e.g., journal starts at page 2; pages 0-1 are headers/accounting).

### 4. Page Header and Flags

- Each page belongs to a single collection; the collection_id is stored in the page header.
- Flags include Free and Dirty; the flag field is currently a `u8` and may be expanded.

### 5. Atomicity and Consistency

- Atomic final commits ensure the database is always in a consistent state, even if interrupted.
- The journal is the authoritative source for recovery and transaction management (rollback, commit).
- Additional recovery mechanisms may be considered.

### 6. Platform Adaptation and Locking

- The engine will measure parameters (e.g., page size) at file creation to optimize for the platform.
- Locking is only required during backend page writes; record-level locking will be handled in-memory in future work.

### 7. Concurrency Model

- Page-level locking is used for simplicity; only whole pages are written.
- Multiple writers are not a core design goal for the initial release, but may be supported later.
- The initial target is a pure Rust replacement for SQLite 3, prioritizing simplicity and reliability.

---
