use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::response::{ResponseElement, ResponseContentElement, ResponseHeadersElement, ResponseLinksElement};

/// 构建 OpenAPI ResponseElement（从 Minim Object 转换）
///
/// 例如：
/// {
///   "description": "Success",
///   "content": {
///     "application/json": {
///       "schema": { ... }
///     }
///   }
/// }
pub fn build_response(element: &Element) -> Option<ResponseElement> {
    let object = element.as_object()?;
    build_and_decorate_response(object.clone(), None::<&mut crate::fold::OpenApiBuilderFolder>)
}

/// Enhanced Response builder with TypeScript ResponseVisitor equivalence
/// Provides fixed field processing, content/headers/links visitors, and metadata injection
pub fn build_and_decorate_response<F>(
    mut obj: ObjectElement,
    mut folder: Option<&mut F>
) -> Option<ResponseElement>
where
    F: Fold,
{
    // Set element type and base classes
    obj.set_element_type("response");
    obj.add_class("openapi-response");
    
    // Create response element
    let mut response = ResponseElement::with_content(obj.clone());
    
    // Process all fields with structured validation (FixedFieldsVisitor equivalent)
    process_response_fields(&mut response, &obj, folder.as_deref_mut());
    
    // Inject comprehensive metadata
    inject_response_metadata(&mut response.object, &obj);
    
    Some(response)
}

/// Process response fields with visitor patterns
fn process_response_fields<F>(response: &mut ResponseElement, source: &ObjectElement, mut folder: Option<&mut F>)
where
    F: Fold,
{
    // Process fixed fields
    process_fixed_fields(response, source, folder.as_deref_mut());
    
    // Process content field with ContentVisitor pattern
    process_content_field(response, source, folder.as_deref_mut());
    
    // Process headers field with HeadersVisitor pattern
    process_headers_field(response, source, folder.as_deref_mut());
    
    // Process links field with LinksVisitor pattern
    process_links_field(response, source, folder.as_deref_mut());
    
    // Handle specification extensions
    process_specification_extensions(response, source);
    
    // Process fallback fields (preserve unknown fields)
    process_fallback_fields(response, source);
}

/// Process fixed fields: description
fn process_fixed_fields<F>(response: &mut ResponseElement, source: &ObjectElement, mut folder: Option<&mut F>)
where
    F: Fold,
{
    let fixed_fields = ["description"];
    
    for field_name in &fixed_fields {
        if let Some(field_value) = source.get(field_name) {
            let processed_value = if let Some(ref mut f) = folder {
                f.fold_element(field_value.clone())
            } else {
                field_value.clone()
            };
            
            // Copy field to response element
            response.object.set(field_name, processed_value);
            
            // Add fixed field metadata
            add_fixed_field_metadata(&mut response.object, field_name);
        }
    }
}

/// Process content field with ContentVisitor pattern
/// Equivalent to TypeScript ContentVisitor with media-type metadata injection
fn process_content_field<F>(response: &mut ResponseElement, source: &ObjectElement, mut folder: Option<&mut F>)
where
    F: Fold,
{
    if let Some(content_element) = source.get("content") {
        if let Some(content_obj) = content_element.as_object() {
            let mut response_content = ResponseContentElement::with_content(content_obj.clone());
            
            // Process each media type with metadata injection (ContentVisitor equivalent)
            for member in &mut response_content.object.content {
                if let Element::String(media_type_key) = &*member.key {
                    let media_type_name = &media_type_key.content;
                    
                    // Fold the media type element
                    let processed_value = if let Some(ref mut f) = folder {
                        f.fold_element((*member.value).clone())
                    } else {
                        (*member.value).clone()
                    };
                    
                    // Check if it's an object before modifying
                    let is_object = matches!(*member.value, Element::Object(_));
                    
                    // Update the member value first
                    *member.value = processed_value;
                    
                    // Then inject metadata if it's an object
                    if is_object {
                        if let Element::Object(ref mut media_type_obj) = *member.value {
                            media_type_obj.meta.properties.insert(
                                "media-type".to_string(),
                                Value::String(media_type_name.clone())
                            );
                            media_type_obj.add_class("media-type");
                            
                            // Add MediaType element metadata
                            add_media_type_metadata(media_type_obj, media_type_name);
                        }
                    }
                }
            }
            
            // Set structured content element
            response.set_response_content(response_content);
            add_fixed_field_metadata(&mut response.object, "content");
            add_content_processing_metadata(&mut response.object);
        }
    }
}

