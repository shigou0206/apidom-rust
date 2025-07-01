use apidom_ast::*;
use crate::elements::tag::TagElement;

/// Comprehensive OpenAPI Tag Builder
/// 
/// This module provides tag construction with full TypeScript TagVisitor equivalence.
/// 
/// Features supported:
/// - Fixed fields support (name, description, externalDocs)
/// - Specification extensions (x-*) with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion between element types
/// - Comprehensive metadata injection
/// - Element classification and spec path metadata
/// - External documentation processing with recursive folding
/// - Validation with early rejection of invalid structures

/// Build a basic TagElement from a generic Element
pub fn build_tag(element: &Element) -> Option<TagElement> {
    let obj = element.as_object()?.clone();
    let tag = TagElement::with_content(obj);
    
    // Validate name field exists (OpenAPI requirement)
    if tag.name().is_none() {
        return None; // Invalid Tag (name is required in OpenAPI spec)
    }
    
    Some(tag)
}

/// Build and decorate TagElement with enhanced visitor pattern features
pub fn build_and_decorate_tag<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<TagElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut tag = TagElement::new();
    
    // Process object members with fixed field support and fallback handling
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            let key_str = &key.content;
            let value = member.value.as_ref();
            
            match key_str.as_str() {
                // Fixed fields - direct mapping with type conversion
                "name" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        tag.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut tag.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        tag.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut tag.object, key_str);
                    }
                }
                "description" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        tag.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut tag.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        tag.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut tag.object, key_str);
                    }
                }
                "externalDocs" => {
                    // Process external documentation with recursive folding
                    if let Element::Object(_external_docs_obj) = value {
                        let processed_external_docs = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        
                        // Try to build as ExternalDocumentation if it's an object
                        if let Element::Object(_processed_obj) = &processed_external_docs {
                            if let Some(external_docs) = crate::builder::external_documentation_builder::build_external_docs(&processed_external_docs) {
                                tag.object.set("externalDocs", Element::Object(external_docs.object));
                            } else {
                                tag.object.set("externalDocs", processed_external_docs);
                            }
                        } else {
                            tag.object.set("externalDocs", processed_external_docs);
                        }
                        
                        add_fixed_field_metadata(&mut tag.object, "externalDocs");
                        add_external_docs_metadata(&mut tag.object);
                    }
                }
                "$ref" => {
                    // Handle $ref with reference metadata
                    if let Element::String(ref_str) = value {
                        tag.object.set("$ref", value.clone());
                        add_reference_metadata(&mut tag.object, &ref_str.content, "tag");
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
                        tag.object.set(key_str, processed_value);
                        add_specification_extension_metadata(&mut tag.object, key_str);
                    } else {
                        // Fallback field - preserve unknown fields
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        tag.object.set(key_str, processed_value);
                        add_fallback_field_metadata(&mut tag.object, key_str);
                    }
                }
            }
        }
    }
    
    // Add element class metadata
    tag.object.add_class("tag");
    tag.object.meta.properties.insert(
        "element-type".to_string(),
        SimpleValue::string("tag".to_string())
    );
    
    // Add spec path metadata (equivalent to TypeScript specPath)
    add_spec_path_metadata(&mut tag.object);
    
    // Validate tag structure
    validate_tag(&tag)?;
    
    Some(tag)
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

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fixed-field-{}", field_name),
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

/// Add metadata for external documentation
fn add_external_docs_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "has-external-docs".to_string(),
        SimpleValue::bool(true)
    );
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "spec-path".to_string(),
        SimpleValue::array(vec![
            SimpleValue::string("document".to_string()),
            SimpleValue::string("objects".to_string()),
            SimpleValue::string("Tag".to_string()),
        ])
    );
}

