//! # External Documentation Builder Module
//!
//! This module provides enhanced external documentation building functionality that is functionally 
//! equivalent to the TypeScript `ExternalDocumentationVisitor` class. It implements visitor pattern 
//! features including fixed fields support, specification extensions, reference handling, and 
//! comprehensive metadata injection.
//!
//! ## Features
//!
//! ### 1. Fixed Fields Support (FixedFieldsVisitor equivalent)
//! - Processes standard OpenAPI ExternalDocumentation fields: `description`, `url`
//! - Validates field types and provides type conversion
//! - Injects metadata for fixed field processing
//!
//! ### 2. Specification Extensions Support
//! - Handles `x-*` prefixed extension fields
//! - Preserves extension values in the external documentation object
//! - Adds metadata to track specification extensions
//!
//! ### 3. Reference Support
//! - Handles `$ref` fields for external documentation references
//! - Adds metadata for reference tracking
//! - Supports recursive resolution when combined with folders
//!
//! ### 4. Fallback Behavior (FallbackVisitor equivalent)
//! - Preserves unknown fields that don't match fixed fields or extensions
//! - Adds metadata to track fallback processing
//! - Maintains full compatibility with custom properties
//!
//! ### 5. Type Conversion and Validation
//! - Converts various element types to appropriate target types
//! - Validates field constraints and relationships
//! - Provides comprehensive error reporting through metadata
//!
//! ### 6. Recursive Folding
//! - Supports optional folder parameter for recursive element processing
//! - Compatible with any type implementing the `Fold` trait
//! - Enables complex document transformation workflows

use apidom_ast::*;
use crate::elements::external_documentation::ExternalDocumentationElement;

/// Basic external documentation builder - equivalent to simple constructor
pub fn build_external_docs(element: &Element) -> Option<ExternalDocumentationElement> {
    let object = element.as_object()?;
    Some(ExternalDocumentationElement::with_content(object.clone()))
}

