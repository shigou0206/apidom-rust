//! # Contact Builder Module
//!
//! This module provides enhanced contact building functionality that is functionally equivalent
//! to the TypeScript `ContactVisitor` class. It implements visitor pattern features including
//! fixed fields support, specification extensions, fallback behavior, and comprehensive metadata injection.
//!
//! ## Features
//!
//! ### 1. Fixed Fields Support (FixedFieldsVisitor equivalent)
//! - Processes standard OpenAPI Contact fields: `name`, `url`, `email`
//! - Validates field types and provides type conversion
//! - Injects metadata for fixed field processing
//!
//! ### 2. Specification Extensions Support
//! - Handles `x-*` prefixed extension fields
//! - Preserves extension values in the contact object
//! - Adds metadata to track specification extensions
//!
//! ### 3. Fallback Behavior (FallbackVisitor equivalent)
//! - Preserves unknown fields that don't match fixed fields or extensions
//! - Adds metadata to track fallback processing
//! - Maintains full compatibility with custom properties
//!
//! ### 4. Reference Support
//! - Handles `$ref` fields for contact references
//! - Adds metadata for reference tracking
//! - Supports recursive resolution when combined with folders
//!
//! ### 5. Type Conversion and Validation
//! - Converts number and boolean values to strings when needed
//! - Validates that contacts have at least one contact method
//! - Provides comprehensive error reporting through metadata
//!
//! ### 6. Recursive Folding
//! - Supports optional folder parameter for recursive element processing
//! - Compatible with any type implementing the `Fold` trait
//! - Enables complex document transformation workflows
//!
//! ## Usage Examples
//!
//! ```rust
//! use apidom_ast::fold::DefaultFolder;
//! use apidom_ast::minim_model::*;
//! use apidom_ns_openapi_3_0::builder::contact_builder::*;
//!
//! // Create a sample element for testing
//! let mut obj = ObjectElement::new();
//! obj.set("name", Element::String(StringElement::new("John Doe")));
//! let element = Element::Object(obj);
//!
//! // Basic usage
//! let contact = build_contact(&element);
//!
//! // Enhanced usage with visitor pattern features
//! let mut folder = DefaultFolder;
//! let contact = build_and_decorate_contact(&element, Some(&mut folder));
//! ```
//!
//! ## TypeScript Equivalence
//!
//! This implementation provides feature parity with the TypeScript `ContactVisitor` class:
//! - ✅ Fixed fields processing
//! - ✅ Specification extensions support
//! - ✅ Fallback behavior for unknown fields
//! - ✅ Recursive folding capability
//! - ✅ Comprehensive metadata injection
//! - ✅ Type validation and conversion
//! - ✅ Reference handling

use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::contact::ContactElement;

/// Basic contact builder - equivalent to simple constructor
pub fn build_contact(element: &Element) -> Option<ContactElement> {
    let object = element.as_object()?;
    Some(ContactElement::with_content(object.clone()))
}

