use epiloglite_core::{DataType, DataTypeError, Metadata};
use epiloglite_core::{try_from_slice, try_into_vec};
use serde_json;

#[test]
fn all_primitives_and_scalar_checks() {
    let primitives = vec![
        DataType::Null,
        DataType::Boolean,
        DataType::I8,
        DataType::U8,
        DataType::I16,
        DataType::U16,
        DataType::I32,
        DataType::U32,
        DataType::I64,
        DataType::U64,
        DataType::I128,
        DataType::U128,
        DataType::Isize,
        DataType::Usize,
        DataType::F32,
        DataType::F64,
        DataType::Char,
    ];

    for p in primitives {
        assert!(p.is_primitive(), "{:?} should be primitive", p);
        assert!(p.is_scalar(), "{:?} should be scalar", p);
    }
}

#[test]
fn vec_and_array_checked_constructors_and_validate() {
    // VecPrimitive with primitive inner
    let vp = DataType::vec_primitive_checked(DataType::I32).unwrap();
    assert!(vp.is_vec_primitive());
    vp.validate().unwrap();

    // VecPrimitive with non-primitive should fail
    let bad = DataType::vec_primitive_checked(DataType::String(None));
    assert!(matches!(bad, Err(DataTypeError::ExpectedPrimitive(_))));

    // VecComplex with complex inner
    let vc = DataType::vec_complex_checked(DataType::Struct(vec![])).unwrap();
    assert!(vc.is_vec_complex());
    vc.validate().unwrap();

    // VecComplex with primitive should fail
    let bad2 = DataType::vec_complex_checked(DataType::I64);
    assert!(matches!(bad2, Err(DataTypeError::ExpectedComplex(_))));

    // ArrayPrimitive positive
    let ap = DataType::array_primitive_checked(DataType::U8, 4).unwrap();
    assert!(ap.is_array_primitive());
    ap.validate().unwrap();

    // ArrayPrimitive length zero -> error
    let ap0 = DataType::array_primitive_checked(DataType::U8, 0);
    assert!(matches!(ap0, Err(DataTypeError::InvalidArrayLength(0))));

    // ArrayComplex positive
    let ac = DataType::array_complex_checked(DataType::Struct(vec![]), 2).unwrap();
    assert!(ac.is_array_complex());
    ac.validate().unwrap();

    // ArrayComplex with primitive inner fails
    let ac_bad = DataType::array_complex_checked(DataType::I8, 3);
    assert!(matches!(ac_bad, Err(DataTypeError::ExpectedComplex(_))));
}

#[test]
fn tuple_option_struct_validate_and_metadata() {
    let tup = DataType::Tuple(vec![DataType::I32, DataType::String(None)]);
    tup.validate().unwrap();

    let opt = DataType::Option(Box::new(DataType::Tuple(vec![DataType::I8, DataType::I16])));
    opt.validate().unwrap();

    let m = Metadata::new("field", DataType::I32);
    let s = DataType::Struct(vec![m.clone()]);
    s.validate().unwrap();
    // metadata equality/clone
    assert_eq!(m, Metadata::new("field", DataType::I32));
}

#[test]
fn enum_checked_and_errors() {
    // empty should error
    let e = DataType::enum_checked(vec![]);
    assert!(matches!(e, Err(DataTypeError::ExpectedComplex(_))));

    // non-empty ok
    let e2 = DataType::enum_checked(vec!["A".to_string(), "B".to_string()]).unwrap();
    if let DataType::Enum(vars) = e2 {
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].0, "A");
    } else {
        panic!("expected enum");
    }
}

#[test]
fn serde_human_readable_roundtrip() {
    let dt = DataType::Tuple(vec![
        DataType::I32,
        DataType::String(Some(10)),
        DataType::VecPrimitive(Box::new(DataType::U8)),
    ]);
    let j = serde_json::to_string(&dt).expect("serialize");
    let parsed: DataType = serde_json::from_str(&j).expect("deserialize");
    assert_eq!(dt, parsed);
}

#[test]
fn serde_compact_roundtrip_bincode() {
    let dt = DataType::ArrayPrimitive(Box::new(DataType::I16), 5);
    let bin = try_into_vec(&dt).expect("bincode serialize");
    let parsed: DataType = try_from_slice(&bin).expect("bincode deserialize");
    assert_eq!(dt, parsed);
}

#[test]
fn validate_recursive_failure_paths() {
    // VecPrimitive containing non-primitive inside nested tuple
    let inner = DataType::Tuple(vec![DataType::I32, DataType::String(None)]);
    let vp = DataType::VecPrimitive(Box::new(inner.clone()));
    // validation should fail because inner is not primitive
    assert!(matches!(
        vp.validate(),
        Err(DataTypeError::ExpectedPrimitive(_))
    ));

    // ArrayPrimitive with zero length wrapped
    let arr = DataType::ArrayPrimitive(Box::new(DataType::I8), 0);
    assert!(matches!(
        arr.validate(),
        Err(DataTypeError::InvalidArrayLength(0))
    ));
}