/// Process headers field with HeadersVisitor pattern
/// Equivalent to TypeScript HeadersVisitor with header-name and reference metadata injection
fn process_headers_field<F>(response: &mut ResponseElement, source: &ObjectElement, mut folder: Option<&mut F>)
where
    F: Fold,
{
    if let Some(headers_element) = source.get("headers") {
        if let Some(headers_obj) = headers_element.as_object() {
            let mut response_headers = ResponseHeadersElement::with_content(headers_obj.clone());
            
            // Process each header with metadata injection (HeadersVisitor equivalent)
            for member in &mut response_headers.object.content {
                if let Element::String(header_key) = &*member.key {
                    let header_name = &header_key.content;
                    
                    // Fold the header element
                    let processed_value = if let Some(ref mut f) = folder {
                        f.fold_element((*member.value).clone())
                    } else {
                        (*member.value).clone()
                    };
                    
                    // Check element type and reference status before modifying
                    let is_object = matches!(*member.value, Element::Object(_));
                    let is_reference = is_reference_like_element(&*member.value);
                    let is_header = is_header_element(&*member.value);
                    
                    // Update the member value first
                    *member.value = processed_value;
                    
                    // Then inject metadata if it's an object
                    if is_object {
                        if let Element::Object(ref mut header_obj) = *member.value {
                            // Add header-name metadata (equivalent to TypeScript setMetaProperty)
                            header_obj.meta.properties.insert(
                                "header-name".to_string(),
                                Value::String(header_name.clone())
                            );
                            
                            // Check for reference and add referenced-element metadata
                            if is_reference {
                                header_obj.meta.properties.insert(
                                    "referenced-element".to_string(),
                                    Value::String("header".to_string())
                                );
                                header_obj.add_class("reference-element");
                            } else if is_header {
                                header_obj.add_class("header-element");
                            }
                            
                            add_header_metadata(header_obj, header_name);
                        }
                    }
                }
            }
            
            // Set structured headers element
            response.set_response_headers(response_headers);
            add_fixed_field_metadata(&mut response.object, "headers");
            add_headers_processing_metadata(&mut response.object);
        }
    }
}

/// Process links field with LinksVisitor pattern
/// Equivalent to TypeScript LinksVisitor with reference metadata injection
fn process_links_field<F>(response: &mut ResponseElement, source: &ObjectElement, mut folder: Option<&mut F>)
where
    F: Fold,
{
    if let Some(links_element) = source.get("links") {
        if let Some(links_obj) = links_element.as_object() {
            let mut response_links = ResponseLinksElement::with_content(links_obj.clone());
            
            // Process each link with metadata injection (LinksVisitor equivalent)
            for member in &mut response_links.object.content {
                if let Element::String(link_key) = &*member.key {
                    let link_name = &link_key.content;
                    
                    // Fold the link element
                    let processed_value = if let Some(ref mut f) = folder {
                        f.fold_element((*member.value).clone())
                    } else {
                        (*member.value).clone()
                    };
                    
                    // Check element type and reference status before modifying
                    let is_object = matches!(*member.value, Element::Object(_));
                    let is_reference = is_reference_like_element(&*member.value);
                    let is_link = is_link_element(&*member.value);
                    
                    // Update the member value first
                    *member.value = processed_value;
                    
                    // Then inject metadata if it's an object
                    if is_object {
                        if let Element::Object(ref mut link_obj) = *member.value {
                            // Check for reference and add referenced-element metadata
                            if is_reference {
                                link_obj.meta.properties.insert(
                                    "referenced-element".to_string(),
                                    Value::String("link".to_string())
                                );
                                link_obj.add_class("reference-element");
                            } else if is_link {
                                link_obj.add_class("link-element");
                            }
                            
                            add_link_metadata(link_obj, link_name);
                        }
                    }
                }
            }
            
            // Set structured links element
            response.set_response_links(response_links);
            add_fixed_field_metadata(&mut response.object, "links");
            add_links_processing_metadata(&mut response.object);
        }
    }
}

/// Handle specification extensions (x-* fields)
fn process_specification_extensions(response: &mut ResponseElement, source: &ObjectElement) {
    let mut spec_extensions = Vec::new();
    
    for member in &source.content {
        if let Element::String(key_str) = &*member.key {
            if key_str.content.starts_with("x-") {
                spec_extensions.push(key_str.content.clone());
                // Copy extension to response
                response.object.set(&key_str.content, (*member.value).clone());
            }
        }
    }
    
    if !spec_extensions.is_empty() {
        // Add specification extensions metadata
        response.object.meta.properties.insert(
            "specification-extensions".to_string(),
            Value::Array(spec_extensions.into_iter().map(Value::String).collect())
        );
        response.object.add_class("specification-extension");
    }
}

