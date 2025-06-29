//! # Header Builder Module
//!
//! This module provides enhanced header building functionality that is functionally 
//! equivalent to the TypeScript `HeaderVisitor` class. It implements visitor pattern 
//! features including fixed fields support, specification extensions, reference handling, 
//! schema processing, content processing, and comprehensive metadata injection.
//!
//! ## Features
//!
//! ### 1. Fixed Fields Support (FixedFieldsVisitor equivalent)
//! - Processes standard OpenAPI Header fields: `description`, `required`, `deprecated`, 
//!   `allowEmptyValue`, `style`, `explode`, `allowReserved`, `schema`, `example`, 
//!   `examples`, `content`
//! - Validates field types and provides type conversion
//! - Injects metadata for fixed field processing
//!
//! ### 2. Specification Extensions Support
//! - Handles `x-*` prefixed extension fields
//! - Preserves extension values in the header object
//! - Adds metadata to track specification extensions
//!
//! ### 3. Reference Support (AlternatingVisitor equivalent)
//! - Handles `$ref` fields for header references
//! - Adds metadata for reference tracking with `referenced-element` = "header"
//! - Supports recursive resolution when combined with folders
//!
//! ### 4. Schema Processing (SchemaVisitor equivalent)
//! - Processes `schema` field with support for both Schema objects and $ref
//! - Uses AlternatingVisitor pattern to handle Reference vs Schema elements
//! - Adds appropriate metadata for schema references
//!
//! ### 5. Content Processing (ContentVisitor equivalent)
//! - Processes `content` field containing MediaType objects
//! - Handles nested content structures with proper metadata
//! - Supports content-specific validation and processing
//!
//! ### 6. Fallback Behavior (FallbackVisitor equivalent)
//! - Preserves unknown fields that don't match fixed fields or extensions
//! - Adds metadata to track fallback processing
//! - Maintains full compatibility with custom properties
//!
//! ### 7. Type Conversion and Validation
//! - Converts various element types to appropriate target types
//! - Validates field constraints and relationships
//! - Provides comprehensive error reporting through metadata
//!
//! ### 8. Recursive Folding
//! - Supports optional folder parameter for recursive element processing
//! - Compatible with any type implementing the `Fold` trait
//! - Enables complex document transformation workflows

use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::header::HeaderElement;

/// Basic header builder - equivalent to simple constructor
pub fn build_header(element: &Element) -> Option<HeaderElement> {
    let object = element.as_object()?;
    Some(HeaderElement::with_content(object.clone()))
}

/// Enhanced header builder with visitor pattern features
/// Equivalent to TypeScript HeaderVisitor with FixedFieldsVisitor and FallbackVisitor
pub fn build_and_decorate_header<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<HeaderElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut header = HeaderElement::with_content(object.clone());
    
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
                    header.set_description(string_val);
                    add_fixed_field_metadata(&mut header, "description");
                } else {
                    add_validation_error_metadata(&mut header, "description", "Expected string value");
                }
            },
            "required" => {
                if let Some(bool_val) = convert_to_boolean_element(&processed_value) {
                    header.set_required(bool_val.content);
                    add_fixed_field_metadata(&mut header, "required");
                } else {
                    add_validation_error_metadata(&mut header, "required", "Expected boolean value");
                }
            },
            "deprecated" => {
                if let Some(bool_val) = convert_to_boolean_element(&processed_value) {
                    header.set_deprecated(bool_val.content);
                    add_fixed_field_metadata(&mut header, "deprecated");
                } else {
                    add_validation_error_metadata(&mut header, "deprecated", "Expected boolean value");
                }
            },
            "allowEmptyValue" => {
                if let Some(bool_val) = convert_to_boolean_element(&processed_value) {
                    header.set_allow_empty_value(bool_val);
                    add_fixed_field_metadata(&mut header, "allowEmptyValue");
                } else {
                    add_validation_error_metadata(&mut header, "allowEmptyValue", "Expected boolean value");
                }
            },
            "style" => {
                if let Some(string_val) = convert_to_string_element(&processed_value) {
                    validate_header_style(&mut header, &string_val.content);
                    header.set_style(string_val);
                    add_fixed_field_metadata(&mut header, "style");
                } else {
                    add_validation_error_metadata(&mut header, "style", "Expected string value");
                }
            },
            "explode" => {
                if let Some(bool_val) = convert_to_boolean_element(&processed_value) {
                    header.set_explode(bool_val);
                    add_fixed_field_metadata(&mut header, "explode");
                } else {
                    add_validation_error_metadata(&mut header, "explode", "Expected boolean value");
                }
            },
            "allowReserved" => {
                if let Some(bool_val) = convert_to_boolean_element(&processed_value) {
                    header.set_allow_reserved(bool_val);
                    add_fixed_field_metadata(&mut header, "allowReserved");
                } else {
                    add_validation_error_metadata(&mut header, "allowReserved", "Expected boolean value");
                }
            },
            "schema" => {
                // Schema processing with AlternatingVisitor pattern (Reference vs Schema)
                let schema_element = process_schema_field(&processed_value, &mut header);
                header.set_schema(schema_element);
                add_fixed_field_metadata(&mut header, "schema");
            },
            "example" => {
                header.set_example(processed_value);
                add_fixed_field_metadata(&mut header, "example");
            },
            "examples" => {
                if let Element::Object(obj) = processed_value {
                    header.set_examples(obj);
                    add_fixed_field_metadata(&mut header, "examples");
                } else {
                    add_validation_error_metadata(&mut header, "examples", "Expected object value");
                }
            },
            "content" => {
                // Content processing with ContentVisitor pattern
                if let Element::Object(obj) = processed_value {
                    let content_element = process_content_field(&obj, &mut header);
                    header.set_content(content_element);
                    add_fixed_field_metadata(&mut header, "content");
                } else {
                    add_validation_error_metadata(&mut header, "content", "Expected object value");
                }
            },
            // $ref handling
            "$ref" => {
                header.object.set("$ref", processed_value);
                add_ref_metadata(&mut header, "$ref");
            },
            // Specification extensions (x-* fields)
            key if key.starts_with("x-") => {
                header.object.set(&key_str, processed_value);
                add_specification_extension_metadata(&mut header, &key_str);
            },
            // Fallback for unknown fields
            _ => {
                header.object.set(&key_str, processed_value);
                add_fallback_metadata(&mut header, &key_str);
            }
        }
    }
    
    // Add processing metadata
    add_processing_metadata(&mut header);
    add_spec_path_metadata(&mut header);
    validate_header(&mut header);
    
    Some(header)
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

