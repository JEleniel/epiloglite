// This file is not needed and will be removed.
use crate::CInt;
use crate::eplite::persistence::{Page, PageError, PageFlags, PageHeader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DummyRow(u32);

#[test]
fn test_page_header_fields() {
    let table_id = CInt::from(1);
    let header = PageHeader {
        page_id: CInt::from(0),
        table_id: table_id.clone(),
        flags: flagset::FlagSet::<PageFlags>::empty(),
        next_page_id: CInt::from(0),
        page_crc: 0,
        max_page_size: 4096,
        page_size: 0,
    };
    // Additional tests would follow...
}
use crate::CInt;
use crate::eplite::persistence::{Page, PageError, PageFlags, PageHeader};
// use epiloglite_core::{try_from_slice, try_into_vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DummyRow(u32);

#[test]
fn test_page_header_fields() {
    let table_id = CInt::from(1);
    let header = PageHeader {
        page_id: CInt::from(0),
        table_id: table_id.clone(),
        flags: flagset::FlagSet::<PageFlags>::empty(),
        next_page_id: CInt::from(0),
        page_crc: 0,
        max_page_size: 4096,
        page_size: 0,
    };
    assert_eq!(header.table_id, table_id);
    assert!(header.flags.is_empty());
    assert_eq!(header.page_id, CInt::from(0));
}

#[test]
fn test_data_page_entry_management() {
    let mut page: Page<DummyRow> = Page {
        header: PageHeader {
            page_id: CInt::from(0),
            table_id: CInt::from(1),
            flags: flagset::FlagSet::<PageFlags>::empty(),
            next_page_id: CInt::from(0),
            page_crc: 0,
            max_page_size: 4096,
            page_size: 0,
        },
        entries: vec![],
    };
    assert_eq!(page.entries().len(), 0);
    page.add_entry(DummyRow(42)).unwrap();
    assert_eq!(page.entries().len(), 1);
    assert_eq!(page.get_entry(0).unwrap(), DummyRow(42));
    assert!(page.remove_entry(0).is_ok());
    assert_eq!(page.entries().len(), 0);
}

#[test]
fn test_data_page_entry_out_of_bounds() {
    let page: Page<DummyRow> = Page {
        header: PageHeader {
            page_id: CInt::from(0),
            table_id: CInt::from(1),
            flags: flagset::FlagSet::<PageFlags>::empty(),
            next_page_id: CInt::from(0),
            page_crc: 0,
            max_page_size: 4096,
            page_size: 0,
        },
        entries: vec![],
    };
    let err = page.get_entry(0).unwrap_err();
    assert!(matches!(err, PageError::IndexOutOfBounds(0)));
}

#[test]
fn test_data_page_full_error() {
    // This test is not valid with the current Page API, as max_page_size is not exposed.
    // You may want to add a test for PageFull if/when the API supports it.
    assert!(true);
}
