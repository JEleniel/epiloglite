use super::super::journalentry::JournalEntry;
use crate::CInt;
use chrono::{TimeZone, Utc};
use serde_json;

fn cint(val: u64) -> CInt {
    CInt::from(val)
}
#[test]
fn test_journalentry_serde_roundtrip() {
    use JournalEntry::*;
    let timestamp = Utc.with_ymd_and_hms(2024, 10, 13, 10, 0, 0).unwrap();
    let entries = vec![
        BeginTransaction {
            timestamp,
            transaction_id: cint(1),
            crc: 1,
        },
        CommitTransaction {
            timestamp,
            transaction_id: cint(2),
            crc: 2,
        },
        RollbackTransaction {
            timestamp,
            transaction_id: cint(3),
            crc: 3,
        },
        CreateTable {
            timestamp,
            table_id: cint(4),
        },
        CreateIndex {
            timestamp,
            index_id: cint(5),
        },
        CreateView {
            timestamp,
            view_id: cint(6),
        },
        AlterTable {
            timestamp,
            after: false,
            table_id: cint(7),
            table_def: vec![1, 2],
            crc: 7,
        },
        DropTable {
            timestamp,
            table_id: cint(8),
            table_def: vec![3, 4],
            crc: 8,
        },
        DropIndex {
            timestamp,
            index_id: cint(9),
            index_def: vec![5, 6],
            crc: 9,
        },
        DropView {
            timestamp,
            view_id: cint(10),
            view_def: vec![7, 8],
            crc: 10,
        },
        Insert {
            timestamp,
            table_id: cint(11),
            row_id: cint(12),
            row_data: vec![9, 10],
            crc: 11,
        },
        Update {
            timestamp,
            after: true,
            upsert: false,
            table_id: cint(13),
            row_id: cint(14),
            row_data: vec![11, 12],
            crc: 12,
        },
        Delete {
            timestamp,
            table_id: cint(15),
            row_id: cint(16),
            row_data: vec![13, 14],
            crc: 13,
        },
    ];
    for entry in entries {
        let json = serde_json::to_string(&entry).expect("serialize");
        let de: JournalEntry = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(entry, de);
    }
}

#[test]
fn test_begin_transaction_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 1, 12, 0, 0).unwrap();
    let entry = JournalEntry::BeginTransaction {
        timestamp,
        transaction_id: cint(42),
        crc: 123456,
    };
    if let JournalEntry::BeginTransaction {
        timestamp: ts,
        transaction_id,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(transaction_id, cint(42));
        assert_eq!(crc, 123456);
    } else {
        panic!("Expected BeginTransaction variant");
    }
}

#[test]
fn test_commit_transaction_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 2, 13, 0, 0).unwrap();
    let entry = JournalEntry::CommitTransaction {
        timestamp,
        transaction_id: cint(99),
        crc: 654321,
    };
    if let JournalEntry::CommitTransaction {
        timestamp: ts,
        transaction_id,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(transaction_id, cint(99));
        assert_eq!(crc, 654321);
    } else {
        panic!("Expected CommitTransaction variant");
    }
}

#[test]
fn test_rollback_transaction_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 2, 13, 30, 0).unwrap();
    let entry = JournalEntry::RollbackTransaction {
        timestamp,
        transaction_id: cint(77),
        crc: 111222,
    };
    if let JournalEntry::RollbackTransaction {
        timestamp: ts,
        transaction_id,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(transaction_id, cint(77));
        assert_eq!(crc, 111222);
    } else {
        panic!("Expected RollbackTransaction variant");
    }
}

#[test]
fn test_create_table_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 3, 14, 0, 0).unwrap();
    let entry = JournalEntry::CreateTable {
        timestamp,
        table_id: cint(7),
    };
    if let JournalEntry::CreateTable {
        timestamp: ts,
        table_id,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(table_id, cint(7));
    } else {
        panic!("Expected CreateTable variant");
    }
}

#[test]
fn test_create_index_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 3, 14, 30, 0).unwrap();
    let entry = JournalEntry::CreateIndex {
        timestamp,
        index_id: cint(8),
    };
    if let JournalEntry::CreateIndex {
        timestamp: ts,
        index_id,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(index_id, cint(8));
    } else {
        panic!("Expected CreateIndex variant");
    }
}

#[test]
fn test_create_view_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 3, 15, 0, 0).unwrap();
    let entry = JournalEntry::CreateView {
        timestamp,
        view_id: cint(9),
    };
    if let JournalEntry::CreateView {
        timestamp: ts,
        view_id,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(view_id, cint(9));
    } else {
        panic!("Expected CreateView variant");
    }
}

