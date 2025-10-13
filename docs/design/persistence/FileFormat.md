# EpilogLite: File Format and Persistence

## Key invariants (summary)

- **Page 0** - Header and Free Page List
- **Page 1** - Secondary Header and Accounting Block
- **Page 2** - First Metadata page
- **Page 3** - First Journal page
- **Page 4 to _n_-1** - All other pages
- **Page n** - guaranteed EOF free page
- **Metadata** is loaded entirely into memory at open. This is table_id=0
- **Journal** is a ring buffer of journal entries. This is table_id=1
- All integer fields ≥ `u16` are encoded as **CInt (1–17 bytes)**; `u8` remains `u8`.
- **OffsetPointer(CInt,CInt)** indicates a page_id and offset within the page
    + `(0,0)` - Null pointer
    + Pages 0-3 are invalid for OffsetPointer

## Database Header

The database header is stored on page 0 (primary) and page 1 (secondary) and includes a CRC32 for validation and recovery. The fields are:

| Offset | Field                      | Type                        | Description                             |
| -----: | -------------------------- | --------------------------- | --------------------------------------- |
|   0x00 | **File Type Identifier**   | `u8[10]`                    | Literal `"EpilogLite"`.                 |
|   0x0A | **Version**                | `u8`                        | Format version.                         |
|   0x0B | **Page Size (Exponent)**   | `u8`                        | Page size = 2^N bytes.                  |
|    var | **Flags**                  | `CInt`                      | Global engine behavior flags.           |
|    var | **Free List Offset**       | `OffsetPointer(CInt, CInt)` | Offset of the start of the Free Page List. |
|    var | **Application ID**         | `CInt`                      | Encoded UUID or integer app identifier. |
|    var | **Migration Version**      | `CInt`                      | Schema migration counter.               |
|    var | **Header CRC32**           | `u32`                       | CRC of all prior header bytes.          |

## Page Types and Envelope

All non-free pages follow a uniform _page envelope_:

```rust
{
    counter: CInt, // Number of live entries on the page
    page_type: u8, //The type of page
    flags: u8, // flags (e.g. Dirty, Freed)
    data: Vec<u8>, // The content of the page
    page_crc: u32, // A CRC32 calculated by the maintenance process
    overflow_pointer: OffsetPointer, // Either the Null pointer or a pointer to an Overflow Page 
}
```

The page loader uses the CRC to validate the page on load. The maintenance process updates the CRC after writes. The CRC is invalid if the page is marked Dirty.

- All Free Pages have a simple format, which makes them easy to identify and usable in recovery:

```rust
{
    front_guard: 0xDECAFACE,
    zeroes: [0;page_size-16],
    rear_guard: 0xECAFACED,
}
```

The page types are (details later):

- Free Page - Empty pages
- Data Page - Rows for a table
- Overflow Page - Continues a page and assumes the format of that page type

There are no separate index pages because indices are treated as tables for persistence.

## Journal

- Journal pages record compressed information necessary to recreate or roll back actions. It is used for Transaction management and Recovery.
- Journal entries are written sequentially in the ring; when the tail wraps, freed/zeroed entries are reused.
- Journal entries are treated as authoritative log for data recovery.

### Journal Entries

Each Journal Page is a page (size = 2^N) with the standard envelope and a sequence of journal entries (which can be treated like a table with predefined metadata):

```rust
JournalPageData {
  entries: Vec<JournalEntry>,        // appended entries; ring writes wrap when full
}
```

Each Journal entry contains all the information required to recreate the action and checksummed for validation.

```rust
enum JournalEntry {
    BeginTransaction {
        timestamp: DateTime<Utc>,
        transaction_id: CInt,
        crc: u32,
    },
    CommitTransaction {
        timestamp: DateTime<Utc>,
        transaction_id: CInt,
        crc: u32,
    },
    RollbackTransaction {
        timestamp: DateTime<Utc>,
        transaction_id: CInt,
        crc: u32,
    },
    CreateTable {
        timestamp: DateTime<Utc>,
        table_id: CInt,
    },
    CreateIndex {
        timestamp: DateTime<Utc>,
        index_id: CInt,
    },
    CreateView {
        timestamp: DateTime<Utc>,
        view_id: CInt,
    },
    AlterTable {
        timestamp: DateTime<Utc>,
        after: bool,
        table_id: CInt,
        table_def: Vec<u8>,
        crc: u32,
    },
    DropTable {
        timestamp: DateTime<Utc>,
        table_id: CInt,
        table_def: Vec<u8>,
        crc: u32,
    },
    DropIndex {
        timestamp: DateTime<Utc>,
        index_id: CInt,
        index_def: Vec<u8>,
        crc: u32,
    },
    DropView {
        timestamp: DateTime<Utc>,
        view_id: CInt,
        view_def: Vec<u8>,
        crc: u32,
    },
    Insert {
        timestamp: DateTime<Utc>,
        table_id: CInt,
        row_id: CInt,
        row_data: Vec<u8>,
        crc: u32,
    },
    Update {
        timestamp: DateTime<Utc>,
        after: bool,
        upsert: bool,
        table_id: CInt,
        row_id: CInt,
        row_data: Vec<u8>,
        crc: u32,
    },
    Delete {
        timestamp: DateTime<Utc>,
        table_id: CInt,
        row_id: CInt,
        row_data: Vec<u8>,
        crc: u32,
    },
}
```

