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
use proc_macro_crate::crate_name as resolve_crate_name;
use proc_macro_crate::FoundCrate;
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
    //// `record_id` field is not of an accepted type.
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
    // Track the detected record_id field form so the generated impl can
    // adapt to either `u128`, `Cu128` or `Option<Cu128>` authored by the
    // user. If absent we'll inject `Option<epiloglite_core::Cu128>`.
    let mut record_id_kind: Option<&str> = None; // "u128", "cu128", "opt_cu128"
    for field in fields.iter() {
        if let Some(ident) = &field.ident {
            if ident == "record_id" {
                has_record_id = true;
                // Detect accepted forms: u128, Cu128, Option<Cu128>
                match &field.ty {
                    Type::Path(TypePath { path, .. }) => {
                        if let Some(seg) = path.segments.last() {
                            let ident = seg.ident.to_string();
                            if ident == "u128" {
                                record_id_kind = Some("u128");
                            } else if ident == "Cu128" {
                                record_id_kind = Some("cu128");
                            } else if ident == "Option" {
                                // Check generic arg is Cu128
                                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                                    for gen in args.args.iter() {
                                        if let syn::GenericArgument::Type(Type::Path(inner)) = gen {
                                            if inner
                                                .path
                                                .segments
                                                .last()
                                                .map(|s| s.ident == "Cu128")
                                                .unwrap_or(false)
                                            {
                                                record_id_kind = Some("opt_cu128");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                if record_id_kind.is_none() {
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
    // Defer injection of missing engine fields until after we resolve the
    // correct `epiloglite_core` crate path so injected field types use the
    // same name the generated impls will reference.

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

    // Resolve the path to the `epiloglite_core` crate as the environment may
    // rename it during workspace builds. We don't want to depend on the
    // epiloglite_core crate at compile time; instead generated code will
    // reference the resolved path so the final consumer's namespace is used.
    let core_crate = match resolve_crate_name("epiloglite-core") {
        Ok(found) => match found {
            FoundCrate::Itself => "crate".to_string(),
            FoundCrate::Name(name) => name,
        },
        // If resolution fails fall back to `crate` which is the common
        // case when the macro is expanding inside the core crate itself.
        Err(_) => "crate".to_string(),
    };
    let core_ident: proc_macro2::Ident = syn::parse_str(&core_crate)
        .unwrap_or_else(|_| syn::Ident::new("epiloglite_core", Span::call_site()));

    // Inject missing fields now that we know the correct epiloglite_core path
    if !has_record_id {
        let field: Field = syn::parse_quote! { pub record_id: Option<#core_ident::Cu128> };
        fields.insert(0, field);
        record_id_kind = Some("opt_cu128");
    }
    if !has_record_flags {
        let field: Field =
            syn::parse_quote! { pub record_flags: flagset::FlagSet<#core_ident::RecordFlags> };
        fields.insert(1, field);
    }

    // Generate an implementation that adapts to the detected record_id field
    // representation.
    // Implement the external Record trait using primitive u128 for record_id
    // while the struct field may be Cu128/Option<Cu128>/u128 as authored.
    let imp = match record_id_kind {
        Some(s) if s == "u128" => {
            quote! {
                impl #core_ident::Record for #struct_ident {
                    fn record_id(&self) -> u128 { self.record_id }
                    fn set_record_id(&mut self, id: u128) { self.record_id = id }
                    fn flags(&self) -> &flagset::FlagSet<#core_ident::RecordFlags> { &self.record_flags }
                    fn flags_mut(&mut self) -> &mut flagset::FlagSet<#core_ident::RecordFlags> { &mut self.record_flags }
                }
            }
        }
        Some(s) if s == "cu128" => {
            quote! {
                impl #core_ident::Record for #struct_ident {
                    fn record_id(&self) -> u128 { u128::try_from(self.record_id.clone()).unwrap_or(0u128) }
                    fn set_record_id(&mut self, id: u128) { self.record_id = #core_ident::Cu128::from(id) }
                    fn flags(&self) -> &flagset::FlagSet<#core_ident::RecordFlags> { &self.record_flags }
                    fn flags_mut(&mut self) -> &mut flagset::FlagSet<#core_ident::RecordFlags> { &mut self.record_flags }
                }
            }
        }
        Some(s) if s == "opt_cu128" => {
            quote! {
                impl #core_ident::Record for #struct_ident {
                    fn record_id(&self) -> u128 { match self.record_id.clone() { Some(cu) => u128::try_from(cu).unwrap_or(0u128), None => 0u128 } }
                    fn set_record_id(&mut self, id: u128) { self.record_id = Some(#core_ident::Cu128::from(id)) }
                    fn flags(&self) -> &flagset::FlagSet<#core_ident::RecordFlags> { &self.record_flags }
                    fn flags_mut(&mut self) -> &mut flagset::FlagSet<#core_ident::RecordFlags> { &mut self.record_flags }
                }
            }
        }
        Some(_) => {
            // Unknown annotation form; treat as Option<Cu128> to be safe.
            quote! {
                impl #core_ident::Record for #struct_ident {
                    fn record_id(&self) -> u128 { match self.record_id.clone() { Some(cu) => u128::try_from(cu).unwrap_or(0u128), None => 0u128 } }
                    fn set_record_id(&mut self, id: u128) { self.record_id = Some(#core_ident::Cu128::from(id)) }
                    fn flags(&self) -> &flagset::FlagSet<#core_ident::RecordFlags> { &self.record_flags }
                    fn flags_mut(&mut self) -> &mut flagset::FlagSet<#core_ident::RecordFlags> { &mut self.record_flags }
                }
            }
        }
        None => {
            // Default: treat as Option<Cu128> injected field
            quote! {
                impl #core_ident::Record for #struct_ident {
                    fn record_id(&self) -> u128 { match self.record_id.clone() { Some(cu) => u128::try_from(cu).unwrap_or(0u128), None => 0u128 } }
                    fn set_record_id(&mut self, id: u128) { self.record_id = Some(#core_ident::Cu128::from(id)) }
                    fn flags(&self) -> &flagset::FlagSet<#core_ident::RecordFlags> { &self.record_flags }
                    fn flags_mut(&mut self) -> &mut flagset::FlagSet<#core_ident::RecordFlags> { &mut self.record_flags }
                }
            }
        }
    };

    fn map_type_to_datatype_expr(
        ty: &Type,
        core_ident: &proc_macro2::Ident,
    ) -> proc_macro2::TokenStream {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                if let Some(seg) = path.segments.last() {
                    let ident = seg.ident.to_string();
                    match ident.as_str() {
                        "u8" => quote! { #core_ident::DataType::U8 },
                        "i8" => quote! { #core_ident::DataType::I8 },
                        "u16" => quote! { #core_ident::DataType::U16 },
                        "i16" => quote! { #core_ident::DataType::I16 },
                        "u32" => quote! { #core_ident::DataType::U32 },
                        "i32" => quote! { #core_ident::DataType::I32 },
                        "u64" => quote! { #core_ident::DataType::U64 },
                        "i64" => quote! { #core_ident::DataType::I64 },
                        "u128" => quote! { #core_ident::DataType::U128 },
                        "i128" => quote! { #core_ident::DataType::I128 },
                        "f32" => quote! { #core_ident::DataType::F32 },
                        "f64" => quote! { #core_ident::DataType::F64 },
                        "bool" => quote! { #core_ident::DataType::Boolean },
                        "String" => quote! { #core_ident::DataType::String(None) },
                        "Vec" => {
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
                                            return quote! { #core_ident::DataType::VecPrimitive(Box::new(#core_ident::DataType::U8)) };
                                        }
                                    }
                                }
                            }
                            quote! { #core_ident::DataType::Null }
                        }
                        "Option" => {
                            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                                for gen in args.args.iter() {
                                    if let syn::GenericArgument::Type(inner_ty) = gen {
                                        let inner_expr =
                                            map_type_to_datatype_expr(inner_ty, core_ident);
                                        return quote! { #core_ident::DataType::Option(Box::new(#inner_expr)) };
                                    }
                                }
                            }
                            quote! { #core_ident::DataType::Option(Box::new(#core_ident::DataType::Null)) }
                        }
                        "str" => quote! { #core_ident::DataType::String(None) },
                        "char" => quote! { #core_ident::DataType::Char },
                        "isize" => quote! { #core_ident::DataType::Isize },
                        "usize" => quote! { #core_ident::DataType::Usize },
                        _other => {
                            let p = path.clone();
                            quote! {
                                {
                                    fn _assert_record_impl<T: #core_ident::Record>() {}
                                    let _ = _assert_record_impl::<#p>;
                                    #p::metadata().dtype
                                }
                            }
                        }
                    }
                } else {
                    quote! { #core_ident::DataType::Null }
                }
            }
            Type::Reference(r) => map_type_to_datatype_expr(&*r.elem, core_ident),
            Type::Slice(s) => {
                if let Type::Path(inner) = &*s.elem {
                    if inner
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident == "u8")
                        .unwrap_or(false)
                    {
                        return quote! { #core_ident::DataType::VecPrimitive(Box::new(#core_ident::DataType::U8)) };
                    }
                }
                quote! { #core_ident::DataType::Null }
            }
            Type::Array(arr) => map_type_to_datatype_expr(&*arr.elem, core_ident),
            Type::Paren(p) => map_type_to_datatype_expr(&*p.elem, core_ident),
            Type::Group(g) => map_type_to_datatype_expr(&*g.elem, core_ident),
            Type::Tuple(t) => {
                let elems: Vec<proc_macro2::TokenStream> = t
                    .elems
                    .iter()
                    .map(|et| map_type_to_datatype_expr(et, core_ident))
                    .collect();
                if elems.is_empty() {
                    quote! { #core_ident::DataType::Null }
                } else {
                    quote! { #core_ident::DataType::Tuple(vec![#(#elems),*]) }
                }
            }
            _ => quote! { #core_ident::DataType::Null },
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
                quote! { #core_ident::DataType::U128 }
            } else if is_flagset_type(&field.ty) {
                // Map FlagSet<...> to a compact u8 representation
                quote! { #core_ident::DataType::U8 }
            } else {
                map_type_to_datatype_expr(&field.ty, &core_ident)
            };
            field_meta_tokens.push(quote! {
                #core_ident::Metadata::new(#name_lit, #dtype_expr)
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
            pub fn metadata() -> #core_ident::Metadata {
                #core_ident::Metadata::new(
                    #struct_name_lit,
                    #core_ident::DataType::Struct(vec![
                        #(#field_meta_tokens),*
                    ])
                )
            }
        }
    };

    // Generate convenient inherent methods that expose primitive u128
    // getters/setters for `record_id` to preserve examples/tests.
    let inherent_impl = match record_id_kind {
        Some(s) if s == "u128" => quote! {
            impl #struct_ident {
                pub fn record_id(&self) -> u128 { self.record_id }
                pub fn set_record_id(&mut self, id: u128) { self.record_id = id }
            }
        },
        Some(s) if s == "cu128" => quote! {
            impl #struct_ident {
                pub fn record_id(&self) -> u128 { u128::try_from(self.record_id.clone()).unwrap_or(0u128) }
                pub fn set_record_id(&mut self, id: u128) { self.record_id = #core_ident::Cu128::from(id) }
            }
        },
        _ => quote! {
            impl #struct_ident {
                pub fn record_id(&self) -> u128 { match self.record_id.clone() { Some(cu) => u128::try_from(cu).unwrap_or(0u128), None => 0u128 } }
                pub fn set_record_id(&mut self, id: u128) { self.record_id = Some(#core_ident::Cu128::from(id)) }
            }
        },
    };

    let expanded = quote! {
        #output_struct

        #imp
        #inherent_impl
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
