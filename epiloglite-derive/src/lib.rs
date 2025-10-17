use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{parse_macro_input, Attribute, Field, Fields, ItemStruct, Type, TypePath};

#[proc_macro_attribute]
pub fn collection(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input struct
    let input = parse_macro_input!(item as ItemStruct);
    let struct_ident = &input.ident;
    let mut fields = match &input.fields {
        Fields::Named(named) => named.named.clone(),
        _ => panic!("collection macro only supports structs with named fields"),
    };

    // Check for record_id field
    let mut has_record_id = false;
    let mut has_record_flags = false;
    for field in fields.iter() {
        if let Some(ident) = &field.ident {
            if ident == "record_id" {
                has_record_id = true;
                // Ensure type is CInt
                if !is_cint_type(&field.ty) {
                    panic!("record_id field must be of type CInt");
                }
            } else if ident == "record_flags" {
                has_record_flags = true;
                // Ensure type is CInt
                if !is_record_flags_type(&field.ty) {
                    panic!("record_flags field must be of type RecordFlags");
                }
            }
        }
    }
    // If not present, add it
    if !has_record_id {
        let field: Field = syn::parse_quote! { pub record_id: CInt };
        fields.insert(0, field);
    }
    if !has_record_flags {
        let field: Field = syn::parse_quote! { pub record_flags: FlagSet<RecordFlags> };
        fields.insert(1, field);
    }

    // Merge derives: only add missing
    let output_struct = ItemStruct {
        attrs: merge_derive_bounds(&input.attrs),
        vis: input.vis.clone(),
        struct_token: input.struct_token,
        ident: struct_ident.clone(),
        generics: input.generics.clone(),
        fields: Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: fields,
        }),
        semi_token: None,
    };

    // Generate the container struct with fields: Vec<FieldMetadata>
    let container_ident = format_ident!("{}Collection", struct_ident);
    let field_metadata = build_field_metadata(&input);
    let container = quote! {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct #container_ident {
            pub container_id: CInt,
            pub name: String,
            pub records: Vec<#struct_ident>,
        }
        impl #container_ident {
            // Use lazy_static for metadata
            pub fn metadata() -> &'static [epiloglite_core::FieldMetadata] {
                static META: once_cell::sync::Lazy<Vec<epiloglite_core::FieldMetadata>> = once_cell::sync::Lazy::new(|| {
                    #field_metadata
                });
                &META
            }

            pub fn new(container_id: CInt, name: String) -> Self {
                Self {
                    container_id,
                    name,
                    records: Vec::new(),
                }
            }
        }
    };

    // Output both structs
    let expanded = quote! {
        #output_struct
        #container
    };
    TokenStream::from(expanded)
}

fn is_cint_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        path.segments
            .last()
            .map(|seg| seg.ident == "CInt")
            .unwrap_or(false)
    } else {
        false
    }
}

/// Checks if the type is `RecordFlags` or `FlagSet<RecordFlags>`.
/// Returns true if the type matches the expected record flags type.
fn is_record_flags_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        let segments: Vec<_> = path.segments.iter().collect();
        // Match RecordFlags directly
        if let Some(last) = segments.last() {
            if last.ident == "RecordFlags" {
                return true;
            }
            // Match FlagSet<RecordFlags>
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

fn merge_derive_bounds(attrs: &[Attribute]) -> Vec<Attribute> {
    let mut new_attrs = Vec::new();
    let mut found = false;
    let mut seen_traits = std::collections::HashSet::new();
    let required = ["Clone", "Debug", "Serialize", "Deserialize"];
    for attr in attrs {
        if attr.path().is_ident("derive") {
            found = true;
            // Parse the derive list using syn v2 API
            if let syn::Meta::List(meta_list) = &attr.meta {
                let parser =
                    syn::punctuated::Punctuated::<syn::Meta, syn::token::Comma>::parse_terminated;
                let nested = parser.parse2(meta_list.tokens.clone()).unwrap_or_default();
                let mut trait_idents = Vec::new();
                for meta in nested.iter() {
                    if let syn::Meta::Path(path) = meta {
                        if let Some(ident) = path.get_ident() {
                            seen_traits.insert(ident.to_string());
                            trait_idents.push(ident.clone());
                        }
                    }
                }
                // Add missing
                for req in &required {
                    if !seen_traits.contains(*req) {
                        let ident: syn::Ident = syn::parse_str(req).unwrap();
                        trait_idents.push(ident);
                    }
                }
                let new_attr: Attribute = syn::parse_quote! {
                    #[derive(#(#trait_idents),*)]
                };
                new_attrs.push(new_attr);
                continue;
            }
            new_attrs.push(attr.clone());
        } else {
            new_attrs.push(attr.clone());
        }
    }
    if !found {
        let mut trait_idents = Vec::new();
        for req in &required {
            let ident: syn::Ident = syn::parse_str(req).unwrap();
            trait_idents.push(ident);
        }
        let new_attr: Attribute = syn::parse_quote! {
            #[derive(#(#trait_idents),*)]
        };
        new_attrs.push(new_attr);
    }
    new_attrs
}

fn build_field_metadata(input: &ItemStruct) -> proc_macro2::TokenStream {
    let mut field_meta = Vec::new();
    if let Fields::Named(named) = &input.fields {
        for field in named.named.iter() {
            let name = field.ident.as_ref().unwrap().to_string();
            let ty = &field.ty;
            let ty_str = quote!(#ty).to_string().replace(' ', "");
            // Try to match primitive types by string
            let ty_expr = match ty_str.as_str() {
                "u8" => quote! { Box::new(epiloglite_core::FieldType::U8) },
                "u16" => quote! { Box::new(epiloglite_core::FieldType::U16) },
                "u32" => quote! { Box::new(epiloglite_core::FieldType::U32) },
                "u64" => quote! { Box::new(epiloglite_core::FieldType::U64) },
                "i8" => quote! { Box::new(epiloglite_core::FieldType::I8) },
                "i16" => quote! { Box::new(epiloglite_core::FieldType::I16) },
                "i32" => quote! { Box::new(epiloglite_core::FieldType::I32) },
                "i64" => quote! { Box::new(epiloglite_core::FieldType::I64) },
                "f32" => quote! { Box::new(epiloglite_core::FieldType::F32) },
                "f64" => quote! { Box::new(epiloglite_core::FieldType::F64) },
                "bool" => quote! { Box::new(epiloglite_core::FieldType::Bool) },
                "String" | "&str" => {
                    quote! { Box::new(epiloglite_core::FieldType::String) }
                }
                _ => {
                    // Nested struct: recurse (for now, just mark as Struct with empty FieldMetadata)
                    quote! { Box::new(epiloglite_core::FieldType::Struct(
                        epiloglite_core::FieldMetadata {
                            name: #name.to_string(),
                            ty: Box::new(epiloglite_core::FieldType::String),
                        }
                    )) }
                }
            };
            field_meta.push(quote! {
                epiloglite_core::FieldMetadata {
                    name: #name.to_string(),
                    ty: #ty_expr,
                }
            });
        }
    }
    // Output as a Vec for lazy_static assignment
    quote! {
        vec![ #(#field_meta),* ]
    }
}
