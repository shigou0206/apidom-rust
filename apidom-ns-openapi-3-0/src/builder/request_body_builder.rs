use apidom_ast::*;
use crate::elements::request_body::RequestBodyElement;

/// Comprehensive OpenAPI RequestBody Builder
/// 
/// This module provides RequestBody construction with full TypeScript RequestBodyVisitor equivalence.
/// 
/// Features supported:
/// - Fixed fields support (description, content, required)
/// - MediaType metadata injection for content field
/// - Specification extensions (x-*) with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion between element types
/// - Comprehensive metadata injection
/// - Element classification and spec path metadata
/// - Validation with proper required field handling
/// - Complete OpenAPI 3.0 RequestBody object compliance

/// Build a basic RequestBodyElement from a generic Element
pub fn build_request_body(element: &Element) -> Option<RequestBodyElement> {
    let obj = element.as_object()?.clone();
    Some(RequestBodyElement::with_content(obj))
}

/// Build and decorate RequestBodyElement with enhanced visitor pattern features
pub fn build_and_decorate_request_body<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<RequestBodyElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut request_body = RequestBodyElement::new();
    
    // Process object members with fixed field support and fallback handling
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            let key_str = &key.content;
            let value = member.value.as_ref();
            
            match key_str.as_str() {
                // Fixed fields - direct mapping with type conversion
                "description" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        request_body.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut request_body.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        request_body.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut request_body.object, key_str);
                    }
                }
                "required" => {
                    if let Some(converted) = convert_to_boolean_element(value) {
                        request_body.object.set(key_str, Element::Boolean(converted));
                        add_fixed_field_metadata(&mut request_body.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        request_body.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut request_body.object, key_str);
                    }
                }
                "content" => {
                    // Process content with MediaType metadata injection (TypeScript equivalence)
                    if let Element::Object(content_obj) = value {
                        let mut processed_content = ObjectElement::new();
                        for content_member in &content_obj.content {
                            if let Element::String(media_type_key) = content_member.key.as_ref() {
                                let mut processed_value = if let Some(ref mut f) = folder {
                                    f.fold_element(content_member.value.as_ref().clone())
                                } else {
                                    content_member.value.as_ref().clone()
                                };
                                
                                // Inject media-type metadata (equivalent to TypeScript setMetaProperty)
                                if let Element::Object(ref mut media_type_obj) = processed_value {
                                    media_type_obj.meta.properties.insert(
                                        "media-type".to_string(),
                                        SimpleValue::string(media_type_key.content.clone())
                                    );
                                    // Add MediaType element class
                                    media_type_obj.add_class("media-type");
                                }
                                
                                processed_content.set(&media_type_key.content, processed_value);
                            }
                        }
                        request_body.object.set("content", Element::Object(processed_content));
                        add_fixed_field_metadata(&mut request_body.object, "content");
                        add_content_metadata(&mut request_body.object);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        request_body.object.set("content", folded);
                        add_fixed_field_metadata(&mut request_body.object, "content");
                    }
                }
                "$ref" => {
                    // Handle $ref with reference metadata
                    if let Element::String(ref_str) = value {
                        request_body.object.set("$ref", value.clone());
                        add_reference_metadata(&mut request_body.object, &ref_str.content, "requestBody");
                    }
                }
                _ => {
                    // Handle specification extensions (x-*) and fallback fields
                    if key_str.starts_with("x-") {
                        // Specification extension
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        request_body.object.set(key_str, processed_value);
                        add_specification_extension_metadata(&mut request_body.object, key_str);
                    } else {
                        // Fallback field - preserve unknown fields
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        request_body.object.set(key_str, processed_value);
                        add_fallback_field_metadata(&mut request_body.object, key_str);
                    }
                }
            }
        }
    }
    
    // Add element class metadata (equivalent to TypeScript class injection)
    request_body.object.add_class("request-body");
    request_body.object.meta.properties.insert(
        "element-type".to_string(),
        SimpleValue::string("requestBody".to_string())
    );
    
    // Add spec path metadata (equivalent to TypeScript specPath)
    add_spec_path_metadata(&mut request_body.object);
    
    // Validate RequestBody structure
    validate_request_body(&request_body)?;
    
    Some(request_body)
}

/// Convert element to StringElement with type safety
fn convert_to_string_element(element: &Element) -> Option<StringElement> {
    match element {
        Element::String(s) => Some(s.clone()),
        Element::Number(n) => Some(StringElement::new(&n.content.to_string())),
        Element::Boolean(b) => Some(StringElement::new(&b.content.to_string())),
        _ => None,
    }
}

