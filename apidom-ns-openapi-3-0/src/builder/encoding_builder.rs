//! # Encoding Builder Module
//!
//! This module provides enhanced encoding building functionality that is functionally equivalent
//! to the TypeScript visitor pattern. It implements specialized headers processing, reference
//! handling, and comprehensive metadata injection.
//!
//! ## Features
//!
//! ### 1. Headers Processing (HeadersVisitor equivalent)
//! - Handles `headers` field as specialized EncodingHeadersElement
//! - Provides reference decoration with `referenced-element=header` metadata
//! - Injects `headerName` metadata for each HeaderElement
//! - Supports polymorphic handling (Reference vs Header elements)
//!
//! ### 2. Recursive Processing (MapVisitor equivalent)
//! - Processes each header value recursively through folder
//! - Maintains proper element transformation
//! - Supports complex document workflows
//!
//! ### 3. Reference Support
//! - Detects and decorates `$ref` elements
//! - Adds appropriate spec path metadata
//! - Maintains reference integrity
//!
//! ### 4. Path Inference
//! - Provides spec path metadata for different element types
//! - Distinguishes between Reference and Header paths
//! - Enables proper navigation and validation
//!
//! ## TypeScript Equivalence
//!
//! This implementation provides feature parity with the TypeScript HeadersVisitor:
//! - ✅ Headers as ObjectElement with specialized processing
//! - ✅ $ref Header support with referenced-element decoration
//! - ✅ HeaderElement name metadata injection
//! - ✅ Reference and Header path inference via specPath
//! - ✅ Recursive processing (MapVisitor equivalent)
//! - ✅ Polymorphic handling (ReferenceLike vs Header)

use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::encoding::EncodingElement;
use crate::builder::encoding_headers_builder::build_and_decorate_encoding_headers;

/// Basic encoding builder - equivalent to simple constructor
pub fn build_encoding(element: &Element) -> Option<EncodingElement> {
    let object = element.as_object()?;
    Some(EncodingElement::with_content(object.clone()))
}

/// Enhanced encoding builder with visitor pattern features
/// Equivalent to TypeScript encoding processing with HeadersVisitor
pub fn build_and_decorate_encoding<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<EncodingElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut encoding = EncodingElement::with_content(object.clone());
    
    // Process each member with visitor pattern
    for member in &object.content {
        let key_str = match &*member.key {
            Element::String(s) => s.content.clone(),
            _ => continue,
        };
        
        let processed_value = if let Some(ref mut f) = folder {
            f.fold_element((*member.value).clone())
        } else {
            (*member.value).clone()
        };
        
        match key_str.as_str() {
            // Special handling for headers field (HeadersVisitor equivalent)
            "headers" => {
                if let Some(headers_obj) = processed_value.as_object() {
                    // Use specialized headers builder
                    if let Some(headers_element) = build_and_decorate_encoding_headers(&processed_value, folder.as_deref_mut()) {
                        encoding.set_headers(headers_element.object);
                        add_headers_processing_metadata(&mut encoding);
                    } else {
                        // Fallback to regular object
                        encoding.set_headers(headers_obj.clone());
                        add_validation_error_metadata(&mut encoding, "headers", "Failed to process headers object");
                    }
                } else {
                    add_validation_error_metadata(&mut encoding, "headers", "Expected object value for headers");
                }
            },
            // Standard fields
            "contentType" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    encoding.set_content_type(string_val);
                    add_fixed_field_metadata(&mut encoding, "contentType");
                } else {
                    add_validation_error_metadata(&mut encoding, "contentType", "Expected string value");
                }
            },
            "style" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    encoding.set_style(string_val);
                    add_fixed_field_metadata(&mut encoding, "style");
                } else {
                    add_validation_error_metadata(&mut encoding, "style", "Expected string value");
                }
            },
            "explode" => {
                if let Some(bool_val) = convert_to_boolean_element(&processed_value) {
                    encoding.set_explode(bool_val);
                    add_fixed_field_metadata(&mut encoding, "explode");
                } else {
                    add_validation_error_metadata(&mut encoding, "explode", "Expected boolean value");
                }
            },
            "allowReserved" => {
                if let Some(bool_val) = convert_to_boolean_element(&processed_value) {
                    encoding.set_allowed_reserved(bool_val);
                    add_fixed_field_metadata(&mut encoding, "allowReserved");
                } else {
                    add_validation_error_metadata(&mut encoding, "allowReserved", "Expected boolean value");
                }
            },
            // $ref handling
            "$ref" => {
                encoding.object.set("$ref", processed_value);
                add_ref_metadata(&mut encoding, "$ref");
            },
            // Fallback for unknown fields
            _ => {
                encoding.object.set(&key_str, processed_value);
                add_fallback_metadata(&mut encoding, &key_str);
            }
        }
    }
    
    // Add processing metadata
    add_processing_metadata(&mut encoding);
    add_spec_path_metadata(&mut encoding);
    
    Some(encoding)
}

