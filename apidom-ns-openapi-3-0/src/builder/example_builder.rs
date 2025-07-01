//! # Example Builder Module
//!
//! This module provides enhanced example building functionality that is functionally equivalent
//! to the TypeScript `ExampleVisitor` class. It implements visitor pattern features including
//! fixed fields support, specification extensions, reference handling, and comprehensive metadata injection.
//!
//! ## Features
//!
//! ### 1. Fixed Fields Support (FixedFieldsVisitor equivalent)
//! - Processes standard OpenAPI Example fields: `summary`, `description`, `value`, `externalValue`
//! - Validates field types and provides type conversion
//! - Injects metadata for fixed field processing
//!
//! ### 2. Specification Extensions Support
//! - Handles `x-*` prefixed extension fields
//! - Preserves extension values in the example object
//! - Adds metadata to track specification extensions
//!
//! ### 3. Reference Support
//! - Handles `$ref` fields for example references
//! - Adds metadata for reference tracking
//! - Supports recursive resolution when combined with folders
//!
//! ### 4. External Value Reference Detection
//! - Detects when `externalValue` is present and adds `reference-element` class
//! - Equivalent to TypeScript's reference element marking
//! - Provides proper classification for external references
//!
//! ### 5. Fallback Behavior (FallbackVisitor equivalent)
//! - Preserves unknown fields that don't match fixed fields or extensions
//! - Adds metadata to track fallback processing
//! - Maintains full compatibility with custom properties
//!
//! ### 6. Type Conversion and Validation
//! - Converts various element types to appropriate target types
//! - Validates field constraints and relationships
//! - Provides comprehensive error reporting through metadata
//!
//! ### 7. Recursive Folding
//! - Supports optional folder parameter for recursive element processing
//! - Compatible with any type implementing the `Fold` trait
//! - Enables complex document transformation workflows
//!
//! ## Usage Examples
//!
//! ```rust
//! use apidom_ast::DefaultFolder;
//! use apidom_ast::*;
//! use apidom_ns_openapi_3_0::builder::example_builder::*;
//!
//! // Create a sample element for testing
//! let mut obj = ObjectElement::new();
//! obj.set("summary", Element::String(StringElement::new("Pet example")));
//! obj.set("value", Element::Object(ObjectElement::new()));
//! let element = Element::Object(obj);
//!
//! // Basic usage
//! let example = build_example(&element);
//!
//! // Enhanced usage with visitor pattern features
//! let mut folder = DefaultFolder;
//! let example = build_and_decorate_example(&element, Some(&mut folder));
//! ```
//!
//! ## TypeScript Equivalence
//!
//! This implementation provides feature parity with the TypeScript `ExampleVisitor` class:
//! - ✅ Fixed fields processing (`summary`, `description`, `value`, `externalValue`)
//! - ✅ Specification extensions support (`x-*` fields)
//! - ✅ Reference element classification (when `externalValue` is present)
//! - ✅ Fallback behavior for unknown fields
//! - ✅ Recursive folding capability
//! - ✅ Comprehensive metadata injection
//! - ✅ Type validation and conversion
//! - ✅ SpecPath metadata injection: `["document", "objects", "Example"]`

use apidom_ast::*;
use serde_json::Value;
use crate::elements::example::ExampleElement;

/// Basic example builder - equivalent to simple constructor
pub fn build_example(element: &Element) -> Option<ExampleElement> {
    let object = element.as_object()?;
    Some(ExampleElement::with_content(object.clone()))
}

