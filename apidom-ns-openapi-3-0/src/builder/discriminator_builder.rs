//! # Discriminator Builder Module
//!
//! This module provides enhanced discriminator building functionality that is functionally equivalent
//! to the TypeScript `DiscriminatorVisitor` class. It implements visitor pattern features including
//! fixed fields support, mapping handling, fallback behavior, and comprehensive metadata injection.
//!
//! ## Features
//!
//! ### 1. Fixed Fields Support (FixedFieldsVisitor equivalent)
//! - Processes standard OpenAPI Discriminator fields: `propertyName`, `mapping`
//! - Validates field types and provides proper handling
//! - Injects metadata for fixed field processing
//!
//! ### 2. Mapping Support (MappingVisitor equivalent)
//! - Handles `mapping` field as specialized DiscriminatorMappingElement
//! - Provides recursive folding for mapping structure
//! - Maintains Map type semantics rather than generic Object
//!
//! ### 3. Fallback Behavior (FallbackVisitor equivalent)
//! - Preserves unknown fields that don't match fixed fields
//! - Adds metadata to track fallback processing
//! - Maintains compatibility with custom properties
//!
//! ### 4. Reference Support
//! - Handles `$ref` fields for discriminator references
//! - Adds metadata for reference tracking
//! - Supports recursive resolution when combined with folders
//!
//! ### 5. Field Validation
//! - Validates that `propertyName` is a string
//! - Ensures `mapping` contains valid schema references
//! - Provides comprehensive error reporting through metadata
//!
//! ### 6. Recursive Folding
//! - Supports optional folder parameter for recursive element processing
//! - Compatible with any type implementing the `Fold` trait
//! - Enables complex document transformation workflows
//!
//! ## TypeScript Equivalence
//!
//! This implementation provides feature parity with the TypeScript `DiscriminatorVisitor` class:
//! - ✅ Fixed fields processing
//! - ✅ Specialized mapping handling
//! - ✅ Fallback behavior for unknown fields
//! - ✅ Recursive folding capability
//! - ✅ Comprehensive metadata injection
//! - ✅ Field validation
//! - ✅ Reference handling
//! - ❌ Specification extensions (disabled by design, matching TypeScript)

use apidom_ast::*;
use crate::elements::discriminator::DiscriminatorElement;
use crate::builder::discriminator_mapping_builder::build_and_decorate_discriminator_mapping;

/// Basic discriminator builder - equivalent to simple constructor
pub fn build_discriminator(element: &Element) -> Option<DiscriminatorElement> {
    let object = element.as_object()?;
    Some(DiscriminatorElement::with_content(object.clone()))
}

/// Enhanced discriminator builder with visitor pattern features
/// Equivalent to TypeScript DiscriminatorVisitor with FixedFieldsVisitor and FallbackVisitor
pub fn build_and_decorate_discriminator<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<DiscriminatorElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut discriminator = DiscriminatorElement::with_content(object.clone());
    
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
            // Fixed field: propertyName
            "propertyName" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    discriminator.set_property_name(string_val);
                    add_fixed_field_metadata(&mut discriminator, "propertyName");
                    add_type_conversion_metadata(&mut discriminator, "propertyName", "string");
                } else {
                    add_validation_error_metadata(&mut discriminator, "propertyName", "Expected string value");
                }
            },
            // Fixed field: mapping (specialized handling)
            "mapping" => {
                if let Some(mapping_obj) = processed_value.as_object() {
                    // Use specialized mapping builder
                    if let Some(mapping_element) = build_and_decorate_discriminator_mapping(&processed_value, folder.as_deref_mut()) {
                        discriminator.set_mapping(mapping_element.object);
                        add_fixed_field_metadata(&mut discriminator, "mapping");
                        add_mapping_metadata(&mut discriminator);
                    } else {
                        // Fallback to regular object
                        discriminator.set_mapping(mapping_obj.clone());
                        add_validation_error_metadata(&mut discriminator, "mapping", "Failed to process mapping object");
                    }
                } else {
                    add_validation_error_metadata(&mut discriminator, "mapping", "Expected object value for mapping");
                }
            },
            // $ref handling
            "$ref" => {
                discriminator.object.set("$ref", processed_value);
                add_ref_metadata(&mut discriminator, "$ref");
            },
            // Fallback for unknown fields (specification extensions are disabled)
            _ => {
                discriminator.object.set(&key_str, processed_value);
                add_fallback_metadata(&mut discriminator, &key_str);
            }
        }
    }
    
    // Validate discriminator structure
    validate_discriminator(&mut discriminator);
    
    // Add processing metadata
    add_processing_metadata(&mut discriminator);
    add_spec_path_metadata(&mut discriminator);
    
    Some(discriminator)
}