/// Convert various element types to StringElement
fn convert_to_string_element(element: &Element) -> Option<StringElement> {
    match element {
        Element::String(s) => Some(s.clone()),
        Element::Number(n) => Some(StringElement::new(&n.content.to_string())),
        Element::Boolean(b) => Some(StringElement::new(&b.content.to_string())),
        _ => None,
    }
}

/// Convert various element types to BooleanElement
fn convert_to_boolean_element(element: &Element) -> Option<BooleanElement> {
    match element {
        Element::Boolean(b) => Some(b.clone()),
        Element::String(s) => {
            match s.content.to_lowercase().as_str() {
                "true" => Some(BooleanElement::new(true)),
                "false" => Some(BooleanElement::new(false)),
                _ => None,
            }
        },
        _ => None,
    }
}

/// Add metadata for headers processing
fn add_headers_processing_metadata(encoding: &mut EncodingElement) {
    encoding.object.meta.properties.insert("headersProcessed".to_string(), Value::Bool(true));
    encoding.object.meta.properties.insert("headersVisitor".to_string(), Value::Bool(true));
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(encoding: &mut EncodingElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    encoding.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for references
fn add_ref_metadata(encoding: &mut EncodingElement, field_name: &str) {
    let key = format!("ref_{}", field_name);
    encoding.object.meta.properties.insert(key, Value::Bool(true));
    encoding.object.meta.properties.insert("referenced-element".to_string(), Value::String("encoding".to_string()));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(encoding: &mut EncodingElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    encoding.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add validation error metadata
fn add_validation_error_metadata(encoding: &mut EncodingElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    encoding.object.meta.properties.insert(key, Value::String(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(encoding: &mut EncodingElement) {
    encoding.object.meta.properties.insert("processed".to_string(), Value::Bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(encoding: &mut EncodingElement) {
    encoding.object.meta.properties.insert("specPath".to_string(), Value::Array(vec![
        Value::String("document".to_string()),
        Value::String("objects".to_string()),
        Value::String("Encoding".to_string())
    ]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::fold::DefaultFolder;

    #[test]
    fn test_basic_encoding_builder() {
        let mut obj = ObjectElement::new();
        obj.set("contentType", Element::String(StringElement::new("application/json")));
        obj.set("style", Element::String(StringElement::new("form")));

        let encoding = build_encoding(&Element::Object(obj));
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        assert_eq!(encoding.content_type().unwrap().content, "application/json");
        assert_eq!(encoding.style().unwrap().content, "form");
    }

    #[test]
    fn test_enhanced_encoding_with_headers() {
        let mut obj = ObjectElement::new();
        obj.set("contentType", Element::String(StringElement::new("multipart/form-data")));
        
        // Create headers object
        let mut headers_obj = ObjectElement::new();
        
        // Add reference header
        let mut ref_header = ObjectElement::new();
        ref_header.set("$ref", Element::String(StringElement::new("#/components/headers/ContentDisposition")));
        headers_obj.set("Content-Disposition", Element::Object(ref_header));
        
        // Add regular header
        let mut regular_header = ObjectElement::new();
        regular_header.set_element_type("header");
        regular_header.set("description", Element::String(StringElement::new("Content encoding")));
        regular_header.set("required", Element::Boolean(BooleanElement::new(false)));
        headers_obj.set("Content-Encoding", Element::Object(regular_header));
        
        obj.set("headers", Element::Object(headers_obj));

        let mut folder = DefaultFolder;
        let encoding = build_and_decorate_encoding(&Element::Object(obj), Some(&mut folder));
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        
        // Verify basic fields
        assert_eq!(encoding.content_type().unwrap().content, "multipart/form-data");
        
        // Verify headers processing
        assert!(encoding.headers().is_some());
        let headers = encoding.headers().unwrap();
        assert_eq!(headers.element, "encodingHeaders");
        
        // Verify headers contain both reference and regular header
        assert!(headers.has_key("Content-Disposition"));
        assert!(headers.has_key("Content-Encoding"));
        
        // Verify processing metadata
        assert!(encoding.object.meta.properties.contains_key("headersProcessed"));
        assert!(encoding.object.meta.properties.contains_key("headersVisitor"));
        assert!(encoding.object.meta.properties.contains_key("fixedField_contentType"));
        assert!(encoding.object.meta.properties.contains_key("processed"));
        assert!(encoding.object.meta.properties.contains_key("specPath"));
        
        // Verify headers metadata (should be handled by headers builder)
        assert!(headers.meta.properties.contains_key("processed"));
        assert!(headers.meta.properties.contains_key("mapVisitor"));
    }

    #[test]
    fn test_encoding_type_conversion() {
        let mut obj = ObjectElement::new();
        // Test boolean conversion
        obj.set("explode", Element::String(StringElement::new("true")));
        obj.set("allowReserved", Element::String(StringElement::new("false")));

        let mut folder = DefaultFolder;
        let encoding = build_and_decorate_encoding(&Element::Object(obj), Some(&mut folder));
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        
        // Verify conversions worked
        assert_eq!(encoding.explode().unwrap().content, true);
        assert_eq!(encoding.allowed_reserved().unwrap().content, false);
        
        // Verify type conversion metadata
        assert!(encoding.object.meta.properties.contains_key("fixedField_explode"));
        assert!(encoding.object.meta.properties.contains_key("fixedField_allowReserved"));
    }

    #[test]
    fn test_encoding_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/encodings/standardEncoding")));

        let encoding = build_and_decorate_encoding::<DefaultFolder>(&Element::Object(obj), None);
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        
        // Verify $ref is preserved
        assert!(encoding.object.get("$ref").is_some());
        assert!(encoding.object.meta.properties.contains_key("ref_$ref"));
        assert!(encoding.object.meta.properties.contains_key("referenced-element"));
    }

    #[test]
    fn test_encoding_validation_errors() {
        let mut obj = ObjectElement::new();
        // Invalid headers (not an object)
        obj.set("headers", Element::String(StringElement::new("invalid")));
        // Invalid explode (not a boolean)
        obj.set("explode", Element::Array(ArrayElement::new_empty()));

        let encoding = build_and_decorate_encoding::<DefaultFolder>(&Element::Object(obj), None);
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        
        // Verify validation errors
        assert!(encoding.object.meta.properties.contains_key("validationError_headers"));
        assert!(encoding.object.meta.properties.contains_key("validationError_explode"));
    }

    #[test]
    fn test_encoding_fallback_behavior() {
        let mut obj = ObjectElement::new();
        obj.set("contentType", Element::String(StringElement::new("application/json")));
        obj.set("custom-field", Element::String(StringElement::new("custom-value")));
        obj.set("x-extension", Element::String(StringElement::new("extension-value")));

        let encoding = build_and_decorate_encoding::<DefaultFolder>(&Element::Object(obj), None);
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        
        // Verify fallback fields are preserved
        assert!(encoding.object.get("custom-field").is_some());
        assert!(encoding.object.get("x-extension").is_some());
        
        // Verify fallback metadata
        assert!(encoding.object.meta.properties.contains_key("fallback_custom-field"));
        assert!(encoding.object.meta.properties.contains_key("fallback_x-extension"));
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // Comprehensive test demonstrating TypeScript HeadersVisitor equivalence
        let mut obj = ObjectElement::new();
        obj.set("contentType", Element::String(StringElement::new("multipart/form-data")));
        obj.set("style", Element::String(StringElement::new("form")));
        obj.set("explode", Element::Boolean(BooleanElement::new(true)));
        
        // Create comprehensive headers
        let mut headers_obj = ObjectElement::new();
        
        // Reference header (isReferenceLikeElement predicate)
        let mut auth_ref = ObjectElement::new();
        auth_ref.set("$ref", Element::String(StringElement::new("#/components/headers/Authorization")));
        headers_obj.set("Authorization", Element::Object(auth_ref));
        
        // Header element (isHeaderElement predicate)
        let mut content_header = ObjectElement::new();
        content_header.set_element_type("header");
        content_header.set("description", Element::String(StringElement::new("Content disposition")));
        content_header.set("required", Element::Boolean(BooleanElement::new(true)));
        content_header.set("style", Element::String(StringElement::new("simple")));
        headers_obj.set("Content-Disposition", Element::Object(content_header));
        
        // Another header element
        let mut custom_header = ObjectElement::new();
        custom_header.set("description", Element::String(StringElement::new("Custom tracking")));
        custom_header.set("schema", Element::Object(ObjectElement::new()));
        headers_obj.set("X-Tracking-ID", Element::Object(custom_header));
        
        obj.set("headers", Element::Object(headers_obj));

        let mut folder = DefaultFolder;
        let encoding = build_and_decorate_encoding(&Element::Object(obj), Some(&mut folder));
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        
        // Verify all TypeScript HeadersVisitor features are implemented:
        
        // 1. Headers as ObjectElement with specialized processing
        assert!(encoding.headers().is_some());
        let headers = encoding.headers().unwrap();
        assert_eq!(headers.element, "encodingHeaders");
        assert!(headers.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "encoding-headers"
            } else {
                false
            }
        }));
        
        // 2. Headers processing metadata
        assert!(encoding.object.meta.properties.contains_key("headersProcessed"));
        assert!(encoding.object.meta.properties.contains_key("headersVisitor"));
        
        // 3. Headers contain all expected elements
        assert!(headers.has_key("Authorization"));
        assert!(headers.has_key("Content-Disposition"));
        assert!(headers.has_key("X-Tracking-ID"));
        
        // 4. Reference decoration should be handled by headers builder
        if let Some(Element::Object(auth_obj)) = headers.get("Authorization") {
            assert!(auth_obj.meta.properties.contains_key("referenced-element"));
        }
        
        // 5. Header name decoration should be handled by headers builder
        if let Some(Element::Object(content_obj)) = headers.get("Content-Disposition") {
            assert!(content_obj.meta.properties.contains_key("headerName"));
        }
        
        // 6. Processing metadata
        assert!(encoding.object.meta.properties.contains_key("processed"));
        assert!(encoding.object.meta.properties.contains_key("specPath"));
        
        // 7. Fixed field processing
        assert!(encoding.object.meta.properties.contains_key("fixedField_contentType"));
        assert!(encoding.object.meta.properties.contains_key("fixedField_style"));
        assert!(encoding.object.meta.properties.contains_key("fixedField_explode"));
        
        // 8. All fields preserved
        assert_eq!(encoding.content_type().unwrap().content, "multipart/form-data");
        assert_eq!(encoding.style().unwrap().content, "form");
        assert_eq!(encoding.explode().unwrap().content, true);
    }

    #[test]
    fn test_encoding_comprehensive_scenario() {
        // Test all supported encoding scenarios
        let mut obj = ObjectElement::new();
        obj.set("contentType", Element::String(StringElement::new("application/x-www-form-urlencoded")));
        obj.set("style", Element::String(StringElement::new("deepObject")));
        obj.set("explode", Element::Boolean(BooleanElement::new(false)));
        obj.set("allowReserved", Element::Boolean(BooleanElement::new(true)));
        
        // Complex headers with mixed types
        let mut headers_obj = ObjectElement::new();
        
        // Multiple reference headers
        let mut auth_ref = ObjectElement::new();
        auth_ref.set("$ref", Element::String(StringElement::new("#/components/headers/BearerAuth")));
        headers_obj.set("Authorization", Element::Object(auth_ref));
        
        let mut api_key_ref = ObjectElement::new();
        api_key_ref.set("$ref", Element::String(StringElement::new("#/components/headers/ApiKey")));
        headers_obj.set("X-API-Key", Element::Object(api_key_ref));
        
        // Multiple header elements
        let mut rate_limit = ObjectElement::new();
        rate_limit.set_element_type("header");
        rate_limit.set("description", Element::String(StringElement::new("Request rate limit")));
        rate_limit.set("required", Element::Boolean(BooleanElement::new(false)));
        headers_obj.set("X-Rate-Limit", Element::Object(rate_limit));
        
        let mut correlation_id = ObjectElement::new();
        correlation_id.set("description", Element::String(StringElement::new("Request correlation ID")));
        correlation_id.set("style", Element::String(StringElement::new("simple")));
        headers_obj.set("X-Correlation-ID", Element::Object(correlation_id));
        
        obj.set("headers", Element::Object(headers_obj));
        
        // Add custom properties
        obj.set("x-custom-encoding", Element::String(StringElement::new("custom")));

        let mut folder = DefaultFolder;
        let encoding = build_and_decorate_encoding(&Element::Object(obj), Some(&mut folder));
        assert!(encoding.is_some());
        
        let encoding = encoding.unwrap();
        
        // Verify comprehensive functionality
        assert_eq!(encoding.content_type().unwrap().content, "application/x-www-form-urlencoded");
        assert_eq!(encoding.style().unwrap().content, "deepObject");
        assert_eq!(encoding.explode().unwrap().content, false);
        assert_eq!(encoding.allowed_reserved().unwrap().content, true);
        
        let headers = encoding.headers().unwrap();
        assert_eq!(headers.content.len(), 4);
        
        // Verify all metadata is present
        assert!(encoding.object.meta.properties.contains_key("processed"));
        assert!(encoding.object.meta.properties.contains_key("headersProcessed"));
        assert!(encoding.object.meta.properties.contains_key("headersVisitor"));
        assert!(encoding.object.meta.properties.contains_key("fallback_x-custom-encoding"));
        
        // Verify headers element has correct type and metadata
        assert_eq!(headers.element, "encodingHeaders");
        assert!(headers.meta.properties.contains_key("processed"));
        assert!(headers.meta.properties.contains_key("mapVisitor"));
    }
}