/// Enhanced external documentation builder with visitor pattern features
/// Equivalent to TypeScript ExternalDocumentationVisitor with FixedFieldsVisitor and FallbackVisitor
pub fn build_and_decorate_external_docs<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<ExternalDocumentationElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut external_docs = ExternalDocumentationElement::with_content(object.clone());
    
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
            "description" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    external_docs.set_description(string_val);
                    add_fixed_field_metadata(&mut external_docs, "description");
                } else {
                    add_validation_error_metadata(&mut external_docs, "description", "Expected string value");
                }
            },
            "url" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    let url_content = string_val.content.clone();
                    external_docs.set_url(string_val);
                    add_fixed_field_metadata(&mut external_docs, "url");
                    validate_url_format(&mut external_docs, &url_content);
                } else {
                    add_validation_error_metadata(&mut external_docs, "url", "Expected string value");
                }
            },
            // $ref handling
            "$ref" => {
                external_docs.object.set("$ref", processed_value);
                add_ref_metadata(&mut external_docs, "$ref");
            },
            // Specification extensions (x-* fields)
            key if key.starts_with("x-") => {
                external_docs.object.set(&key_str, processed_value);
                add_specification_extension_metadata(&mut external_docs, &key_str);
            },
            // Fallback for unknown fields
            _ => {
                external_docs.object.set(&key_str, processed_value);
                add_fallback_metadata(&mut external_docs, &key_str);
            }
        }
    }
    
    // Add processing metadata
    add_processing_metadata(&mut external_docs);
    add_spec_path_metadata(&mut external_docs);
    validate_external_docs(&mut external_docs);
    
    Some(external_docs)
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
fn add_fixed_field_metadata(external_docs: &mut ExternalDocumentationElement, _field_name: &str) {
    let key = format!("fixed-field_{}", _field_name);
    external_docs.object.meta.properties.insert(key, SimpleValue::bool(true));
    external_docs.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(external_docs: &mut ExternalDocumentationElement, _field_name: &str) {
    let key = format!("ref_{}", _field_name);
    external_docs.object.meta.properties.insert(key, SimpleValue::bool(true));
    external_docs.object.meta.properties.insert("referenced-element".to_string(), SimpleValue::string("externalDocumentation".to_string()));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(external_docs: &mut ExternalDocumentationElement, _field_name: &str) {
    let key = format!("specification-extension_{}", _field_name);
    external_docs.object.meta.properties.insert(key, SimpleValue::bool(true));
    external_docs.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(external_docs: &mut ExternalDocumentationElement, _field_name: &str) {
    let key = format!("fallback_{}", _field_name);
    external_docs.object.meta.properties.insert(key, SimpleValue::bool(true));
}

/// Add validation error metadata
fn add_validation_error_metadata(external_docs: &mut ExternalDocumentationElement, _field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", _field_name);
    external_docs.object.meta.properties.insert(key, SimpleValue::string(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(external_docs: &mut ExternalDocumentationElement) {
    external_docs.object.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
    external_docs.object.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::bool(true));
    external_docs.object.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(external_docs: &mut ExternalDocumentationElement) {
    external_docs.object.meta.properties.insert("specPath".to_string(), SimpleValue::array(vec![
        SimpleValue::string("document".to_string()),
        SimpleValue::string("objects".to_string()),
        SimpleValue::string("ExternalDocumentation".to_string())
    ]));
}

/// Validate URL format (basic validation)
fn validate_url_format(external_docs: &mut ExternalDocumentationElement, url: &str) {
    if url.starts_with("http://") || url.starts_with("https://") {
        external_docs.object.meta.properties.insert("validUrl".to_string(), SimpleValue::bool(true));
    } else {
        add_validation_error_metadata(external_docs, "url", "URL should start with http:// or https://");
    }
}

/// Validate that external documentation has required fields
fn validate_external_docs(external_docs: &mut ExternalDocumentationElement) {
    let has_url = external_docs.url().is_some();
    
    if !has_url {
        add_validation_error_metadata(
            external_docs, 
            "externalDocumentation", 
            "ExternalDocumentation must have a url field"
        );
    } else {
        external_docs.object.meta.properties.insert("validExternalDocumentation".to_string(), SimpleValue::bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_external_docs_builder() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Additional documentation")));
        obj.set("url", Element::String(StringElement::new("https://example.com/docs")));

        let external_docs = build_external_docs(&Element::Object(obj));
        assert!(external_docs.is_some());
        
        let external_docs = external_docs.unwrap();
        assert_eq!(external_docs.description().unwrap().content, "Additional documentation");
        assert_eq!(external_docs.url().unwrap().content, "https://example.com/docs");
    }

    #[test]
    fn test_enhanced_external_docs_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("API Documentation")));
        obj.set("url", Element::String(StringElement::new("https://api.example.com/docs")));

        let mut folder = DefaultFolder;
        let external_docs = build_and_decorate_external_docs(&Element::Object(obj), Some(&mut folder));
        assert!(external_docs.is_some());
        
        let external_docs = external_docs.unwrap();
        
        // Verify basic fields
        assert_eq!(external_docs.description().unwrap().content, "API Documentation");
        assert_eq!(external_docs.url().unwrap().content, "https://api.example.com/docs");
        
        // Verify fixed field metadata
        assert!(external_docs.object.meta.properties.contains_key("fixed-field_description"));
        assert!(external_docs.object.meta.properties.contains_key("fixed-field_url"));
        
        // Verify processing metadata
        assert!(external_docs.object.meta.properties.contains_key("processed"));
        assert!(external_docs.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(external_docs.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(external_docs.object.meta.properties.contains_key("specPath"));
        
        // Verify spec path
        if let Some(SimpleValue::Array(path)) = external_docs.object.meta.properties.get("specPath") {
            assert_eq!(path.len(), 3);
            assert!(matches!(&path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&path[2], SimpleValue::String(s) if s == "ExternalDocumentation"));
        }
        
        // Verify validation
        assert!(external_docs.object.meta.properties.contains_key("validUrl"));
        assert!(external_docs.object.meta.properties.contains_key("validExternalDocumentation"));
        
        // Verify fixed-field classes
        let has_fixed_field_class = external_docs.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "fixed-field"
            } else {
                false
            }
        });
        assert!(has_fixed_field_class);
    }

    #[test]
    fn test_external_docs_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("url", Element::String(StringElement::new("https://docs.example.com")));
        obj.set("x-internal-docs", Element::String(StringElement::new("internal-link")));
        obj.set("x-doc-version", Element::String(StringElement::new("v2.1")));
        obj.set("x-team-owner", Element::String(StringElement::new("API Team")));

        let external_docs = build_and_decorate_external_docs::<DefaultFolder>(&Element::Object(obj), None);
        assert!(external_docs.is_some());
        
        let external_docs = external_docs.unwrap();
        
        // Verify specification extensions are preserved
        assert!(external_docs.object.get("x-internal-docs").is_some());
        assert!(external_docs.object.get("x-doc-version").is_some());
        assert!(external_docs.object.get("x-team-owner").is_some());
        
        // Verify specification extension metadata
        assert!(external_docs.object.meta.properties.contains_key("specification-extension_x-internal-docs"));
        assert!(external_docs.object.meta.properties.contains_key("specification-extension_x-doc-version"));
        assert!(external_docs.object.meta.properties.contains_key("specification-extension_x-team-owner"));
        
        // Verify specification extension classes
        let has_spec_extension_class = external_docs.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "specification-extension"
            } else {
                false
            }
        });
        assert!(has_spec_extension_class);
    }

    #[test]
    fn test_external_docs_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/externalDocs/mainDocs")));

        let external_docs = build_and_decorate_external_docs::<DefaultFolder>(&Element::Object(obj), None);
        assert!(external_docs.is_some());
        
        let external_docs = external_docs.unwrap();
        
        // Verify $ref is preserved
        assert!(external_docs.object.get("$ref").is_some());
        
        // Verify reference metadata
        assert!(external_docs.object.meta.properties.contains_key("ref_$ref"));
        assert!(external_docs.object.meta.properties.contains_key("referenced-element"));
        if let Some(SimpleValue::String(ref_type)) = external_docs.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_type, "externalDocumentation");
        }
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // Comprehensive test demonstrating TypeScript ExternalDocumentationVisitor equivalence
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Comprehensive API Documentation")));
        obj.set("url", Element::String(StringElement::new("https://api.example.com/docs")));
        
        // Add specification extensions
        obj.set("x-doc-language", Element::String(StringElement::new("en")));
        obj.set("x-last-updated", Element::String(StringElement::new("2023-12-01")));
        
        // Add unknown fields
        obj.set("custom-property", Element::String(StringElement::new("custom-value")));

        let mut folder = DefaultFolder;
        let external_docs = build_and_decorate_external_docs(&Element::Object(obj), Some(&mut folder));
        assert!(external_docs.is_some());
        
        let external_docs = external_docs.unwrap();
        
        // Verify all TypeScript ExternalDocumentationVisitor features are implemented:
        
        // 1. Fixed fields processing
        assert_eq!(external_docs.description().unwrap().content, "Comprehensive API Documentation");
        assert_eq!(external_docs.url().unwrap().content, "https://api.example.com/docs");
        
        // 2. Fixed field metadata
        assert!(external_docs.object.meta.properties.contains_key("fixed-field_description"));
        assert!(external_docs.object.meta.properties.contains_key("fixed-field_url"));
        
        // 3. Specification extensions support
        assert!(external_docs.object.get("x-doc-language").is_some());
        assert!(external_docs.object.get("x-last-updated").is_some());
        assert!(external_docs.object.meta.properties.contains_key("specification-extension_x-doc-language"));
        assert!(external_docs.object.meta.properties.contains_key("specification-extension_x-last-updated"));
        
        // 4. Fallback behavior
        assert!(external_docs.object.get("custom-property").is_some());
        assert!(external_docs.object.meta.properties.contains_key("fallback_custom-property"));
        
        // 5. Processing metadata
        assert!(external_docs.object.meta.properties.contains_key("processed"));
        assert!(external_docs.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(external_docs.object.meta.properties.contains_key("fallbackVisitor"));
        
        // 6. SpecPath metadata
        if let Some(SimpleValue::Array(path)) = external_docs.object.meta.properties.get("specPath") {
            assert_eq!(path.len(), 3);
            assert!(matches!(&path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&path[2], SimpleValue::String(s) if s == "ExternalDocumentation"));
        }
        
        // 7. Validation
        assert!(external_docs.object.meta.properties.contains_key("validUrl"));
        assert!(external_docs.object.meta.properties.contains_key("validExternalDocumentation"));
        
        // 8. Classes
        let has_fixed_field_class = external_docs.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "fixed-field"
            } else {
                false
            }
        });
        assert!(has_fixed_field_class);
        
        let has_spec_extension_class = external_docs.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "specification-extension"
            } else {
                false
            }
        });
        assert!(has_spec_extension_class);
    }
}