/// Process fallback fields (preserve unknown fields for compatibility)
fn process_fallback_fields(response: &mut ResponseElement, source: &ObjectElement) {
    let known_fields = ["description", "headers", "content", "links"];
    
    for member in &source.content {
        if let Element::String(key_str) = &*member.key {
            let field_name = &key_str.content;
            
            // Skip known fields and spec extensions (already handled)
            if !known_fields.contains(&field_name.as_str()) && !field_name.starts_with("x-") {
                // Add as fallback field
                response.object.set(field_name, (*member.value).clone());
                add_fallback_field_metadata(&mut response.object, field_name);
            }
        }
    }
}

/// Inject comprehensive metadata for response
fn inject_response_metadata(obj: &mut ObjectElement, source: &ObjectElement) {
    // Add element type metadata
    obj.meta.properties.insert(
        "element-type".to_string(),
        Value::String("response".to_string())
    );
    
    // Add spec path metadata (equivalent to TypeScript specPath)
    obj.meta.properties.insert(
        "spec-path".to_string(),
        Value::Array(vec![
            Value::String("document".to_string()),
            Value::String("objects".to_string()),
            Value::String("Response".to_string())
        ])
    );
    
    // Add field count metadata
    obj.meta.properties.insert(
        "field-count".to_string(),
        Value::from(source.content.len())
    );
    
    // Add specification extensions support flag
    obj.meta.properties.insert(
        "can-support-specification-extensions".to_string(),
        Value::Bool(true)
    );
    
    // Add processing timestamp
    obj.meta.properties.insert(
        "processed-at".to_string(),
        Value::String(chrono::Utc::now().to_rfc3339())
    );
    
    // Add visitor information
    obj.meta.properties.insert(
        "processed-by".to_string(),
        Value::String("ResponseVisitor".to_string())
    );
}

// Helper functions for element type checking

/// Check if element is reference-like (has $ref)
fn is_reference_like_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.get("$ref").is_some()
    } else {
        false
    }
}

/// Check if element is a header element
fn is_header_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.element == "header" || 
        // Check for typical header fields
        obj.has_key("description") || obj.has_key("required") || 
        obj.has_key("deprecated") || obj.has_key("style") ||
        obj.has_key("explode") || obj.has_key("schema") || obj.has_key("content")
    } else {
        false
    }
}

/// Check if element is a link element
fn is_link_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.element == "link" ||
        // Check for typical link fields
        obj.has_key("operationRef") || obj.has_key("operationId") ||
        obj.has_key("parameters") || obj.has_key("requestBody") ||
        obj.has_key("description") || obj.has_key("server")
    } else {
        false
    }
}

// Metadata helper functions

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fixed-field-{}", field_name),
        Value::Bool(true)
    );
}

/// Add metadata for media types
fn add_media_type_metadata(obj: &mut ObjectElement, media_type: &str) {
    obj.meta.properties.insert(
        "media-type-name".to_string(),
        Value::String(media_type.to_string())
    );
    obj.meta.properties.insert(
        "is-media-type".to_string(),
        Value::Bool(true)
    );
}

/// Add metadata for headers
fn add_header_metadata(obj: &mut ObjectElement, header_name: &str) {
    obj.meta.properties.insert(
        "header-name-metadata".to_string(),
        Value::String(header_name.to_string())
    );
    obj.meta.properties.insert(
        "is-header".to_string(),
        Value::Bool(true)
    );
}

/// Add metadata for links
fn add_link_metadata(obj: &mut ObjectElement, link_name: &str) {
    obj.meta.properties.insert(
        "link-name".to_string(),
        Value::String(link_name.to_string())
    );
    obj.meta.properties.insert(
        "is-link".to_string(),
        Value::Bool(true)
    );
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        Value::Bool(true)
    );
}

/// Add metadata for content processing
fn add_content_processing_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "content-processed".to_string(),
        Value::Bool(true)
    );
    obj.meta.properties.insert(
        "content-visitor".to_string(),
        Value::Bool(true)
    );
}

/// Add metadata for headers processing
fn add_headers_processing_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "headers-processed".to_string(),
        Value::Bool(true)
    );
    obj.meta.properties.insert(
        "headers-visitor".to_string(),
        Value::Bool(true)
    );
}

