//! # Media Type Builder Module
//!
//! This module provides enhanced media type building functionality that is functionally equivalent
//! to the TypeScript MediaTypeVisitor pattern. It implements specialized schema, examples, and
//! encoding processing with comprehensive metadata injection and validation.
//!
//! ## Features
//!
//! ### 1. Schema Processing (SchemaVisitor equivalent)
//! - Handles `schema` field with AlternatingVisitor pattern (Reference vs Schema)
//! - Provides reference decoration with `referenced-element=schema` metadata
//! - Supports polymorphic handling (Reference vs Schema elements)
//!
//! ### 2. Examples Processing (ExamplesVisitor equivalent)
//! - Processes `examples` field as ObjectElement with structured processing
//! - Provides recursive processing through folder for each example
//! - Injects example-specific metadata for each ExampleElement
//!
//! ### 3. Encoding Processing (EncodingVisitor equivalent)
//! - Handles `encoding` field as ObjectElement with specialized processing
//! - Processes each encoding value recursively through folder
//! - Maintains proper element transformation and metadata injection
//!
//! ### 4. Fixed Fields Processing
//! - Handles `example` field with proper processing
//! - Supports type validation and conversion
//! - Maintains backward compatibility
//!
//! ### 5. Specification Extensions & Fallback
//! - Processes x-* fields with appropriate metadata
//! - Preserves unknown fields for debugging
//! - Comprehensive validation and error handling
//!
//! ## TypeScript Equivalence
//!
//! This implementation provides feature parity with the TypeScript MediaTypeVisitor:
//! - ✅ Schema processing with AlternatingVisitor (Reference vs Schema)
//! - ✅ Examples as ObjectElement with recursive processing
//! - ✅ Encoding as ObjectElement with specialized processing
//! - ✅ Fixed fields processing (example)
//! - ✅ Specification extensions handling (x-* fields)
//! - ✅ Fallback processing for unknown fields
//! - ✅ Comprehensive metadata injection and validation

use apidom_ast::*;
use serde_json::Value;
use crate::elements::media_type::MediaTypeElement;
use crate::builder::encoding_builder::build_and_decorate_encoding;
use crate::builder::example_builder::build_and_decorate_example;

/// Basic media type builder - equivalent to simple constructor
pub fn build_media_type(element: &Element) -> Option<MediaTypeElement> {
    let obj = element.as_object()?;
    Some(MediaTypeElement::with_content(obj.clone()))
}

/// Enhanced media type builder with visitor pattern features
/// Equivalent to TypeScript MediaTypeVisitor with SchemaVisitor, ExamplesVisitor, and EncodingVisitor
pub fn build_and_decorate_media_type<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<MediaTypeElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut media_type = MediaTypeElement::with_content(object.clone());
    
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
            // Schema processing with AlternatingVisitor pattern (Reference vs Schema)
            "schema" => {
                let schema_element = process_schema_field(&processed_value, &mut media_type);
                media_type.set_schema(schema_element);
                add_fixed_field_metadata(&mut media_type, "schema");
            },
            
            // Example processing (single example value)
            "example" => {
                media_type.set_example(processed_value);
                add_fixed_field_metadata(&mut media_type, "example");
            },
            
            // Examples processing with ExamplesVisitor pattern
            "examples" => {
                if let Element::Object(examples_obj) = processed_value {
                    let examples_element = process_examples_field(&examples_obj, &mut media_type, folder.as_deref_mut());
                    media_type.set_examples(examples_element);
                    add_fixed_field_metadata(&mut media_type, "examples");
                } else {
                    add_validation_error_metadata(&mut media_type, "examples", "Expected object value");
                }
            },
            
            // Encoding processing with EncodingVisitor pattern
            "encoding" => {
                if let Element::Object(encoding_obj) = processed_value {
                    let encoding_element = process_encoding_field(&encoding_obj, &mut media_type, folder.as_deref_mut());
                    media_type.set_encoding(encoding_element);
                    add_fixed_field_metadata(&mut media_type, "encoding");
                } else {
                    add_validation_error_metadata(&mut media_type, "encoding", "Expected object value");
                }
            },
            
            // $ref handling
            "$ref" => {
                media_type.object.set("$ref", processed_value);
                add_ref_metadata(&mut media_type, "$ref");
            },
            
            // Specification extensions (x-* fields)
            key if key.starts_with("x-") => {
                media_type.object.set(&key_str, processed_value);
                add_specification_extension_metadata(&mut media_type, &key_str);
            },
            
            // Fallback for unknown fields
            _ => {
                media_type.object.set(&key_str, processed_value);
                add_fallback_metadata(&mut media_type, &key_str);
            }
        }
    }
    
    // Add processing metadata
    add_processing_metadata(&mut media_type);
    add_spec_path_metadata(&mut media_type);
    validate_media_type(&mut media_type);
    
    Some(media_type)
}

