/*!
 * OpenAPI 3.0 Info Element Builder
 * 
 * This module provides comprehensive Info element building functionality
 * equivalent to the TypeScript InfoVisitor. It supports:
 * - Fixed fields support (title, version, description, etc.)
 * - Specification extensions support (x-* fields)
 * - Reference support ($ref handling)
 * - Fallback behavior for unknown fields
 * - Type conversion and validation
 * - Recursive folding support
 * - Contact and License sub-element processing
 * - Version element class injection (api-version, version)
 */

use apidom_ast::*;
use crate::elements::info::InfoElement;
use crate::builder::{build_contact, build_license};

/// Build a basic InfoElement from a generic Element
pub fn build_info(element: &Element) -> Option<InfoElement> {
    let object = element.as_object()?;
    Some(InfoElement::with_content(object.clone()))
}

/// Build and decorate InfoElement with enhanced visitor pattern features
/// 
/// This function provides equivalent functionality to the TypeScript InfoVisitor:
/// - Fixed fields processing with metadata injection
/// - Specification extensions support
/// - Reference handling with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion and validation
/// - Recursive folding support
/// - Contact/License sub-element processing
pub fn build_and_decorate_info<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<InfoElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut info = InfoElement::new();
    
    // Add processing metadata
    add_processing_metadata(&mut info);
    add_spec_path_metadata(&mut info);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            info.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut info, &ref_str.content);
            return Some(info);
        }
    }
    
    // Process all object members
    for member in &obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            match key.as_str() {
                // Fixed fields
                "title" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        info.set_title(string_elem);
                        add_fixed_field_metadata(&mut info, "title");
                    } else {
                        add_validation_error_metadata(&mut info, "title", "Expected string value");
                    }
                }
                "version" => {
                    if let Some(mut string_elem) = convert_to_string_element(value) {
                        // Add version-specific classes (equivalent to VersionVisitor)
                        string_elem.meta.properties.insert("api-version".to_string(), SimpleValue::bool(true));
                        string_elem.meta.properties.insert("version".to_string(), SimpleValue::bool(true));
                        info.set_version(string_elem);
                        add_fixed_field_metadata(&mut info, "version");
                    } else {
                        add_validation_error_metadata(&mut info, "version", "Expected string value");
                    }
                }
                "description" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        info.set_description(string_elem);
                        add_fixed_field_metadata(&mut info, "description");
                    } else {
                        add_validation_error_metadata(&mut info, "description", "Expected string value");
                    }
                }
                "termsOfService" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        info.set_terms_of_service(string_elem);
                        add_fixed_field_metadata(&mut info, "termsOfService");
                    } else {
                        add_validation_error_metadata(&mut info, "termsOfService", "Expected string value");
                    }
                }
                "contact" => {
                    // Process contact with recursive folding
                    let processed_contact = if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    };
                    
                    if let Some(contact_elem) = build_contact(&processed_contact) {
                        info.set_contact(contact_elem);
                        add_fixed_field_metadata(&mut info, "contact");
                    } else {
                        // Fallback: set as generic object
                        if let Some(obj_elem) = processed_contact.as_object() {
                            info.object.set("contact", Element::Object(obj_elem.clone()));
                            add_fallback_metadata(&mut info, "contact");
                        }
                    }
                }
                "license" => {
                    // Process license with recursive folding
                    let processed_license = if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    };
                    
                    if let Some(license_elem) = build_license(&processed_license) {
                        info.set_license(license_elem);
                        add_fixed_field_metadata(&mut info, "license");
                    } else {
                        // Fallback: set as generic object
                        if let Some(obj_elem) = processed_license.as_object() {
                            info.object.set("license", Element::Object(obj_elem.clone()));
                            add_fallback_metadata(&mut info, "license");
                        }
                    }
                }
                _ => {
                    // Handle specification extensions and fallback fields
                    if key.starts_with("x-") {
                        // Specification extension
                        info.object.set(key, value.clone());
                        add_specification_extension_metadata(&mut info, key);
                    } else {
                        // Fallback field
                        info.object.set(key, value.clone());
                        add_fallback_metadata(&mut info, key);
                    }
                }
            }
        }
    }
    
    // Validate required fields
    validate_info(&mut info);
    
    Some(info)
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
fn add_fixed_field_metadata(info: &mut InfoElement, field_name: &str) {
    let key = format!("fixed-field_{}", field_name);
    info.object.meta.properties.insert(key, SimpleValue::Bool(true));
    info.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(info: &mut InfoElement, ref_path: &str) {
    info.object.meta.properties.insert("referenced-element".to_string(), SimpleValue::String("info".to_string()));
    info.object.meta.properties.insert("reference-path".to_string(), SimpleValue::String(ref_path.to_string()));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(info: &mut InfoElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    info.object.meta.properties.insert(key, SimpleValue::bool(true));
    info.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(info: &mut InfoElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    info.object.meta.properties.insert(key, SimpleValue::bool(true));
    info.object.classes.content.push(Element::String(StringElement::new("fallback-field")));
}

/// Add metadata for validation errors
fn add_validation_error_metadata(info: &mut InfoElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    info.object.meta.properties.insert(key, SimpleValue::string(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(info: &mut InfoElement) {
    info.object.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
    info.object.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::bool(true));
    info.object.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::bool(true));
    info.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), SimpleValue::bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(info: &mut InfoElement) {
    info.object.meta.properties.insert("specPath".to_string(), SimpleValue::array(vec![
        SimpleValue::string("document".to_string()),
        SimpleValue::string("objects".to_string()),
        SimpleValue::string("Info".to_string())
    ]));
}

/// Validate info constraints
fn validate_info(info: &mut InfoElement) {
    // Check for required fields
    if info.title().is_none() {
        add_validation_error_metadata(info, "info", "Missing required field: title");
    }
    
    if info.version().is_none() {
        add_validation_error_metadata(info, "info", "Missing required field: version");
    }
    
    // If validation passes
    if info.title().is_some() && info.version().is_some() {
        info.object.meta.properties.insert("validInfo".to_string(), SimpleValue::bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_info_builder() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Pet Store API")));
        obj.set("version", Element::String(StringElement::new("1.0.0")));
        obj.set("description", Element::String(StringElement::new("A sample API")));

        let info = build_info(&Element::Object(obj));
        assert!(info.is_some());
        
        let info = info.unwrap();
        assert_eq!(info.title().unwrap().content, "Pet Store API");
        assert_eq!(info.version().unwrap().content, "1.0.0");
        assert_eq!(info.description().unwrap().content, "A sample API");
    }

    #[test]
    fn test_enhanced_info_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Pet Store API")));
        obj.set("version", Element::String(StringElement::new("1.0.0")));
        obj.set("description", Element::String(StringElement::new("A comprehensive pet store API")));
        obj.set("termsOfService", Element::String(StringElement::new("https://example.com/terms")));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify basic fields
        assert_eq!(info.title().unwrap().content, "Pet Store API");
        assert_eq!(info.version().unwrap().content, "1.0.0");
        assert_eq!(info.description().unwrap().content, "A comprehensive pet store API");
        assert_eq!(info.terms_of_service().unwrap().content, "https://example.com/terms");
        
        // Verify version has special classes
        let version = info.version().unwrap();
        assert!(version.meta.properties.contains_key("api-version"));
        assert!(version.meta.properties.contains_key("version"));
        
        // Verify fixed field metadata
        assert!(info.object.meta.properties.contains_key("fixed-field_title"));
        assert!(info.object.meta.properties.contains_key("fixed-field_version"));
        assert!(info.object.meta.properties.contains_key("fixed-field_description"));
        assert!(info.object.meta.properties.contains_key("fixed-field_termsOfService"));
    }

    #[test]
    fn test_info_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Extended API")));
        obj.set("version", Element::String(StringElement::new("2.0.0")));
        obj.set("x-api-id", Element::String(StringElement::new("pet-store-v2")));
        obj.set("x-logo", Element::Object({
            let mut logo = ObjectElement::new();
            logo.set("url", Element::String(StringElement::new("https://example.com/logo.png")));
            logo
        }));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify specification extensions are preserved
        assert!(info.object.get("x-api-id").is_some());
        assert!(info.object.get("x-logo").is_some());
        
        // Verify specification extension metadata
        assert!(info.object.meta.properties.contains_key("specificationExtension_x-api-id"));
        assert!(info.object.meta.properties.contains_key("specificationExtension_x-logo"));
        
        // Verify specification extension classes
        assert!(info.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_info_with_contact_and_license() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Contact API")));
        obj.set("version", Element::String(StringElement::new("1.0.0")));
        
        // Add contact
        obj.set("contact", Element::Object({
            let mut contact = ObjectElement::new();
            contact.set("name", Element::String(StringElement::new("API Support")));
            contact.set("email", Element::String(StringElement::new("support@example.com")));
            contact
        }));
        
        // Add license
        obj.set("license", Element::Object({
            let mut license = ObjectElement::new();
            license.set("name", Element::String(StringElement::new("MIT")));
            license.set("url", Element::String(StringElement::new("https://opensource.org/licenses/MIT")));
            license
        }));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify contact and license are processed
        assert!(info.contact().is_some());
        assert!(info.license().is_some());
        
        // Verify fixed field metadata for sub-elements
        assert!(info.object.meta.properties.contains_key("fixed-field_contact"));
        assert!(info.object.meta.properties.contains_key("fixed-field_license"));
    }

    #[test]
    fn test_info_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/info/PetStoreInfo")));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify reference is preserved
        if let Some(Element::String(ref_str)) = info.object.get("$ref") {
            assert_eq!(ref_str.content, "#/components/info/PetStoreInfo");
        }
        
        // Verify reference metadata
        assert!(info.object.meta.properties.contains_key("referenced-element"));
        assert!(info.object.meta.properties.contains_key("reference-path"));
        
        if let Some(SimpleValue::String(ref_elem)) = info.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_elem, "info");
        }
        
        if let Some(SimpleValue::String(ref_path)) = info.object.meta.properties.get("reference-path") {
            assert_eq!(ref_path, "#/components/info/PetStoreInfo");
        }
    }

    #[test]
    fn test_info_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Fallback API")));
        obj.set("version", Element::String(StringElement::new("1.0.0")));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("anotherField", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 42.0,
        }));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify fallback fields are preserved
        assert!(info.object.get("customField").is_some());
        assert!(info.object.get("anotherField").is_some());
        
        // Verify fallback metadata
        assert!(info.object.meta.properties.contains_key("fallback_customField"));
        assert!(info.object.meta.properties.contains_key("fallback_anotherField"));
        
        // Verify fallback classes
        assert!(info.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "fallback-field"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_info_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        })); // Should convert to string
        obj.set("version", Element::Boolean(BooleanElement::new(true))); // Should convert to string
        obj.set("description", Element::String(StringElement::new("Valid description")));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify type conversions
        assert_eq!(info.title().unwrap().content, "123");
        assert_eq!(info.version().unwrap().content, "true");
        assert_eq!(info.description().unwrap().content, "Valid description");
    }

    #[test]
    fn test_info_validation_errors() {
        let mut obj = ObjectElement::new();
        // Missing required fields: title and version
        obj.set("description", Element::String(StringElement::new("Only description")));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify validation error metadata
        assert!(info.object.meta.properties.contains_key("validationError_info"));
        
        if let Some(SimpleValue::String(error)) = info.object.meta.properties.get("validationError_info") {
            assert!(error.contains("Missing required field"));
        }
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript InfoVisitor
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("TypeScript Equivalent API")));
        obj.set("version", Element::String(StringElement::new("1.0.0")));
        obj.set("description", Element::String(StringElement::new("Demonstrates TypeScript equivalence")));
        obj.set("termsOfService", Element::String(StringElement::new("https://example.com/terms")));
        
        // Add contact
        obj.set("contact", Element::Object({
            let mut contact = ObjectElement::new();
            contact.set("name", Element::String(StringElement::new("API Team")));
            contact.set("email", Element::String(StringElement::new("api@example.com")));
            contact
        }));
        
        // Add specification extensions
        obj.set("x-api-id", Element::String(StringElement::new("typescript-equiv")));
        obj.set("x-internal", Element::Boolean(BooleanElement::new(false)));
        
        // Add fallback field
        obj.set("customMetadata", Element::Object(ObjectElement::new()));

        let mut folder = DefaultFolder;
        let info = build_and_decorate_info(&Element::Object(obj), Some(&mut folder));
        assert!(info.is_some());
        
        let info = info.unwrap();
        
        // Verify all TypeScript InfoVisitor features are present:
        
        // 1. Fixed fields processing
        assert!(info.object.meta.properties.contains_key("fixed-field_title"));
        assert!(info.object.meta.properties.contains_key("fixed-field_version"));
        assert!(info.object.meta.properties.contains_key("fixed-field_description"));
        assert!(info.object.meta.properties.contains_key("fixed-field_termsOfService"));
        assert!(info.object.meta.properties.contains_key("fixed-field_contact"));
        
        // 2. Version element classes (VersionVisitor equivalent)
        let version = info.version().unwrap();
        assert!(version.meta.properties.contains_key("api-version"));
        assert!(version.meta.properties.contains_key("version"));
        
        // 3. Specification extensions support
        assert!(info.object.meta.properties.contains_key("specificationExtension_x-api-id"));
        assert!(info.object.meta.properties.contains_key("specificationExtension_x-internal"));
        
        // 4. Fallback field handling
        assert!(info.object.meta.properties.contains_key("fallback_customMetadata"));
        
        // 5. Spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = info.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "Info"));
        }
        
        // 6. Overall processing metadata
        assert!(info.object.meta.properties.contains_key("processed"));
        assert!(info.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(info.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(info.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // 7. Validation status
        assert!(info.object.meta.properties.contains_key("validInfo"));
    }
} 