#[test]
fn test_alter_table_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 4, 15, 0, 0).unwrap();
    let table_def = vec![1, 2, 3];
    let entry = JournalEntry::AlterTable {
        timestamp,
        after: true,
        table_id: cint(8),
        table_def: table_def.clone(),
        crc: 1111,
    };
    if let JournalEntry::AlterTable {
        timestamp: ts,
        after,
        table_id,
        table_def: def,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert!(after);
        assert_eq!(table_id, cint(8));
        assert_eq!(def, table_def);
        assert_eq!(crc, 1111);
    } else {
        panic!("Expected AlterTable variant");
    }
}

#[test]
fn test_drop_table_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 8, 19, 0, 0).unwrap();
    let table_def = vec![1, 2, 3, 4];
    let entry = JournalEntry::DropTable {
        timestamp,
        table_id: cint(12),
        table_def: table_def.clone(),
        crc: 5555,
    };
    if let JournalEntry::DropTable {
        timestamp: ts,
        table_id,
        table_def: def,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(table_id, cint(12));
        assert_eq!(def, table_def);
        assert_eq!(crc, 5555);
    } else {
        panic!("Expected DropTable variant");
    }
}

#[test]
fn test_drop_index_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 8, 20, 0, 0).unwrap();
    let index_def = vec![5, 6, 7];
    let entry = JournalEntry::DropIndex {
        timestamp,
        index_id: cint(13),
        index_def: index_def.clone(),
        crc: 6666,
    };
    if let JournalEntry::DropIndex {
        timestamp: ts,
        index_id,
        index_def: def,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(index_id, cint(13));
        assert_eq!(def, index_def);
        assert_eq!(crc, 6666);
    } else {
        panic!("Expected DropIndex variant");
    }
}

#[test]
fn test_drop_view_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 8, 21, 0, 0).unwrap();
    let view_def = vec![8, 9, 10];
    let entry = JournalEntry::DropView {
        timestamp,
        view_id: cint(14),
        view_def: view_def.clone(),
        crc: 7777,
    };
    if let JournalEntry::DropView {
        timestamp: ts,
        view_id,
        view_def: def,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(view_id, cint(14));
        assert_eq!(def, view_def);
        assert_eq!(crc, 7777);
    } else {
        panic!("Expected DropView variant");
    }
}

#[test]
fn test_insert_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 5, 16, 0, 0).unwrap();
    let row_data = vec![10, 20, 30];
    let entry = JournalEntry::Insert {
        timestamp,
        table_id: cint(5),
        row_id: cint(100),
        row_data: row_data.clone(),
        crc: 2222,
    };
    if let JournalEntry::Insert {
        timestamp: ts,
        table_id,
        row_id,
        row_data: data,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(table_id, cint(5));
        assert_eq!(row_id, cint(100));
        assert_eq!(data, row_data);
        assert_eq!(crc, 2222);
    } else {
        panic!("Expected Insert variant");
    }
}

#[test]
fn test_update_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 6, 17, 0, 0).unwrap();
    let row_data = vec![42, 43];
    let entry = JournalEntry::Update {
        timestamp,
        after: false,
        upsert: true,
        table_id: cint(3),
        row_id: cint(200),
        row_data: row_data.clone(),
        crc: 3333,
    };
    if let JournalEntry::Update {
        timestamp: ts,
        after,
        upsert,
        table_id,
        row_id,
        row_data: data,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert!(!after);
        assert!(upsert);
        assert_eq!(table_id, cint(3));
        assert_eq!(row_id, cint(200));
        assert_eq!(data, row_data);
        assert_eq!(crc, 3333);
    } else {
        panic!("Expected Update variant");
    }
}

#[test]
fn test_delete_entry() {
    let timestamp = Utc.with_ymd_and_hms(2024, 6, 7, 18, 0, 0).unwrap();
    let row_data = vec![99, 100];
    let entry = JournalEntry::Delete {
        timestamp,
        table_id: cint(2),
        row_id: cint(300),
        row_data: row_data.clone(),
        crc: 4444,
    };
    if let JournalEntry::Delete {
        timestamp: ts,
        table_id,
        row_id,
        row_data: data,
        crc,
    } = entry
    {
        assert_eq!(ts, timestamp);
        assert_eq!(table_id, cint(2));
        assert_eq!(row_id, cint(300));
        assert_eq!(data, row_data);
        assert_eq!(crc, 4444);
    } else {
        panic!("Expected Delete variant");
    }
}