/// Process schema field with AlternatingVisitor pattern (Reference vs Schema)
fn process_schema_field(element: &Element, media_type: &mut MediaTypeElement) -> Element {
    // Check if it's a reference-like element
    if is_reference_like_element(element) {
        // Add reference metadata
        if let Element::Object(obj) = element {
            if obj.get("$ref").is_some() {
                media_type.object.meta.properties.insert("schema-referenced-element".to_string(), SimpleValue::String("schema".to_string()));
                add_schema_ref_metadata(media_type);
            }
        }
    } else {
        // It's a schema object
        add_schema_metadata(media_type);
    }
    
    element.clone()
}

/// Process examples field with ExamplesVisitor pattern
/// Equivalent to TypeScript ExamplesVisitor with recursive processing
fn process_examples_field<F>(
    examples_obj: &ObjectElement, 
    media_type: &mut MediaTypeElement,
    mut folder: Option<&mut F>
) -> ObjectElement
where
    F: Fold,
{
    let mut processed_examples = ObjectElement::new();
    
    // Process each example with recursive processing
    for member in &examples_obj.content {
        if let Element::String(example_key) = member.key.as_ref() {
            let example_name = &example_key.content;
            
            // Process example through folder if available
            let processed_example = if let Some(ref mut f) = folder {
                f.fold_element(member.value.as_ref().clone())
            } else {
                member.value.as_ref().clone()
            };
            
            // Enhanced example processing
            let enhanced_example = if let Element::Object(example_obj) = processed_example {
                // Use example builder for enhanced processing
                if let Some(example_element) = build_and_decorate_example(&Element::Object(example_obj.clone()), folder.as_deref_mut()) {
                    // Inject example name metadata
                    let mut enhanced_obj = example_element.object;
                    enhanced_obj.meta.properties.insert("exampleName".to_string(), SimpleValue::String(example_name.clone()));
                    Element::Object(enhanced_obj)
                } else {
                    Element::Object(example_obj)
                }
            } else {
                processed_example
            };
            
            processed_examples.set(example_name, enhanced_example);
        }
    }
    
    // Add examples processing metadata
    add_examples_processing_metadata(media_type);
    processed_examples
}

/// Process encoding field with EncodingVisitor pattern
/// Equivalent to TypeScript EncodingVisitor with recursive processing
fn process_encoding_field<F>(
    encoding_obj: &ObjectElement, 
    media_type: &mut MediaTypeElement,
    mut folder: Option<&mut F>
) -> ObjectElement
where
    F: Fold,
{
    let mut processed_encoding = ObjectElement::new();
    
    // Process each encoding with recursive processing
    for member in &encoding_obj.content {
        if let Element::String(encoding_key) = member.key.as_ref() {
            let encoding_name = &encoding_key.content;
            
            // Process encoding through folder if available
            let processed_value = if let Some(ref mut f) = folder {
                f.fold_element(member.value.as_ref().clone())
            } else {
                member.value.as_ref().clone()
            };
            
            // Enhanced encoding processing
            let enhanced_encoding = if let Element::Object(encoding_obj) = processed_value {
                // Use encoding builder for enhanced processing
                if let Some(encoding_element) = build_and_decorate_encoding(&Element::Object(encoding_obj.clone()), folder.as_deref_mut()) {
                    // Inject encoding name metadata
                    let mut enhanced_obj = encoding_element.object;
                    enhanced_obj.meta.properties.insert("encodingName".to_string(), SimpleValue::String(encoding_name.clone()));
                    Element::Object(enhanced_obj)
                } else {
                    Element::Object(encoding_obj)
                }
            } else {
                processed_value
            };
            
            processed_encoding.set(encoding_name, enhanced_encoding);
        }
    }
    
    // Add encoding processing metadata
    add_encoding_processing_metadata(media_type);
    processed_encoding
}

