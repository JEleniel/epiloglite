epiloglite-derive
===================

This crate provides the `#[record]` attribute macro used by EpilogLite to
annotate record types. The macro's responsibilities are intentionally small
and idiomatic:

- Ensure the struct has the engine storage fields `record_id: u128` and
  `record_flags: flagset::FlagSet<epiloglite_core::RecordFlags>`. If those
  fields are missing, the macro inserts them with the correct types.
- Implement the `epiloglite_core::Record` trait for the annotated type.
- Emit a runtime metadata root via a generated `pub fn metadata() ->
  epiloglite_core::Metadata` on the type. The metadata tree uses
  `epiloglite_core::DataType` for describing field shapes.

What the macro does NOT do
-------------------------

- The macro does not implicitly add `#[derive(...)]` attributes. It preserves
  the struct attributes as authored. This keeps behavior predictable and
  avoids surprising additions to user types. Because `epiloglite_core::Record`
  is bound by `Clone + Debug + serde::Serialize + DeserializeOwned`, your
  struct must implement (typically via `#[derive(...)]`) the required
  traits when you intend to use the `Record` functionality.

Usage example
-------------

```rust
use epiloglite_core::Record;
use epiloglite_core::RecordFlags;
use epiloglite_derive::record;
use flagset::FlagSet;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[record]
pub struct MyRecord {
    pub name: String,
    pub value: i32,
    // You can omit record_id/record_flags and the macro will inject them,
    // but you still need to derive the traits required by `Record`.
}

// Use the generated API
fn example() {
    let mut r = MyRecord {
        name: "a".into(),
        value: 1,
        record_id: 0u128,
        record_flags: FlagSet::empty(),
    };
    r.set_record_id(10);
    let md = MyRecord::metadata();
    println!("metadata: {:?}", md);
}
```

Rationale for named fields only
-------------------------------

Tuple (unnamed-field) structs do not provide stable, serializable field
names which are necessary for a metadata tree. The macro therefore only
supports named-field structs. If you need tuple-like behavior, wrap the
positional data in a named struct and derive `From`/`Into` conversions.
