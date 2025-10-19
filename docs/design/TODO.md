# EpilogLite Implementation TODO

This document outlines a phased plan to complete EpilogLite. It focuses on deliverables, priorities, estimated effort (relative), risks, and immediate next steps. Use this as the canonical roadmap for engineering work and contributor onboarding.

## High-level goals

- Deliver a safe, test-covered Rust database engine that implements core SQL (CREATE, INSERT, SELECT, UPDATE, DELETE) with durable storage and transactions.
- Keep 100% safe Rust (no `unsafe`) in core engine.
- Support both in-memory and disk-backed storage with recovery guarantees.
- Provide a usable developer API (builders, derive macros) and optional server with TLS.

## Phasing and milestones

Phase 0 — Audit, CI, and contributor hygiene (now)

- Actions
    + Audit current codebase, tests, and docs. Produce a gap analysis (files, missing features, tests).
    + Add or validate CI workflow that runs: `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test` and `cargo test --features no-std` when applicable.
    + Enforce repo conventions: formatting, lint rules, test naming.
- Deliverables
    + This `docs/TODO.md` and the TODO management list in repo.
    + A small CONTRIBUTING checklist for development and PRs.
- Risks
    + Large, untracked tech debt may hide inside tests or feature flags.

Phase 1 — Storage primitives and backing stores (core foundation)

- Actions
    + Finalize on-disk file format (align with `docs/design/persistence/FileFormat.md` and `docs/WAL_IMPLEMENTATION.md`).
    + Harden `pager`, `page`, and `backingstores` modules. Ensure `backingstore` trait is well-defined.
    + Implement atomic page write semantics and a simple journal (if WAL is deferred).
    + Expand backing store tests (file and memory drivers). Add deterministic recovery tests.
- Deliverables
    + `src/eplite/persistence/*` with robust pager and backingstores.
    + Unit + integration tests for page read/write, crash recovery.