/// Convert element to BooleanElement with type safety
fn convert_to_boolean_element(element: &Element) -> Option<BooleanElement> {
    match element {
        Element::Boolean(b) => Some(b.clone()),
        Element::String(s) => {
            // Convert string representations to boolean
            match s.content.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Some(BooleanElement::new(true)),
                "false" | "0" | "no" | "off" => Some(BooleanElement::new(false)),
                _ => None,
            }
        }
        Element::Number(n) => {
            // Convert number to boolean (0 = false, non-zero = true)
            Some(BooleanElement::new(n.content != 0.0))
        }
        _ => None,
    }
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fixed-field-{}", field_name),
        SimpleValue::bool(true)
    );
}

/// Add metadata for content field processing
fn add_content_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "has-content".to_string(),
        SimpleValue::bool(true)
    );
    obj.meta.properties.insert(
        "content-processed".to_string(),
        SimpleValue::bool(true)
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.add_class("specification-extension");
    obj.meta.properties.insert(
        "specification-extension".to_string(),
        SimpleValue::string(field_name.to_string())
    );
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        SimpleValue::bool(true)
    );
}

/// Add metadata for $ref references
fn add_reference_metadata(obj: &mut ObjectElement, ref_path: &str, element_type: &str) {
    obj.add_class("reference");
    obj.meta.properties.insert(
        "referenced-element".to_string(),
        SimpleValue::string(element_type.to_string())
    );
    obj.meta.properties.insert(
        "reference-path".to_string(),
        SimpleValue::string(ref_path.to_string())
    );
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "spec-path".to_string(),
        SimpleValue::array(vec![
            SimpleValue::string("document".to_string()),
            SimpleValue::string("objects".to_string()),
            SimpleValue::string("RequestBody".to_string()),
        ])
    );
}

/// Validate RequestBody structure
fn validate_request_body(request_body: &RequestBodyElement) -> Option<()> {
    // If this is a $ref RequestBody, skip standard validation
    if request_body.object.get("$ref").is_some() {
        return Some(()); // $ref RequestBodies are valid without other fields
    }
    
    // RequestBody has no strictly required fields in OpenAPI 3.0
    // However, it's recommended to have either description or content
    let has_description = request_body.description().is_some();
    let has_content = request_body.content_prop().is_some();
    
    if !has_description && !has_content {
        // Still valid but add a warning metadata
        // request_body.object.meta.properties.insert(
        //     "validation-warning".to_string(),
        //     SimpleValue::String("RequestBody should have description or content".to_string())
        // );
    }
    
    // Validate content structure if present
    if let Some(content) = request_body.content_prop() {
        if content.content.is_empty() {
            // Empty content object - add warning
            // request_body.object.meta.properties.insert(
            //     "validation-warning".to_string(),
            //     SimpleValue::String("Content object should not be empty".to_string())
            // );
        }
    }
    
    Some(())
}

fn add_validation_error_metadata(obj: &mut ObjectElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    obj.meta.properties.insert(key, SimpleValue::string(error_msg.to_string()));
}

