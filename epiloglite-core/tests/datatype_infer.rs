use epiloglite_core::DataType;

#[test]
fn infer_simple_tuple() {
    let dt = DataType::of::<(i32, String)>();
    match dt {
        DataType::Tuple(v) => {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], DataType::I32);
            assert_eq!(v[1], DataType::String(None));
        }
        _ => panic!("expected tuple"),
    }
}

#[test]
fn infer_three_tuple() {
    let dt = DataType::of::<(i32, f64, bool)>();
    match dt {
        DataType::Tuple(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], DataType::I32);
            assert_eq!(v[1], DataType::F64);
            assert_eq!(v[2], DataType::Boolean);
        }
        _ => panic!("expected tuple"),
    }
}

#[test]
fn infer_one_tuple() {
    let dt = DataType::of::<(i32,)>();
    match dt {
        DataType::Tuple(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0], DataType::I32);
        }
        _ => panic!("expected tuple"),
    }
}

#[test]
fn infer_four_tuple() {
    let dt = DataType::of::<(i32, u8, String, bool)>();
    match dt {
        DataType::Tuple(v) => {
            assert_eq!(v.len(), 4);
        }
        _ => panic!("expected tuple"),
    }
}

#[test]
fn infer_six_tuple() {
    let dt = DataType::of::<(i8, u8, i16, u16, i32, u32)>();
    match dt {
        DataType::Tuple(v) => {
            assert_eq!(v.len(), 6);
        }
        _ => panic!("expected tuple"),
    }
}
