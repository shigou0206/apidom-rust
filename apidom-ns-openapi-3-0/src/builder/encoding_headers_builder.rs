use apidom_ast::*;
use crate::elements::encoding_headers::EncodingHeadersElement;

/// Basic encoding headers builder
pub fn build_encoding_headers(element: &Element) -> Option<EncodingHeadersElement> {
    let object = element.as_object()?;
    Some(EncodingHeadersElement::with_content(object.clone()))
}

/// Enhanced encoding headers builder with visitor pattern features
/// Equivalent to TypeScript HeadersVisitor with MapVisitor and FallbackVisitor
pub fn build_and_decorate_encoding_headers<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<EncodingHeadersElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut headers = EncodingHeadersElement::with_content(object.clone());
    
    // Process each header with visitor pattern (MapVisitor equivalent)
    for member in &object.content {
        let header_name = match &*member.key {
            Element::String(s) => s.content.clone(),
            _ => continue,
        };
        
        let processed_value = if let Some(ref mut f) = folder {
            f.fold_element((*member.value).clone())
        } else {
            (*member.value).clone()
        };
        
        // Check if this is a reference element
        if is_reference_element(&processed_value) {
            // Decorate ReferenceElement with metadata about their referencing type
            if let Element::Object(mut ref_obj) = processed_value {
                add_referenced_element_metadata(&mut ref_obj, "header");
                add_spec_path_metadata(&mut ref_obj, &["document", "objects", "Reference"]);
                headers.set_header(&header_name, Element::Object(ref_obj));
            } else {
                headers.set_header(&header_name, processed_value);
            }
        } else if is_header_element(&processed_value) {
            // Decorate HeaderElement with metadata about their name
            if let Element::Object(mut header_obj) = processed_value {
                add_header_name_metadata(&mut header_obj, &header_name);
                add_spec_path_metadata(&mut header_obj, &["document", "objects", "Header"]);
                headers.set_header(&header_name, Element::Object(header_obj));
            } else {
                headers.set_header(&header_name, processed_value);
            }
        } else {
            // Fallback for other elements
            headers.set_header(&header_name, processed_value);
            add_fallback_metadata(&mut headers, &header_name);
        }
    }
    
    // Add processing metadata
    add_processing_metadata(&mut headers);
    
    headers.object.meta.properties.insert(
        "fixed-field".to_string(), 
        SimpleValue::bool(true)
    );
    
    Some(headers)
}

/// Check if element is a reference element (has $ref field)
fn is_reference_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.has_key("$ref")
    } else {
        false
    }
}

/// Check if element is a header element
fn is_header_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.element == "header" || 
        // Also check for typical header fields
        obj.has_key("description") || obj.has_key("required") || 
        obj.has_key("deprecated") || obj.has_key("style") ||
        obj.has_key("explode") || obj.has_key("schema")
    } else {
        false
    }
}

/// Add metadata for referenced elements
fn add_referenced_element_metadata(element: &mut ObjectElement, referenced_type: &str) {
    element.meta.properties.insert("referenced-element".to_string(), SimpleValue::string(referenced_type.to_string()));
}

/// Add metadata for header names
fn add_header_name_metadata(element: &mut ObjectElement, header_name: &str) {
    element.meta.properties.insert("headerName".to_string(), SimpleValue::string(header_name.to_string()));
}

