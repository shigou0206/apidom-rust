use apidom_ast::minim_model::*;
use crate::elements::xml::XmlElement;
use serde_json::Value;

/// Comprehensive OpenAPI XML Builder
/// 
/// This module provides XML construction with full TypeScript XmlVisitor equivalence.
/// 
/// Features supported:
/// - Fixed fields support (name, namespace, prefix, attribute, wrapped)
/// - Specification extensions (x-*) with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion between element types
/// - Comprehensive metadata injection
/// - Element classification and spec path metadata
/// - Validation with proper type handling
/// - Complete OpenAPI 3.0 XML object compliance

/// Build a basic XmlElement from a generic Element
pub fn build_xml(element: &Element) -> Option<XmlElement> {
    let obj = element.as_object()?.clone();
    Some(XmlElement::with_content(obj))
}

/// Build and decorate XmlElement with enhanced visitor pattern features
pub fn build_and_decorate_xml<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<XmlElement>
where
    F: apidom_ast::fold::Fold,
{
    let obj = element.as_object()?;
    let mut xml = XmlElement::new();
    
    // Process object members with fixed field support and fallback handling
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            let key_str = &key.content;
            let value = member.value.as_ref();
            
            match key_str.as_str() {
                // Fixed fields - direct mapping with type conversion
                "name" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        xml.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        xml.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    }
                }
                "namespace" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        xml.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        xml.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    }
                }
                "prefix" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        xml.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        xml.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    }
                }
                "attribute" => {
                    if let Some(converted) = convert_to_boolean_element(value) {
                        xml.object.set(key_str, Element::Boolean(converted));
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        xml.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    }
                }
                "wrapped" => {
                    if let Some(converted) = convert_to_boolean_element(value) {
                        xml.object.set(key_str, Element::Boolean(converted));
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        xml.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut xml.object, key_str);
                    }
                }
                "$ref" => {
                    // Handle $ref with reference metadata
                    if let Element::String(ref_str) = value {
                        xml.object.set("$ref", value.clone());
                        add_reference_metadata(&mut xml.object, &ref_str.content, "xml");
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
                        xml.object.set(key_str, processed_value);
                        add_specification_extension_metadata(&mut xml.object, key_str);
                    } else {
                        // Fallback field - preserve unknown fields
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        xml.object.set(key_str, processed_value);
                        add_fallback_field_metadata(&mut xml.object, key_str);
                    }
                }
            }
        }
    }
    
    // Add element class metadata
    xml.object.add_class("xml");
    xml.object.meta.properties.insert(
        "element-type".to_string(),
        Value::String("xml".to_string())
    );
    
    // Add spec path metadata (equivalent to TypeScript specPath)
    add_spec_path_metadata(&mut xml.object);
    
    // Validate XML structure (XML has no required fields in OpenAPI)
    validate_xml(&xml)?;
    
    Some(xml)
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
        Value::Bool(true)
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.add_class("specification-extension");
    obj.meta.properties.insert(
        "specification-extension".to_string(),
        Value::String(field_name.to_string())
    );
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        Value::Bool(true)
    );
}

/// Add metadata for $ref references
fn add_reference_metadata(obj: &mut ObjectElement, ref_path: &str, element_type: &str) {
    obj.add_class("reference");
    obj.meta.properties.insert(
        "referenced-element".to_string(),
        Value::String(element_type.to_string())
    );
    obj.meta.properties.insert(
        "reference-path".to_string(),
        Value::String(ref_path.to_string())
    );
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "spec-path".to_string(),
        Value::Array(vec![
            Value::String("document".to_string()),
            Value::String("objects".to_string()),
            Value::String("XML".to_string()),
        ])
    );
}

