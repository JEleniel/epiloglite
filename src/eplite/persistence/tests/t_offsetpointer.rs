//! Tests for OffsetPointer

use crate::CInt;
use crate::eplite::persistence::offsetpointer::OffsetPointer;

#[test]
fn test_offsetpointer_null() {
    let ptr = OffsetPointer::null();
    assert_eq!(ptr.page_number, CInt::zero());
    assert_eq!(ptr.offset, CInt::zero());
}

#[test]
fn test_offsetpointer_equality() {
    let ptr1 = OffsetPointer {
        page_number: 5u32.into(),
        offset: 10u32.into(),
    };
    let ptr2 = OffsetPointer {
        page_number: 5u32.into(),
        offset: 10u32.into(),
    };
    assert_eq!(ptr1, ptr2);
}

#[test]
fn test_offsetpointer_inequality() {
    let ptr1 = OffsetPointer {
        page_number: 1u32.into(),
        offset: 2u32.into(),
    };
    let ptr2 = OffsetPointer {
        page_number: 2u32.into(),
        offset: 1u32.into(),
    };
    assert_ne!(ptr1, ptr2);
}

#[test]
fn test_offsetpointer_ordering() {
    let ptr1 = OffsetPointer {
        page_number: 1u32.into(),
        offset: 5u32.into(),
    };
    let ptr2 = OffsetPointer {
        page_number: 2u32.into(),
        offset: 1u32.into(),
    };
    assert!(ptr1 < ptr2);

    let ptr3 = OffsetPointer {
        page_number: 1u32.into(),
        offset: 10u32.into(),
    };
    assert!(ptr1 < ptr3);
}

#[test]
fn test_offsetpointer_clone() {
    let ptr1 = OffsetPointer {
        page_number: 3u32.into(),
        offset: 7u32.into(),
    };
    let ptr2 = ptr1.clone();
    assert_eq!(ptr1, ptr2);
}

#[test]
fn test_offsetpointer_debug_display() {
    let ptr = OffsetPointer {
        page_number: 123u32.into(),
        offset: 456u32.into(),
    };
    let debug_str = format!("{:?}", ptr);
    assert!(debug_str.contains("OffsetPointer"));
    assert!(debug_str.contains("page_number"));
    assert!(debug_str.contains("offset"));
}

#[test]
fn test_offsetpointer_zero_offset_nonzero_page() {
    let ptr = OffsetPointer {
        page_number: 1u32.into(),
        offset: CInt::zero(),
    };
    assert_eq!(ptr.page_number, 1u32.into());
    assert_eq!(ptr.offset, CInt::zero());
}

#[test]
fn test_offsetpointer_zero_page_nonzero_offset() {
    let ptr = OffsetPointer {
        page_number: CInt::zero(),
        offset: 99u32.into(),
    };
    assert_eq!(ptr.page_number, CInt::zero());
    assert_eq!(ptr.offset, 99u32.into());
}

#[test]
fn test_offsetpointer_ord_eq() {
    let ptr1 = OffsetPointer {
        page_number: 10u32.into(),
        offset: 20u32.into(),
    };
    let ptr2 = OffsetPointer {
        page_number: 10u32.into(),
        offset: 20u32.into(),
    };
    assert!(ptr1 <= ptr2);
    assert!(ptr1 >= ptr2);
}

#[test]
fn test_offsetpointer_ord_page_then_offset() {
    let ptr1 = OffsetPointer {
        page_number: 2u32.into(),
        offset: 1u32.into(),
    };
    let ptr2 = OffsetPointer {
        page_number: 2u32.into(),
        offset: 2u32.into(),
    };
    assert!(ptr1 < ptr2);
}

#[test]
fn test_offsetpointer_large_values() {
    let ptr = OffsetPointer {
        page_number: u32::MAX.into(),
        offset: u32::MAX.into(),
    };
    assert_eq!(ptr.page_number, u32::MAX.into());
    assert_eq!(ptr.offset, u32::MAX.into());
}

#[test]
fn test_offsetpointer_serde() {
    let ptr = OffsetPointer {
        page_number: 42u32.into(),
        offset: 99u32.into(),
    };
    // Use serde_json for roundtrip serialization instead of bincode
    let serialized = serde_json::to_string(&ptr).unwrap();
    let deserialized: OffsetPointer = serde_json::from_str(&serialized).unwrap();
    assert_eq!(ptr, deserialized);
}
