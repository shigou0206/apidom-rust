use apidom_ast::*;
use crate::elements::server_variable::ServerVariableElement;
use serde_json::Value;

/// Comprehensive OpenAPI ServerVariable Builder
/// 
/// This module provides server variable construction with full TypeScript ServerVariableVisitor equivalence.
/// 
/// Features supported:
/// - Fixed fields support (enum, default, description)
/// - Specification extensions (x-*) with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion between element types
/// - Comprehensive metadata injection
/// - Element classification and spec path metadata
/// - Validation with early rejection of invalid structures

/// Build a basic ServerVariableElement from a generic Element
pub fn build_server_variable(element: &Element) -> Option<ServerVariableElement> {
    let object = element.as_object()?.clone();
    let server_var = ServerVariableElement::with_content(object);

    // Validate default field exists (OpenAPI requirement)
    if server_var.default_value().is_none() {
        return None; // Invalid ServerVariable (default is required in OpenAPI spec)
    }

    Some(server_var)
}

/// Build and decorate ServerVariableElement with enhanced visitor pattern features
pub fn build_and_decorate_server_variable<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<ServerVariableElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut server_var = ServerVariableElement::new();
    
    // Process object members with fixed field support and fallback handling
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            let key_str = &key.content;
            let value = member.value.as_ref();
            
            match key_str.as_str() {
                // Fixed fields - direct mapping with type conversion
                "enum" => {
                    if let Some(converted) = convert_to_array_element(value) {
                        server_var.object.set(key_str, Element::Array(converted));
                        add_fixed_field_metadata(&mut server_var.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        server_var.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut server_var.object, key_str);
                    }
                }
                "default" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        server_var.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut server_var.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        server_var.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut server_var.object, key_str);
                    }
                }
                "description" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        server_var.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut server_var.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        server_var.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut server_var.object, key_str);
                    }
                }
                "$ref" => {
                    // Handle $ref with reference metadata
                    if let Element::String(ref_str) = value {
                        server_var.object.set("$ref", value.clone());
                        add_reference_metadata(&mut server_var.object, &ref_str.content, "serverVariable");
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
                        server_var.object.set(key_str, processed_value);
                        add_specification_extension_metadata(&mut server_var.object, key_str);
                    } else {
                        // Fallback field - preserve unknown fields
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        server_var.object.set(key_str, processed_value);
                        add_fallback_field_metadata(&mut server_var.object, key_str);
                    }
                }
            }
        }
    }
    
    // Add element class metadata
    server_var.object.add_class("server-variable");
    server_var.object.meta.properties.insert(
        "element-type".to_string(),
        SimpleValue::String("serverVariable".to_string())
    );
    
    // Add spec path metadata (equivalent to TypeScript specPath)
    add_spec_path_metadata(&mut server_var.object);
    
    // Validate server variable structure
    validate_server_variable(&server_var)?;
    
    Some(server_var)
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

/// Convert element to ArrayElement with type safety
fn convert_to_array_element(element: &Element) -> Option<ArrayElement> {
    match element {
        Element::Array(a) => Some(a.clone()),
        Element::String(s) => {
            // Convert single string to array
            let mut arr = ArrayElement::new_empty();
            arr.content.push(Element::String(s.clone()));
            Some(arr)
        }
        _ => None,
    }
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fixed-field-{}", field_name),
        SimpleValue::Bool(true)
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.add_class("specification-extension");
    obj.meta.properties.insert(
        "specification-extension".to_string(),
        SimpleValue::String(field_name.to_string())
    );
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        SimpleValue::Bool(true)
    );
}