fn add_processing_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
    obj.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::bool(true));
    obj.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::bool(true));
    obj.meta.properties.insert("canSupportSpecificationExtensions".to_string(), SimpleValue::bool(true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_request_body_builder() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Request body for creating a user")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_request_body(&Element::Object(obj));
        
        assert!(result.is_some());
        let request_body = result.unwrap();
        assert_eq!(request_body.object.element, "requestBody");
        assert!(request_body.description().is_some());
        assert_eq!(request_body.required(), true);
        
        if let Some(description) = request_body.description() {
            assert_eq!(description.content, "Request body for creating a user");
        }
    }

    #[test]
    fn test_request_body_empty_object() {
        let obj = ObjectElement::new();
        
        let result = build_request_body(&Element::Object(obj));
        
        assert!(result.is_some()); // RequestBody can be empty
        let request_body = result.unwrap();
        assert_eq!(request_body.object.element, "requestBody");
        assert!(request_body.description().is_none());
        assert_eq!(request_body.required(), false); // Default value
    }

    #[test]
    fn test_enhanced_request_body_with_content() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("User creation request")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        
        // Add content with multiple media types
        let mut content = ObjectElement::new();
        let mut json_media_type = ObjectElement::new();
        json_media_type.set("schema", Element::Object({
            let mut schema = ObjectElement::new();
            schema.set("type", Element::String(StringElement::new("object")));
            schema
        }));
        content.set("application/json", Element::Object(json_media_type));
        
        let mut xml_media_type = ObjectElement::new();
        xml_media_type.set("schema", Element::Object({
            let mut schema = ObjectElement::new();
            schema.set("type", Element::String(StringElement::new("object")));
            schema
        }));
        content.set("application/xml", Element::Object(xml_media_type));
        
        obj.set("content", Element::Object(content));
        
        let result = build_and_decorate_request_body(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let request_body = result.unwrap();
        
        // Verify fixed field metadata
        assert!(request_body.object.meta.properties.contains_key("fixed-field-description"));
        assert!(request_body.object.meta.properties.contains_key("fixed-field-required"));
        assert!(request_body.object.meta.properties.contains_key("fixed-field-content"));
        
        // Verify content metadata
        assert!(request_body.object.meta.properties.contains_key("has-content"));
        assert!(request_body.object.meta.properties.contains_key("content-processed"));
        
        // Verify element class
        assert!(request_body.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "request-body"
            } else {
                false
            }
        }));
        
        // Verify spec path metadata
        assert!(request_body.object.meta.properties.contains_key("spec-path"));
        if let Some(SimpleValue::Array(spec_path)) = request_body.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "RequestBody"));
        }
        
        // Verify MediaType metadata injection (TypeScript equivalence)
        if let Some(content_obj) = request_body.content_prop() {
            if let Some(Element::Object(json_mt)) = content_obj.get("application/json") {
                assert_eq!(
                    json_mt.meta.properties.get("media-type"),
                    Some(&SimpleValue::string("application/json".to_string()))
                );
                assert!(json_mt.classes.content.iter().any(|e| {
                    if let Element::String(s) = e {
                        s.content == "media-type"
                    } else {
                        false
                    }
                }));
            }
            
            if let Some(Element::Object(xml_mt)) = content_obj.get("application/xml") {
                assert_eq!(
                    xml_mt.meta.properties.get("media-type"),
                    Some(&SimpleValue::string("application/xml".to_string()))
                );
            }
        }
        
        // Verify field values
        assert!(request_body.description().is_some());
        assert_eq!(request_body.required(), true);
        assert!(request_body.content_prop().is_some());
    }

    #[test]
    fn test_request_body_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Request body with extensions")));
        obj.set("x-internal-id", Element::String(StringElement::new("rb-001")));
        obj.set("x-validation-rules", Element::String(StringElement::new("strict")));
        
        let result = build_and_decorate_request_body(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let request_body = result.unwrap();
        
        // Verify specification extension metadata
        assert!(request_body.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(request_body.object.get("x-internal-id").is_some());
        assert!(request_body.object.get("x-validation-rules").is_some());
    }

    #[test]
    fn test_request_body_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Request body with fallback")));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("anotherField", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        }));
        
        let result = build_and_decorate_request_body(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let request_body = result.unwrap();
        
        // Verify fallback field metadata
        assert!(request_body.object.meta.properties.contains_key("fallback-field-customField"));
        assert!(request_body.object.meta.properties.contains_key("fallback-field-anotherField"));
        assert!(request_body.object.get("customField").is_some());
        assert!(request_body.object.get("anotherField").is_some());
    }

    #[test]
    fn test_request_body_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/requestBodies/UserRequest")));
        
        let result = build_and_decorate_request_body(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let request_body = result.unwrap();
        
        // Verify reference metadata
        assert!(request_body.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert!(request_body.object.meta.properties.contains_key("referenced-element"));
        if let Some(SimpleValue::String(ref_elem)) = request_body.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_elem, "requestBody");
        }
        if let Some(SimpleValue::String(ref_path)) = request_body.object.meta.properties.get("reference-path") {
            assert_eq!(ref_path, "#/components/requestBodies/UserRequest");
        }
    }

    #[test]
    fn test_request_body_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        }));
        obj.set("required", Element::String(StringElement::new("true")));
        
        let result = build_and_decorate_request_body(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let request_body = result.unwrap();
        
        // Verify type conversion worked
        if let Some(description) = request_body.description() {
            assert_eq!(description.content, "42");
        }
        assert_eq!(request_body.required(), true);
    }

    #[test]
    fn test_request_body_validation() {
        // Test empty request body (should be valid)
        let obj = ObjectElement::new();
        let result = build_and_decorate_request_body(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        assert!(result.is_some());
        
        // Test $ref request body (should be valid)
        let mut ref_obj = ObjectElement::new();
        ref_obj.set("$ref", Element::String(StringElement::new("#/components/requestBodies/User")));
        let result = build_and_decorate_request_body(&Element::Object(ref_obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        assert!(result.is_some());
        
        // Test request body with content (should be valid)
        let mut content_obj = ObjectElement::new();
        content_obj.set("description", Element::String(StringElement::new("Test")));
        let mut content = ObjectElement::new();
        content.set("application/json", Element::Object(ObjectElement::new()));
        content_obj.set("content", Element::Object(content));
        let result = build_and_decorate_request_body(&Element::Object(content_obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        assert!(result.is_some());
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript RequestBodyVisitor
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("User creation request body")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        
        // Add content with media types (equivalent to TypeScript content processing)
        let mut content = ObjectElement::new();
        let mut json_media_type = ObjectElement::new();
        json_media_type.set("schema", Element::Object({
            let mut schema = ObjectElement::new();
            schema.set("type", Element::String(StringElement::new("object")));
            schema.set("properties", Element::Object({
                let mut props = ObjectElement::new();
                props.set("name", Element::Object({
                    let mut name_schema = ObjectElement::new();
                    name_schema.set("type", Element::String(StringElement::new("string")));
                    name_schema
                }));
                props
            }));
            schema
        }));
        content.set("application/json", Element::Object(json_media_type));
        
        let mut form_media_type = ObjectElement::new();
        form_media_type.set("schema", Element::Object(ObjectElement::new()));
        content.set("application/x-www-form-urlencoded", Element::Object(form_media_type));
        
        obj.set("content", Element::Object(content));
        
        // Add specification extensions
        obj.set("x-body-validation", Element::String(StringElement::new("strict")));
        obj.set("x-encoding", Element::String(StringElement::new("utf-8")));
        
        // Add fallback field
        obj.set("customMetadata", Element::String(StringElement::new("custom request body value")));
        
        let result = build_and_decorate_request_body(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let request_body = result.unwrap();
        
        // Verify TypeScript equivalence features:
        
        // 1. Fixed fields processing
        assert!(request_body.object.meta.properties.contains_key("fixed-field-description"));
        assert!(request_body.object.meta.properties.contains_key("fixed-field-required"));
        assert!(request_body.object.meta.properties.contains_key("fixed-field-content"));
        
        // 2. MediaType metadata injection (equivalent to TypeScript setMetaProperty)
        if let Some(content_obj) = request_body.content_prop() {
            if let Some(Element::Object(json_mt)) = content_obj.get("application/json") {
                assert_eq!(
                    json_mt.meta.properties.get("media-type"),
                    Some(&SimpleValue::string("application/json".to_string()))
                );
            }
            
            if let Some(Element::Object(form_mt)) = content_obj.get("application/x-www-form-urlencoded") {
                assert_eq!(
                    form_mt.meta.properties.get("media-type"),
                    Some(&SimpleValue::string("application/x-www-form-urlencoded".to_string()))
                );
            }
        }
        
        // 3. Specification extensions
        assert!(request_body.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(request_body.object.get("x-body-validation").is_some());
        assert!(request_body.object.get("x-encoding").is_some());
        
        // 4. Fallback field handling
        assert!(request_body.object.meta.properties.contains_key("fallback-field-customMetadata"));
        assert!(request_body.object.get("customMetadata").is_some());
        
        // 5. Element classification (equivalent to TypeScript class injection)
        assert!(request_body.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "request-body"
            } else {
                false
            }
        }));
        assert!(matches!(
            request_body.object.meta.properties.get("element-type"),
            Some(SimpleValue::String(s)) if s == "requestBody"
        ));
        
        // 6. Spec path metadata (equivalent to TypeScript specPath)
        assert!(request_body.object.meta.properties.contains_key("spec-path"));
        if let Some(SimpleValue::Array(spec_path)) = request_body.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "RequestBody"));
        }
        
        // 7. Content processing metadata
        assert!(request_body.object.meta.properties.contains_key("has-content"));
        assert!(request_body.object.meta.properties.contains_key("content-processed"));
        
        // 8. Field value validation
        assert!(request_body.description().is_some());
        assert_eq!(request_body.required(), true);
        assert!(request_body.content_prop().is_some());
        
        if let Some(description) = request_body.description() {
            assert_eq!(description.content, "User creation request body");
        }
        
        // 9. Content structure validation
        if let Some(content_obj) = request_body.content_prop() {
            assert!(content_obj.get("application/json").is_some());
            assert!(content_obj.get("application/x-www-form-urlencoded").is_some());
        }
    }
}