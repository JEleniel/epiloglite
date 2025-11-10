use epiloglite_core::DataType;

#[test]
fn infer_box_and_rc_arc() {
    let dt_box = DataType::of::<Box<i32>>();
    assert_eq!(dt_box, DataType::I32);

    let dt_rc = DataType::of::<std::rc::Rc<i64>>();
    assert_eq!(dt_rc, DataType::I64);

    let dt_arc = DataType::of::<std::sync::Arc<f32>>();
    assert_eq!(dt_arc, DataType::F32);
}

#[test]
fn infer_slice_and_vec() {
    let dt_slice = DataType::of::<&[i32]>();
    assert!(matches!(dt_slice, DataType::VecPrimitive(_)));

    let dt_vec = DataType::of::<Vec<String>>();
    assert!(matches!(dt_vec, DataType::VecComplex(_)));
}

#[test]
fn enum_builder() {
    let e = DataType::enum_checked(vec!["A".to_string(), "B".to_string()]).unwrap();
    assert!(matches!(e, DataType::Enum(_)));
}
