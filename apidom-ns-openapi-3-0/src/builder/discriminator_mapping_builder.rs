use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::discriminator_mapping::DiscriminatorMappingElement;

/// Basic discriminator mapping builder
pub fn build_discriminator_mapping(element: &Element) -> Option<DiscriminatorMappingElement> {
    let object = element.as_object()?;
    Some(DiscriminatorMappingElement::with_content(object.clone()))
}

/// Enhanced discriminator mapping builder with visitor pattern features
/// Equivalent to TypeScript MappingVisitor with MapVisitor and FallbackVisitor
pub fn build_and_decorate_discriminator_mapping<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<DiscriminatorMappingElement>
where
    F: Fold,
{
    let object = element.as_object()?;
    let mut mapping = DiscriminatorMappingElement::with_content(object.clone());
    
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
        
        // All fields in discriminator mapping should be string values
        if let Some(string_val) = convert_to_string_element(&processed_value) {
            mapping.set_mapping(&key_str, string_val);
            add_mapping_field_metadata(&mut mapping, &key_str);
            add_type_conversion_metadata(&mut mapping, &key_str, "string");
        } else {
            // Add fallback for non-string values
            mapping.object.set(&key_str, processed_value);
            add_fallback_metadata(&mut mapping, &key_str);
            add_validation_error_metadata(&mut mapping, &key_str, "Expected string value for mapping");
        }
    }
    
    // Add processing metadata
    add_processing_metadata(&mut mapping);
    add_spec_path_metadata(&mut mapping);
    
    Some(mapping)
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

