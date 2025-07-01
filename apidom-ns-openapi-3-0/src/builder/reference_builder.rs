use apidom_ast::*;
use crate::elements::reference::ReferenceElement;
use std::collections::HashMap;

/// Enhanced Reference builder with TypeScript ReferenceVisitor equivalence
/// Provides fixed field processing, $ref validation, class injection, and metadata handling
pub fn build_reference(element: &Element) -> Option<ReferenceElement> {
    let obj = element.as_object()?.clone();
    build_and_decorate_reference(obj)
}

/// Core builder function with comprehensive field processing
pub fn build_and_decorate_reference(mut obj: ObjectElement) -> Option<ReferenceElement> {
    // Set element type and base classes
    obj.set_element_type("reference");
    obj.add_class("openapi-reference");
    
    // Create reference element
    let mut reference = ReferenceElement::with_content(obj.clone());
    
    // Process all fields with structured validation
    process_reference_fields(&mut reference, &obj);
    
    // Inject comprehensive metadata
    inject_reference_metadata(&mut reference.object, &obj);
    
    Some(reference)
}

/// Process reference fields with validation and class injection
fn process_reference_fields(reference: &mut ReferenceElement, source: &ObjectElement) {
    // Process fixed fields
    process_fixed_fields(reference, source);
    
    // Process $ref field with special handling
    process_ref_field(reference, source);
    
    // Handle specification extensions (explicitly disabled for Reference)
    validate_no_spec_extensions(reference, source);
    
    // Process fallback fields (preserve unknown fields)
    process_fallback_fields(reference, source);
}

/// Process fixed fields: $ref, summary, description
fn process_fixed_fields(reference: &mut ReferenceElement, source: &ObjectElement) {
    let fixed_fields = ["$ref", "summary", "description"];
    
    for field_name in &fixed_fields {
        if let Some(field_value) = source.get(field_name) {
            // Copy field to reference element
            reference.object.set(field_name, field_value.clone());
            
            // Add fixed field metadata
            add_fixed_field_metadata(&mut reference.object, field_name);
        }
    }
}

/// Process $ref field with type validation and class injection
fn process_ref_field(reference: &mut ReferenceElement, source: &ObjectElement) {
    if let Some(ref_elem) = source.get("$ref") {
        if let Some(ref_str) = ref_elem.as_string() {
            // Valid $ref string element
            
            // 1. Inject reference-element class (equivalent to TypeScript ReferenceVisitor)
            reference.object.add_class("reference-element");
            
            // 2. Process $ref value with reference-value class (equivalent to $RefVisitor)
            // Since StringElement doesn't have classes, use metadata instead
            let mut enhanced_ref = ref_str.clone();
            enhanced_ref.meta.properties.insert(
                "class".to_string(),
                SimpleValue::string("reference-value".to_string())
            );
            reference.object.set("$ref", Element::String(enhanced_ref));
            
            // 3. Add $ref validation metadata
            add_ref_validation_metadata(&mut reference.object, &ref_str.content, true);
            
        } else {
            // Invalid $ref type - add validation error
            add_ref_validation_metadata(&mut reference.object, "", false);
            
            // Still preserve the original value for debugging
            reference.object.set("$ref", ref_elem.clone());
        }
    }
}

/// Validate that no specification extensions are present (canSupportSpecificationExtensions = false)
fn validate_no_spec_extensions(reference: &mut ReferenceElement, source: &ObjectElement) {
    let mut spec_extensions = Vec::new();
    
    for member in &source.content {
        if let Element::String(key_str) = &*member.key {
            if key_str.content.starts_with("x-") {
                spec_extensions.push(key_str.content.clone());
            }
        }
    }
    
    if !spec_extensions.is_empty() {
        // Add validation warning for specification extensions
        reference.object.meta.properties.insert(
            "specification-extensions-not-supported".to_string(),
            SimpleValue::array(spec_extensions.into_iter().map(SimpleValue::string).collect())
        );
        
        // Add warning class
        reference.object.add_class("specification-extensions-warning");
    }
}

/// Process fallback fields (preserve unknown fields for debugging/compatibility)
fn process_fallback_fields(reference: &mut ReferenceElement, source: &ObjectElement) {
    let known_fields = ["$ref", "summary", "description"];
    
    for member in &source.content {
        if let Element::String(key_str) = &*member.key {
            let field_name = &key_str.content;
            
            // Skip known fields and spec extensions (already handled)
            if !known_fields.contains(&field_name.as_str()) && !field_name.starts_with("x-") {
                // Add as fallback field
                reference.object.set(field_name, (*member.value).clone());
                add_fallback_field_metadata(&mut reference.object, field_name);
            }
        }
    }
}