- Risks

  # EpilogLite Implementation TODO

  This document outlines a phased plan to complete EpilogLite. It focuses on deliverables, priorities, estimated effort (relative), risks, and immediate next steps. Use this as the canonical roadmap for engineering work and contributor onboarding.

  ## High-level goals

    + Deliver a safe, test-covered Rust database engine that implements core SQL (CREATE, INSERT, SELECT, UPDATE, DELETE) with durable storage and transactions.
    + Keep 100% safe Rust (no `unsafe`) in core engine.
    + Support both in-memory and disk-backed storage with recovery guarantees.
    + Provide a usable developer API (builders, derive macros) and optional server with TLS.

  ## Phasing and milestones

  Phase 0 — Audit, CI, and contributor hygiene (now)

    + Actions

        - Audit current codebase, tests, and docs. Produce a gap analysis (files, missing features, tests).

        - Add or validate CI workflow that runs: `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test` and `cargo test --features no-std` when applicable.

        - Enforce repo conventions: formatting, lint rules, test naming.

    + Deliverables

        - This `docs/TODO.md` and the TODO management list in repo.

        - A small CONTRIBUTING checklist for development and PRs.

    + Risks

        - Large, untracked tech debt may hide inside tests or feature flags.

  Phase 1 — Storage primitives and backing stores (core foundation)

    + Actions

        - Finalize on-disk file format (align with `docs/design/persistence/FileFormat.md` and `docs/WAL_IMPLEMENTATION.md`).

        - Harden `pager`, `page`, and `backingstores` modules. Ensure `backingstore` trait is well-defined.

        - Implement atomic page write semantics and a simple journal (if WAL is deferred).

        - Expand backing store tests (file and memory drivers). Add deterministic recovery tests.

    + Deliverables

        - `src/eplite/persistence/*` with robust pager and backingstores.

        - Unit + integration tests for page read/write, crash recovery.

    + Risks

        - Cross-platform file semantics.

  Phase 2 — Transactions, WAL/journal, and concurrency

    + Actions

        - Implement transactional commit/rollback primitives in journal/pager layer.

        - Implement durable WAL or journal system as described in `docs/WAL_IMPLEMENTATION.md`.

        - Design concurrency strategy: MVCC or simple write-locks. Implement minimal isolation guarantees for now (SERIALIZABLE not required at start).

        - Add savepoint support.

    + Deliverables

        - Transaction API used by higher layers; recovery unit tests.

        - Benchmarks showing commit latency.

    + Risks

        - Complexity of MVCC; prefer a lock-based simple transaction engine first, iterate to MVCC.

  Phase 3 — Storage layout, records, and indexes

    + Actions

        - Define row format, record headers, and column encoding (NULL, text, integers, blobs) in `epiloglite-core` and `src/eplite/persistence` according to `docs/design/` files.

        - Implement rowid index and at least one on-disk index (B-Tree or sorted pages).

        - Add index maintenance logic for INSERT/UPDATE/DELETE.

    + Deliverables

        - Working row storage and a rowid index with tests.

    + Risks

        - On-disk format changes can be disruptive; version headers and migration path needed.

  Phase 4 — Parser, AST, and execution engine (VM/planner)

    + Actions

        - Implement or integrate a SQL parser that produces an AST. Support core statements: CREATE, INSERT, SELECT, UPDATE, DELETE, and transactions.

        - Design a simple planner to convert AST to either an execution plan or direct interpreter ops.

        - Implement an execution engine: table scans, projection, predicate filtering, simple joins (nested-loop), and aggregation primitives as a follow-up.

        - Wire executor to storage layer and indexes.

    + Deliverables

        - Parser unit tests covering statements and error cases.

        - Executor integration tests exercising end-to-end SQL operations against in-memory backing store.

    + Risks

        - Designing AST shapes that are flexible enough for optimization later. Keep AST small and well-documented.

  Phase 5 — Query planner & optimizer

    + Actions

        - Implement a heuristic planner: choose indexes, push predicates, prefer index scans when available.

        - Add cost model primitives later.

        - Add basic statistics collection (counts) to help planning.

    + Deliverables

        - Planner tests and measurable improvements for index-using queries.

  Phase 6 — Developer ergonomics, derive macros, builders, and docs

    + Actions

        - Complete `epiloglite-derive` macros and query builder APIs with examples.

        - Add `examples/` directory and runnable examples for common usage (in-memory, file-backed, migrations).

        - Finish `docs/` pages (`NO_STD.md`, `api/README.md`, and design documents) and publish developer guides.

    + Deliverables

        - Working derive macros validated in tests and CI.

        - Example programs in `examples/` and a small `examples/README.md` showing how to run each.

        - Updated `docs/` with how-to guides for contributors and an API reference sketch.

    + Risks

        - Usage or macro edge-cases breaking backward compatibility; lock integration tests early.

  Phase 7 — Release, packaging, and maintenance

    + Actions

        - Finalize `Cargo.toml` metadata, license, and crate features.

        - Prepare CHANGELOG entries and a release checklist.

        - Configure GitHub Actions to publish to crates.io on release tags (manual approval required).

    + Deliverables

        - Release candidate (tagged) and published crate.

        - Release checklist and rollback plan documented in `docs/release.md`.

  Immediate next steps (1-2 week sprint)

    + 1. Finalize this `docs/TODO.md` (you are reading the result).

    + 2. Add a minimal CI workflow (GitHub Actions) that runs `cargo fmt -- --check`, `cargo clippy -- -D warnings`, and `cargo test` on PRs.

    + 3. Add or update `CONTRIBUTING.md` with branch, commit and PR guidelines and a local test/run checklist.

    + 4. Run a focused audit on `src/eplite/persistence` and `epiloglite-core` modules and attach short findings to the PR.

  Assumptions

    + The existing code in `epiloglite-core` and `src/eplite` is the starting point for the implementation; large refactors may be required but should be staged.

    + We will keep the core safe Rust (`no unsafe`) guarantee where possible and only add `unsafe` when a clear perf or FFI requirement demands it (documented and reviewed).

  Acceptance criteria for major phases

    + Phase 1: Pager and backing stores are covered by unit tests and a crash-recovery integration test. CI passes on these tests.

    + Phase 2: Basic transactions (BEGIN/COMMIT/ROLLBACK) are reliable and durability is demonstrated by recovery tests.

    + Phase 4: Parser accepts core statements and executor runs end-to-end SQL against an in-memory DB in tests.

    + Phase 6: `epiloglite-derive` macros compile and tests exercise the derive-generated code.

  Risks and mitigation

    + Risk: On-disk format churn.

        - Mitigation: Add versioned headers and an explicit migration path.

    + Risk: Concurrency and durability bugs are subtle and hard to reproduce.

        - Mitigation: Add deterministic concurrency tests and a fuzz harness for journaling and recovery logic.

  Project estimation (relative effort)

    + Phase 0: 1 engineer-week (audit and CI)

    + Phase 1: 2-4 engineer-weeks (pager/backing store & tests)

    + Phase 2: 3-6 engineer-weeks (transactions & WAL)

    + Phase 3: 3-8 engineer-weeks (row format & indexes)

    + Phase 4: 4-12 engineer-weeks (parser, AST, executor)

    + Phase 5: 4-10 engineer-weeks (planner & optimizer)

    + Phase 6: 2-4 engineer-weeks (derive, builders, docs)

    + Phase 7: 1-2 engineer-weeks (release, packaging)

  Notes and recommended tooling

    + Use `cargo fmt`, `cargo clippy`, and `rustfmt` hooks in CI.

    + Prefer test-driven development: write small unit tests first for parser nodes, page behaviors, and index maintenance.

    + Add lightweight fuzzing for input parsing and journaling using `cargo-fuzz` when feasible.

  Contact & ownership

    + Primary maintainer: repository owner (see `README.md` contact).

    + Suggested initial contributors: someone familiar with storage engines and Rust (assign on issues).

  Appendix: quick links

    + Design docs: `docs/design/`

    + Persistence details: `docs/design/persistence/` and `docs/WAL_IMPLEMENTATION.md`

    + Core library crate: `epiloglite-core/`

    + Runtime crate and server: `src/` and `server/`

  This TODO is intended to be a living document. Keep it updated as decisions are made and milestones are completed.

