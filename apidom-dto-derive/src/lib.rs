//! # apidom-dto-derive
//! 
//! This crate provides a set of derive macros for apidom DTOs.
//! 
//! ## Main Features
//! 
//! - `#[derive(DtoSpec)]`: Generate field specifications for describing DTO field types and mappings
//! - `#[derive(FromObjectElement)]`: Generate conversion implementation from AST ObjectElement to DTO
//! - `#[derive(IntoFrbDto)]`: Generate Flutter-Rust Bridge compatible DTO structure and conversion implementation
//! 
//! ## Field Attributes
//! 
//! - `#[dto(rename = "...")]`: Specify JSON key for the field
//! - `#[dto(usize)]`: Mark field as usize type (automatic type conversion)
//! - `#[dto(reference)]`: Mark field as reference type
//! - `#[dto(json)]`: Mark field as JSON value
//! - `#[dto(extensions)]`: Mark field as extensions container
//! 
//! ## Example
//! 
//! ```rust,ignore
//! use std::collections::HashMap;
//! use serde::{Serialize, Deserialize};
//! use apidom_ast::*;
//! use apidom_dto_derive::{DtoSpec, FromObjectElement, IntoFrbDto};
//! 
//! pub trait DtoFieldSpecs {
//!     fn field_specs() -> Vec<FieldSpec>;
//! }
//! 
//! pub trait FromObjectElement {
//!     fn from_object_element(obj: &ObjectElement) -> Self;
//! }
//! 
//! pub struct FieldSpec {
//!     pub name: String,
//!     pub field_type: FieldType,
//!     pub json_key: String,
//! }
//! 
//! #[derive(Debug, PartialEq)]
//! pub enum FieldType {
//!     String,
//!     Number,
//!     Reference,
//! }
//! 
//! #[derive(Debug, Clone, Default, DtoSpec, FromObjectElement, IntoFrbDto)]
//! pub struct ExampleDto {
//!     // Basic field
//!     pub title: Option<String>,
//!     
//!     // Renamed field
//!     #[dto(rename = "minLength")]
//!     pub min_length: Option<usize>,
//!     
//!     // Reference field
//!     #[dto(reference, rename = "$ref")]
//!     pub reference: Option<String>,
//!     
//!     // JSON value field
//!     #[dto(json)]
//!     pub value: Option<String>,
//!     
//!     // Extensions field
//!     #[dto(extensions)]
//!     pub extensions: HashMap<String, String>,
//! }
//! ```
//! 
//! ## Generated Implementations
//! 
//! 1. DtoSpec derive macro generates:
//!    - `impl DtoFieldSpecs for ExampleDto`
//! 
//! 2. FromObjectElement derive macro generates:
//!    - `impl FromObjectElement for ExampleDto`
//! 
//! 3. IntoFrbDto derive macro generates:
//!    - `struct ExampleDtoFrb`
//!    - `impl From<ExampleDto> for ExampleDtoFrb`
//!    - `impl From<ExampleDtoFrb> for ExampleDto`

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta};
use darling::FromDeriveInput;
use apidom_dto_core::{DtoFieldSpecs, FromObjectElement, FieldSpec, FieldType, ObjectElement};

/// Field attribute configuration
#[derive(Debug, FromDeriveInput, Default)]
#[darling(attributes(dto))]
struct DtoOpts {
    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    usize: bool,
    #[darling(default)]
    reference: bool,
    #[darling(default)]
    extensions: bool,
}

/// Generate DTO field specifications
#[proc_macro_derive(DtoSpec, attributes(dto))]
pub fn derive_dto_spec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("DtoSpec can only be derived for structs"),
    };
    
    let field_specs = generate_field_specs(fields, &DtoOpts::default());
    
    TokenStream::from(quote! {
        impl DtoFieldSpecs for #name {
            fn field_specs() -> Vec<FieldSpec> {
                vec![
                    #(#field_specs),*
                ]
            }
        }
    })
}

/// Automatic conversion from ObjectElement
#[proc_macro_derive(FromObjectElement, attributes(dto))]
pub fn derive_from_object_element(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("FromObjectElement can only be derived for structs"),
    };
    
    let field_extractions = generate_field_extractions(fields, &DtoOpts::default());
    
    TokenStream::from(quote! {
        impl FromObjectElement for #name {
            fn from_object_element(obj: &ObjectElement) -> Self {
                let mut dto = Self::default();
                #(#field_extractions)*
                dto
            }
        }
    })
}