/// Inject comprehensive metadata for reference
fn inject_reference_metadata(obj: &mut ObjectElement, source: &ObjectElement) {
    // Add element type metadata
    obj.meta.properties.insert(
        "element-type".to_string(),
        SimpleValue::string("reference".to_string())
    );
    
    // Add spec path metadata
    obj.meta.properties.insert(
        "spec-path".to_string(),
        SimpleValue::array(vec![
            SimpleValue::string("document".to_string()),
            SimpleValue::string("objects".to_string()),
            SimpleValue::string("Reference".to_string())
        ])
    );
    
    // Add field count metadata
    obj.meta.properties.insert(
        "field-count".to_string(),
        SimpleValue::integer(source.content.len() as i64)
    );
    
    // Add specification extensions support flag
    obj.meta.properties.insert(
        "can-support-specification-extensions".to_string(),
        SimpleValue::bool(false)
    );
    
    // Add processing timestamp
    obj.meta.properties.insert(
        "processed-at".to_string(),
        SimpleValue::string(chrono::Utc::now().to_rfc3339())
    );
    
    // Add visitor information
    obj.meta.properties.insert(
        "processed-by".to_string(),
        SimpleValue::string("ReferenceVisitor".to_string())
    );
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fixed-field-{}", field_name),
        SimpleValue::bool(true)
    );
}

