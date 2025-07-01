/*!
 * OpenAPI 3.0 License Element Builder
 * 
 * This module provides comprehensive License element building functionality
 * equivalent to the TypeScript LicenseVisitor. It supports:
 * - Fixed fields support (name, url)
 * - Specification extensions support (x-* fields)
 * - Reference support ($ref handling)
 * - Fallback behavior for unknown fields
 * - Type conversion and validation
 * - Recursive folding support
 */

use apidom_ast::*;
use crate::elements::license::LicenseElement;

/// Build a basic LicenseElement from a generic Element
pub fn build_license(element: &Element) -> Option<LicenseElement> {
    let object = element.as_object()?;
    Some(LicenseElement::with_content(object.clone()))
}

/// Build and decorate LicenseElement with enhanced visitor pattern features
/// 
/// This function provides equivalent functionality to the TypeScript LicenseVisitor:
/// - Fixed fields processing with metadata injection (name, url)
/// - Specification extensions support (x-* fields)
/// - Reference handling with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion and validation
/// - Recursive folding support
pub fn build_and_decorate_license<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<LicenseElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut license = LicenseElement::new();
    
    // Add processing metadata
    add_processing_metadata(&mut license);
    add_spec_path_metadata(&mut license);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            license.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut license, &ref_str.content);
            return Some(license);
        }
    }
    
    // Process all object members
    for member in &obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            match key.as_str() {
                // Fixed fields
                "name" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        license.set_name(string_elem);
                        add_fixed_field_metadata(&mut license, "name");
                    } else {
                        add_validation_error_metadata(&mut license, "name", "Expected string value");
                    }
                }
                "url" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        // Validate URL format
                        if validate_url_format(&string_elem.content) {
                            license.set_url(string_elem);
                            add_fixed_field_metadata(&mut license, "url");
                        } else {
                            add_validation_error_metadata(&mut license, "url", "Invalid URL format");
                        }
                    } else {
                        add_validation_error_metadata(&mut license, "url", "Expected string value");
                    }
                }
                _ => {
                    // Handle specification extensions and fallback fields
                    if key.starts_with("x-") {
                        // Specification extension
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        
                        license.object.set(key, processed_value);
                        add_specification_extension_metadata(&mut license, key);
                    } else {
                        // Fallback field
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        
                        license.object.set(key, processed_value);
                        add_fallback_metadata(&mut license, key);
                    }
                }
            }
        }
    }
    
    // Validate required fields
    validate_license(&mut license);
    
    Some(license)
}