/// Check if element is reference-like (has $ref)
fn is_reference_like_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.get("$ref").is_some()
    } else {
        false
    }
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(media_type: &mut MediaTypeElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    media_type.object.meta.properties.insert(key, SimpleValue::Bool(true));
    media_type.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(media_type: &mut MediaTypeElement, field_name: &str) {
    let key = format!("ref_{}", field_name);
    media_type.object.meta.properties.insert(key, SimpleValue::Bool(true));
    media_type.object.meta.properties.insert("referenced-element".to_string(), SimpleValue::String("mediaType".to_string()));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(media_type: &mut MediaTypeElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    media_type.object.meta.properties.insert(key, SimpleValue::Bool(true));
    media_type.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(media_type: &mut MediaTypeElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    media_type.object.meta.properties.insert(key, SimpleValue::Bool(true));
}

/// Add validation error metadata
fn add_validation_error_metadata(media_type: &mut MediaTypeElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    media_type.object.meta.properties.insert(key, SimpleValue::String(error_msg.to_string()));
}

/// Add metadata for schema processing
fn add_schema_metadata(media_type: &mut MediaTypeElement) {
    media_type.object.meta.properties.insert("hasSchema".to_string(), SimpleValue::Bool(true));
}

/// Add metadata for schema references
fn add_schema_ref_metadata(media_type: &mut MediaTypeElement) {
    media_type.object.meta.properties.insert("hasSchemaRef".to_string(), SimpleValue::Bool(true));
}

/// Add metadata for examples processing
fn add_examples_processing_metadata(media_type: &mut MediaTypeElement) {
    media_type.object.meta.properties.insert("examplesProcessed".to_string(), SimpleValue::Bool(true));
    media_type.object.meta.properties.insert("examplesVisitor".to_string(), SimpleValue::Bool(true));
}

/// Add metadata for encoding processing
fn add_encoding_processing_metadata(media_type: &mut MediaTypeElement) {
    media_type.object.meta.properties.insert("encodingProcessed".to_string(), SimpleValue::Bool(true));
    media_type.object.meta.properties.insert("encodingVisitor".to_string(), SimpleValue::Bool(true));
}

/// Add overall processing metadata
fn add_processing_metadata(media_type: &mut MediaTypeElement) {
    media_type.object.meta.properties.insert("processed".to_string(), SimpleValue::Bool(true));
    media_type.object.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::Bool(true));
    media_type.object.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::Bool(true));
    media_type.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), SimpleValue::Bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(media_type: &mut MediaTypeElement) {
    media_type.object.meta.properties.insert("specPath".to_string(), SimpleValue::Array(vec![
        SimpleValue::String("document".to_string()),
        SimpleValue::String("objects".to_string()),
        SimpleValue::String("MediaType".to_string())
    ]));
}

/// Validate media type constraints
fn validate_media_type(media_type: &mut MediaTypeElement) {
    // Check for mutually exclusive example and examples
    let has_example = media_type.example().is_some();
    let has_examples = media_type.examples().is_some();
    
    if has_example && has_examples {
        add_validation_error_metadata(
            media_type, 
            "mediaType", 
            "MediaType cannot have both 'example' and 'examples' properties"
        );
    } else {
        media_type.object.meta.properties.insert("validMediaType".to_string(), SimpleValue::Bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_media_type_builder() {
        let mut obj = ObjectElement::new();
        obj.set("schema", Element::Object(ObjectElement::new()));
        obj.set("example", Element::String(StringElement::new("sample")));

        let media_type = build_media_type(&Element::Object(obj));
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        assert!(media_type.schema().is_some());
        assert!(media_type.example().is_some());
    }

    #[test]
    fn test_enhanced_media_type_with_schema() {
        let mut obj = ObjectElement::new();
        
        // Create a schema object
        let mut schema_obj = ObjectElement::new();
        schema_obj.set("type", Element::String(StringElement::new("object")));
        schema_obj.set("properties", Element::Object(ObjectElement::new()));
        obj.set("schema", Element::Object(schema_obj));
        
        obj.set("example", Element::String(StringElement::new("sample data")));

        let mut folder = DefaultFolder;
        let media_type = build_and_decorate_media_type(&Element::Object(obj), Some(&mut folder));
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify basic fields
        assert!(media_type.schema().is_some());
        assert!(media_type.example().is_some());
        
        // Verify schema metadata
        assert!(media_type.object.meta.properties.contains_key("hasSchema"));
        assert!(media_type.object.meta.properties.contains_key("fixedField_schema"));
        assert!(media_type.object.meta.properties.contains_key("fixedField_example"));
        
        // Verify processing metadata
        assert!(media_type.object.meta.properties.contains_key("processed"));
        assert!(media_type.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(media_type.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(media_type.object.meta.properties.contains_key("specPath"));
        assert!(media_type.object.meta.properties.contains_key("validMediaType"));
    }

    #[test]
    fn test_media_type_with_schema_reference() {
        let mut obj = ObjectElement::new();
        
        // Create a schema reference
        let mut schema_ref = ObjectElement::new();
        schema_ref.set("$ref", Element::String(StringElement::new("#/components/schemas/Pet")));
        obj.set("schema", Element::Object(schema_ref));

        let media_type = build_and_decorate_media_type::<DefaultFolder>(&Element::Object(obj), None);
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify schema reference
        assert!(media_type.schema().is_some());
        
        // Verify reference metadata
        assert!(media_type.object.meta.properties.contains_key("hasSchemaRef"));
        assert!(media_type.object.meta.properties.contains_key("schema-referenced-element"));
        if let Some(SimpleValue::String(ref_type)) = media_type.object.meta.properties.get("schema-referenced-element") {
            assert_eq!(ref_type, "schema");
        }
    }

    #[test]
    fn test_media_type_with_examples() {
        let mut obj = ObjectElement::new();
        
        // Create examples object
        let mut examples_obj = ObjectElement::new();
        
        // Add simple example
        let mut simple_example = ObjectElement::new();
        simple_example.set("summary", Element::String(StringElement::new("Simple example")));
        simple_example.set("value", Element::String(StringElement::new("simple")));
        examples_obj.set("simple", Element::Object(simple_example));
        
        // Add complex example
        let mut complex_example = ObjectElement::new();
        complex_example.set("summary", Element::String(StringElement::new("Complex example")));
        let mut complex_value = ObjectElement::new();
        complex_value.set("id", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        }));
        complex_value.set("name", Element::String(StringElement::new("Test")));
        complex_example.set("value", Element::Object(complex_value));
        examples_obj.set("complex", Element::Object(complex_example));
        
        obj.set("examples", Element::Object(examples_obj));

        let mut folder = DefaultFolder;
        let media_type = build_and_decorate_media_type(&Element::Object(obj), Some(&mut folder));
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify examples
        assert!(media_type.examples().is_some());
        let examples = media_type.examples().unwrap();
        assert!(examples.get("simple").is_some());
        assert!(examples.get("complex").is_some());
        
        // Verify examples processing metadata
        assert!(media_type.object.meta.properties.contains_key("examplesProcessed"));
        assert!(media_type.object.meta.properties.contains_key("examplesVisitor"));
        assert!(media_type.object.meta.properties.contains_key("fixedField_examples"));
        
        // Verify example name metadata is injected
        if let Some(Element::Object(simple_obj)) = examples.get("simple") {
            assert!(simple_obj.meta.properties.contains_key("exampleName"));
            if let Some(SimpleValue::String(name)) = simple_obj.meta.properties.get("exampleName") {
                assert_eq!(name, "simple");
            }
        }
    }

    #[test]
    fn test_media_type_with_encoding() {
        let mut obj = ObjectElement::new();
        
        // Create encoding object
        let mut encoding_obj = ObjectElement::new();
        
        // Add form encoding
        let mut form_encoding = ObjectElement::new();
        form_encoding.set("contentType", Element::String(StringElement::new("application/x-www-form-urlencoded")));
        form_encoding.set("style", Element::String(StringElement::new("form")));
        form_encoding.set("explode", Element::Boolean(BooleanElement::new(true)));
        encoding_obj.set("form", Element::Object(form_encoding));
        
        // Add json encoding
        let mut json_encoding = ObjectElement::new();
        json_encoding.set("contentType", Element::String(StringElement::new("application/json")));
        encoding_obj.set("json", Element::Object(json_encoding));
        
        obj.set("encoding", Element::Object(encoding_obj));

        let mut folder = DefaultFolder;
        let media_type = build_and_decorate_media_type(&Element::Object(obj), Some(&mut folder));
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify encoding
        assert!(media_type.encoding().is_some());
        let encoding = media_type.encoding().unwrap();
        assert!(encoding.get("form").is_some());
        assert!(encoding.get("json").is_some());
        
        // Verify encoding processing metadata
        assert!(media_type.object.meta.properties.contains_key("encodingProcessed"));
        assert!(media_type.object.meta.properties.contains_key("encodingVisitor"));
        assert!(media_type.object.meta.properties.contains_key("fixedField_encoding"));
        
        // Verify encoding name metadata is injected
        if let Some(Element::Object(form_obj)) = encoding.get("form") {
            assert!(form_obj.meta.properties.contains_key("encodingName"));
            if let Some(SimpleValue::String(name)) = form_obj.meta.properties.get("encodingName") {
                assert_eq!(name, "form");
            }
        }
    }

    #[test]
    fn test_media_type_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/mediaTypes/JsonMediaType")));

        let media_type = build_and_decorate_media_type::<DefaultFolder>(&Element::Object(obj), None);
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify $ref is preserved
        assert!(media_type.object.get("$ref").is_some());
        
        // Verify reference metadata
        assert!(media_type.object.meta.properties.contains_key("ref_$ref"));
        assert!(media_type.object.meta.properties.contains_key("referenced-element"));
        if let Some(SimpleValue::String(ref_type)) = media_type.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_type, "mediaType");
        }
    }

    #[test]
    fn test_media_type_validation_errors() {
        let mut obj = ObjectElement::new();
        // Invalid examples (not an object)
        obj.set("examples", Element::String(StringElement::new("invalid")));
        // Invalid encoding (not an object)
        obj.set("encoding", Element::Array(ArrayElement::new_empty()));

        let media_type = build_and_decorate_media_type::<DefaultFolder>(&Element::Object(obj), None);
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify validation errors for invalid field types
        assert!(media_type.object.meta.properties.contains_key("validationError_examples"));
        assert!(media_type.object.meta.properties.contains_key("validationError_encoding"));
    }

    #[test]
    fn test_media_type_mutual_exclusion_validation() {
        let mut obj = ObjectElement::new();
        
        // Valid examples object
        let mut examples_obj = ObjectElement::new();
        let mut simple_example = ObjectElement::new();
        simple_example.set("summary", Element::String(StringElement::new("Simple example")));
        simple_example.set("value", Element::String(StringElement::new("simple")));
        examples_obj.set("simple", Element::Object(simple_example));
        obj.set("examples", Element::Object(examples_obj));
        
        // Also set example (mutually exclusive)
        obj.set("example", Element::String(StringElement::new("test")));

        let media_type = build_and_decorate_media_type::<DefaultFolder>(&Element::Object(obj), None);
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify mutual exclusion validation error
        assert!(media_type.object.meta.properties.contains_key("validationError_mediaType"));
        
        // Verify the error message
        if let Some(SimpleValue::String(error_msg)) = media_type.object.meta.properties.get("validationError_mediaType") {
            assert!(error_msg.contains("cannot have both 'example' and 'examples'"));
        }
    }

    #[test]
    fn test_media_type_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("schema", Element::Object(ObjectElement::new()));
        obj.set("x-custom-property", Element::String(StringElement::new("custom-value")));
        obj.set("x-vendor-extension", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 42.0,
        }));

        let media_type = build_and_decorate_media_type::<DefaultFolder>(&Element::Object(obj), None);
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify extensions are preserved
        assert!(media_type.object.get("x-custom-property").is_some());
        assert!(media_type.object.get("x-vendor-extension").is_some());
        
        // Verify extension metadata
        assert!(media_type.object.meta.properties.contains_key("specificationExtension_x-custom-property"));
        assert!(media_type.object.meta.properties.contains_key("specificationExtension_x-vendor-extension"));
        
        // Verify extension classes
        let has_extension_class = media_type.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "specification-extension"
            } else {
                false
            }
        });
        assert!(has_extension_class);
    }

    #[test]
    fn test_media_type_fallback_behavior() {
        let mut obj = ObjectElement::new();
        obj.set("schema", Element::Object(ObjectElement::new()));
        obj.set("custom-field", Element::String(StringElement::new("custom-value")));
        obj.set("unknown-property", Element::Boolean(BooleanElement::new(true)));

        let media_type = build_and_decorate_media_type::<DefaultFolder>(&Element::Object(obj), None);
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify fallback fields are preserved
        assert!(media_type.object.get("custom-field").is_some());
        assert!(media_type.object.get("unknown-property").is_some());
        
        // Verify fallback metadata
        assert!(media_type.object.meta.properties.contains_key("fallback_custom-field"));
        assert!(media_type.object.meta.properties.contains_key("fallback_unknown-property"));
        
        // Verify fixed field is still processed normally
        assert!(media_type.object.meta.properties.contains_key("fixedField_schema"));
    }

    #[test]
    fn test_typescript_equivalence_comprehensive() {
        // Comprehensive test demonstrating full TypeScript MediaTypeVisitor equivalence
        let mut obj = ObjectElement::new();
        
        // Schema with reference
        let mut schema_ref = ObjectElement::new();
        schema_ref.set("$ref", Element::String(StringElement::new("#/components/schemas/Pet")));
        obj.set("schema", Element::Object(schema_ref));
        
        // Examples with multiple entries
        let mut examples_obj = ObjectElement::new();
        
        let mut cat_example = ObjectElement::new();
        cat_example.set("summary", Element::String(StringElement::new("Cat example")));
        cat_example.set("description", Element::String(StringElement::new("A sample cat")));
        let mut cat_value = ObjectElement::new();
        cat_value.set("name", Element::String(StringElement::new("Fluffy")));
        cat_value.set("type", Element::String(StringElement::new("cat")));
        cat_example.set("value", Element::Object(cat_value));
        examples_obj.set("cat", Element::Object(cat_example));
        
        let mut dog_example = ObjectElement::new();
        dog_example.set("summary", Element::String(StringElement::new("Dog example")));
        dog_example.set("externalValue", Element::String(StringElement::new("http://example.com/dog.json")));
        examples_obj.set("dog", Element::Object(dog_example));
        
        obj.set("examples", Element::Object(examples_obj));
        
        // Encoding with multiple entries
        let mut encoding_obj = ObjectElement::new();
        
        let mut form_encoding = ObjectElement::new();
        form_encoding.set("contentType", Element::String(StringElement::new("application/x-www-form-urlencoded")));
        form_encoding.set("style", Element::String(StringElement::new("form")));
        form_encoding.set("explode", Element::Boolean(BooleanElement::new(true)));
        
        // Headers in encoding
        let mut headers_obj = ObjectElement::new();
        let mut content_header = ObjectElement::new();
        content_header.set("description", Element::String(StringElement::new("Content disposition")));
        headers_obj.set("Content-Disposition", Element::Object(content_header));
        form_encoding.set("headers", Element::Object(headers_obj));
        
        encoding_obj.set("profileImage", Element::Object(form_encoding));
        
        obj.set("encoding", Element::Object(encoding_obj));
        
        // Specification extensions
        obj.set("x-media-type-version", Element::String(StringElement::new("1.0")));
        obj.set("x-custom-validator", Element::Boolean(BooleanElement::new(true)));

        let mut folder = DefaultFolder;
        let media_type = build_and_decorate_media_type(&Element::Object(obj), Some(&mut folder));
        assert!(media_type.is_some());
        
        let media_type = media_type.unwrap();
        
        // Verify comprehensive functionality
        assert!(media_type.schema().is_some());
        assert!(media_type.examples().is_some());
        assert!(media_type.encoding().is_some());
        
        // Verify schema reference processing
        assert!(media_type.object.meta.properties.contains_key("hasSchemaRef"));
        assert!(media_type.object.meta.properties.contains_key("schema-referenced-element"));
        
        // Verify examples processing
        let examples = media_type.examples().unwrap();
        assert_eq!(examples.content.len(), 2);
        assert!(examples.get("cat").is_some());
        assert!(examples.get("dog").is_some());
        
        // Verify encoding processing
        let encoding = media_type.encoding().unwrap();
        assert_eq!(encoding.content.len(), 1);
        assert!(encoding.get("profileImage").is_some());
        
        // Verify comprehensive metadata
        assert!(media_type.object.meta.properties.contains_key("processed"));
        assert!(media_type.object.meta.properties.contains_key("examplesProcessed"));
        assert!(media_type.object.meta.properties.contains_key("examplesVisitor"));
        assert!(media_type.object.meta.properties.contains_key("encodingProcessed"));
        assert!(media_type.object.meta.properties.contains_key("encodingVisitor"));
        
        // Verify specification extensions
        assert!(media_type.object.meta.properties.contains_key("specificationExtension_x-media-type-version"));
        assert!(media_type.object.meta.properties.contains_key("specificationExtension_x-custom-validator"));
        
        // Verify example name metadata injection
        if let Some(Element::Object(cat_obj)) = examples.get("cat") {
            assert!(cat_obj.meta.properties.contains_key("exampleName"));
            if let Some(SimpleValue::String(name)) = cat_obj.meta.properties.get("exampleName") {
                assert_eq!(name, "cat");
            }
        }
        
        // Verify encoding name metadata injection
        if let Some(Element::Object(profile_obj)) = encoding.get("profileImage") {
            assert!(profile_obj.meta.properties.contains_key("encodingName"));
            if let Some(SimpleValue::String(name)) = profile_obj.meta.properties.get("encodingName") {
                assert_eq!(name, "profileImage");
            }
        }
    }
}