/// Add metadata for $ref references
fn add_reference_metadata(obj: &mut ObjectElement, ref_path: &str, element_type: &str) {
    obj.add_class("reference");
    obj.meta.properties.insert(
        "referenced-element".to_string(),
        SimpleValue::String(element_type.to_string())
    );
    obj.meta.properties.insert(
        "reference-path".to_string(),
        SimpleValue::String(ref_path.to_string())
    );
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "spec-path".to_string(),
        SimpleValue::Array(vec![
            SimpleValue::String("document".to_string()),
            SimpleValue::String("objects".to_string()),
            SimpleValue::String("ServerVariable".to_string()),
        ])
    );
}

/// Validate server variable structure
fn validate_server_variable(server_var: &ServerVariableElement) -> Option<()> {
    // If this is a $ref server variable, skip standard validation
    if server_var.object.get("$ref").is_some() {
        return Some(()); // $ref server variables are valid without other required fields
    }
    
    // Check required fields for non-reference server variables
    if server_var.default_value().is_none() {
        return None; // default is required for non-reference server variables
    }
    
    // Additional validation can be added here
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_server_variable_builder() {
        let mut obj = ObjectElement::new();
        obj.set("default", Element::String(StringElement::new("localhost")));
        obj.set("description", Element::String(StringElement::new("Server hostname")));
        
        let result = build_server_variable(&Element::Object(obj));
        
        assert!(result.is_some());
        let server_var = result.unwrap();
        assert_eq!(server_var.object.element, "serverVariable");
        assert!(server_var.default_value().is_some());
        assert!(server_var.description().is_some());
    }

    #[test]
    fn test_server_variable_missing_default() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Server hostname")));
        
        let result = build_server_variable(&Element::Object(obj));
        
        assert!(result.is_none()); // Should fail validation without default
    }

    #[test]
    fn test_enhanced_server_variable_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("default", Element::String(StringElement::new("localhost")));
        obj.set("description", Element::String(StringElement::new("Server hostname")));
        
        // Add enum values
        let mut enum_arr = ArrayElement::new_empty();
        enum_arr.content.push(Element::String(StringElement::new("localhost")));
        enum_arr.content.push(Element::String(StringElement::new("staging.example.com")));
        enum_arr.content.push(Element::String(StringElement::new("api.example.com")));
        obj.set("enum", Element::Array(enum_arr));
        
        let result = build_and_decorate_server_variable(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let server_var = result.unwrap();
        
        // Verify fixed field metadata
        assert!(server_var.object.meta.properties.contains_key("fixed-field-default"));
        assert!(server_var.object.meta.properties.contains_key("fixed-field-description"));
        assert!(server_var.object.meta.properties.contains_key("fixed-field-enum"));
        
        // Verify element class
        assert!(server_var.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "server-variable"
            } else {
                false
            }
        }));
        
        // Verify spec path metadata
        assert!(server_var.object.meta.properties.contains_key("spec-path"));
        if let Some(SimpleValue::Array(spec_path)) = server_var.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], SimpleValue::String("document".to_string()));
            assert_eq!(spec_path[1], SimpleValue::String("objects".to_string()));
            assert_eq!(spec_path[2], SimpleValue::String("ServerVariable".to_string()));
        }
    }

    #[test]
    fn test_server_variable_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("default", Element::String(StringElement::new("localhost")));
        obj.set("x-internal-name", Element::String(StringElement::new("host-variable")));
        obj.set("x-validation-pattern", Element::String(StringElement::new("^[a-zA-Z0-9.-]+$")));
        
        let result = build_and_decorate_server_variable(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let server_var = result.unwrap();
        
        // Verify specification extension metadata
        assert!(server_var.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(server_var.object.get("x-internal-name").is_some());
        assert!(server_var.object.get("x-validation-pattern").is_some());
    }

    #[test]
    fn test_server_variable_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("default", Element::String(StringElement::new("localhost")));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("anotherField", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        }));
        
        let result = build_and_decorate_server_variable(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let server_var = result.unwrap();
        
        // Verify fallback field metadata
        assert!(server_var.object.meta.properties.contains_key("fallback-field-customField"));
        assert!(server_var.object.meta.properties.contains_key("fallback-field-anotherField"));
        assert!(server_var.object.get("customField").is_some());
        assert!(server_var.object.get("anotherField").is_some());
    }

    #[test]
    fn test_server_variable_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/serverVariables/host")));
        
        let result = build_and_decorate_server_variable(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let server_var = result.unwrap();
        
        // Verify reference metadata
        assert!(server_var.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            server_var.object.meta.properties.get("referenced-element"),
            Some(&SimpleValue::String("serverVariable".to_string()))
        );
        assert_eq!(
            server_var.object.meta.properties.get("reference-path"),
            Some(&SimpleValue::String("#/components/serverVariables/host".to_string()))
        );
    }

    #[test]
    fn test_server_variable_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("default", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 8080.0,
        }));
        obj.set("description", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_server_variable(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let server_var = result.unwrap();
        
        // Verify type conversion worked
        if let Some(Element::String(default_str)) = server_var.object.get("default") {
            assert_eq!(default_str.content, "8080");
        }
        if let Some(Element::String(desc_str)) = server_var.object.get("description") {
            assert_eq!(desc_str.content, "true");
        }
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript ServerVariableVisitor
        let mut obj = ObjectElement::new();
        obj.set("default", Element::String(StringElement::new("api.example.com")));
        obj.set("description", Element::String(StringElement::new("API server hostname")));
        
        // Add enum values
        let mut enum_arr = ArrayElement::new_empty();
        enum_arr.content.push(Element::String(StringElement::new("api.example.com")));
        enum_arr.content.push(Element::String(StringElement::new("staging-api.example.com")));
        enum_arr.content.push(Element::String(StringElement::new("localhost")));
        obj.set("enum", Element::Array(enum_arr));
        
        // Add specification extensions
        obj.set("x-environment", Element::String(StringElement::new("production")));
        obj.set("x-validation-regex", Element::String(StringElement::new("^[a-zA-Z0-9.-]+$")));
        
        // Add fallback field
        obj.set("customMetadata", Element::String(StringElement::new("custom value")));
        
        let result = build_and_decorate_server_variable(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let server_var = result.unwrap();
        
        // Verify TypeScript equivalence features:
        
        // 1. Fixed fields processing
        assert!(server_var.object.meta.properties.contains_key("fixed-field-default"));
        assert!(server_var.object.meta.properties.contains_key("fixed-field-description"));
        assert!(server_var.object.meta.properties.contains_key("fixed-field-enum"));
        
        // 2. Specification extensions
        assert!(server_var.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(server_var.object.get("x-environment").is_some());
        assert!(server_var.object.get("x-validation-regex").is_some());
        
        // 3. Fallback field handling
        assert!(server_var.object.meta.properties.contains_key("fallback-field-customMetadata"));
        assert!(server_var.object.get("customMetadata").is_some());
        
        // 4. Element classification
        assert!(server_var.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "server-variable"
            } else {
                false
            }
        }));
        assert_eq!(
            server_var.object.meta.properties.get("element-type"),
            Some(&SimpleValue::String("serverVariable".to_string()))
        );
        
        // 5. Spec path metadata (equivalent to TypeScript specPath)
        assert!(server_var.object.meta.properties.contains_key("spec-path"));
        if let Some(SimpleValue::Array(spec_path)) = server_var.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], SimpleValue::String("document".to_string()));
            assert_eq!(spec_path[1], SimpleValue::String("objects".to_string()));
            assert_eq!(spec_path[2], SimpleValue::String("ServerVariable".to_string()));
        }
        
        // 6. Proper enum handling
        assert!(server_var.enum_values().is_some());
        if let Some(enum_vals) = server_var.enum_values() {
            assert_eq!(enum_vals.content.len(), 3);
        }
        
        // 7. Required field validation
        assert!(server_var.default_value().is_some());
        if let Some(default_val) = server_var.default_value() {
            assert_eq!(default_val.content, "api.example.com");
        }
    }
}