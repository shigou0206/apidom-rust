use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, GenericArgument, LitStr,
    PathArguments, Type, TypePath,
};

/// A no-op attribute macro so that `#[element(...)]` can be attached to any item without
/// producing a compiler error when the item does *not* also derive `BuildFromElement`.
#[proc_macro_attribute]
pub fn element(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Just pass the item through unchanged.
    item
}

/// BuildFromElement 派生
#[proc_macro_derive(BuildFromElement, attributes(element))]
pub fn derive_build_from_element(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(ds) => ds.fields,
        _ => {
            return syn::Error::new_spanned(
                struct_name,
                "BuildFromElement can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut build_fields = Vec::new();

    // Handle empty structs
    if fields.is_empty() {
        let expanded = quote! {
            impl BuildFromElement for #struct_name {
                fn build_from_element(el: &Element) -> Option<Self> {
                    el.as_object()?;
                    Some(Self {})
                }
            }
        };
        return TokenStream::from(expanded);
    }

    for field in fields.iter() {
        let ident = field.ident.as_ref().unwrap();
        let (key, attrs) = parse_element_attr(field);

        let field_kind = if attrs.extension { 
            FieldKind::Extension 
        } else if attrs.flatten {
            FieldKind::Flatten
        } else { 
            classify_type(&field.ty) 
        };

        let needs_default = attrs.default && matches!(&field_kind,
            FieldKind::OptString | FieldKind::OptU64 | FieldKind::OptBool | FieldKind::OptF64 | 
            FieldKind::OptSimple | FieldKind::OptBox(_) | FieldKind::OptVec(_) | FieldKind::OptOrRef(_) | 
            FieldKind::OptJsonValue
        );

        let mut accessor = match &field_kind {
            FieldKind::OptString => quote! { obj.get_str(#key) },
            FieldKind::OptU64 =>    quote! { obj.get_u64(#key) },
            FieldKind::OptBool =>   quote! { obj.get_bool(#key) },
            FieldKind::OptF64 =>    quote! { obj.get_f64(#key) },
            FieldKind::OptSimple => quote! { obj.get_value(#key).map(SimpleValue::from_element) },
            FieldKind::OptJsonValue => quote! { obj.get_value(#key).and_then(|e| e.to_json_value()) },
            FieldKind::OptBox(inner) => quote! {
                obj.get(#key)
                   .and_then(|e| #inner::build_from_element(e))
                   .map(Box::new)
            },
            FieldKind::OptVec(inner) => quote! {
                obj.get_array(#key).map(|arr| {
                    arr.iter().filter_map(|e| #inner::build_from_element(e)).collect::<Vec<_>>()
                })
            },
            FieldKind::Vec(inner) => quote! {
                obj.get_array(#key).map(|arr| {
                    arr.iter()
                       .filter_map(|e| #inner::build_from_element(e))
                       .collect::<Vec<_>>()
                }).unwrap_or_default()
            },
            FieldKind::Map(inner) => quote! {
                obj.get_object(#key).map(|o| {
                    o.content.iter()
                        .filter_map(|m| {
                            let k = m.key.as_string()?.content.clone();
                            let v = #inner::build_from_element(&m.value)?;
                            Some((k, v))
                        })
                        .collect::<std::collections::HashMap<_,_>>()
                }).unwrap_or_default()
            },
            FieldKind::MapSimple => quote! {
                obj.get_object(#key).map(|o| {
                    o.content.iter()
                        .filter_map(|m| {
                            let k = m.key.as_string()?.content.clone();
                            let v = SimpleValue::from_element(&m.value);
                            Some((k, v))
                        })
                        .collect::<std::collections::HashMap<_,_>>()
                }).unwrap_or_default()
            },
            FieldKind::OrRef(inner) => quote! {
                obj.get(#key).and_then(|e| {
                    if let Some(item) = #inner::build_from_element(e) {
                        Some(OrReference::Item(item))
                    } else if let Some(r) = e.as_ref_string() {
                        Some(OrReference::Ref(Reference::new(r.to_string())))
                    } else { None }
                })
            },
            FieldKind::OptOrRef(inner) => quote! {
                obj.get(#key).and_then(|e| {
                    if let Some(item) = #inner::build_from_element(e) {
                        Some(OrReference::Item(item))
                    } else if let Some(r) = e.as_ref_string() {
                        Some(OrReference::Ref(Reference::new(r.to_string())))
                    } else { None }
                })
            },
            FieldKind::Extension => quote! {
                obj.content.iter().filter_map(|m| {
                    if let Some(key_node) = m.key.as_string() {
                        if key_node.content.starts_with("x-") {
                            let k = key_node.content.clone();
                            let v = SimpleValue::from_element(&m.value);
                            Some((k, v))
                        } else { None }
                    } else { None }
                }).collect::<std::collections::HashMap<_,_>>()
            },
            FieldKind::Flatten => {
                let field_ty = &field.ty;
                quote! {
                    {
                        // For flatten, we try to build the type from the entire object
                        match <#field_ty>::build_from_element(el) {
                            Some(val) => val,
                            None => {
                                eprintln!("Warning: Failed to flatten field '{}' of type {}", stringify!(#ident), stringify!(#field_ty));
                                Default::default()
                            }
                        }
                    }
                }
            },
            FieldKind::String =>    quote! { 
                obj.get_str(#key).unwrap_or_else(|| {
                    eprintln!("Warning: Missing required string field '{}'", #key);
                    String::new()
                })
            },
            FieldKind::U64 =>       quote! { 
                obj.get_u64(#key).unwrap_or_else(|| {
                    eprintln!("Warning: Missing required u64 field '{}'", #key);
                    0
                })
            },
            FieldKind::Bool =>      quote! { 
                obj.get_bool(#key).unwrap_or_else(|| {
                    eprintln!("Warning: Missing required bool field '{}'", #key);
                    false
                })
            },
            FieldKind::F64 =>       quote! { 
                obj.get_f64(#key).unwrap_or_else(|| {
                    eprintln!("Warning: Missing required f64 field '{}'", #key);
                    0.0
                })
            },
            FieldKind::JsonValue => quote! { 
                obj.get_value(#key).and_then(|e| e.to_json_value()).unwrap_or_else(|| {
                    eprintln!("Warning: Missing required json value field '{}'", #key);
                    serde_json::Value::Null
                })
            },
            FieldKind::OneOf(variants) => {
                // Generate logic to try each variant type
                let variant_checks = variants.iter().map(|variant| {
                    quote! {
                        if let Some(val) = #variant::build_from_element(el) {
                            return Some(#ident::#variant(val));
                        }
                    }
                });
                quote! {
                    {
                        #(#variant_checks)*
                        eprintln!("Warning: OneOf field '{}' failed to match any variant", #key);
                        None
                    }
                }
            },
            FieldKind::Unsupported => quote! {
                compile_error!("BuildFromElement: unsupported field type")
            },
        };

        // Add error logging for missing optional fields
        if matches!(&field_kind, 
            FieldKind::OptString | FieldKind::OptU64 | FieldKind::OptBool | FieldKind::OptF64 |
            FieldKind::OptSimple | FieldKind::OptBox(_) | FieldKind::OptVec(_) | FieldKind::OptOrRef(_) |
            FieldKind::OptJsonValue
        ) {
            accessor = quote! {
                {
                    let result = #accessor;
                    if result.is_none() {
                        eprintln!("Info: Optional field '{}' not found", #key);
                    }
                    result
                }
            };
        }

        accessor = if needs_default {
            quote! { (#accessor).unwrap_or_default() }
        } else { accessor };

        build_fields.push(quote! { #ident: #accessor });
    }

    let expanded = quote! {
        impl BuildFromElement for #struct_name {
            fn build_from_element(el: &Element) -> Option<Self> {
                let obj = el.as_object()?;
                Some(Self { #(#build_fields),* })
            }
        }
    };
    TokenStream::from(expanded)
}

/* ------------------------------------------------------------------------- */
/*                        helpers                                             */
/* ------------------------------------------------------------------------- */

#[derive(Default)]
struct ElementAttrs {
    key: Option<String>,
    default: bool,
    extension: bool,
    flatten: bool,
    one_of: Vec<syn::Type>,
}

/// 返回 (key, attrs)
fn parse_element_attr(field: &Field) -> (String, ElementAttrs) {
    let mut attrs = ElementAttrs::default();
    attrs.key = Some(field.ident.as_ref().unwrap().to_string());
    
    for attr in &field.attrs {
        if attr.path().is_ident("element") {
            // try as string first
            if let Ok(ls) = attr.parse_args::<LitStr>() {
                attrs.key = Some(ls.value());
                continue;
            }
            // parse #[element(default)] or #[element(extension)] etc.
            if let Ok(id) = attr.parse_args::<syn::Ident>() {
                match id.to_string().as_str() {
                    "default" => attrs.default = true,
                    "extension" => attrs.extension = true,
                    "flatten" => attrs.flatten = true,
                    _ => {}
                }
            }
            // TODO: parse #[element(one_of(Type1, Type2))] - would need custom parsing
        }
    }
    
    let key = attrs.key.clone().unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());
    (key, attrs)
}

enum FieldKind {
    OptString,
    OptU64,
    OptBool,
    OptF64,
    OptSimple,
    OptBox(syn::Type),
    OptVec(syn::Type),
    Vec(syn::Type),
    Map(syn::Type),
    MapSimple,
    OrRef(syn::Type),
    OptOrRef(syn::Type),
    Extension,
    Flatten,
    String,
    U64,
    Bool,
    F64,
    OptJsonValue,
    JsonValue,
    OneOf(Vec<syn::Type>),
    Unsupported,
}

/// 粗略判断常见类型；如需更多类型请扩展
fn classify_type(ty: &Type) -> FieldKind {
    // Option<...>
    if let Some(inner) = extract_generic(ty, "Option") {
        if is_ident(&inner, "String") {
            return FieldKind::OptString;
        }
        if is_ident(&inner, "u64") {
            return FieldKind::OptU64;
        }
        if is_ident(&inner, "bool") {
            return FieldKind::OptBool;
        }
        if is_ident(&inner, "f64") {
            return FieldKind::OptF64;
        }
        if is_ident(&inner, "SimpleValue") {
            return FieldKind::OptSimple;
        }
        if is_serde_json_value(&inner) {
            return FieldKind::OptJsonValue;
        }
        if let Some(inner_box) = extract_generic(&inner, "Box") {
            return FieldKind::OptBox(inner_box.clone());
        }
        if let Some(inner_vec) = extract_generic(&inner, "Vec") {
            return FieldKind::OptVec(inner_vec.clone());
        }
        if let Some(inner_or) = extract_generic(&inner, "OrReference") {
            return FieldKind::OptOrRef(inner_or.clone());
        }
    }
    // Vec<T>
    if let Some(inner) = extract_generic(ty, "Vec") {
        return FieldKind::Vec(inner.clone());
    }
    // HashMap<String,T>
    if let Some(inner) = extract_generic(ty, "HashMap") {
        if is_ident(&inner, "SimpleValue") {
            return FieldKind::MapSimple;
        }
        return FieldKind::Map(inner.clone());
    }
    // OrReference<T>
    if let Some(inner) = extract_generic(ty, "OrReference") {
        return FieldKind::OrRef(inner.clone());
    }
    if is_ident(ty, "u64") {
        return FieldKind::U64;
    }
    if is_ident(ty, "bool") {
        return FieldKind::Bool;
    }
    if is_ident(ty, "f64") {
        return FieldKind::F64;
    }
    if is_ident(ty, "String") {
        return FieldKind::String;
    }
    if is_serde_json_value(ty) {
        return FieldKind::JsonValue;
    }
    FieldKind::Unsupported
}

fn extract_generic<'a>(ty: &'a Type, ident: &str) -> Option<&'a Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        let seg = path.segments.last()?;
        if seg.ident == ident {
            if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                if let Some(GenericArgument::Type(t)) = ab.args.first() {
                    // HashMap has <K,V>；Vec/Option 只有一个
                    return Some(t);
                }
            }
        }
    }
    None
}

fn is_ident(ty: &Type, name: &str) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(seg) = path.segments.last() {
            return seg.ident == name;
        }
    }
    false
}

fn is_serde_json_value(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        // Check for serde_json::Value
        if path.segments.len() >= 2 {
            let last_two: Vec<_> = path.segments.iter().rev().take(2).collect();
            if last_two.len() == 2 && 
               last_two[0].ident == "Value" && 
               last_two[1].ident == "serde_json" {
                return true;
            }
        }
        // Also check for just "Value" in case of use statement
        if let Some(seg) = path.segments.last() {
            return seg.ident == "Value";
        }
    }
    false
}