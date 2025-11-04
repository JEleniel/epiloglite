//! epiloglite-derive
//!
//! Provides the `#[record]` attribute macro used by EpilogLite to annotate
//! record types. The macro's responsibilities are intentionally small and
//! idiomatic:
//!
//! - Ensure the struct contains the engine storage fields `record_id: u128`
//!   and `record_flags: flagset::FlagSet<epiloglite_core::RecordFlags>`;
//!   the macro will inject these fields if missing.
//! - Implement the `epiloglite_core::Record` trait for the type.
//! - Emit a runtime metadata root via a generated `pub fn metadata() ->
//!   epiloglite_core::Metadata` on the type using `epiloglite_core::DataType`.
//!
//! Important: the macro intentionally does NOT add `#[derive(...)]`
//! attributes. It preserves the struct attributes as authored. Because the
//! `Record` trait requires `Clone + Debug + serde::Serialize +
//! DeserializeOwned`, your type must implement those traits (usually via
//! `#[derive(...)]`) when you plan to use the `Record` APIs.
//!
//! Example:
//!
//! ```rust,ignore
//! use epiloglite_core::Record;
//! use epiloglite_core::RecordFlags;
//! use epiloglite_derive::record;
//! use flagset::FlagSet;
//!
//! #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
//! #[record]
//! pub struct MyRecord {
//!     pub name: String,
//!     pub value: i32,
//! }
//!
//! fn example() {
//!     // The macro ensures `record_id`/`record_flags` exist on the type. You
//!     // still need the derives required by `Record` when you instantiate or
//!     // use trait methods. For documentation purposes we can inspect the
//!     // generated metadata without creating an instance:
//!     let md = MyRecord::metadata();
//!     println!("metadata: {:?}", md);
//! }
//! ```
//!
//! Why we don't support unnamed (tuple) fields
//! -------------------------------------------
//! The derive macro only supports structs with named fields (regular struct
//! syntax `struct S { a: u32, b: String }`). Tuple/unnamed-field structs
//! (e.g. `struct S(u32, String);`) are not supported because:
//! - Metadata requires stable, serializable field names. Tuple fields do not
//!   have identifiers and cannot be expressed as a named metadata tree.
//! - The macro inserts `record_id` and `record_flags` as named fields. For a
//!   tuple struct that would change the tuple layout and break code that
//!   depends on positional indexing; it is not safe to inject named fields
//!   into an unnamed-field type.
//! - Enforcing named fields keeps the implementation simple and the resulting
//!   metadata clear and deterministic for the storage engine.
//!
//! If you need tuple-like behavior, wrap positional fields in a named-field
//! struct or provide an explicit conversion layer so the derive can operate on
//! a named representation.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Field, Fields, ItemStruct, Type, TypePath};
use thiserror::Error as ThisError;

/// Errors produced by the `#[record]` macro during expansion.
#[derive(Debug, ThisError)]
enum RecordMacroError {
    /// Struct shape is not supported (only named-field structs are supported).
    #[error("record macro only supports structs with named fields")]
    UnsupportedStructShape,
    //// `record_id` field is not of type `u128`.
    #[error("record_id field must be of type u128")]
    InvalidRecordIdType,
    /// `record_flags` field is not of type `FlagSet<RecordFlags>`.
    #[error("record_flags field must be of type FlagSet<RecordFlags>")]
    InvalidRecordFlagsType,
}

impl RecordMacroError {
    /// Convert to a `syn::Error` anchored at a token span.
    fn to_compile_error(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        syn::Error::new(span, format!("{}", self)).to_compile_error()
    }
}

