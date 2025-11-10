use epiloglite_core::RecordFlags;
use epiloglite_core::{DataType, Metadata};
use epiloglite_derive::record;
use flagset::FlagSet;

#[record]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NestedRec {
    pub inner: i32,
}

#[record]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetaRec {
    pub name: String,
    pub blob: Vec<u8>,
    pub opt: Option<i16>,
    pub tup: (u8, i16),
    pub flags: FlagSet<RecordFlags>,
    pub nested: NestedRec,
}

#[test]
fn metadata_structure_and_types() {
    let md: Metadata = MetaRec::metadata();
    // root should be Struct
    match md.dtype {
        DataType::Struct(fields) => {
            let get = |n: &str| fields.iter().find(|f| f.name == n).expect(n);

            // name -> String
            match &get("name").dtype {
                DataType::String(_) => {}
                other => panic!("expected String, got {:?}", other),
            }

            // blob -> VecPrimitive(U8)
            match &get("blob").dtype {
                DataType::VecPrimitive(inner) => match **inner {
                    DataType::U8 => {}
                    ref o => panic!("expected VecPrimitive(U8), got {:?}", o),
                },
                other => panic!("expected VecPrimitive(U8), got {:?}", other),
            }

            // opt -> Option(i16)
            match &get("opt").dtype {
                DataType::Option(inner) => match **inner {
                    DataType::I16 => {}
                    ref o => panic!("expected Option(I16), got {:?}", o),
                },
                ref o => panic!("expected Option, got {:?}", o),
            }

            // tup -> Tuple(u8, i16)
            match &get("tup").dtype {
                DataType::Tuple(items) => {
                    assert_eq!(items.len(), 2);
                    match &items[0] {
                        DataType::U8 => {}
                        _ => panic!("tup[0] expected U8"),
                    }
                    match &items[1] {
                        DataType::I16 => {}
                        _ => panic!("tup[1] expected I16"),
                    }
                }
                other => panic!("expected Tuple, got {:?}", other),
            }

            // flags -> U8 (mapped)
            match &get("flags").dtype {
                DataType::U8 => {}
                other => panic!("expected U8, got {:?}", other),
            }

            // nested -> Struct
            match &get("nested").dtype {
                DataType::Struct(_) => {}
                other => panic!("expected Struct, got {:?}", other),
            }
        }
        other => panic!("expected root Struct, got {:?}", other),
    }
}