/// Convert various element types to BooleanElement
fn convert_to_boolean_element(element: &Element) -> Option<BooleanElement> {
    match element {
        Element::Boolean(b) => Some(b.clone()),
        Element::String(s) => {
            match s.content.to_lowercase().as_str() {
                "true" => Some(BooleanElement::new(true)),
                "false" => Some(BooleanElement::new(false)),
                _ => None,
            }
        },
        _ => None,
    }
}

/// Process schema field with AlternatingVisitor pattern
fn process_schema_field(element: &Element, header: &mut HeaderElement) -> Element {
    // Check if it's a reference-like element
    if is_reference_like_element(element) {
        // Add reference metadata
        if let Element::Object(obj) = element {
            if obj.get("$ref").is_some() {
                header.object.meta.properties.insert("schema-referenced-element".to_string(), Value::String("schema".to_string()));
                add_schema_ref_metadata(header);
            }
        }
    } else {
        // It's a schema object
        add_schema_metadata(header);
    }
    
    element.clone()
}

/// Process content field with ContentVisitor pattern
fn process_content_field(content_obj: &ObjectElement, header: &mut HeaderElement) -> ObjectElement {
    let mut processed_content = content_obj.clone();
    
    // Process each media type in content
    for member in &mut processed_content.content {
        if let Element::Object(ref mut media_type_obj) = *member.value {
            // Add media type metadata
            media_type_obj.meta.properties.insert("media-type".to_string(), Value::Bool(true));
            
            // Process schema within media type if present
            if let Some(schema_element) = media_type_obj.get("schema") {
                if is_reference_like_element(schema_element) {
                    media_type_obj.meta.properties.insert("schema-referenced-element".to_string(), Value::String("schema".to_string()));
                }
            }
        }
    }
    
    add_content_metadata(header);
    processed_content
}

/// Check if element is reference-like (has $ref)
fn is_reference_like_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.get("$ref").is_some()
    } else {
        false
    }
}