/// Validate tag structure
fn validate_tag(tag: &TagElement) -> Option<()> {
    // If this is a $ref tag, skip standard validation
    if tag.object.get("$ref").is_some() {
        return Some(()); // $ref tags are valid without other required fields
    }
    
    // Check required fields for non-reference tags
    if tag.name().is_none() {
        return None; // name is required for non-reference tags
    }
    
    // Additional validation can be added here
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tag_builder() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("pets")));
        obj.set("description", Element::String(StringElement::new("Everything about your pets")));
        
        let result = build_tag(&Element::Object(obj));
        
        assert!(result.is_some());
        let tag = result.unwrap();
        assert_eq!(tag.object.element, "tag");
        assert!(tag.name().is_some());
        assert!(tag.description().is_some());
        
        if let Some(name) = tag.name() {
            assert_eq!(name.content, "pets");
        }
    }

    #[test]
    fn test_tag_missing_name() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("A description without name")));
        
        let result = build_tag(&Element::Object(obj));
        
        assert!(result.is_none()); // Should fail validation without name
    }

    #[test]
    fn test_enhanced_tag_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("pets")));
        obj.set("description", Element::String(StringElement::new("Everything about your pets")));
        
        let result = build_and_decorate_tag(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let tag = result.unwrap();
        
        // Verify fixed field metadata
        assert!(tag.object.meta.properties.contains_key("fixed-field-name"));
        assert!(tag.object.meta.properties.contains_key("fixed-field-description"));
        
        // Verify element class
        assert!(tag.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "tag"
            } else {
                false
            }
        }));
        
        // Verify spec path metadata
        assert!(tag.object.meta.properties.contains_key("spec-path"));
        if let Some(SimpleValue::Array(spec_path)) = tag.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], SimpleValue::string("document".to_string()));
            assert_eq!(spec_path[1], SimpleValue::string("objects".to_string()));
            assert_eq!(spec_path[2], SimpleValue::string("Tag".to_string()));
        }
    }

    #[test]
    fn test_tag_with_external_docs() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("pets")));
        obj.set("description", Element::String(StringElement::new("Everything about your pets")));
        
        // Add external documentation
        let mut external_docs = ObjectElement::new();
        external_docs.set("url", Element::String(StringElement::new("https://example.com/docs")));
        external_docs.set("description", Element::String(StringElement::new("More info about pets")));
        obj.set("externalDocs", Element::Object(external_docs));
        
        let result = build_and_decorate_tag(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let tag = result.unwrap();
        
        // Verify external docs processing
        assert!(tag.external_docs().is_some());
        assert!(tag.object.meta.properties.contains_key("fixed-field-externalDocs"));
        assert!(tag.object.meta.properties.contains_key("has-external-docs"));
        
        if let Some(external_docs) = tag.external_docs() {
            assert!(external_docs.url().is_some());
            assert!(external_docs.description().is_some());
        }
    }

    #[test]
    fn test_tag_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("pets")));
        obj.set("x-internal-id", Element::String(StringElement::new("tag-001")));
        obj.set("x-display-order", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 1.0,
        }));
        
        let result = build_and_decorate_tag(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let tag = result.unwrap();
        
        // Verify specification extension metadata
        assert!(tag.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(tag.object.get("x-internal-id").is_some());
        assert!(tag.object.get("x-display-order").is_some());
    }

    #[test]
    fn test_tag_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("pets")));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("anotherField", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_tag(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let tag = result.unwrap();
        
        // Verify fallback field metadata
        assert!(tag.object.meta.properties.contains_key("fallback-field-customField"));
        assert!(tag.object.meta.properties.contains_key("fallback-field-anotherField"));
        assert!(tag.object.get("customField").is_some());
        assert!(tag.object.get("anotherField").is_some());
    }

    #[test]
    fn test_tag_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/tags/pets")));
        
        let result = build_and_decorate_tag(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let tag = result.unwrap();
        
        // Verify reference metadata
        assert!(tag.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            tag.object.meta.properties.get("referenced-element"),
            Some(&SimpleValue::string("tag".to_string()))
        );
        assert_eq!(
            tag.object.meta.properties.get("reference-path"),
            Some(&SimpleValue::string("#/components/tags/pets".to_string()))
        );
    }

    #[test]
    fn test_tag_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        }));
        obj.set("description", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_tag(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let tag = result.unwrap();
        
        // Verify type conversion worked
        if let Some(name) = tag.name() {
            assert_eq!(name.content, "42");
        }
        if let Some(description) = tag.description() {
            assert_eq!(description.content, "true");
        }
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript TagVisitor
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("pets")));
        obj.set("description", Element::String(StringElement::new("Everything about your pets")));
        
        // Add external documentation
        let mut external_docs = ObjectElement::new();
        external_docs.set("url", Element::String(StringElement::new("https://petstore.swagger.io/docs")));
        external_docs.set("description", Element::String(StringElement::new("Find more info here")));
        obj.set("externalDocs", Element::Object(external_docs));
        
        // Add specification extensions
        obj.set("x-tag-category", Element::String(StringElement::new("animals")));
        obj.set("x-display-order", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 1.0,
        }));
        
        // Add fallback field
        obj.set("customMetadata", Element::String(StringElement::new("custom value")));
        
        let result = build_and_decorate_tag(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let tag = result.unwrap();
        
        // Verify TypeScript equivalence features:
        
        // 1. Fixed fields processing
        assert!(tag.object.meta.properties.contains_key("fixed-field-name"));
        assert!(tag.object.meta.properties.contains_key("fixed-field-description"));
        assert!(tag.object.meta.properties.contains_key("fixed-field-externalDocs"));
        
        // 2. External documentation processing
        assert!(tag.external_docs().is_some());
        assert!(tag.object.meta.properties.contains_key("has-external-docs"));
        if let Some(external_docs) = tag.external_docs() {
            assert!(external_docs.url().is_some());
            assert!(external_docs.description().is_some());
        }
        
        // 3. Specification extensions
        assert!(tag.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(tag.object.get("x-tag-category").is_some());
        assert!(tag.object.get("x-display-order").is_some());
        
        // 4. Fallback field handling
        assert!(tag.object.meta.properties.contains_key("fallback-field-customMetadata"));
        assert!(tag.object.get("customMetadata").is_some());
        
        // 5. Element classification
        assert!(tag.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "tag"
            } else {
                false
            }
        }));
        assert_eq!(
            tag.object.meta.properties.get("element-type"),
            Some(&SimpleValue::string("tag".to_string()))
        );
        
        // 6. Spec path metadata (equivalent to TypeScript specPath)
        assert!(tag.object.meta.properties.contains_key("spec-path"));
        if let Some(SimpleValue::Array(spec_path)) = tag.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], SimpleValue::string("document".to_string()));
            assert_eq!(spec_path[1], SimpleValue::string("objects".to_string()));
            assert_eq!(spec_path[2], SimpleValue::string("Tag".to_string()));
        }
        
        // 7. Required field validation
        assert!(tag.name().is_some());
        if let Some(name) = tag.name() {
            assert_eq!(name.content, "pets");
        }
        
        // 8. Complete structure validation
        assert!(tag.description().is_some());
        if let Some(description) = tag.description() {
            assert_eq!(description.content, "Everything about your pets");
        }
    }
}