### Journal Ring Buffer

- **Tail** appends new entries; **head** advances as maintenance validates & purges consumed entries.
- Maintenance continuously:
    + Reads journal entries.
    + Validates `AFTER_ROW` entries against data pages (check `page_crc` + stored offsets).
    + Once confirmed, marks corresponding entries as _consumed_ and zeroes them (or marks them free) so the ring head can advance.
    + Reclaims overflow pages when their entries are all consumed.
- If journal tail would overrun head (lack of free room), maintenance must preemptively compact data pages or force reclamation, but the engine will avoid this by ensuring a minimum free capacity in the free page list (page 0 invariant).
- Deleted metadata/data pages discovered by maintenance are added to page 0 free list.

## Atomic write sequence (per single change)

1. If appropriate, a Before entry is created in the Journal.
2. A After entry is created in the journal, along with the new row_id.
3. Mark the page to be written as dirty (if not already marked).
4. The Journal write the change using Copy On Write (COW).
5. Update the counters on the page(s) to reflect the COW, if necessary.
    + If the counter reaches zero, mark the page Freed and update the prior page's overflow_pointer to point to the value in the freed page's overflow_pointer. (This may be Null)
6. Add or Update the table's Row Index entry to point to the new location. This doesn't need to be journaled because the Before and After contain both row_id's.
7. Zero the Journal entries. This is the Commit.
8. Return success to caller.

> [!NOTE] All SQL operations are Journaled, allowing even DDL actions to be reverted or recovered. The Journal even keeps the necessary information to recover data after a DROP TABLE operation until it is fully committed. All DDL operations are conducted inside an internal Transaction, if not already inside one.

## Crash recovery algorithm (startup)

1. Load **Primary Header** (page 0); if CRC invalid, validate secondary and repair as described previously.
2. Determine tuned page size, load **Application ID** and **Migration Version**, load full **Metadata** (page 2+), build in-memory catalog.
3. Initialize the RowID indices and in-memory caches per metadata entries.
4. Scan Journal pages from earliest unconsumed offset to tail:

    + If a Before entry exists without an After entry or Begin Transaction without End Transaction → rollback using the Before entries.
    + If an After exists (if in a Transaction the Transaction must have been ended) → ensure data consistency; if inconsistent, reapply the After entry. This is an idempotent operation, as applying it if the write was successfuly only results in a new COW but does not alter the data.
    + If journal page is partially written (CRC mismatch or Dirty) → ignore trailing corrupted entries; use head/tail markers and continue.

5. After journal-consistency repairs, journal entries marked consumed are zeroed; head pointer set accordingly.
6. Start normal operation with metadata in-memory and valid indices.

_Rationale:_ because changes write BEFORE → data → update → AFTER, partial sequences are safely recoverable: an incomplete AFTER indicates the change did not fully commit; restore BEFORE. If AFTER exists, change is treated as complete and can be enforced. Because the journal also contains the Start, Commit, and Rollback entries for Transactions they can be rolled back or rolled forward during recovery safely.

## Data Page

Like all non-free pages, the Data Page uses the standard envelope. The format of the specific rows depends on the Table Definition.

```text
DataPageData {
  counter: CInt,               // number of rows in the page
  rows: Vec<Row>,
}
```

## Indexes

- Each table maintains a **Table index**: a sequential, sorted array of `OffsetPointer`s stored on one or more pages (COW updated).
- Deleted records are represented by `OffsetPointer = (0,0)` at that index slot.
- Reuse strategy controlled by the `RowID Strategy Flag` (now in header flags). When reusing, insertion scans to first `(0,0)` to allocate new RowID; otherwise append new RowIDs.
- Random access uses the RowID index; sequential scans follow linked data pages via page envelope overflow pointers.

## Free Page List

- **Free Page List** resides on page 0 immediately after header; it is the authoritative allocation pool.
- Maintenance tries to ensures the free list never exceeds  a few pages (it compacts, merges, or allocates as necessary).
- When pages become empty the Free Page front and rear guard values are added. Maintenance zeroes the page.

## Overflow Pages

- Any page can chain via `overflow_pointer` to continuation page(s).
- Overflow pages follow the form of the page linking to them.

## Concurrency

- COW provides isolation for readers: readers always view records in the current state; writers write new entries then change pointers (an atomic write). There is no need for locking, either, thanks to the journal writes happen in the order they arrive, even to the same row.

## Accounting Block

With continuous maintenance and the ring-journal, only a small set of runtime fields are required on page 1:

```rust
AccountingBlock {
    journal_head_pointer: CInt,    // pointer to the head of the Journal ring, useful for recovery
    journal_tail_pointer: CInt,    // pointer to the tail of the Journal ring, useful for recovery
    accounting_crc: u32
}
```

- These are _rebuildable_ if corrupted; maintenance can recompute by scanning header + free list + journal.