/// Enhanced example builder with visitor pattern features
/// Equivalent to TypeScript ExampleVisitor with FixedFieldsVisitor and FallbackVisitor
pub fn build_and_decorate_example<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<ExampleElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut example = ExampleElement::with_content(object.clone());
    
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
            // Fixed fields
            "summary" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    example.set_summary(string_val);
                    add_fixed_field_metadata(&mut example, "summary");
                } else {
                    add_validation_error_metadata(&mut example, "summary", "Expected string value");
                }
            },
            "description" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    example.set_description(string_val);
                    add_fixed_field_metadata(&mut example, "description");
                } else {
                    add_validation_error_metadata(&mut example, "description", "Expected string value");
                }
            },
            "value" => {
                example.set_value(processed_value);
                add_fixed_field_metadata(&mut example, "value");
            },
            "externalValue" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    example.set_external_value(string_val);
                    add_fixed_field_metadata(&mut example, "externalValue");
                    // Mark as reference element when externalValue is present (TypeScript equivalent)
                    add_reference_element_class(&mut example);
                } else {
                    add_validation_error_metadata(&mut example, "externalValue", "Expected string value");
                }
            },
            // $ref handling
            "$ref" => {
                example.object.set("$ref", processed_value);
                add_ref_metadata(&mut example, "$ref");
            },
            // Specification extensions (x-* fields)
            key if key.starts_with("x-") => {
                example.object.set(&key_str, processed_value);
                add_specification_extension_metadata(&mut example, &key_str);
            },
            // Fallback for unknown fields
            _ => {
                example.object.set(&key_str, processed_value);
                add_fallback_metadata(&mut example, &key_str);
            }
        }
    }
    
    // Add processing metadata
    add_processing_metadata(&mut example);
    add_spec_path_metadata(&mut example);
    
    Some(example)
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