/// Add metadata for mapping fields
fn add_mapping_field_metadata(mapping: &mut DiscriminatorMappingElement, field_name: &str) {
    let key = format!("mappingField_{}", field_name);
    mapping.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for type conversions
fn add_type_conversion_metadata(mapping: &mut DiscriminatorMappingElement, field_name: &str, expected_type: &str) {
    let key = format!("typeConversion_{}", field_name);
    mapping.object.meta.properties.insert(key, Value::String(expected_type.to_string()));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(mapping: &mut DiscriminatorMappingElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    mapping.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add validation error metadata
fn add_validation_error_metadata(mapping: &mut DiscriminatorMappingElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    mapping.object.meta.properties.insert(key, Value::String(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(mapping: &mut DiscriminatorMappingElement) {
    mapping.object.meta.properties.insert("processed".to_string(), Value::Bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(mapping: &mut DiscriminatorMappingElement) {
    mapping.object.meta.properties.insert("specPath".to_string(), Value::Array(vec![
        Value::String("value".to_string())
    ]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::fold::DefaultFolder;

    #[test]
    fn test_basic_discriminator_mapping_builder() {
        let mut obj = ObjectElement::new();
        obj.set("petType", Element::String(StringElement::new("#/components/schemas/Pet")));
        obj.set("dogType", Element::String(StringElement::new("#/components/schemas/Dog")));

        let mapping = build_discriminator_mapping(&Element::Object(obj));
        assert!(mapping.is_some());
        
        let mapping = mapping.unwrap();
        assert_eq!(mapping.get_mapping("petType").unwrap().content, "#/components/schemas/Pet");
        assert_eq!(mapping.get_mapping("dogType").unwrap().content, "#/components/schemas/Dog");
    }

    #[test]
    fn test_enhanced_discriminator_mapping_builder() {
        let mut obj = ObjectElement::new();
        obj.set("cat", Element::String(StringElement::new("#/components/schemas/Cat")));
        obj.set("dog", Element::String(StringElement::new("#/components/schemas/Dog")));
        obj.set("bird", Element::String(StringElement::new("#/components/schemas/Bird")));

        let mut folder = DefaultFolder;
        let mapping = build_and_decorate_discriminator_mapping(&Element::Object(obj), Some(&mut folder));
        assert!(mapping.is_some());
        
        let mapping = mapping.unwrap();
        
        // Verify mappings are preserved
        assert_eq!(mapping.get_mapping("cat").unwrap().content, "#/components/schemas/Cat");
        assert_eq!(mapping.get_mapping("dog").unwrap().content, "#/components/schemas/Dog");
        assert_eq!(mapping.get_mapping("bird").unwrap().content, "#/components/schemas/Bird");
        
        // Verify metadata injection
        assert!(mapping.object.meta.properties.contains_key("mappingField_cat"));
        assert!(mapping.object.meta.properties.contains_key("mappingField_dog"));
        assert!(mapping.object.meta.properties.contains_key("mappingField_bird"));
        assert!(mapping.object.meta.properties.contains_key("processed"));
        assert!(mapping.object.meta.properties.contains_key("specPath"));
        
        // Verify element type and class
        assert_eq!(mapping.object.element, "discriminatorMapping");
        assert!(mapping.object.classes.content.iter().any(|class| {
            if let Element::String(s) = class {
                s.content == "map"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_discriminator_mapping_type_conversion() {
        let mut obj = ObjectElement::new();
        // Test type conversion from number to string
        obj.set("numericKey", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        }));
        // Test type conversion from boolean to string
        obj.set("boolKey", Element::Boolean(BooleanElement::new(true)));

        let mut folder = DefaultFolder;
        let mapping = build_and_decorate_discriminator_mapping(&Element::Object(obj), Some(&mut folder));
        assert!(mapping.is_some());
        
        let mapping = mapping.unwrap();
        
        // Verify conversions worked
        assert_eq!(mapping.get_mapping("numericKey").unwrap().content, "123");
        assert_eq!(mapping.get_mapping("boolKey").unwrap().content, "true");
        
        // Verify type conversion metadata
        assert!(mapping.object.meta.properties.contains_key("typeConversion_numericKey"));
        assert!(mapping.object.meta.properties.contains_key("typeConversion_boolKey"));
    }

    #[test]
    fn test_discriminator_mapping_fallback_behavior() {
        let mut obj = ObjectElement::new();
        obj.set("validMapping", Element::String(StringElement::new("#/components/schemas/Valid")));
        // Add invalid mapping (array instead of string)
        obj.set("invalidMapping", Element::Array(ArrayElement::new_empty()));

        let mapping = build_and_decorate_discriminator_mapping::<DefaultFolder>(&Element::Object(obj), None);
        assert!(mapping.is_some());
        
        let mapping = mapping.unwrap();
        
        // Verify valid mapping works
        assert!(mapping.get_mapping("validMapping").is_some());
        
        // Verify fallback for invalid mapping
        assert!(mapping.object.get("invalidMapping").is_some());
        assert!(mapping.object.meta.properties.contains_key("fallback_invalidMapping"));
        assert!(mapping.object.meta.properties.contains_key("validationError_invalidMapping"));
    }

    #[test]
    fn test_discriminator_mapping_utilities() {
        let mut obj = ObjectElement::new();
        obj.set("cat", Element::String(StringElement::new("#/components/schemas/Cat")));
        obj.set("dog", Element::String(StringElement::new("#/components/schemas/Dog")));
        obj.set("bird", Element::String(StringElement::new("#/components/schemas/Bird")));

        let mapping = build_discriminator_mapping(&Element::Object(obj)).unwrap();
        
        // Test utility methods
        assert_eq!(mapping.mapping_count(), 3);
        assert!(mapping.has_mapping("cat"));
        assert!(mapping.has_mapping("dog"));
        assert!(mapping.has_mapping("bird"));
        assert!(!mapping.has_mapping("fish"));
        
        let keys = mapping.mapping_keys();
        assert!(keys.contains(&"cat".to_string()));
        assert!(keys.contains(&"dog".to_string()));
        assert!(keys.contains(&"bird".to_string()));
        
        // Test iterator
        let mappings: Vec<_> = mapping.mappings().collect();
        assert_eq!(mappings.len(), 3);
    }
} 