/// Add spec path metadata
fn add_spec_path_metadata(element: &mut ObjectElement, path: &[&str]) {
    let path_values: Vec<SimpleValue> = path.iter().map(|s| SimpleValue::string(s.to_string())).collect();
    element.meta.properties.insert("specPath".to_string(), SimpleValue::array(path_values));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(headers: &mut EncodingHeadersElement, header_name: &str) {
    let key = format!("fallback_{}", header_name);
    headers.object.meta.properties.insert(key, SimpleValue::bool(true));
}

/// Add overall processing metadata
fn add_processing_metadata(headers: &mut EncodingHeadersElement) {
    headers.object.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
    headers.object.meta.properties.insert("mapVisitor".to_string(), SimpleValue::bool(true));
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::DefaultFolder;
    use apidom_ast::{Element, ObjectElement, StringElement, BooleanElement};

    #[test]
    fn test_basic_encoding_headers_builder() {
        let mut obj = ObjectElement::new();
        let mut header1 = ObjectElement::new();
        header1.set("description", Element::String(StringElement::new("Rate limit header")));
        obj.set("X-Rate-Limit", Element::Object(header1));

        let headers = build_encoding_headers(&Element::Object(obj));
        assert!(headers.is_some());
        
        let headers = headers.unwrap();
        assert!(headers.has_header("X-Rate-Limit"));
        assert_eq!(headers.header_count(), 1);
    }

    #[test]
    fn test_enhanced_encoding_headers_with_references() {
        let mut obj = ObjectElement::new();
        
        // Add a reference header
        let mut ref_header = ObjectElement::new();
        ref_header.set("$ref", Element::String(StringElement::new("#/components/headers/CommonHeader")));
        obj.set("X-Common", Element::Object(ref_header));
        
        // Add a regular header
        let mut regular_header = ObjectElement::new();
        regular_header.set_element_type("header");
        regular_header.set("description", Element::String(StringElement::new("Custom header")));
        regular_header.set("required", Element::Boolean(BooleanElement::new(true)));
        obj.set("X-Custom", Element::Object(regular_header));

        let mut folder = DefaultFolder;
        let headers = build_and_decorate_encoding_headers(&Element::Object(obj), Some(&mut folder));
        assert!(headers.is_some());
        
        let headers = headers.unwrap();
        
        // Verify headers are preserved
        assert!(headers.has_header("X-Common"));
        assert!(headers.has_header("X-Custom"));
        assert_eq!(headers.header_count(), 2);
        
        // Verify reference metadata
        if let Some(Element::Object(ref_obj)) = headers.get_header("X-Common") {
            assert!(ref_obj.meta.properties.contains_key("referenced-element"));
            if let Some(SimpleValue::String(ref_type)) = ref_obj.meta.properties.get("referenced-element") {
                assert_eq!(ref_type, "header");
            }
            assert!(ref_obj.meta.properties.contains_key("specPath"));
        }
        
        // Verify header name metadata
        if let Some(Element::Object(header_obj)) = headers.get_header("X-Custom") {
            assert!(header_obj.meta.properties.contains_key("headerName"));
            if let Some(SimpleValue::String(name)) = header_obj.meta.properties.get("headerName") {
                assert_eq!(name, "X-Custom");
            }
            assert!(header_obj.meta.properties.contains_key("specPath"));
        }
        
        // Verify processing metadata
        assert!(headers.object.meta.properties.contains_key("processed"));
        assert!(headers.object.meta.properties.contains_key("mapVisitor"));
        
        // Verify element type and class
        assert_eq!(headers.object.element, "encodingHeaders");
        assert!(headers.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "encoding-headers"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_encoding_headers_polymorphic_handling() {
        let mut obj = ObjectElement::new();
        
        // Reference-like element
        let mut ref_element = ObjectElement::new();
        ref_element.set("$ref", Element::String(StringElement::new("#/components/headers/Auth")));
        obj.set("Authorization", Element::Object(ref_element));
        
        // Header-like element
        let mut header_element = ObjectElement::new();
        header_element.set("description", Element::String(StringElement::new("Content type")));
        header_element.set("schema", Element::Object(ObjectElement::new()));
        obj.set("Content-Type", Element::Object(header_element));
        
        // Unknown element (fallback)
        obj.set("Unknown", Element::String(StringElement::new("unknown-value")));

        let headers = build_and_decorate_encoding_headers::<DefaultFolder>(&Element::Object(obj), None);
        assert!(headers.is_some());
        
        let headers = headers.unwrap();
        
        // Verify all elements are preserved
        assert!(headers.has_header("Authorization"));
        assert!(headers.has_header("Content-Type"));
        assert!(headers.has_header("Unknown"));
        
        // Verify reference handling
        if let Some(Element::Object(auth_obj)) = headers.get_header("Authorization") {
            assert!(auth_obj.meta.properties.contains_key("referenced-element"));
        }
        
        // Verify header handling
        if let Some(Element::Object(content_obj)) = headers.get_header("Content-Type") {
            assert!(content_obj.meta.properties.contains_key("headerName"));
        }
        
        // Verify fallback handling
        assert!(headers.object.meta.properties.contains_key("fallback_Unknown"));
    }

    #[test]
    fn test_encoding_headers_utilities() {
        let mut obj = ObjectElement::new();
        obj.set("X-Rate-Limit", Element::Object(ObjectElement::new()));
        obj.set("X-Auth-Token", Element::Object(ObjectElement::new()));
        obj.set("Content-Type", Element::Object(ObjectElement::new()));

        let headers = build_encoding_headers(&Element::Object(obj)).unwrap();
        
        // Test utility methods
        assert_eq!(headers.header_count(), 3);
        assert!(headers.has_header("X-Rate-Limit"));
        assert!(headers.has_header("X-Auth-Token"));
        assert!(headers.has_header("Content-Type"));
        assert!(!headers.has_header("X-Missing"));
        
        let names = headers.header_names();
        assert!(names.contains(&"X-Rate-Limit".to_string()));
        assert!(names.contains(&"X-Auth-Token".to_string()));
        assert!(names.contains(&"Content-Type".to_string()));
        
        // Test iterator
        let headers_vec: Vec<_> = headers.headers().collect();
        assert_eq!(headers_vec.len(), 3);
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // Comprehensive test demonstrating TypeScript HeadersVisitor equivalence
        let mut obj = ObjectElement::new();
        
        // Reference header (isReferenceElement predicate)
        let mut ref_header = ObjectElement::new();
        ref_header.set("$ref", Element::String(StringElement::new("#/components/headers/StandardAuth")));
        obj.set("Authorization", Element::Object(ref_header));
        
        // Header element (isHeaderElement predicate)
        let mut rate_limit_header = ObjectElement::new();
        rate_limit_header.set_element_type("header");
        rate_limit_header.set("description", Element::String(StringElement::new("Rate limit remaining")));
        rate_limit_header.set("required", Element::Boolean(BooleanElement::new(false)));
        rate_limit_header.set("schema", Element::Object(ObjectElement::new()));
        obj.set("X-Rate-Limit-Remaining", Element::Object(rate_limit_header));
        
        // Another header element
        let mut custom_header = ObjectElement::new();
        custom_header.set("description", Element::String(StringElement::new("Custom tracking header")));
        custom_header.set("style", Element::String(StringElement::new("simple")));
        obj.set("X-Tracking-ID", Element::Object(custom_header));

        let mut folder = DefaultFolder;
        let headers = build_and_decorate_encoding_headers(&Element::Object(obj), Some(&mut folder));
        assert!(headers.is_some());
        
        let headers = headers.unwrap();
        
        // Verify all TypeScript HeadersVisitor features are implemented:
        
        // 1. MapVisitor functionality - all headers processed
        assert_eq!(headers.header_count(), 3);
        assert!(headers.has_header("Authorization"));
        assert!(headers.has_header("X-Rate-Limit-Remaining"));
        assert!(headers.has_header("X-Tracking-ID"));
        
        // 2. Reference decoration with referenced-element metadata
        if let Some(Element::Object(auth_obj)) = headers.get_header("Authorization") {
            assert!(auth_obj.meta.properties.contains_key("referenced-element"));
            if let Some(SimpleValue::String(ref_type)) = auth_obj.meta.properties.get("referenced-element") {
                assert_eq!(ref_type, "header");
            }
            assert!(auth_obj.meta.properties.contains_key("specPath"));
        }
        
        // 3. Header decoration with headerName metadata
        if let Some(Element::Object(rate_obj)) = headers.get_header("X-Rate-Limit-Remaining") {
            assert!(rate_obj.meta.properties.contains_key("headerName"));
            if let Some(SimpleValue::String(name)) = rate_obj.meta.properties.get("headerName") {
                assert_eq!(name, "X-Rate-Limit-Remaining");
            }
        }
        
        if let Some(Element::Object(tracking_obj)) = headers.get_header("X-Tracking-ID") {
            assert!(tracking_obj.meta.properties.contains_key("headerName"));
            if let Some(SimpleValue::String(name)) = tracking_obj.meta.properties.get("headerName") {
                assert_eq!(name, "X-Tracking-ID");
            }
        }
        
        // 4. Processing metadata
        assert!(headers.object.meta.properties.contains_key("processed"));
        assert!(headers.object.meta.properties.contains_key("mapVisitor"));
        
        // 5. Correct element type and class
        assert_eq!(headers.object.element, "encodingHeaders");
        assert!(headers.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "encoding-headers"
            } else {
                false
            }
        }));
    }
} 