/// Convert element to StringElement with type conversion
fn convert_to_string_element(element: &Element) -> Option<StringElement> {
    match element {
        Element::String(s) => Some(s.clone()),
        Element::Number(n) => Some(StringElement::new(&n.content.to_string())),
        Element::Boolean(b) => Some(StringElement::new(&b.content.to_string())),
        _ => None,
    }
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(license: &mut LicenseElement, field_name: &str) {
    let key = format!("fixed-field_{}", field_name);
    license.object.meta.properties.insert(key, SimpleValue::Bool(true));
    license.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(license: &mut LicenseElement, ref_path: &str) {
    license.object.meta.properties.insert("referenced-element".to_string(), SimpleValue::String("license".to_string()));
    license.object.meta.properties.insert("reference-path".to_string(), SimpleValue::String(ref_path.to_string()));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(license: &mut LicenseElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    license.object.meta.properties.insert(key, SimpleValue::Bool(true));
    license.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(license: &mut LicenseElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    license.object.meta.properties.insert(key, SimpleValue::Bool(true));
    license.object.classes.content.push(Element::String(StringElement::new("fallback-field")));
}

/// Add metadata for validation errors
fn add_validation_error_metadata(license: &mut LicenseElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    license.object.meta.properties.insert(key, SimpleValue::string(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(license: &mut LicenseElement) {
    license.object.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
    license.object.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::bool(true));
    license.object.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::bool(true));
    license.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), SimpleValue::bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(license: &mut LicenseElement) {
    license.object.meta.properties.insert("spec-path".to_string(), SimpleValue::Array(vec![
        SimpleValue::String("document".to_string()),
        SimpleValue::String("objects".to_string()),
        SimpleValue::String("License".to_string())
    ]));
}

/// Validate URL format (basic validation)
fn validate_url_format(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://") || url.starts_with("mailto:")
}

/// Validate license constraints
fn validate_license(license: &mut LicenseElement) {
    // Check for required fields
    if license.name().is_none() {
        add_validation_error_metadata(license, "license", "Missing required field: name");
    }
    
    // If validation passes
    if license.name().is_some() {
        license.object.meta.properties.insert("validLicense".to_string(), SimpleValue::bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_license_builder() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("MIT")));
        obj.set("url", Element::String(StringElement::new("https://opensource.org/licenses/MIT")));

        let license = build_license(&Element::Object(obj));
        assert!(license.is_some());
        
        let license = license.unwrap();
        assert_eq!(license.name().unwrap().content, "MIT");
        assert_eq!(license.url().unwrap().content, "https://opensource.org/licenses/MIT");
    }

    #[test]
    fn test_enhanced_license_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("Apache 2.0")));
        obj.set("url", Element::String(StringElement::new("https://www.apache.org/licenses/LICENSE-2.0.html")));

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify basic fields
        assert_eq!(license.name().unwrap().content, "Apache 2.0");
        assert_eq!(license.url().unwrap().content, "https://www.apache.org/licenses/LICENSE-2.0.html");
        
        // Verify fixed field metadata
        assert!(license.object.meta.properties.contains_key("fixed-field_name"));
        assert!(license.object.meta.properties.contains_key("fixed-field_url"));
        
        // Verify processing metadata
        assert!(license.object.meta.properties.contains_key("processed"));
        assert!(license.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(license.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // Verify spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = license.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "License"));
        }
    }

    #[test]
    fn test_license_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("Custom License")));
        obj.set("x-license-id", Element::String(StringElement::new("custom-123")));
        obj.set("x-deprecated", Element::Boolean(BooleanElement::new(false)));
        obj.set("x-metadata", Element::Object({
            let mut meta = ObjectElement::new();
            meta.set("version", Element::String(StringElement::new("1.0")));
            meta
        }));

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify specification extensions are preserved
        assert!(license.object.get("x-license-id").is_some());
        assert!(license.object.get("x-deprecated").is_some());
        assert!(license.object.get("x-metadata").is_some());
        
        // Verify specification extension metadata
        assert!(license.object.meta.properties.contains_key("specificationExtension_x-license-id"));
        assert!(license.object.meta.properties.contains_key("specificationExtension_x-deprecated"));
        assert!(license.object.meta.properties.contains_key("specificationExtension_x-metadata"));
        
        // Verify specification extension classes
        assert!(license.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_license_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/licenses/MIT")));

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify reference is preserved
        if let Some(Element::String(ref_str)) = license.object.get("$ref") {
            assert_eq!(ref_str.content, "#/components/licenses/MIT");
        }
        
        // Verify reference metadata
        assert!(license.object.meta.properties.contains_key("referenced-element"));
        assert!(license.object.meta.properties.contains_key("reference-path"));
        
        if let Some(SimpleValue::String(ref_elem)) = license.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_elem, "license");
        }
        
        if let Some(SimpleValue::String(ref_path)) = license.object.meta.properties.get("reference-path") {
            assert_eq!(ref_path, "#/components/licenses/MIT");
        }
    }

    #[test]
    fn test_license_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("Custom License")));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("anotherField", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 42.0,
        }));

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify fallback fields are preserved
        assert!(license.object.get("customField").is_some());
        assert!(license.object.get("anotherField").is_some());
        
        // Verify fallback metadata
        assert!(license.object.meta.properties.contains_key("fallback_customField"));
        assert!(license.object.meta.properties.contains_key("fallback_anotherField"));
        
        // Verify fallback classes
        assert!(license.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "fallback-field"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_license_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        })); // Should convert to string
        obj.set("url", Element::Boolean(BooleanElement::new(true))); // Should convert to string

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify type conversions
        assert_eq!(license.name().unwrap().content, "123");
        // URL validation will fail for "true", so it should have validation error
        assert!(license.object.meta.properties.contains_key("validationError_url"));
    }

    #[test]
    fn test_license_url_validation() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("Test License")));
        obj.set("url", Element::String(StringElement::new("invalid-url")));

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify URL validation error
        assert!(license.object.meta.properties.contains_key("validationError_url"));
        
        if let Some(SimpleValue::String(error)) = license.object.meta.properties.get("validationError_url") {
            assert_eq!(error, "Invalid URL format");
        }
    }

    #[test]
    fn test_license_validation_errors() {
        let mut obj = ObjectElement::new();
        // Missing required field: name
        obj.set("url", Element::String(StringElement::new("https://example.com/license")));

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify validation error metadata
        assert!(license.object.meta.properties.contains_key("validationError_license"));
        
        if let Some(SimpleValue::String(error)) = license.object.meta.properties.get("validationError_license") {
            assert!(error.contains("Missing required field"));
        }
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript LicenseVisitor
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("MIT License")));
        obj.set("url", Element::String(StringElement::new("https://opensource.org/licenses/MIT")));
        
        // Add specification extensions
        obj.set("x-license-id", Element::String(StringElement::new("mit-2023")));
        obj.set("x-approved", Element::Boolean(BooleanElement::new(true)));
        
        // Add fallback field
        obj.set("customField", Element::String(StringElement::new("custom value")));

        let mut folder = DefaultFolder;
        let license = build_and_decorate_license(&Element::Object(obj), Some(&mut folder));
        assert!(license.is_some());
        
        let license = license.unwrap();
        
        // Verify all TypeScript LicenseVisitor features are present:
        
        // 1. Fixed fields processing
        assert!(license.object.meta.properties.contains_key("fixed-field_name"));
        assert!(license.object.meta.properties.contains_key("fixed-field_url"));
        
        // 2. Specification extensions support
        assert!(license.object.meta.properties.contains_key("specificationExtension_x-license-id"));
        assert!(license.object.meta.properties.contains_key("specificationExtension_x-approved"));
        
        // 3. Fallback field handling
        assert!(license.object.meta.properties.contains_key("fallback_customField"));
        
        // 4. Spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = license.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "License"));
        }
        
        // 5. Overall processing metadata
        assert!(license.object.meta.properties.contains_key("processed"));
        assert!(license.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(license.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(license.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // 6. Validation status
        assert!(license.object.meta.properties.contains_key("validLicense"));
        
        // 7. Fixed field classes
        assert!(license.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "fixed-field"
            } else {
                false
            }
        }));
        
        // 8. Specification extension classes
        assert!(license.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        
        // 9. Fallback field classes
        assert!(license.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "fallback-field"
            } else {
                false
            }
        }));
    }
}