/// Validate header style values
fn validate_header_style(header: &mut HeaderElement, style: &str) {
    let valid_styles = ["simple"];
    if valid_styles.contains(&style) {
        header.object.meta.properties.insert("validStyle".to_string(), Value::Bool(true));
    } else {
        add_validation_error_metadata(header, "style", &format!("Invalid style '{}'. Must be one of: {:?}", style, valid_styles));
    }
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(header: &mut HeaderElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    header.object.meta.properties.insert(key, Value::Bool(true));
    header.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(header: &mut HeaderElement, field_name: &str) {
    let key = format!("ref_{}", field_name);
    header.object.meta.properties.insert(key, Value::Bool(true));
    header.object.meta.properties.insert("referenced-element".to_string(), Value::String("header".to_string()));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(header: &mut HeaderElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    header.object.meta.properties.insert(key, Value::Bool(true));
    header.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(header: &mut HeaderElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    header.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for validation errors
fn add_validation_error_metadata(header: &mut HeaderElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    header.object.meta.properties.insert(key, Value::String(error_msg.to_string()));
}

/// Add metadata for schema processing
fn add_schema_metadata(header: &mut HeaderElement) {
    header.object.meta.properties.insert("hasSchema".to_string(), Value::Bool(true));
}

/// Add metadata for schema references
fn add_schema_ref_metadata(header: &mut HeaderElement) {
    header.object.meta.properties.insert("hasSchemaRef".to_string(), Value::Bool(true));
}

/// Add metadata for content processing
fn add_content_metadata(header: &mut HeaderElement) {
    header.object.meta.properties.insert("hasContent".to_string(), Value::Bool(true));
}

/// Add overall processing metadata
fn add_processing_metadata(header: &mut HeaderElement) {
    header.object.meta.properties.insert("processed".to_string(), Value::Bool(true));
    header.object.meta.properties.insert("fixedFieldsVisitor".to_string(), Value::Bool(true));
    header.object.meta.properties.insert("fallbackVisitor".to_string(), Value::Bool(true));
    header.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), Value::Bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(header: &mut HeaderElement) {
    header.object.meta.properties.insert("specPath".to_string(), Value::Array(vec![
        Value::String("document".to_string()),
        Value::String("objects".to_string()),
        Value::String("Header".to_string())
    ]));
}

/// Validate header constraints
fn validate_header(header: &mut HeaderElement) {
    // Check for mutually exclusive content and schema
    let has_content = header.content().is_some();
    let has_schema = header.schema().is_some();
    
    if has_content && has_schema {
        add_validation_error_metadata(
            header, 
            "header", 
            "Header cannot have both 'content' and 'schema' properties"
        );
    } else if !has_content && !has_schema {
        add_validation_error_metadata(
            header, 
            "header", 
            "Header must have either 'content' or 'schema' property"
        );
    } else {
        header.object.meta.properties.insert("validHeader".to_string(), Value::Bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::fold::DefaultFolder;

    #[test]
    fn test_basic_header_builder() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Authorization header")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        obj.set("schema", Element::Object({
            let mut schema = ObjectElement::new();
            schema.set("type", Element::String(StringElement::new("string")));
            schema
        }));

        let header = build_header(&Element::Object(obj));
        assert!(header.is_some());
        
        let header = header.unwrap();
        assert_eq!(header.description().unwrap().content, "Authorization header");
        assert_eq!(header.required(), true);
        assert!(header.schema().is_some());
    }

    #[test]
    fn test_enhanced_header_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("API Key header")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        obj.set("deprecated", Element::Boolean(BooleanElement::new(false)));
        obj.set("style", Element::String(StringElement::new("simple")));
        obj.set("explode", Element::Boolean(BooleanElement::new(false)));
        obj.set("schema", Element::Object({
            let mut schema = ObjectElement::new();
            schema.set("type", Element::String(StringElement::new("string")));
            schema
        }));

        let mut folder = DefaultFolder;
        let header = build_and_decorate_header(&Element::Object(obj), Some(&mut folder));
        assert!(header.is_some());
        
        let header = header.unwrap();
        
        // Verify basic fields
        assert_eq!(header.description().unwrap().content, "API Key header");
        assert_eq!(header.required(), true);
        assert_eq!(header.deprecated(), false);
        assert_eq!(header.style().unwrap().content, "simple");
        assert_eq!(header.explode().unwrap().content, false);
        
        // Verify fixed field metadata
        assert!(header.object.meta.properties.contains_key("fixedField_description"));
        assert!(header.object.meta.properties.contains_key("fixedField_required"));
        assert!(header.object.meta.properties.contains_key("fixedField_deprecated"));
        assert!(header.object.meta.properties.contains_key("fixedField_style"));
        assert!(header.object.meta.properties.contains_key("fixedField_explode"));
        assert!(header.object.meta.properties.contains_key("fixedField_schema"));
        
        // Verify processing metadata
        assert!(header.object.meta.properties.contains_key("processed"));
        assert!(header.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(header.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(header.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        assert!(header.object.meta.properties.contains_key("specPath"));
        
        // Verify spec path
        if let Some(Value::Array(path)) = header.object.meta.properties.get("specPath") {
            assert_eq!(path.len(), 3);
            assert_eq!(path[0], Value::String("document".to_string()));
            assert_eq!(path[1], Value::String("objects".to_string()));
            assert_eq!(path[2], Value::String("Header".to_string()));
        }
        
        // Verify validation
        assert!(header.object.meta.properties.contains_key("validStyle"));
        assert!(header.object.meta.properties.contains_key("validHeader"));
        assert!(header.object.meta.properties.contains_key("hasSchema"));
        
        // Verify fixed-field classes
        let has_fixed_field_class = header.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "fixed-field"
            } else {
                false
            }
        });
        assert!(has_fixed_field_class);
    }

    #[test]
    fn test_header_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Custom header")));
        obj.set("schema", Element::Object({
            let mut schema = ObjectElement::new();
            schema.set("type", Element::String(StringElement::new("string")));
            schema
        }));
        obj.set("x-custom-validation", Element::String(StringElement::new("pattern")));
        obj.set("x-header-source", Element::String(StringElement::new("client")));
        obj.set("x-deprecated-version", Element::String(StringElement::new("2.0")));

        let header = build_and_decorate_header::<DefaultFolder>(&Element::Object(obj), None);
        assert!(header.is_some());
        
        let header = header.unwrap();
        
        // Verify specification extensions are preserved
        assert!(header.object.get("x-custom-validation").is_some());
        assert!(header.object.get("x-header-source").is_some());
        assert!(header.object.get("x-deprecated-version").is_some());
        
        // Verify specification extension metadata
        assert!(header.object.meta.properties.contains_key("specificationExtension_x-custom-validation"));
        assert!(header.object.meta.properties.contains_key("specificationExtension_x-header-source"));
        assert!(header.object.meta.properties.contains_key("specificationExtension_x-deprecated-version"));
        
        // Verify specification extension classes
        let has_spec_extension_class = header.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "specification-extension"
            } else {
                false
            }
        });
        assert!(has_spec_extension_class);
    }

    #[test]
    fn test_header_with_schema_ref() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Referenced schema header")));
        obj.set("schema", Element::Object({
            let mut schema_ref = ObjectElement::new();
            schema_ref.set("$ref", Element::String(StringElement::new("#/components/schemas/ApiKey")));
            schema_ref
        }));

        let header = build_and_decorate_header::<DefaultFolder>(&Element::Object(obj), None);
        assert!(header.is_some());
        
        let header = header.unwrap();
        
        // Verify schema reference is preserved
        if let Some(Element::Object(schema_obj)) = header.schema() {
            assert!(schema_obj.get("$ref").is_some());
        }
        
        // Verify schema reference metadata
        assert!(header.object.meta.properties.contains_key("schema-referenced-element"));
        assert!(header.object.meta.properties.contains_key("hasSchemaRef"));
        if let Some(Value::String(ref_type)) = header.object.meta.properties.get("schema-referenced-element") {
            assert_eq!(ref_type, "schema");
        }
    }

    #[test]
    fn test_header_with_content() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Content header")));
        
        let mut content = ObjectElement::new();
        let mut media_type = ObjectElement::new();
        media_type.set("schema", Element::Object({
            let mut schema = ObjectElement::new();
            schema.set("type", Element::String(StringElement::new("string")));
            schema
        }));
        content.set("application/json", Element::Object(media_type));
        obj.set("content", Element::Object(content));

        let header = build_and_decorate_header::<DefaultFolder>(&Element::Object(obj), None);
        assert!(header.is_some());
        
        let header = header.unwrap();
        
        // Verify content is preserved
        assert!(header.content().is_some());
        
        // Verify content metadata
        assert!(header.object.meta.properties.contains_key("hasContent"));
        assert!(header.object.meta.properties.contains_key("fixedField_content"));
        
        // Verify validation
        assert!(header.object.meta.properties.contains_key("validHeader"));
    }

    #[test]
    fn test_header_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/headers/ApiKeyHeader")));

        let header = build_and_decorate_header::<DefaultFolder>(&Element::Object(obj), None);
        assert!(header.is_some());
        
        let header = header.unwrap();
        
        // Verify $ref is preserved
        assert!(header.object.get("$ref").is_some());
        
        // Verify reference metadata
        assert!(header.object.meta.properties.contains_key("ref_$ref"));
        assert!(header.object.meta.properties.contains_key("referenced-element"));
        if let Some(Value::String(ref_type)) = header.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_type, "header");
        }
    }

    #[test]
    fn test_header_validation_errors() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Invalid header")));
        obj.set("required", Element::String(StringElement::new("not-a-boolean")));
        obj.set("style", Element::String(StringElement::new("invalid-style")));
        // Both content and schema (invalid)
        obj.set("schema", Element::Object(ObjectElement::new()));
        obj.set("content", Element::Object(ObjectElement::new()));

        let header = build_and_decorate_header::<DefaultFolder>(&Element::Object(obj), None);
        assert!(header.is_some());
        
        let header = header.unwrap();
        
        // Verify validation errors
        assert!(header.object.meta.properties.contains_key("validationError_required"));
        assert!(header.object.meta.properties.contains_key("validationError_style"));
        assert!(header.object.meta.properties.contains_key("validationError_header"));
        
        // Should not be valid
        assert!(!header.object.meta.properties.contains_key("validHeader"));
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // Comprehensive test demonstrating TypeScript HeaderVisitor equivalence
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Comprehensive API Header")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        obj.set("deprecated", Element::Boolean(BooleanElement::new(false)));
        obj.set("style", Element::String(StringElement::new("simple")));
        obj.set("explode", Element::Boolean(BooleanElement::new(false)));
        obj.set("allowReserved", Element::Boolean(BooleanElement::new(true)));
        
        // Add schema with reference
        obj.set("schema", Element::Object({
            let mut schema_ref = ObjectElement::new();
            schema_ref.set("$ref", Element::String(StringElement::new("#/components/schemas/HeaderValue")));
            schema_ref
        }));
        
        // Add specification extensions
        obj.set("x-header-category", Element::String(StringElement::new("authentication")));
        obj.set("x-validation-rules", Element::String(StringElement::new("strict")));
        
        // Add unknown fields
        obj.set("custom-property", Element::String(StringElement::new("custom-value")));

        let mut folder = DefaultFolder;
        let header = build_and_decorate_header(&Element::Object(obj), Some(&mut folder));
        assert!(header.is_some());
        
        let header = header.unwrap();
        
        // Verify all TypeScript HeaderVisitor features are implemented:
        
        // 1. Fixed fields processing (FixedFieldsVisitor)
        assert_eq!(header.description().unwrap().content, "Comprehensive API Header");
        assert_eq!(header.required(), true);
        assert_eq!(header.deprecated(), false);
        assert_eq!(header.style().unwrap().content, "simple");
        assert_eq!(header.explode().unwrap().content, false);
        assert_eq!(header.allow_reserved().unwrap().content, true);
        
        // 2. Fixed field metadata
        assert!(header.object.meta.properties.contains_key("fixedField_description"));
        assert!(header.object.meta.properties.contains_key("fixedField_required"));
        assert!(header.object.meta.properties.contains_key("fixedField_deprecated"));
        assert!(header.object.meta.properties.contains_key("fixedField_style"));
        assert!(header.object.meta.properties.contains_key("fixedField_explode"));
        assert!(header.object.meta.properties.contains_key("fixedField_allowReserved"));
        assert!(header.object.meta.properties.contains_key("fixedField_schema"));
        
        // 3. Schema processing (SchemaVisitor/AlternatingVisitor)
        assert!(header.schema().is_some());
        assert!(header.object.meta.properties.contains_key("schema-referenced-element"));
        assert!(header.object.meta.properties.contains_key("hasSchemaRef"));
        
        // 4. Specification extensions support
        assert!(header.object.get("x-header-category").is_some());
        assert!(header.object.get("x-validation-rules").is_some());
        assert!(header.object.meta.properties.contains_key("specificationExtension_x-header-category"));
        assert!(header.object.meta.properties.contains_key("specificationExtension_x-validation-rules"));
        
        // 5. Fallback behavior
        assert!(header.object.get("custom-property").is_some());
        assert!(header.object.meta.properties.contains_key("fallback_custom-property"));
        
        // 6. Processing metadata
        assert!(header.object.meta.properties.contains_key("processed"));
        assert!(header.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(header.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(header.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // 7. SpecPath metadata
        if let Some(Value::Array(path)) = header.object.meta.properties.get("specPath") {
            assert_eq!(path.len(), 3);
            assert_eq!(path[0], Value::String("document".to_string()));
            assert_eq!(path[1], Value::String("objects".to_string()));
            assert_eq!(path[2], Value::String("Header".to_string()));
        }
        
        // 8. Validation
        assert!(header.object.meta.properties.contains_key("validStyle"));
        assert!(header.object.meta.properties.contains_key("validHeader"));
        
        // 9. Classes
        let has_fixed_field_class = header.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "fixed-field"
            } else {
                false
            }
        });
        assert!(has_fixed_field_class);
        
        let has_spec_extension_class = header.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "specification-extension"
            } else {
                false
            }
        });
        assert!(has_spec_extension_class);
    }
}