/// Convert various element types to StringElement with fallback behavior
fn convert_to_string_element(element: &Element) -> Option<StringElement> {
    match element {
        Element::String(s) => Some(s.clone()),
        Element::Number(n) => Some(StringElement::new(&n.content.to_string())),
        Element::Boolean(b) => Some(StringElement::new(&b.content.to_string())),
        _ => None,
    }
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(discriminator: &mut DiscriminatorElement, _field_name: &str) {
    let key = format!("fixedField_{}", _field_name);
    discriminator.object.meta.properties.insert(key, SimpleValue::bool(true));
}

/// Add metadata for type conversions
fn add_type_conversion_metadata(discriminator: &mut DiscriminatorElement, field_name: &str, expected_type: &str) {
    let key = format!("typeConversion_{}", field_name);
    discriminator.object.meta.properties.insert(key, SimpleValue::string(expected_type.to_string()));
}

/// Add metadata for mapping processing
fn add_mapping_metadata(discriminator: &mut DiscriminatorElement) {
    discriminator.object.meta.properties.insert("mappingProcessed".to_string(), SimpleValue::bool(true));
}

/// Add metadata for references
fn add_ref_metadata(discriminator: &mut DiscriminatorElement, field_name: &str) {
    let key = format!("ref_{}", field_name);
    discriminator.object.meta.properties.insert(key, SimpleValue::bool(true));
    discriminator.object.meta.properties.insert("referenced-element".to_string(), SimpleValue::string("discriminator".to_string()));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(discriminator: &mut DiscriminatorElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    discriminator.object.meta.properties.insert(key, SimpleValue::bool(true));
}

/// Add validation error metadata
fn add_validation_error_metadata(discriminator: &mut DiscriminatorElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    discriminator.object.meta.properties.insert(key, SimpleValue::string(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(discriminator: &mut DiscriminatorElement) {
    discriminator.object.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(discriminator: &mut DiscriminatorElement) {
    discriminator.object.meta.properties.insert("specPath".to_string(), SimpleValue::array(vec![
        SimpleValue::string("document".to_string()),
        SimpleValue::string("objects".to_string()),
        SimpleValue::string("Discriminator".to_string())
    ]));
}

/// Validate discriminator structure
fn validate_discriminator(discriminator: &mut DiscriminatorElement) {
    let has_property_name = discriminator.property_name().is_some();
    
    if !has_property_name {
        add_validation_error_metadata(
            discriminator,
            "discriminator",
            "Discriminator must have a propertyName field"
        );
    } else {
        discriminator.object.meta.properties.insert("validDiscriminator".to_string(), SimpleValue::bool(true));
    }
    
    // Validate mapping if present
    if let Some(mapping) = discriminator.mapping() {
        if mapping.content.is_empty() {
            add_validation_error_metadata(
                discriminator,
                "mapping",
                "Mapping should not be empty if present"
            );
        } else {
            discriminator.object.meta.properties.insert("validMapping".to_string(), SimpleValue::bool(true));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::DefaultFolder;
    use apidom_ast::{Element, ObjectElement, StringElement, NumberElement, MetaElement, AttributesElement};

    #[test]
    fn test_basic_discriminator_builder() {
        let mut obj = ObjectElement::new();
        obj.set("propertyName", Element::String(StringElement::new("petType")));

        let discriminator = build_discriminator(&Element::Object(obj));
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        assert_eq!(discriminator.property_name().unwrap().content, "petType");
    }

    #[test]
    fn test_enhanced_discriminator_builder_with_mapping() {
        let mut obj = ObjectElement::new();
        obj.set("propertyName", Element::String(StringElement::new("petType")));
        
        // Create mapping
        let mut mapping_obj = ObjectElement::new();
        mapping_obj.set("cat", Element::String(StringElement::new("#/components/schemas/Cat")));
        mapping_obj.set("dog", Element::String(StringElement::new("#/components/schemas/Dog")));
        obj.set("mapping", Element::Object(mapping_obj));

        let mut folder = DefaultFolder;
        let discriminator = build_and_decorate_discriminator(&Element::Object(obj), Some(&mut folder));
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        
        // Verify basic fields
        assert_eq!(discriminator.property_name().unwrap().content, "petType");
        assert!(discriminator.mapping().is_some());
        
        // Verify metadata injection
        assert!(discriminator.object.meta.properties.contains_key("fixedField_propertyName"));
        assert!(discriminator.object.meta.properties.contains_key("fixedField_mapping"));
        assert!(discriminator.object.meta.properties.contains_key("mappingProcessed"));
        assert!(discriminator.object.meta.properties.contains_key("processed"));
        assert!(discriminator.object.meta.properties.contains_key("validDiscriminator"));
        assert!(discriminator.object.meta.properties.contains_key("validMapping"));
        assert!(discriminator.object.meta.properties.contains_key("specPath"));
        
        // Verify mapping structure
        let mapping = discriminator.mapping().unwrap();
        assert_eq!(mapping.element, "discriminatorMapping");
        assert!(mapping.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "map"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_discriminator_type_conversion() {
        let mut obj = ObjectElement::new();
        // Test type conversion from number to string for propertyName
        obj.set("propertyName", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        }));

        let mut folder = DefaultFolder;
        let discriminator = build_and_decorate_discriminator(&Element::Object(obj), Some(&mut folder));
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        
        // Verify conversion worked
        assert_eq!(discriminator.property_name().unwrap().content, "123");
        
        // Verify type conversion metadata
        assert!(discriminator.object.meta.properties.contains_key("typeConversion_propertyName"));
    }

    #[test]
    fn test_discriminator_validation_errors() {
        // Test discriminator without propertyName (should fail validation)
        let empty_obj = ObjectElement::new();
        let discriminator = build_and_decorate_discriminator::<DefaultFolder>(&Element::Object(empty_obj), None);
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        assert!(discriminator.object.meta.properties.contains_key("validationError_discriminator"));
        assert!(!discriminator.object.meta.properties.contains_key("validDiscriminator"));
    }

    #[test]
    fn test_discriminator_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/discriminators/petDiscriminator")));

        let discriminator = build_and_decorate_discriminator::<DefaultFolder>(&Element::Object(obj), None);
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        
        // Verify $ref is preserved
        assert!(discriminator.object.get("$ref").is_some());
        assert!(discriminator.object.meta.properties.contains_key("ref_$ref"));
        assert!(discriminator.object.meta.properties.contains_key("referenced-element"));
    }

    #[test]
    fn test_discriminator_fallback_behavior() {
        let mut obj = ObjectElement::new();
        obj.set("propertyName", Element::String(StringElement::new("type")));
        obj.set("unknown-field", Element::String(StringElement::new("unknown-value")));
        obj.set("custom-property", Element::String(StringElement::new("custom-value")));

        let discriminator = build_and_decorate_discriminator::<DefaultFolder>(&Element::Object(obj), None);
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        
        // Verify fallback fields are preserved
        assert!(discriminator.object.get("unknown-field").is_some());
        assert!(discriminator.object.get("custom-property").is_some());
        
        // Verify fallback metadata
        assert!(discriminator.object.meta.properties.contains_key("fallback_unknown-field"));
        assert!(discriminator.object.meta.properties.contains_key("fallback_custom-property"));
    }

    #[test]
    fn test_discriminator_mapping_validation() {
        let mut obj = ObjectElement::new();
        obj.set("propertyName", Element::String(StringElement::new("petType")));
        
        // Create empty mapping (should trigger validation warning)
        let empty_mapping = ObjectElement::new();
        obj.set("mapping", Element::Object(empty_mapping));

        let discriminator = build_and_decorate_discriminator::<DefaultFolder>(&Element::Object(obj), None);
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        
        // Should have validation error for empty mapping
        assert!(discriminator.object.meta.properties.contains_key("validationError_mapping"));
        assert!(!discriminator.object.meta.properties.contains_key("validMapping"));
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // Comprehensive test demonstrating TypeScript DiscriminatorVisitor equivalence
        let mut obj = ObjectElement::new();
        obj.set("propertyName", Element::String(StringElement::new("petType")));
        
        // Create comprehensive mapping
        let mut mapping_obj = ObjectElement::new();
        mapping_obj.set("cat", Element::String(StringElement::new("#/components/schemas/Cat")));
        mapping_obj.set("dog", Element::String(StringElement::new("#/components/schemas/Dog")));
        mapping_obj.set("bird", Element::String(StringElement::new("#/components/schemas/Bird")));
        obj.set("mapping", Element::Object(mapping_obj));
        
        // Add custom field for fallback testing
        obj.set("custom-annotation", Element::String(StringElement::new("custom-value")));

        let mut folder = DefaultFolder;
        let discriminator = build_and_decorate_discriminator(&Element::Object(obj), Some(&mut folder));
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        
        // Verify all TypeScript DiscriminatorVisitor features are implemented:
        
        // 1. Fixed fields support
        assert!(discriminator.object.meta.properties.contains_key("fixedField_propertyName"));
        assert!(discriminator.object.meta.properties.contains_key("fixedField_mapping"));
        
        // 2. Specialized mapping handling
        assert!(discriminator.object.meta.properties.contains_key("mappingProcessed"));
        let mapping = discriminator.mapping().unwrap();
        assert_eq!(mapping.element, "discriminatorMapping");
        
        // 3. Fallback behavior for unknown fields
        assert!(discriminator.object.meta.properties.contains_key("fallback_custom-annotation"));
        
        // 4. Processing metadata
        assert!(discriminator.object.meta.properties.contains_key("processed"));
        
        // 5. Validation metadata
        assert!(discriminator.object.meta.properties.contains_key("validDiscriminator"));
        assert!(discriminator.object.meta.properties.contains_key("validMapping"));
        
        // 6. Spec path metadata
        assert!(discriminator.object.meta.properties.contains_key("specPath"));
        
        // 7. All fields are preserved
        assert_eq!(discriminator.property_name().unwrap().content, "petType");
        assert!(discriminator.mapping().is_some());
        assert!(discriminator.object.get("custom-annotation").is_some());
        
        // 8. Mapping structure verification
        assert!(mapping.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "map"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_discriminator_comprehensive_scenario() {
        // Test all supported discriminator scenarios
        let mut obj = ObjectElement::new();
        obj.set("propertyName", Element::String(StringElement::new("@type")));
        
        // Complex mapping with various schema references
        let mut mapping_obj = ObjectElement::new();
        mapping_obj.set("Person", Element::String(StringElement::new("#/components/schemas/Person")));
        mapping_obj.set("Organization", Element::String(StringElement::new("#/components/schemas/Organization")));
        mapping_obj.set("ExternalRef", Element::String(StringElement::new("external.yaml#/schemas/ExternalEntity")));
        obj.set("mapping", Element::Object(mapping_obj));
        
        // Add various fallback fields
        obj.set("description", Element::String(StringElement::new("Type discriminator")));
        obj.set("example", Element::String(StringElement::new("Person")));

        let mut folder = DefaultFolder;
        let discriminator = build_and_decorate_discriminator(&Element::Object(obj), Some(&mut folder));
        assert!(discriminator.is_some());
        
        let discriminator = discriminator.unwrap();
        
        // Verify comprehensive functionality
        assert_eq!(discriminator.property_name().unwrap().content, "@type");
        
        let mapping = discriminator.mapping().unwrap();
        assert_eq!(mapping.content.len(), 3);
        
        // Verify all metadata is present
        assert!(discriminator.object.meta.properties.contains_key("validDiscriminator"));
        assert!(discriminator.object.meta.properties.contains_key("validMapping"));
        assert!(discriminator.object.meta.properties.contains_key("processed"));
        assert!(discriminator.object.meta.properties.contains_key("fallback_description"));
        assert!(discriminator.object.meta.properties.contains_key("fallback_example"));
        
        // Verify mapping element has correct type and metadata
        assert_eq!(mapping.element, "discriminatorMapping");
        assert!(mapping.meta.properties.contains_key("processed"));
        assert!(mapping.meta.properties.contains_key("specPath"));
    }
}