/// Add metadata for $ref validation
fn add_ref_validation_metadata(obj: &mut ObjectElement, ref_value: &str, is_valid: bool) {
    let mut validation_map = HashMap::new();
    validation_map.insert("is-valid".to_string(), SimpleValue::bool(is_valid));
    validation_map.insert("ref-value".to_string(), SimpleValue::string(ref_value.to_string()));
    validation_map.insert("is-string-element".to_string(), SimpleValue::bool(is_valid));

    obj.meta.properties.insert(
        "ref-validation".to_string(),
        SimpleValue::object(validation_map)
    );
    
    if is_valid {
        obj.meta.properties.insert(
            "referenced-element".to_string(),
            SimpleValue::string("reference".to_string())
        );
        obj.meta.properties.insert(
            "reference-path".to_string(),
            SimpleValue::string(ref_value.to_string())
        );
    }
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        SimpleValue::bool(true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_object(json_val: serde_json::Value) -> ObjectElement {
        let mut obj = ObjectElement::new();
        if let serde_json::Value::Object(map) = json_val {
            for (key, value) in map {
                let element = json_value_to_element(value);
                obj.set(&key, element);
            }
        }
        obj
    }

    fn json_value_to_element(value: serde_json::Value) -> Element {
        match value {
            serde_json::Value::String(s) => Element::String(StringElement::new(&s)),
            serde_json::Value::Bool(b) => Element::Boolean(BooleanElement::new(b)),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Element::Number(NumberElement { 
                        element: "number".to_string(),
                        meta: MetaElement::default(),
                        attributes: AttributesElement::default(),
                        content: f 
                    })
                } else {
                    Element::String(StringElement::new(&n.to_string()))
                }
            },
            serde_json::Value::Array(arr) => {
                let mut array = ArrayElement::new_empty();
                for item in arr {
                    array.content.push(json_value_to_element(item));
                }
                Element::Array(array)
            }
            serde_json::Value::Object(map) => {
                let mut obj = ObjectElement::new();
                for (key, value) in map {
                    obj.set(&key, json_value_to_element(value));
                }
                Element::Object(obj)
            }
            serde_json::Value::Null => Element::Null(NullElement::default()),
        }
    }

    #[test]
    fn test_build_reference_basic() {
        let obj = create_test_object(json!({
            "$ref": "#/components/schemas/User"
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Check basic structure
        assert_eq!(reference.object.element, "reference");
        
        // Check $ref field
        assert!(reference.ref_().is_some());
        assert_eq!(reference.ref_().unwrap().content, "#/components/schemas/User");
        
        // Check classes
        let classes: Vec<String> = reference.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(classes.contains(&"openapi-reference".to_string()));
        assert!(classes.contains(&"reference-element".to_string()));
    }

    #[test]
    fn test_build_reference_with_ref_value_class() {
        let obj = create_test_object(json!({
            "$ref": "#/components/schemas/Pet"
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Check that $ref element has reference-value class in metadata
        if let Some(Element::String(ref_elem)) = reference.object.get("$ref") {
            // StringElement uses metadata instead of classes
            assert_eq!(
                ref_elem.meta.properties.get("class"),
                Some(&SimpleValue::string("reference-value".to_string()))
            );
        } else {
            panic!("$ref should be a StringElement with reference-value class in metadata");
        }
    }

    #[test]
    fn test_build_reference_with_summary_description() {
        let obj = create_test_object(json!({
            "$ref": "#/components/schemas/User",
            "summary": "User reference",
            "description": "A reference to the User schema"
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Check fixed fields
        assert!(reference.object.get("summary").is_some());
        assert!(reference.object.get("description").is_some());
        
        if let Some(Element::String(summary)) = reference.object.get("summary") {
            assert_eq!(summary.content, "User reference");
        }
        
        if let Some(Element::String(description)) = reference.object.get("description") {
            assert_eq!(description.content, "A reference to the User schema");
        }

        // Check fixed field metadata
        assert_eq!(
            reference.object.meta.properties.get("fixed-field-$ref"),
            Some(&SimpleValue::bool(true))
        );
        assert_eq!(
            reference.object.meta.properties.get("fixed-field-summary"),
            Some(&SimpleValue::bool(true))
        );
        assert_eq!(
            reference.object.meta.properties.get("fixed-field-description"),
            Some(&SimpleValue::bool(true))
        );
    }

    #[test]
    fn test_build_reference_invalid_ref_type() {
        let obj = create_test_object(json!({
            "$ref": 123  // Invalid: should be string
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Check validation metadata for invalid $ref
        assert!(reference.object.meta.properties.contains_key("ref-validation"));
        if let Some(SimpleValue::Object(ref_validation)) = reference.object.meta.properties.get("ref-validation") {
            assert!(matches!(ref_validation.get("is-valid"), Some(&SimpleValue::Bool(false))));
            assert!(matches!(ref_validation.get("is-string-element"), Some(&SimpleValue::Bool(false))));
        }

        // Should NOT have reference-element class for invalid $ref
        let classes: Vec<String> = reference.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(!classes.contains(&"reference-element".to_string()));
    }

    #[test]
    fn test_build_reference_spec_extensions_warning() {
        let obj = create_test_object(json!({
            "$ref": "#/components/schemas/User",
            "x-custom-field": "should-not-be-supported",
            "x-another-extension": "also-not-supported"
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Check specification extensions warning
        assert!(reference.object.meta.properties.contains_key("specification-extensions-not-supported"));
        if let Some(SimpleValue::Array(extensions)) = reference.object.meta.properties.get("specification-extensions-not-supported") {
            assert_eq!(extensions.len(), 2);
            assert!(extensions.iter().any(|e| if let SimpleValue::String(s) = e { s.as_str() == "x-custom-field" } else { false }));
            assert!(extensions.iter().any(|e| if let SimpleValue::String(s) = e { s.as_str() == "x-another-extension" } else { false }));
        }

        // Check warning class
        let classes: Vec<String> = reference.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(classes.contains(&"specification-extensions-warning".to_string()));
    }

    #[test]
    fn test_build_reference_fallback_fields() {
        let obj = create_test_object(json!({
            "$ref": "#/components/schemas/User",
            "unknownField": "unknown-value",
            "anotherUnknown": "another-value"
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Check fallback fields are preserved
        assert!(reference.object.get("unknownField").is_some());
        assert!(reference.object.get("anotherUnknown").is_some());

        // Check fallback field metadata
        assert_eq!(
            reference.object.meta.properties.get("fallback-field-unknownField"),
            Some(&SimpleValue::bool(true))
        );
        assert_eq!(
            reference.object.meta.properties.get("fallback-field-anotherUnknown"),
            Some(&SimpleValue::bool(true))
        );
    }

    #[test]
    fn test_build_reference_comprehensive_metadata() {
        let obj = create_test_object(json!({
            "$ref": "#/components/schemas/Pet",
            "summary": "Pet reference"
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Check comprehensive metadata
        assert_eq!(
            reference.object.meta.properties.get("element-type"),
            Some(&SimpleValue::string("reference".to_string()))
        );
        
        assert_eq!(
            reference.object.meta.properties.get("can-support-specification-extensions"),
            Some(&SimpleValue::bool(false))
        );
        
        assert_eq!(
            reference.object.meta.properties.get("processed-by"),
            Some(&SimpleValue::string("ReferenceVisitor".to_string()))
        );
        
        // Check spec path
        if let Some(SimpleValue::Array(spec_path)) = reference.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "Reference"));
        }

        // Check $ref validation metadata
        if let Some(SimpleValue::Object(ref_validation)) = reference.object.meta.properties.get("ref-validation") {
            assert!(matches!(ref_validation.get("is-valid"), Some(&SimpleValue::Bool(true))));
            assert!(ref_validation.get("ref-value").and_then(|v| if let SimpleValue::String(s) = v { Some(s.as_str()) } else { None }) == Some("#/components/schemas/Pet"));
            assert!(matches!(ref_validation.get("is-string-element"), Some(&SimpleValue::Bool(true))));
        }

        // Check reference metadata
        assert!(matches!(
            reference.object.meta.properties.get("referenced-element"),
            Some(SimpleValue::String(s)) if s == "reference"
        ));
        assert!(matches!(
            reference.object.meta.properties.get("reference-path"),
            Some(SimpleValue::String(s)) if s == "#/components/schemas/Pet"
        ));

        assert!(reference.object.meta.properties.contains_key("field-count"));
        assert!(reference.object.meta.properties.contains_key("processed-at"));
    }

    #[test]
    fn test_build_reference_typescript_equivalence() {
        // This test verifies full TypeScript equivalence
        let obj = create_test_object(json!({
            "$ref": "#/components/schemas/User",
            "summary": "User schema reference"
        }));

        let reference = build_and_decorate_reference(obj).unwrap();

        // 1. Verify ReferenceElement structure (equivalent to TypeScript ReferenceElement)
        assert_eq!(reference.object.element, "reference");
        
        // 2. Verify FixedFieldsVisitor behavior (fixed fields processing)
        assert!(reference.object.get("$ref").is_some());
        assert!(reference.object.get("summary").is_some());
        assert_eq!(
            reference.object.meta.properties.get("fixed-field-$ref"),
            Some(&SimpleValue::bool(true))
        );
        
        // 3. Verify ReferenceVisitor behavior (reference-element class injection)
        let classes: Vec<String> = reference.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(classes.contains(&"reference-element".to_string()));
        
        // 4. Verify $RefVisitor behavior (reference-value class on $ref StringElement)
        if let Some(Element::String(ref_elem)) = reference.object.get("$ref") {
            // StringElement uses metadata instead of classes
            assert_eq!(
                ref_elem.meta.properties.get("class"),
                Some(&SimpleValue::string("reference-value".to_string()))
            );
        }
        
        // 5. Verify canSupportSpecificationExtensions = false behavior
        assert_eq!(
            reference.object.meta.properties.get("can-support-specification-extensions"),
            Some(&SimpleValue::bool(false))
        );
        
        // 6. Verify specPath metadata
        if let Some(SimpleValue::Array(spec_path)) = reference.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path, &vec![
                SimpleValue::string("document".to_string()),
                SimpleValue::string("objects".to_string()),
                SimpleValue::string("Reference".to_string())
            ]);
        }
        
        // 7. Verify comprehensive metadata injection
        assert!(reference.object.meta.properties.len() >= 8);
        assert!(reference.object.meta.properties.contains_key("element-type"));
        assert!(reference.object.meta.properties.contains_key("processed-by"));
        assert!(reference.object.meta.properties.contains_key("ref-validation"));
    }

    #[test]
    fn test_build_reference_empty() {
        let obj = create_test_object(json!({}));

        let reference = build_and_decorate_reference(obj).unwrap();

        // Should still create valid reference element
        assert_eq!(reference.object.element, "reference");
        
        // Should have base classes
        let classes: Vec<String> = reference.object.classes.content.iter()
            .filter_map(|e| e.as_string().map(|s| s.content.clone()))
            .collect();
        assert!(classes.contains(&"openapi-reference".to_string()));
        
        // Should NOT have reference-element class (no valid $ref)
        assert!(!classes.contains(&"reference-element".to_string()));
        
        // Should have basic metadata
        assert!(reference.object.meta.properties.contains_key("element-type"));
        assert!(reference.object.meta.properties.contains_key("field-count"));
    }
}