/// Add metadata for links processing
fn add_links_processing_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "links-processed".to_string(),
        Value::Bool(true)
    );
    obj.meta.properties.insert(
        "links-visitor".to_string(),
        Value::Bool(true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fold::OpenApiBuilderFolder;
    use serde_json::json;

    fn create_test_object(json_val: serde_json::Value) -> ObjectElement {
        let mut obj = ObjectElement::new();
        if let serde_json::Value::Object(map) = json_val {
            for (key, value) in map {
                let element = json_value_to_element(value);
                obj.set(&key, element);
            }
        }
        obj
    }

    fn json_value_to_element(value: serde_json::Value) -> Element {
        match value {
            serde_json::Value::String(s) => Element::String(StringElement::new(&s)),
            serde_json::Value::Bool(b) => Element::Boolean(BooleanElement::new(b)),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Element::Number(NumberElement { 
                        element: "number".to_string(),
                        meta: MetaElement::default(),
                        attributes: AttributesElement::default(),
                        content: f 
                    })
                } else {
                    Element::String(StringElement::new(&n.to_string()))
                }
            },
            serde_json::Value::Array(arr) => {
                let mut array = ArrayElement::new_empty();
                for item in arr {
                    array.content.push(json_value_to_element(item));
                }
                Element::Array(array)
            }
            serde_json::Value::Object(map) => {
                let mut obj = ObjectElement::new();
                for (key, value) in map {
                    obj.set(&key, json_value_to_element(value));
                }
                Element::Object(obj)
            }
            serde_json::Value::Null => Element::Null(NullElement::default()),
        }
    }

    #[test]
    fn test_build_response_basic() {
        let obj = create_test_object(json!({
            "description": "Successful response"
        }));

        let response = build_and_decorate_response(obj, None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check basic structure
        assert_eq!(response.object.element, "response");
        
        // Check description field
        assert!(response.description().is_some());
        assert_eq!(response.description().unwrap().content, "Successful response");
        
        // Check classes
        let classes: Vec<String> = response.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(classes.contains(&"openapi-response".to_string()));
    }

    #[test]
    fn test_build_response_with_content() {
        let obj = create_test_object(json!({
            "description": "User data",
            "content": {
                "application/json": {
                    "schema": {
                        "type": "object"
                    }
                },
                "application/xml": {
                    "schema": {
                        "type": "object"
                    }
                }
            }
        }));

        let mut folder = OpenApiBuilderFolder::new();
        let response = build_and_decorate_response(obj, Some(&mut folder)).unwrap();

        // Check content processing
        assert!(response.content_prop().is_some());
        let content = response.content_prop().unwrap();
        assert_eq!(content.element, "responseContent");

        // Check media-type metadata injection
        if let Some(Element::Object(json_mt)) = content.get("application/json") {
            assert_eq!(
                json_mt.meta.properties.get("media-type"),
                Some(&Value::String("application/json".to_string()))
            );
            assert_eq!(
                json_mt.meta.properties.get("media-type-name"),
                Some(&Value::String("application/json".to_string()))
            );
            assert_eq!(
                json_mt.meta.properties.get("is-media-type"),
                Some(&Value::Bool(true))
            );
        }

        if let Some(Element::Object(xml_mt)) = content.get("application/xml") {
            assert_eq!(
                xml_mt.meta.properties.get("media-type"),
                Some(&Value::String("application/xml".to_string()))
            );
        }

        // Check processing metadata
        assert_eq!(
            response.object.meta.properties.get("fixed-field-content"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            response.object.meta.properties.get("content-processed"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            response.object.meta.properties.get("content-visitor"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn test_build_response_with_headers() {
        let obj = create_test_object(json!({
            "description": "Response with headers",
            "headers": {
                "X-Rate-Limit": {
                    "description": "Rate limit info",
                    "schema": {
                        "type": "integer"
                    }
                },
                "Authorization": {
                    "$ref": "#/components/headers/AuthHeader"
                }
            }
        }));

        let mut folder = OpenApiBuilderFolder::new();
        let response = build_and_decorate_response(obj, Some(&mut folder)).unwrap();

        // Check headers processing
        assert!(response.headers().is_some());
        let headers = response.headers().unwrap();
        assert_eq!(headers.element, "responseHeaders");

        // Check header-name metadata injection
        if let Some(Element::Object(rate_limit_header)) = headers.get("X-Rate-Limit") {
            assert_eq!(
                rate_limit_header.meta.properties.get("header-name"),
                Some(&Value::String("X-Rate-Limit".to_string()))
            );
            assert_eq!(
                rate_limit_header.meta.properties.get("header-name-metadata"),
                Some(&Value::String("X-Rate-Limit".to_string()))
            );
            assert_eq!(
                rate_limit_header.meta.properties.get("is-header"),
                Some(&Value::Bool(true))
            );
        }

        // Check reference metadata injection
        if let Some(Element::Object(auth_header)) = headers.get("Authorization") {
            assert_eq!(
                auth_header.meta.properties.get("header-name"),
                Some(&Value::String("Authorization".to_string()))
            );
            assert_eq!(
                auth_header.meta.properties.get("referenced-element"),
                Some(&Value::String("header".to_string()))
            );
        }

        // Check processing metadata
        assert_eq!(
            response.object.meta.properties.get("fixed-field-headers"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            response.object.meta.properties.get("headers-processed"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            response.object.meta.properties.get("headers-visitor"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn test_build_response_with_links() {
        let obj = create_test_object(json!({
            "description": "Response with links",
            "links": {
                "GetUserByName": {
                    "operationId": "getUserByName",
                    "parameters": {
                        "username": "$response.body#/username"
                    }
                },
                "UserLink": {
                    "$ref": "#/components/links/UserLink"
                }
            }
        }));

        let mut folder = OpenApiBuilderFolder::new();
        let response = build_and_decorate_response(obj, Some(&mut folder)).unwrap();

        // Check links processing
        assert!(response.links().is_some());
        let links = response.links().unwrap();
        assert_eq!(links.element, "responseLinks");

        // Check link metadata injection
        if let Some(Element::Object(get_user_link)) = links.get("GetUserByName") {
            assert_eq!(
                get_user_link.meta.properties.get("link-name"),
                Some(&Value::String("GetUserByName".to_string()))
            );
            assert_eq!(
                get_user_link.meta.properties.get("is-link"),
                Some(&Value::Bool(true))
            );
        }

        // Check reference metadata injection
        if let Some(Element::Object(user_link)) = links.get("UserLink") {
            assert_eq!(
                user_link.meta.properties.get("link-name"),
                Some(&Value::String("UserLink".to_string()))
            );
            assert_eq!(
                user_link.meta.properties.get("referenced-element"),
                Some(&Value::String("link".to_string()))
            );
        }

        // Check processing metadata
        assert_eq!(
            response.object.meta.properties.get("fixed-field-links"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            response.object.meta.properties.get("links-processed"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            response.object.meta.properties.get("links-visitor"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn test_build_response_with_spec_extensions() {
        let obj = create_test_object(json!({
            "description": "Response with extensions",
            "x-custom-field": "custom-value",
            "x-another-extension": "another-value"
        }));

        let response = build_and_decorate_response(obj, None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check specification extensions are preserved
        assert!(response.object.get("x-custom-field").is_some());
        assert!(response.object.get("x-another-extension").is_some());

        // Check specification extensions metadata
        assert!(response.object.meta.properties.contains_key("specification-extensions"));
        if let Some(Value::Array(extensions)) = response.object.meta.properties.get("specification-extensions") {
            assert_eq!(extensions.len(), 2);
            assert!(extensions.contains(&Value::String("x-custom-field".to_string())));
            assert!(extensions.contains(&Value::String("x-another-extension".to_string())));
        }

        // Check specification extension class
        let classes: Vec<String> = response.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(classes.contains(&"specification-extension".to_string()));
    }

    #[test]
    fn test_build_response_with_fallback_fields() {
        let obj = create_test_object(json!({
            "description": "Response with unknown fields",
            "unknownField": "unknown-value",
            "anotherUnknown": "another-value"
        }));

        let response = build_and_decorate_response(obj, None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check fallback fields are preserved
        assert!(response.object.get("unknownField").is_some());
        assert!(response.object.get("anotherUnknown").is_some());

        // Check fallback field metadata
        assert_eq!(
            response.object.meta.properties.get("fallback-field-unknownField"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            response.object.meta.properties.get("fallback-field-anotherUnknown"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn test_build_response_comprehensive_metadata() {
        let obj = create_test_object(json!({
            "description": "Comprehensive response",
            "content": {
                "application/json": {}
            }
        }));

        let response = build_and_decorate_response(obj, None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check comprehensive metadata
        assert_eq!(
            response.object.meta.properties.get("element-type"),
            Some(&Value::String("response".to_string()))
        );
        
        assert_eq!(
            response.object.meta.properties.get("can-support-specification-extensions"),
            Some(&Value::Bool(true))
        );
        
        assert_eq!(
            response.object.meta.properties.get("processed-by"),
            Some(&Value::String("ResponseVisitor".to_string()))
        );
        
        // Check spec path
        if let Some(Value::Array(spec_path)) = response.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], Value::String("document".to_string()));
            assert_eq!(spec_path[1], Value::String("objects".to_string()));
            assert_eq!(spec_path[2], Value::String("Response".to_string()));
        }

        assert!(response.object.meta.properties.contains_key("field-count"));
        assert!(response.object.meta.properties.contains_key("processed-at"));
    }

    #[test]
    fn test_build_response_typescript_equivalence() {
        // This test verifies full TypeScript equivalence
        let obj = create_test_object(json!({
            "description": "User response",
            "content": {
                "application/json": {
                    "schema": {"type": "object"}
                }
            },
            "headers": {
                "X-Rate-Limit": {
                    "description": "Rate limit",
                    "schema": {"type": "integer"}
                },
                "Authorization": {
                    "$ref": "#/components/headers/Auth"
                }
            },
            "links": {
                "GetUser": {
                    "operationId": "getUser"
                },
                "UserRef": {
                    "$ref": "#/components/links/User"
                }
            }
        }));

        let mut folder = OpenApiBuilderFolder::new();
        let response = build_and_decorate_response(obj, Some(&mut folder)).unwrap();

        // 1. Verify ResponseElement structure (equivalent to TypeScript ResponseElement)
        assert_eq!(response.object.element, "response");
        
        // 2. Verify FixedFieldsVisitor behavior (fixed fields processing)
        assert!(response.object.get("description").is_some());
        assert_eq!(
            response.object.meta.properties.get("fixed-field-description"),
            Some(&Value::Bool(true))
        );
        
        // 3. Verify ContentVisitor behavior (media-type metadata injection)
        if let Some(content) = response.content_prop() {
            if let Some(Element::Object(json_mt)) = content.get("application/json") {
                assert_eq!(
                    json_mt.meta.properties.get("media-type"),
                    Some(&Value::String("application/json".to_string()))
                );
            }
        }
        
        // 4. Verify HeadersVisitor behavior (header-name and reference metadata)
        if let Some(headers) = response.headers() {
            if let Some(Element::Object(rate_limit)) = headers.get("X-Rate-Limit") {
                assert_eq!(
                    rate_limit.meta.properties.get("header-name"),
                    Some(&Value::String("X-Rate-Limit".to_string()))
                );
            }
            if let Some(Element::Object(auth)) = headers.get("Authorization") {
                assert_eq!(
                    auth.meta.properties.get("referenced-element"),
                    Some(&Value::String("header".to_string()))
                );
            }
        }
        
        // 5. Verify LinksVisitor behavior (reference metadata)
        if let Some(links) = response.links() {
            if let Some(Element::Object(user_ref)) = links.get("UserRef") {
                assert_eq!(
                    user_ref.meta.properties.get("referenced-element"),
                    Some(&Value::String("link".to_string()))
                );
            }
        }
        
        // 6. Verify specPath metadata
        if let Some(Value::Array(spec_path)) = response.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path, &vec![
                Value::String("document".to_string()),
                Value::String("objects".to_string()),
                Value::String("Response".to_string())
            ]);
        }
        
        // 7. Verify comprehensive metadata injection
        assert!(response.object.meta.properties.len() >= 10);
        assert!(response.object.meta.properties.contains_key("element-type"));
        assert!(response.object.meta.properties.contains_key("processed-by"));
        assert!(response.object.meta.properties.contains_key("content-visitor"));
        assert!(response.object.meta.properties.contains_key("headers-visitor"));
        assert!(response.object.meta.properties.contains_key("links-visitor"));
    }

    #[test]
    fn test_build_response_empty() {
        let obj = create_test_object(json!({
            "description": "Empty response"
        }));

        let response = build_and_decorate_response(obj, None::<&mut OpenApiBuilderFolder>).unwrap();

        // Should still create valid response element
        assert_eq!(response.object.element, "response");
        
        // Should have base classes
        let classes: Vec<String> = response.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(classes.contains(&"openapi-response".to_string()));
        
        // Should have basic metadata
        assert!(response.object.meta.properties.contains_key("element-type"));
        assert!(response.object.meta.properties.contains_key("field-count"));
        assert!(response.object.meta.properties.contains_key("processed-by"));
    }
}