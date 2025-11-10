use epiloglite_core::{DataType, Metadata};

#[test]
fn roundtrip_json_and_bincode() {
    use serde_json as json;

    let cases: Vec<DataType> = vec![
        DataType::Null,
        DataType::Boolean,
        DataType::I32,
        DataType::String(Some(64)),
        DataType::Enum(vec![
            ("A".to_string(), None),
            (
                "B".to_string(),
                Some(DataType::Tuple(vec![DataType::I32, DataType::String(None)])),
            ),
            (
                "C".to_string(),
                Some(DataType::Struct(vec![Metadata::new("x", DataType::I32)])),
            ),
        ]),
        DataType::Struct(vec![
            Metadata::new("id", DataType::U128),
            Metadata::new(
                "payload",
                DataType::VecComplex(Box::new(DataType::Struct(vec![Metadata::new(
                    "f",
                    DataType::I32,
                )]))),
            ),
        ]),
        DataType::Option(Box::new(DataType::VecPrimitive(Box::new(DataType::I64)))),
        DataType::ArrayPrimitive(Box::new(DataType::U8), 16),
        DataType::Tuple(vec![DataType::I32, DataType::F64, DataType::String(None)]),
    ];

    for c in cases.iter() {
        println!("testing case: {:?}", c);
        // JSON round-trip (human-readable)
        let s = json::to_string(c).expect("json serialize");
        let de: DataType = json::from_str(&s).expect("json deserialize");
        assert_eq!(&de, c, "json roundtrip failed for {:?}", c);

        // bincode round-trip (binary compact) using the project's bincode helpers
        use epiloglite_core::try_from_slice;
        use epiloglite_core::try_into_vec;
        let b = try_into_vec(c).expect("bincode serialize");
        // debug: print raw bytes in hex for failing investigation
        eprintln!("bincode bytes (len={}): {:02x?}", b.len(), b);
        let de2: DataType = try_from_slice(&b).expect("bincode deserialize");
        assert_eq!(&de2, c, "bincode roundtrip failed for {:?}", c);
    }
}