/// Generate Flutter-Rust Bridge compatible DTO
#[proc_macro_derive(IntoFrbDto, attributes(dto))]
pub fn derive_into_frb_dto(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let frb_name = format_ident!("{}Frb", name);
    
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("IntoFrbDto can only be derived for structs"),
    };
    
    let mut frb_fields = Vec::new();
    let mut to_frb_conversions = Vec::new();
    let mut from_frb_conversions = Vec::new();
    
    if let Fields::Named(fields) = fields {
        for field in &fields.named {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            
            let mut is_usize = false;
            
            // Check if field needs usize conversion
            for attr in field.attrs.iter().filter_map(parse_dto_attr) {
                if has_flag(&attr, "usize") {
                    is_usize = true;
                    break;
                }
            }
            
            // Generate FRB compatible field type
            let frb_type = if is_usize {
                quote! { Option<u64> }
            } else {
                quote! { #field_type }
            };
            
            // Generate field definition
            frb_fields.push(quote! {
                pub #field_name: #frb_type
            });
            
            // Generate conversion code
            let to_frb = if is_usize {
                quote! {
                    #field_name: dto.#field_name.map(|v| v as u64)
                }
            } else {
                quote! {
                    #field_name: dto.#field_name.clone()
                }
            };
            
            let from_frb = if is_usize {
                quote! {
                    #field_name: frb.#field_name.map(|v| v as usize)
                }
            } else {
                quote! {
                    #field_name: frb.#field_name.clone()
                }
            };
            
            to_frb_conversions.push(to_frb);
            from_frb_conversions.push(from_frb);
        }
    }
    
    TokenStream::from(quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct #frb_name {
            #(#frb_fields),*
        }
        
        impl From<#name> for #frb_name {
            fn from(dto: #name) -> Self {
                Self {
                    #(#to_frb_conversions),*
                }
            }
        }
        
        impl From<#frb_name> for #name {
            fn from(frb: #frb_name) -> Self {
                Self {
                    #(#from_frb_conversions),*
                }
            }
        }
    })
}

// Helper functions
fn parse_dto_attr(attr: &syn::Attribute) -> Option<Meta> {
    if !attr.path().is_ident("dto") {
        return None;
    }
    attr.parse_args().ok()
}

fn get_rename_value(meta: &Meta) -> Option<String> {
    match meta {
        Meta::NameValue(nv) => {
            if nv.path.is_ident("rename") {
                if let syn::Expr::Lit(lit) = &nv.value {
                    if let syn::Lit::Str(s) = &lit.lit {
                        return Some(s.value());
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn has_flag(meta: &Meta, flag: &str) -> bool {
    match meta {
        Meta::Path(path) => path.is_ident(flag),
        _ => false,
    }
}

fn generate_field_specs(fields: &Fields, _opts: &DtoOpts) -> Vec<proc_macro2::TokenStream> {
    let mut specs = Vec::new();
    
    if let Fields::Named(fields) = fields {
        for field in &fields.named {
            let field_name = field.ident.as_ref().unwrap();
            let field_name_str = field_name.to_string();
            
            let mut field_type = quote! { FieldType::String };
            let mut json_key = field_name_str.clone();
            let mut is_extensions = false;
            
            // Parse field attributes
            for attr in field.attrs.iter().filter_map(parse_dto_attr) {
                if has_flag(&attr, "usize") {
                    field_type = quote! { FieldType::Number };
                } else if has_flag(&attr, "reference") {
                    field_type = quote! { FieldType::Reference };
                } else if has_flag(&attr, "extensions") {
                    is_extensions = true;
                }
                
                if let Some(rename) = get_rename_value(&attr) {
                    json_key = rename;
                }
            }
            
            // Generate field specification
            if !is_extensions {
                let spec = quote! {
                    FieldSpec::new(#field_name_str, #field_type)
                        .with_json_key(#json_key)
                };
                
                specs.push(spec);
            }
        }
    }
    
    specs
}

fn generate_field_extractions(fields: &Fields, _opts: &DtoOpts) -> Vec<proc_macro2::TokenStream> {
    let mut extractions = Vec::new();
    
    if let Fields::Named(fields) = fields {
        for field in &fields.named {
            let field_name = field.ident.as_ref().unwrap();
            let field_name_str = field_name.to_string();
            
            let mut extraction_type = "string";
            let mut json_key = field_name_str.clone();
            let mut is_extensions = false;
            
            // Parse field attributes
            for attr in field.attrs.iter().filter_map(parse_dto_attr) {
                if has_flag(&attr, "usize") {
                    extraction_type = "usize";
                } else if has_flag(&attr, "reference") {
                    extraction_type = "reference";
                } else if has_flag(&attr, "extensions") {
                    is_extensions = true;
                }
                
                if let Some(rename) = get_rename_value(&attr) {
                    json_key = rename;
                }
            }
            
            // Generate field extraction code
            let extraction = if is_extensions {
                quote! {
                    dto.#field_name = obj.get_extensions();
                }
            } else {
                match extraction_type {
                    "usize" => quote! {
                        if let Some(val) = obj.get_number(#json_key) {
                            dto.#field_name = Some(val as usize);
                        }
                    },
                    "reference" => quote! {
                        dto.#field_name = extract_reference(obj);
                    },
                    _ => quote! {
                        if let Some(val) = obj.get_string(#json_key) {
                            dto.#field_name = Some(val);
                        }
                    },
                }
            };
            
            extractions.push(extraction);
        }
    }
    
    extractions
}