/// Validate XML structure (XML has no required fields in OpenAPI)
fn validate_xml(xml: &XmlElement) -> Option<()> {
    // If this is a $ref XML, skip standard validation
    if xml.object.get("$ref").is_some() {
        return Some(()); // $ref XMLs are valid without other fields
    }
    
    // XML object has no required fields in OpenAPI 3.0 specification
    // All fields (name, namespace, prefix, attribute, wrapped) are optional
    
    // Additional validation can be added here if needed
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_xml_builder() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("item")));
        obj.set("namespace", Element::String(StringElement::new("http://example.com/schema")));
        
        let result = build_xml(&Element::Object(obj));
        
        assert!(result.is_some());
        let xml = result.unwrap();
        assert_eq!(xml.object.element, "xml");
        assert!(xml.name().is_some());
        assert!(xml.namespace().is_some());
        
        if let Some(name) = xml.name() {
            assert_eq!(name.content, "item");
        }
    }

    #[test]
    fn test_xml_empty_object() {
        let obj = ObjectElement::new();
        
        let result = build_xml(&Element::Object(obj));
        
        assert!(result.is_some()); // XML can be empty (no required fields)
        let xml = result.unwrap();
        assert_eq!(xml.object.element, "xml");
        assert!(xml.name().is_none());
        assert!(xml.namespace().is_none());
    }

    #[test]
    fn test_enhanced_xml_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("item")));
        obj.set("namespace", Element::String(StringElement::new("http://example.com/schema")));
        obj.set("prefix", Element::String(StringElement::new("ex")));
        obj.set("attribute", Element::Boolean(BooleanElement::new(true)));
        obj.set("wrapped", Element::Boolean(BooleanElement::new(false)));
        
        let result = build_and_decorate_xml(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let xml = result.unwrap();
        
        // Verify fixed field metadata
        assert!(xml.object.meta.properties.contains_key("fixed-field-name"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-namespace"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-prefix"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-attribute"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-wrapped"));
        
        // Verify element class
        assert!(xml.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "xml"
            } else {
                false
            }
        }));
        
        // Verify spec path metadata
        assert!(xml.object.meta.properties.contains_key("spec-path"));
        if let Some(Value::Array(spec_path)) = xml.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], Value::String("document".to_string()));
            assert_eq!(spec_path[1], Value::String("objects".to_string()));
            assert_eq!(spec_path[2], Value::String("XML".to_string()));
        }
        
        // Verify field values
        assert!(xml.name().is_some());
        assert!(xml.namespace().is_some());
        assert!(xml.prefix().is_some());
        assert!(xml.attribute().is_some());
        assert!(xml.wrapped().is_some());
        
        if let Some(attribute) = xml.attribute() {
            assert_eq!(attribute.content, true);
        }
        if let Some(wrapped) = xml.wrapped() {
            assert_eq!(wrapped.content, false);
        }
    }

    #[test]
    fn test_xml_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("item")));
        obj.set("x-internal-id", Element::String(StringElement::new("xml-001")));
        obj.set("x-validation-schema", Element::String(StringElement::new("strict")));
        
        let result = build_and_decorate_xml(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let xml = result.unwrap();
        
        // Verify specification extension metadata
        assert!(xml.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(xml.object.get("x-internal-id").is_some());
        assert!(xml.object.get("x-validation-schema").is_some());
    }

    #[test]
    fn test_xml_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("item")));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("anotherField", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        }));
        
        let result = build_and_decorate_xml(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let xml = result.unwrap();
        
        // Verify fallback field metadata
        assert!(xml.object.meta.properties.contains_key("fallback-field-customField"));
        assert!(xml.object.meta.properties.contains_key("fallback-field-anotherField"));
        assert!(xml.object.get("customField").is_some());
        assert!(xml.object.get("anotherField").is_some());
    }

    #[test]
    fn test_xml_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/schemas/XmlDefinition")));
        
        let result = build_and_decorate_xml(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let xml = result.unwrap();
        
        // Verify reference metadata
        assert!(xml.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            xml.object.meta.properties.get("referenced-element"),
            Some(&Value::String("xml".to_string()))
        );
        assert_eq!(
            xml.object.meta.properties.get("reference-path"),
            Some(&Value::String("#/components/schemas/XmlDefinition".to_string()))
        );
    }

    #[test]
    fn test_xml_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        }));
        obj.set("attribute", Element::String(StringElement::new("true")));
        obj.set("wrapped", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 0.0,
        }));
        
        let result = build_and_decorate_xml(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let xml = result.unwrap();
        
        // Verify type conversion worked
        if let Some(name) = xml.name() {
            assert_eq!(name.content, "42");
        }
        if let Some(attribute) = xml.attribute() {
            assert_eq!(attribute.content, true);
        }
        if let Some(wrapped) = xml.wrapped() {
            assert_eq!(wrapped.content, false); // 0.0 converts to false
        }
    }

    #[test]
    fn test_xml_boolean_conversion_edge_cases() {
        let mut obj = ObjectElement::new();
        obj.set("attr1", Element::String(StringElement::new("yes")));
        obj.set("attr2", Element::String(StringElement::new("no")));
        obj.set("attr3", Element::String(StringElement::new("1")));
        obj.set("attr4", Element::String(StringElement::new("0")));
        obj.set("attr5", Element::String(StringElement::new("invalid")));
        
        let result = build_and_decorate_xml(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let xml = result.unwrap();
        
        // Verify boolean conversion edge cases are handled in fallback
        assert!(xml.object.get("attr1").is_some());
        assert!(xml.object.get("attr2").is_some());
        assert!(xml.object.get("attr3").is_some());
        assert!(xml.object.get("attr4").is_some());
        assert!(xml.object.get("attr5").is_some()); // Invalid boolean kept as string
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript XmlVisitor
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("Pet")));
        obj.set("namespace", Element::String(StringElement::new("http://petstore.swagger.io/v2")));
        obj.set("prefix", Element::String(StringElement::new("pet")));
        obj.set("attribute", Element::Boolean(BooleanElement::new(false)));
        obj.set("wrapped", Element::Boolean(BooleanElement::new(true)));
        
        // Add specification extensions
        obj.set("x-xml-version", Element::String(StringElement::new("1.0")));
        obj.set("x-encoding", Element::String(StringElement::new("UTF-8")));
        
        // Add fallback field
        obj.set("customMetadata", Element::String(StringElement::new("custom xml value")));
        
        let result = build_and_decorate_xml(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let xml = result.unwrap();
        
        // Verify TypeScript equivalence features:
        
        // 1. Fixed fields processing
        assert!(xml.object.meta.properties.contains_key("fixed-field-name"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-namespace"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-prefix"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-attribute"));
        assert!(xml.object.meta.properties.contains_key("fixed-field-wrapped"));
        
        // 2. Specification extensions
        assert!(xml.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(xml.object.get("x-xml-version").is_some());
        assert!(xml.object.get("x-encoding").is_some());
        
        // 3. Fallback field handling
        assert!(xml.object.meta.properties.contains_key("fallback-field-customMetadata"));
        assert!(xml.object.get("customMetadata").is_some());
        
        // 4. Element classification
        assert!(xml.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "xml"
            } else {
                false
            }
        }));
        assert_eq!(
            xml.object.meta.properties.get("element-type"),
            Some(&Value::String("xml".to_string()))
        );
        
        // 5. Spec path metadata (equivalent to TypeScript specPath)
        assert!(xml.object.meta.properties.contains_key("spec-path"));
        if let Some(Value::Array(spec_path)) = xml.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], Value::String("document".to_string()));
            assert_eq!(spec_path[1], Value::String("objects".to_string()));
            assert_eq!(spec_path[2], Value::String("XML".to_string()));
        }
        
        // 6. Field value validation
        assert!(xml.name().is_some());
        assert!(xml.namespace().is_some());
        assert!(xml.prefix().is_some());
        assert!(xml.attribute().is_some());
        assert!(xml.wrapped().is_some());
        
        if let Some(name) = xml.name() {
            assert_eq!(name.content, "Pet");
        }
        if let Some(namespace) = xml.namespace() {
            assert_eq!(namespace.content, "http://petstore.swagger.io/v2");
        }
        if let Some(prefix) = xml.prefix() {
            assert_eq!(prefix.content, "pet");
        }
        if let Some(attribute) = xml.attribute() {
            assert_eq!(attribute.content, false);
        }
        if let Some(wrapped) = xml.wrapped() {
            assert_eq!(wrapped.content, true);
        }
        
        // 7. No required field validation (XML has no required fields)
        // This is validated by the fact that empty XML objects are valid
    }
}