/// Add metadata for fixed fields
fn add_fixed_field_metadata(example: &mut ExampleElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    example.object.meta.properties.insert(key, SimpleValue::Bool(true));
    example.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(example: &mut ExampleElement, field_name: &str) {
    let key = format!("ref_{}", field_name);
    example.object.meta.properties.insert(key, SimpleValue::Bool(true));
    example.object.meta.properties.insert("referenced-element".to_string(), SimpleValue::String("example".to_string()));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(example: &mut ExampleElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    example.object.meta.properties.insert(key, SimpleValue::Bool(true));
    example.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(example: &mut ExampleElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    example.object.meta.properties.insert(key, SimpleValue::Bool(true));
}

/// Add validation error metadata
fn add_validation_error_metadata(example: &mut ExampleElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    example.object.meta.properties.insert(key, SimpleValue::String(error_msg.to_string()));
}

/// Add reference element class when externalValue is present
fn add_reference_element_class(example: &mut ExampleElement) {
    example.object.classes.content.push(Element::String(StringElement::new("reference-element")));
    example.object.meta.properties.insert("isReferenceElement".to_string(), SimpleValue::Bool(true));
}

/// Add overall processing metadata
fn add_processing_metadata(example: &mut ExampleElement) {
    example.object.meta.properties.insert("processed".to_string(), SimpleValue::Bool(true));
    example.object.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::Bool(true));
    example.object.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::Bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(example: &mut ExampleElement) {
    example.object.meta.properties.insert("specPath".to_string(), SimpleValue::Array(vec![
        SimpleValue::String("document".to_string()),
        SimpleValue::String("objects".to_string()),
        SimpleValue::String("Example".to_string())
    ]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::DefaultFolder;
    use apidom_ast::*;

    #[test]
    fn test_basic_example_builder() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("Pet example")));
        obj.set("description", Element::String(StringElement::new("A sample pet")));
        obj.set("value", Element::Object(ObjectElement::new()));

        let example = build_example(&Element::Object(obj));
        assert!(example.is_some());
        
        let example = example.unwrap();
        assert_eq!(example.summary().unwrap().content, "Pet example");
        assert_eq!(example.description().unwrap().content, "A sample pet");
        assert!(example.value().is_some());
    }

    #[test]
    fn test_enhanced_example_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("User example")));
        obj.set("description", Element::String(StringElement::new("A sample user object")));
        
        // Create a complex value object
        let mut value_obj = ObjectElement::new();
        value_obj.set("id", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        }));
        value_obj.set("name", Element::String(StringElement::new("John Doe")));
        obj.set("value", Element::Object(value_obj));

        let mut folder = DefaultFolder;
        let example = build_and_decorate_example(&Element::Object(obj), Some(&mut folder));
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify basic fields
        assert_eq!(example.summary().unwrap().content, "User example");
        assert_eq!(example.description().unwrap().content, "A sample user object");
        assert!(example.value().is_some());
        
        // Verify fixed field metadata
        assert!(example.object.meta.properties.contains_key("fixedField_summary"));
        assert!(example.object.meta.properties.contains_key("fixedField_description"));
        assert!(example.object.meta.properties.contains_key("fixedField_value"));
        
        // Verify processing metadata
        assert!(example.object.meta.properties.contains_key("processed"));
        assert!(example.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(example.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(example.object.meta.properties.contains_key("specPath"));
        
        // Verify spec path
        if let Some(SimpleValue::Array(path)) = example.object.meta.properties.get("specPath") {
            assert_eq!(path.len(), 3);
            assert!(matches!(&path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&path[2], SimpleValue::String(s) if s == "Example"));
        }
        
        // Verify fixed-field classes
        let has_fixed_field_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "fixed-field"
            } else {
                false
            }
        });
        assert!(has_fixed_field_class);
    }

    #[test]
    fn test_example_with_external_value_reference() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("External example")));
        obj.set("externalValue", Element::String(StringElement::new("https://example.com/examples/pet.json")));

        let example = build_and_decorate_example::<DefaultFolder>(&Element::Object(obj), None);
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify external value is set
        assert_eq!(example.external_value().unwrap().content, "https://example.com/examples/pet.json");
        
        // Verify reference element class is added (TypeScript equivalent)
        let has_reference_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "reference-element"
            } else {
                false
            }
        });
        assert!(has_reference_class);
        
        // Verify reference metadata
        assert!(example.object.meta.properties.contains_key("isReferenceElement"));
        if let Some(SimpleValue::Bool(is_ref)) = example.object.meta.properties.get("isReferenceElement") {
            assert!(is_ref);
        }
        
        // Verify fixed field metadata
        assert!(example.object.meta.properties.contains_key("fixedField_externalValue"));
    }

    #[test]
    fn test_example_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("Extended example")));
        obj.set("x-custom-property", Element::String(StringElement::new("custom-value")));
        obj.set("x-vendor-extension", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 42.0,
        }));
        obj.set("x-boolean-ext", Element::Boolean(BooleanElement::new(true)));

        let example = build_and_decorate_example::<DefaultFolder>(&Element::Object(obj), None);
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify specification extensions are preserved
        assert!(example.object.get("x-custom-property").is_some());
        assert!(example.object.get("x-vendor-extension").is_some());
        assert!(example.object.get("x-boolean-ext").is_some());
        
        // Verify specification extension metadata
        assert!(example.object.meta.properties.contains_key("specificationExtension_x-custom-property"));
        assert!(example.object.meta.properties.contains_key("specificationExtension_x-vendor-extension"));
        assert!(example.object.meta.properties.contains_key("specificationExtension_x-boolean-ext"));
        
        // Verify specification extension classes
        let has_spec_extension_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "specification-extension"
            } else {
                false
            }
        });
        assert!(has_spec_extension_class);
    }

    #[test]
    fn test_example_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/examples/PetExample")));

        let example = build_and_decorate_example::<DefaultFolder>(&Element::Object(obj), None);
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify $ref is preserved
        assert!(example.object.get("$ref").is_some());
        
        // Verify reference metadata
        assert!(example.object.meta.properties.contains_key("ref_$ref"));
        assert!(example.object.meta.properties.contains_key("referenced-element"));
        if let Some(SimpleValue::String(ref_type)) = example.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_type, "example");
        }
    }

    #[test]
    fn test_example_type_conversion() {
        let mut obj = ObjectElement::new();
        // Test type conversion for summary and description
        obj.set("summary", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        }));
        obj.set("description", Element::Boolean(BooleanElement::new(true)));
        obj.set("externalValue", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 456.0,
        }));

        let example = build_and_decorate_example::<DefaultFolder>(&Element::Object(obj), None);
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify conversions worked
        assert_eq!(example.summary().unwrap().content, "123");
        assert_eq!(example.description().unwrap().content, "true");
        assert_eq!(example.external_value().unwrap().content, "456");
        
        // Verify it's marked as reference element due to externalValue
        let has_reference_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "reference-element"
            } else {
                false
            }
        });
        assert!(has_reference_class);
    }

    #[test]
    fn test_example_validation_errors() {
        let mut obj = ObjectElement::new();
        // Invalid types that can't be converted
        obj.set("summary", Element::Array(ArrayElement::new_empty()));
        obj.set("description", Element::Object(ObjectElement::new()));
        obj.set("externalValue", Element::Array(ArrayElement::new_empty()));

        let example = build_and_decorate_example::<DefaultFolder>(&Element::Object(obj), None);
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify validation errors
        assert!(example.object.meta.properties.contains_key("validationError_summary"));
        assert!(example.object.meta.properties.contains_key("validationError_description"));
        assert!(example.object.meta.properties.contains_key("validationError_externalValue"));
        
        // Verify no reference element class is added due to invalid externalValue
        let has_reference_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "reference-element"
            } else {
                false
            }
        });
        assert!(!has_reference_class);
    }

    #[test]
    fn test_example_fallback_behavior() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("Test example")));
        obj.set("custom-field", Element::String(StringElement::new("custom-value")));
        obj.set("unknown-property", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 42.0,
        }));

        let example = build_and_decorate_example::<DefaultFolder>(&Element::Object(obj), None);
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify fallback fields are preserved
        assert!(example.object.get("custom-field").is_some());
        assert!(example.object.get("unknown-property").is_some());
        
        // Verify fallback metadata
        assert!(example.object.meta.properties.contains_key("fallback_custom-field"));
        assert!(example.object.meta.properties.contains_key("fallback_unknown-property"));
        
        // Verify fixed field is still processed normally
        assert!(example.object.meta.properties.contains_key("fixedField_summary"));
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // Comprehensive test demonstrating TypeScript ExampleVisitor equivalence
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("Comprehensive example")));
        obj.set("description", Element::String(StringElement::new("A complete example with all features")));
        obj.set("externalValue", Element::String(StringElement::new("https://api.example.com/examples/complete.json")));
        
        // Add specification extensions
        obj.set("x-custom-metadata", Element::String(StringElement::new("metadata-value")));
        obj.set("x-vendor-specific", Element::Boolean(BooleanElement::new(true)));
        
        // Add unknown fields
        obj.set("custom-property", Element::String(StringElement::new("custom-value")));

        let mut folder = DefaultFolder;
        let example = build_and_decorate_example(&Element::Object(obj), Some(&mut folder));
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify all TypeScript ExampleVisitor features are implemented:
        
        // 1. Fixed fields processing
        assert_eq!(example.summary().unwrap().content, "Comprehensive example");
        assert_eq!(example.description().unwrap().content, "A complete example with all features");
        assert_eq!(example.external_value().unwrap().content, "https://api.example.com/examples/complete.json");
        
        // 2. Fixed field metadata
        assert!(example.object.meta.properties.contains_key("fixedField_summary"));
        assert!(example.object.meta.properties.contains_key("fixedField_description"));
        assert!(example.object.meta.properties.contains_key("fixedField_externalValue"));
        
        // 3. Reference element classification (externalValue present)
        let has_reference_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "reference-element"
            } else {
                false
            }
        });
        assert!(has_reference_class);
        assert!(example.object.meta.properties.contains_key("isReferenceElement"));
        
        // 4. Specification extensions support
        assert!(example.object.get("x-custom-metadata").is_some());
        assert!(example.object.get("x-vendor-specific").is_some());
        assert!(example.object.meta.properties.contains_key("specificationExtension_x-custom-metadata"));
        assert!(example.object.meta.properties.contains_key("specificationExtension_x-vendor-specific"));
        
        // 5. Fallback behavior
        assert!(example.object.get("custom-property").is_some());
        assert!(example.object.meta.properties.contains_key("fallback_custom-property"));
        
        // 6. Processing metadata
        assert!(example.object.meta.properties.contains_key("processed"));
        assert!(example.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(example.object.meta.properties.contains_key("fallbackVisitor"));
        
        // 7. SpecPath metadata
        if let Some(SimpleValue::Array(path)) = example.object.meta.properties.get("specPath") {
            assert_eq!(path.len(), 3);
            assert!(matches!(&path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&path[2], SimpleValue::String(s) if s == "Example"));
        }
        
        // 8. Classes
        let has_fixed_field_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "fixed-field"
            } else {
                false
            }
        });
        assert!(has_fixed_field_class);
        
        let has_spec_extension_class = example.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "specification-extension"
            } else {
                false
            }
        });
        assert!(has_spec_extension_class);
    }

    #[test]
    fn test_example_comprehensive_scenario() {
        // Test all supported example scenarios
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("Pet Store Example")));
        obj.set("description", Element::String(StringElement::new("Example of a pet object")));
        
        // Complex value object
        let mut pet_obj = ObjectElement::new();
        pet_obj.set("id", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 1.0,
        }));
        pet_obj.set("name", Element::String(StringElement::new("Fluffy")));
        pet_obj.set("status", Element::String(StringElement::new("available")));
        
        // Nested category object
        let mut category_obj = ObjectElement::new();
        category_obj.set("id", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 1.0,
        }));
        category_obj.set("name", Element::String(StringElement::new("Dogs")));
        pet_obj.set("category", Element::Object(category_obj));
        
        obj.set("value", Element::Object(pet_obj));
        
        // Multiple specification extensions
        obj.set("x-example-id", Element::String(StringElement::new("pet-example-1")));
        obj.set("x-created-by", Element::String(StringElement::new("API Team")));
        obj.set("x-last-updated", Element::String(StringElement::new("2023-01-01")));
        
        // Custom properties
        obj.set("internal-notes", Element::String(StringElement::new("For documentation purposes")));

        let mut folder = DefaultFolder;
        let example = build_and_decorate_example(&Element::Object(obj), Some(&mut folder));
        assert!(example.is_some());
        
        let example = example.unwrap();
        
        // Verify comprehensive functionality
        assert_eq!(example.summary().unwrap().content, "Pet Store Example");
        assert_eq!(example.description().unwrap().content, "Example of a pet object");
        assert!(example.value().is_some());
        
        // Verify complex value structure is preserved
        if let Some(Element::Object(pet_value)) = example.value() {
            assert!(pet_value.get("id").is_some());
            assert!(pet_value.get("name").is_some());
            assert!(pet_value.get("category").is_some());
        }
        
        // Verify all metadata is present
        assert!(example.object.meta.properties.contains_key("processed"));
        assert!(example.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(example.object.meta.properties.contains_key("fallbackVisitor"));
        
        // Verify specification extensions
        assert_eq!(example.object.content.iter().filter(|m| {
            if let Element::String(key) = m.key.as_ref() {
                key.content.starts_with("x-")
            } else {
                false
            }
        }).count(), 3);
        
        // Verify fallback fields
        assert!(example.object.get("internal-notes").is_some());
        assert!(example.object.meta.properties.contains_key("fallback_internal-notes"));
        
        // Verify element type and metadata
        assert_eq!(example.object.element, "example");
        assert!(example.object.meta.properties.contains_key("specPath"));
    }
}