/// Procedural attribute macro that ensures a struct has `record_id: u128` and
/// `record_flags: FlagSet<RecordFlags>` fields, derives required traits, and
/// implements `epiloglite_core::Record` for the struct so it can be used by
/// the EpilogLite core APIs.
///
/// The macro only supports structs with named fields. See the crate-level
/// documentation at the top of this file for the rationale.
#[proc_macro_attribute]
pub fn record(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_ident = &input.ident;
    let mut fields = match &input.fields {
        Fields::Named(named) => named.named.clone(),
        _ => {
            let ts = RecordMacroError::UnsupportedStructShape.to_compile_error(Span::call_site());
            return TokenStream::from(ts);
        }
    };

    // Check for record_id and record_flags fields
    let mut has_record_id = false;
    let mut has_record_flags = false;
    for field in fields.iter() {
        if let Some(ident) = &field.ident {
            if ident == "record_id" {
                has_record_id = true;
                // Inline check for u128
                let is_u128 = if let Type::Path(TypePath { path, .. }) = &field.ty {
                    path.segments
                        .last()
                        .map(|seg| seg.ident == "u128")
                        .unwrap_or(false)
                } else {
                    false
                };
                if !is_u128 {
                    let ts =
                        RecordMacroError::InvalidRecordIdType.to_compile_error(field.ty.span());
                    return TokenStream::from(ts);
                }
            } else if ident == "record_flags" {
                has_record_flags = true;
                // Inline check for FlagSet<RecordFlags> or RecordFlags
                let mut is_flagset = false;
                if let Type::Path(TypePath { path, .. }) = &field.ty {
                    let segments: Vec<_> = path.segments.iter().collect();
                    if let Some(last) = segments.last() {
                        if last.ident == "RecordFlags" {
                            is_flagset = true;
                        }
                        if last.ident == "FlagSet" {
                            if let syn::PathArguments::AngleBracketed(ref args) = last.arguments {
                                for arg in &args.args {
                                    if let syn::GenericArgument::Type(Type::Path(ref inner_path)) =
                                        arg
                                    {
                                        if inner_path
                                            .path
                                            .segments
                                            .last()
                                            .map(|seg| seg.ident == "RecordFlags")
                                            .unwrap_or(false)
                                        {
                                            is_flagset = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if !is_flagset {
                    let ts =
                        RecordMacroError::InvalidRecordFlagsType.to_compile_error(field.ty.span());
                    return TokenStream::from(ts);
                }
            }
        }
    }
    // If not present, add them
    if !has_record_id {
        let field: Field = syn::parse_quote! { pub record_id: u128 };
        fields.insert(0, field);
    }
    if !has_record_flags {
        let field: Field =
            syn::parse_quote! { pub record_flags: flagset::FlagSet<epiloglite_core::RecordFlags> };
        fields.insert(1, field);
    }

    // Preserve the original struct attributes and adds the possibly modified
    // field list (we do not emit compatibility shims or extra helper
    // functions here; the macro purposely keeps the struct attributes as
    // authored by the user).
    let output_struct = ItemStruct {
        attrs: input.attrs.clone(),
        vis: input.vis.clone(),
        struct_token: input.struct_token,
        ident: struct_ident.clone(),
        generics: input.generics.clone(),
        fields: Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: fields.clone(),
        }),
        semi_token: None,
    };

    let imp = quote! {
        impl epiloglite_core::Record for #struct_ident {
            fn record_id(&self) -> u128 {
                self.record_id
            }
            fn set_record_id(&mut self, id: u128) {
                self.record_id = id;
            }
            fn flags(&self) -> &flagset::FlagSet<epiloglite_core::RecordFlags> {
                &self.record_flags
            }
            fn flags_mut(&mut self) -> &mut flagset::FlagSet<epiloglite_core::RecordFlags> {
                &mut self.record_flags
            }
        }
    };

    /// Map a `syn::Type` into a `proc_macro2::TokenStream` expression that,
    /// when expanded, evaluates to an `epiloglite_core::DataType` value.
    ///
    /// This function handles Rust primitive types, `String`, `Vec<u8>`/`[u8]`,
    /// references, arrays and simple custom struct types (by calling
    /// `<T>::metadata()` for non-primitive path types). Unknown types are
    /// mapped to `DataType::Null` as a conservative fallback.
    fn map_type_to_datatype_expr(ty: &Type) -> proc_macro2::TokenStream {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                if let Some(seg) = path.segments.last() {
                    let ident = seg.ident.to_string();
                    match ident.as_str() {
                        "u8" => quote! { epiloglite_core::DataType::U8 },
                        "i8" => quote! { epiloglite_core::DataType::I8 },
                        "u16" => quote! { epiloglite_core::DataType::U16 },
                        "i16" => quote! { epiloglite_core::DataType::I16 },
                        "u32" => quote! { epiloglite_core::DataType::U32 },
                        "i32" => quote! { epiloglite_core::DataType::I32 },
                        "u64" => quote! { epiloglite_core::DataType::U64 },
                        "i64" => quote! { epiloglite_core::DataType::I64 },
                        "u128" => quote! { epiloglite_core::DataType::U128 },
                        "i128" => quote! { epiloglite_core::DataType::I128 },
                        "f32" => quote! { epiloglite_core::DataType::F32 },
                        "f64" => quote! { epiloglite_core::DataType::F64 },
                        "bool" => quote! { epiloglite_core::DataType::Boolean },
                        "String" => quote! { epiloglite_core::DataType::String(None) },
                        "Vec" => {
                            // Detect Vec<u8> -> ByteArray, else treat as Null
                            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                                for gen in args.args.iter() {
                                    if let syn::GenericArgument::Type(Type::Path(inner)) = gen {
                                        if inner
                                            .path
                                            .segments
                                            .last()
                                            .map(|s| s.ident == "u8")
                                            .unwrap_or(false)
                                        {
                                            return quote! { epiloglite_core::DataType::ByteArray };
                                        }
                                    }
                                }
                            }
                            quote! { epiloglite_core::DataType::Null }
                        }
                        "Option" => {
                            // Option<T> -> DataType::Option(Box::new(<T>))
                            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                                for gen in args.args.iter() {
                                    if let syn::GenericArgument::Type(inner_ty) = gen {
                                        let inner_expr = map_type_to_datatype_expr(inner_ty);
                                        return quote! { epiloglite_core::DataType::Option(Box::new(#inner_expr)) };
                                    }
                                }
                            }
                            quote! { epiloglite_core::DataType::Option(Box::new(epiloglite_core::DataType::Null)) }
                        }
                        "str" => quote! { epiloglite_core::DataType::String(None) },
                        "char" => quote! { epiloglite_core::DataType::Char },
                        "isize" => quote! { epiloglite_core::DataType::Isize },
                        "usize" => quote! { epiloglite_core::DataType::Usize },
                        _other => {
                            // Treat as custom struct: use the full path and assert it implements Record
                            let p = path.clone();
                            quote! {
                                {
                                    // Ensure the custom type implements `epiloglite_core::Record`.
                                    // Emit a clear trait-bound error if it does not.
                                    fn _assert_record_impl<T: epiloglite_core::Record>() {}
                                    let _ = _assert_record_impl::<#p>;
                                    #p::metadata().dtype
                                }
                            }
                        }
                    }
                } else {
                    quote! { epiloglite_core::DataType::Null }
                }
            }
            Type::Reference(r) => {
                // &T -> map T
                map_type_to_datatype_expr(&*r.elem)
            }
            Type::Slice(s) => {
                // [u8] -> ByteArray
                if let Type::Path(inner) = &*s.elem {
                    if inner
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident == "u8")
                        .unwrap_or(false)
                    {
                        return quote! { epiloglite_core::DataType::ByteArray };
                    }
                }
                quote! { epiloglite_core::DataType::Null }
            }
            Type::Array(arr) => {
                // [T; N] -> map T (ignore length for now)
                map_type_to_datatype_expr(&*arr.elem)
            }
            Type::Paren(p) => map_type_to_datatype_expr(&*p.elem),
            Type::Group(g) => map_type_to_datatype_expr(&*g.elem),
            Type::Tuple(t) => {
                // Empty tuple -> Null, otherwise Tuple([...])
                let elems: Vec<proc_macro2::TokenStream> = t
                    .elems
                    .iter()
                    .map(|et| map_type_to_datatype_expr(et))
                    .collect();
                if elems.is_empty() {
                    quote! { epiloglite_core::DataType::Null }
                } else {
                    quote! { epiloglite_core::DataType::Tuple(vec![#(#elems),*]) }
                }
            }
            _ => quote! { epiloglite_core::DataType::Null },
        }
    }

    let mut field_meta_tokens = Vec::new();
    for field in fields.iter() {
        if let Some(ident) = &field.ident {
            let name = ident.to_string();
            let name_lit = syn::LitStr::new(&name, Span::call_site());
            // Special-case the engine-injected fields and FlagSet containers.
            let dtype_expr = if name == "record_id" {
                // Always treat record_id as primitive u128
                quote! { epiloglite_core::DataType::U128 }
            } else if is_flagset_type(&field.ty) {
                // Map FlagSet<...> to a compact u8 representation
                quote! { epiloglite_core::DataType::U8 }
            } else {
                map_type_to_datatype_expr(&field.ty)
            };
            field_meta_tokens.push(quote! {
                epiloglite_core::Metadata::new(#name_lit, #dtype_expr)
            });
        }
    }

    let struct_name_str = struct_ident.to_string();
    let struct_name_lit = syn::LitStr::new(&struct_name_str, Span::call_site());

    let meta_impl = quote! {
        impl #struct_ident {
            /// Return runtime metadata for this struct as a named `Metadata` root.
            /// The returned `Metadata` has `name` set to the struct name and
            /// `dtype = DataType::Struct(fields)`.
            pub fn metadata() -> epiloglite_core::Metadata {
                epiloglite_core::Metadata::new(
                    #struct_name_lit,
                    epiloglite_core::DataType::Struct(vec![
                        #(#field_meta_tokens),*
                    ])
                )
            }
        }
    };

    let expanded = quote! {
        #output_struct

        #imp
        #meta_impl
    };
    TokenStream::from(expanded)
}

/// Checks if the type is `RecordFlags` or `FlagSet<RecordFlags>`.
/// Returns true if the type matches the expected record flags type.
fn is_flagset_type(ty: &Type) -> bool {
    // Inline check removed; keep a conservative path-based detection to
    // recognise `RecordFlags` and `FlagSet<RecordFlags>`.
    if let Type::Path(TypePath { path, .. }) = ty {
        let segments: Vec<_> = path.segments.iter().collect();
        if let Some(last) = segments.last() {
            if last.ident == "RecordFlags" {
                return true;
            }
            if last.ident == "FlagSet" {
                if let syn::PathArguments::AngleBracketed(ref args) = last.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(Type::Path(ref inner_path)) = arg {
                            if inner_path
                                .path
                                .segments
                                .last()
                                .map(|seg| seg.ident == "RecordFlags")
                                .unwrap_or(false)
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