- 3. Add or update `CONTRIBUTING.md` with branch, commit and PR guidelines and a local test/run checklist.

- 4. Run a focused audit on `src/eplite/persistence` and `epiloglite-core` modules and attach short findings to the PR.

Assumptions

- + The existing code in `epiloglite-core` and `src/eplite` is the starting point for the implementation; large refactors may be required but should be staged.

- + We will keep the core safe Rust (`no unsafe`) guarantee where possible and only add `unsafe` when a clear perf or FFI requirement demands it (documented and reviewed).

Acceptance criteria for major phases

- + Phase 1: Pager and backing stores are covered by unit tests and a crash-recovery integration test. CI passes on these tests.

- + Phase 2: Basic transactions (BEGIN/COMMIT/ROLLBACK) are reliable and durability is demonstrated by recovery tests.

- + Phase 4: Parser accepts core statements and executor runs end-to-end SQL against an in-memory DB in tests.

- + Phase 6: `epiloglite-derive` macros compile and tests exercise the derive-generated code.

Risks and mitigation

- + Risk: On-disk format churn.

    + Mitigation: Add versioned headers and an explicit migration path.

- + Risk: Concurrency and durability bugs are subtle and hard to reproduce.

    + Mitigation: Add deterministic concurrency tests and a fuzz harness for journaling and recovery logic.

Project estimation (relative effort)

- + Phase 0: 1 engineer-week (audit and CI)

- + Phase 1: 2-4 engineer-weeks (pager/backing store & tests)

- + Phase 2: 3-6 engineer-weeks (transactions & WAL)

- + Phase 3: 3-8 engineer-weeks (row format & indexes)

- + Phase 4: 4-12 engineer-weeks (parser, AST, executor)

- + Phase 5: 4-10 engineer-weeks (planner & optimizer)

- + Phase 6: 2-4 engineer-weeks (derive, builders, docs)

- + Phase 7: 1-2 engineer-weeks (release, packaging)

Notes and recommended tooling

- + Use `cargo fmt`, `cargo clippy`, and `rustfmt` hooks in CI.

- + Prefer test-driven development: write small unit tests first for parser nodes, page behaviors, and index maintenance.

- + Add lightweight fuzzing for input parsing and journaling using `cargo-fuzz` when feasible.

Contact & ownership

- + Primary maintainer: repository owner (see `README.md` contact).

- + Suggested initial contributors: someone familiar with storage engines and Rust (assign on issues).

Appendix: quick links

- + Design docs: `docs/design/`

- + Persistence details: `docs/design/persistence/` and `docs/WAL_IMPLEMENTATION.md`

- + Core library crate: `epiloglite-core/`

- + Runtime crate and server: `src/` and `server/`

This TODO is intended to be a living document. Keep it updated as decisions are made and milestones are completed.