/// Enhanced contact builder with visitor pattern features
/// Equivalent to TypeScript ContactVisitor with FixedFieldsVisitor and FallbackVisitor
pub fn build_and_decorate_contact<F>(
    element: &Element, 
    mut folder: Option<&mut F>
) -> Option<ContactElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut contact = ContactElement::with_content(object.clone());
    
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
            "name" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    contact.set_name(string_val);
                    add_fixed_field_metadata(&mut contact, "name");
                    add_type_conversion_metadata(&mut contact, "name", "string");
                } else {
                    add_validation_error_metadata(&mut contact, "name", "Expected string value");
                }
            },
            "url" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    contact.set_url(string_val);
                    add_fixed_field_metadata(&mut contact, "url");
                    add_type_conversion_metadata(&mut contact, "url", "string");
                } else {
                    add_validation_error_metadata(&mut contact, "url", "Expected string value");
                }
            },
            "email" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    contact.set_email(string_val);
                    add_fixed_field_metadata(&mut contact, "email");
                    add_type_conversion_metadata(&mut contact, "email", "string");
                } else {
                    add_validation_error_metadata(&mut contact, "email", "Expected string value");
                }
            },
            // Specification extensions (x-* fields)
            key if key.starts_with("x-") => {
                contact.object.set(&key_str, processed_value);
                add_specification_extension_metadata(&mut contact, &key_str);
            },
            // $ref handling
            "$ref" => {
                contact.object.set("$ref", processed_value);
                add_ref_metadata(&mut contact, "$ref");
            },
            // Fallback for unknown fields
            _ => {
                contact.object.set(&key_str, processed_value);
                add_fallback_metadata(&mut contact, &key_str);
            }
        }
    }
    
    // Validate contact has at least one contact method
    validate_contact(&mut contact);
    
    // Add processing metadata
    add_processing_metadata(&mut contact);
    
    Some(contact)
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
fn add_fixed_field_metadata(contact: &mut ContactElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    contact.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for type conversions
fn add_type_conversion_metadata(contact: &mut ContactElement, field_name: &str, expected_type: &str) {
    let key = format!("typeConversion_{}", field_name);
    contact.object.meta.properties.insert(key, Value::String(expected_type.to_string()));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(contact: &mut ContactElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    contact.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for references
fn add_ref_metadata(contact: &mut ContactElement, field_name: &str) {
    let key = format!("ref_{}", field_name);
    contact.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(contact: &mut ContactElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    contact.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add validation error metadata
fn add_validation_error_metadata(contact: &mut ContactElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    contact.object.meta.properties.insert(key, Value::String(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(contact: &mut ContactElement) {
    contact.object.meta.properties.insert("processed".to_string(), Value::Bool(true));
}

/// Validate that contact has at least one contact method
fn validate_contact(contact: &mut ContactElement) {
    let has_name = contact.name().is_some();
    let has_url = contact.url().is_some();
    let has_email = contact.email().is_some();
    
    if !has_name && !has_url && !has_email {
        add_validation_error_metadata(
            contact, 
            "contact", 
            "Contact must have at least one of: name, url, or email"
        );
    } else {
        contact.object.meta.properties.insert("validContact".to_string(), Value::Bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::fold::DefaultFolder;

    #[test]
    fn test_basic_contact_builder() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("John Doe")));
        obj.set("email", Element::String(StringElement::new("john@example.com")));

        let contact = build_contact(&Element::Object(obj));
        assert!(contact.is_some());
        
        let contact = contact.unwrap();
        assert_eq!(contact.name().unwrap().content, "John Doe");
        assert_eq!(contact.email().unwrap().content, "john@example.com");
    }

    #[test]
    fn test_enhanced_contact_builder_with_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("API Support Team")));
        obj.set("url", Element::String(StringElement::new("https://api.example.com")));
        obj.set("x-team-id", Element::String(StringElement::new("team-123")));
        obj.set("x-slack-channel", Element::String(StringElement::new("#api-support")));

        let mut folder = DefaultFolder;
        let contact = build_and_decorate_contact(&Element::Object(obj), Some(&mut folder));
        assert!(contact.is_some());
        
        let contact = contact.unwrap();
        
        // Verify basic fields
        assert_eq!(contact.name().unwrap().content, "API Support Team");
        assert_eq!(contact.url().unwrap().content, "https://api.example.com");
        
        // Verify metadata injection
        assert!(contact.object.meta.properties.contains_key("fixedField_name"));
        assert!(contact.object.meta.properties.contains_key("fixedField_url"));
        assert!(contact.object.meta.properties.contains_key("specificationExtension_x-team-id"));
        assert!(contact.object.meta.properties.contains_key("specificationExtension_x-slack-channel"));
        assert!(contact.object.meta.properties.contains_key("processed"));
        assert!(contact.object.meta.properties.contains_key("validContact"));
        
        // Verify specification extensions are preserved
        assert!(contact.object.get("x-team-id").is_some());
        assert!(contact.object.get("x-slack-channel").is_some());
    }

    #[test]
    fn test_contact_type_conversion() {
        let mut obj = ObjectElement::new();
        // Test type conversion from number to string
        obj.set("name", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 12345.0,
        }));
        // Test type conversion from boolean to string
        obj.set("email", Element::Boolean(BooleanElement::new(true)));

        let mut folder = DefaultFolder;
        let contact = build_and_decorate_contact(&Element::Object(obj), Some(&mut folder));
        assert!(contact.is_some());
        
        let contact = contact.unwrap();
        
        // Verify conversions worked
        assert_eq!(contact.name().unwrap().content, "12345");
        assert_eq!(contact.email().unwrap().content, "true");
        
        // Verify type conversion metadata
        assert!(contact.object.meta.properties.contains_key("typeConversion_name"));
        assert!(contact.object.meta.properties.contains_key("typeConversion_email"));
    }

    #[test]
    fn test_contact_validation_errors() {
        // Test empty contact (should fail validation)
        let empty_obj = ObjectElement::new();
        let contact = build_and_decorate_contact::<DefaultFolder>(&Element::Object(empty_obj), None);
        assert!(contact.is_some());
        
        let contact = contact.unwrap();
        assert!(contact.object.meta.properties.contains_key("validationError_contact"));
        assert!(!contact.object.meta.properties.contains_key("validContact"));
    }

    #[test]
    fn test_contact_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/contacts/support")));

        let contact = build_and_decorate_contact::<DefaultFolder>(&Element::Object(obj), None);
        assert!(contact.is_some());
        
        let contact = contact.unwrap();
        
        // Verify $ref is preserved
        assert!(contact.object.get("$ref").is_some());
        assert!(contact.object.meta.properties.contains_key("ref_$ref"));
    }

    #[test]
    fn test_contact_fallback_behavior() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("Test Contact")));
        obj.set("unknown-field", Element::String(StringElement::new("unknown-value")));
        obj.set("custom-property", Element::String(StringElement::new("custom-value")));

        let contact = build_and_decorate_contact::<DefaultFolder>(&Element::Object(obj), None);
        assert!(contact.is_some());
        
        let contact = contact.unwrap();
        
        // Verify fallback fields are preserved
        assert!(contact.object.get("unknown-field").is_some());
        assert!(contact.object.get("custom-property").is_some());
        
        // Verify fallback metadata
        assert!(contact.object.meta.properties.contains_key("fallback_unknown-field"));
        assert!(contact.object.meta.properties.contains_key("fallback_custom-property"));
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // Comprehensive test demonstrating TypeScript ContactVisitor equivalence
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("API Support Team")));
        obj.set("url", Element::String(StringElement::new("https://api.example.com/support")));
        obj.set("email", Element::String(StringElement::new("support@api.example.com")));
        obj.set("x-team-lead", Element::String(StringElement::new("John Doe")));
        obj.set("x-response-time", Element::String(StringElement::new("24h")));
        obj.set("custom-field", Element::String(StringElement::new("custom-value")));

        let mut folder = DefaultFolder;
        let contact = build_and_decorate_contact(&Element::Object(obj), Some(&mut folder));
        assert!(contact.is_some());
        
        let contact = contact.unwrap();
        
        // Verify all TypeScript ContactVisitor features are implemented:
        
        // 1. Fixed fields support
        assert!(contact.object.meta.properties.contains_key("fixedField_name"));
        assert!(contact.object.meta.properties.contains_key("fixedField_url"));
        assert!(contact.object.meta.properties.contains_key("fixedField_email"));
        
        // 2. Specification extensions support
        assert!(contact.object.meta.properties.contains_key("specificationExtension_x-team-lead"));
        assert!(contact.object.meta.properties.contains_key("specificationExtension_x-response-time"));
        
        // 3. Fallback behavior for unknown fields
        assert!(contact.object.meta.properties.contains_key("fallback_custom-field"));
        
        // 4. Processing metadata
        assert!(contact.object.meta.properties.contains_key("processed"));
        
        // 5. Validation metadata
        assert!(contact.object.meta.properties.contains_key("validContact"));
        
        // 6. All fields are preserved
        assert_eq!(contact.name().unwrap().content, "API Support Team");
        assert_eq!(contact.url().unwrap().content, "https://api.example.com/support");
        assert_eq!(contact.email().unwrap().content, "support@api.example.com");
        assert!(contact.object.get("x-team-lead").is_some());
        assert!(contact.object.get("x-response-time").is_some());
        assert!(contact.object.get("custom-field").is_some());
    }
}