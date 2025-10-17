// This file has been deleted.
use crate::CInt;
use std::io::BufReader;

// Helper function for powers of two
fn two_pow(i: u32) -> u128 {
    1u128 << i
}

// Macro stub for test_xxx (should be replaced with actual macro if available)
macro_rules! test_xxx {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            // TODO: Implement test for $ty
        }
    };
}
#[test]
fn test_macro_code() {
    let b: usize = u128::BITS as usize;
    for i in 0..b {
        let v: u128 = two_pow(i as u32);
        println!("Test 2^{} {}", i, v);
        let c: CInt = v.try_into().unwrap();
        let v2: u128 = c.try_into().unwrap();
        assert_eq!(v, v2);
    }
    let v: u128 = (u128::MAX).try_into().unwrap();
    let c: CInt = v.try_into().unwrap();
    let v2: u128 = c.try_into().unwrap();
    assert_eq!(v, v2);
}

test_xxx!(test_cint_u16, u16);
test_xxx!(test_cint_u32, u32);
test_xxx!(test_cint_u64, u64);
test_xxx!(test_cint_u128, u128);

test_xxx!(test_cint_i16, i16);
test_xxx!(test_cint_i32, i32);
test_xxx!(test_cint_i64, i64);
test_xxx!(test_cint_i128, i128);

#[test]
fn test_cint_vec() {
    for i in 0..128 {
        let v: u128 = two_pow(i);
        println!("Test 2^{} {}", i, v);
        let c: CInt = v.try_into().unwrap();
        let v2: &mut Vec<u8> = &mut c.try_into().unwrap();
        let v3: u128 = CInt::try_from(v2).unwrap().try_into().unwrap();
        assert_eq!(v, v3);
    }
}

#[test]
fn test_cint_arr() {
    for i in 0..128 {
        let v: u128 = two_pow(i);
        let c: CInt = v.try_into().unwrap();
        let v2: Vec<u8> = c.try_into().unwrap();
        let v3: u128 = CInt::try_from(v2.as_slice()).unwrap().try_into().unwrap();
        assert_eq!(v, v3);
    }
}

#[test]
fn test_cint_read() {
    for i in 0..128 {
        let v: u128 = two_pow(i);
        let c: CInt = v.try_into().unwrap();
        let v2: Vec<u8> = c.try_into().unwrap();
        let v3: u128 = CInt::read_from(&mut BufReader::new(v2.as_slice()))
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(v, v3);
    }
}
