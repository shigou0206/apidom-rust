use apidom_ast::*;
use crate::elements::parameter::ParameterElement;
use serde_json::Value;

/// Build a basic ParameterElement from a generic Element
pub fn build_parameter(element: &Element) -> Option<ParameterElement> {
    let obj = element.as_object()?;
    let parameter = ParameterElement::with_content(obj.clone());
    Some(parameter)
}

/// Build and decorate ParameterElement with enhanced visitor pattern features
pub fn build_and_decorate_parameter<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<ParameterElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut parameter = ParameterElement::new();
    
    // Process object members with fixed field support and fallback handling
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            let key_str = &key.content;
            let value = member.value.as_ref();
            
            match key_str.as_str() {
                // Fixed fields - direct mapping with type conversion
                "name" | "in" | "description" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        parameter.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut parameter.object, key_str);
                    }
                }
                "required" | "deprecated" | "allowEmptyValue" | "explode" | "allowReserved" => {
                    if let Some(converted) = convert_to_boolean_element(value) {
                        parameter.object.set(key_str, Element::Boolean(converted));
                        add_fixed_field_metadata(&mut parameter.object, key_str);
                    }
                }
                "style" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        parameter.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut parameter.object, key_str);
                    }
                }
                "schema" => {
                    // Recursive processing for schema element
                    let processed_schema = if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    };
                    parameter.object.set("schema", processed_schema);
                    add_fixed_field_metadata(&mut parameter.object, "schema");
                }
                "example" => {
                    // Handle example element
                    let processed_example = if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    };
                    parameter.object.set("example", processed_example);
                    add_fixed_field_metadata(&mut parameter.object, "example");
                }
                "examples" => {
                    // Process examples with media-type metadata injection
                    if let Element::Object(examples_obj) = value {
                        let mut processed_examples = ObjectElement::new();
                        for example_member in &examples_obj.content {
                            if let Element::String(example_key) = example_member.key.as_ref() {
                                let mut processed_value = if let Some(ref mut f) = folder {
                                    f.fold_element(example_member.value.as_ref().clone())
                                } else {
                                    example_member.value.as_ref().clone()
                                };
                                
                                // Inject media-type metadata for each example
                                if let Element::Object(ref mut example_obj) = processed_value {
                                    example_obj.meta.properties.insert(
                                        "media-type".to_string(),
                                        SimpleValue::String(example_key.content.clone())
                                    );
                                }
                                
                                processed_examples.set(&example_key.content, processed_value);
                            }
                        }
                        parameter.object.set("examples", Element::Object(processed_examples));
                        add_fixed_field_metadata(&mut parameter.object, "examples");
                    }
                }
                "content" => {
                    // Process content with media-type metadata injection
                    if let Element::Object(content_obj) = value {
                        let mut processed_content = ObjectElement::new();
                        for content_member in &content_obj.content {
                            if let Element::String(media_type_key) = content_member.key.as_ref() {
                                let mut processed_value = if let Some(ref mut f) = folder {
                                    f.fold_element(content_member.value.as_ref().clone())
                                } else {
                                    content_member.value.as_ref().clone()
                                };
                                
                                // Inject media-type metadata
                                if let Element::Object(ref mut media_type_obj) = processed_value {
                                    media_type_obj.meta.properties.insert(
                                        "media-type".to_string(),
                                        SimpleValue::String(media_type_key.content.clone())
                                    );
                                }
                                
                                processed_content.set(&media_type_key.content, processed_value);
                            }
                        }
                        parameter.object.set("content", Element::Object(processed_content));
                        add_fixed_field_metadata(&mut parameter.object, "content");
                    }
                }
                "$ref" => {
                    // Handle $ref with reference metadata
                    if let Element::String(ref_str) = value {
                        parameter.object.set("$ref", value.clone());
                        add_reference_metadata(&mut parameter.object, &ref_str.content, "parameter");
                    }
                }
                _ => {
                    // Handle specification extensions (x-*) and fallback fields
                    if key_str.starts_with("x-") {
                        // Specification extension
                        parameter.object.set(key_str, value.clone());
                        add_specification_extension_metadata(&mut parameter.object, key_str);
                    } else {
                        // Fallback field - preserve unknown fields
                        parameter.object.set(key_str, value.clone());
                        add_fallback_field_metadata(&mut parameter.object, key_str);
                    }
                }
            }
        }
    }
    
    // Add element class metadata
    parameter.object.add_class("parameter");
    parameter.object.meta.properties.insert(
        "element-type".to_string(),
        SimpleValue::String("parameter".to_string())
    );
    
    // Validate parameter structure
    validate_parameter(&parameter)?;
    
    Some(parameter)
}

/// Convert element to StringElement with type safety
fn convert_to_string_element(element: &Element) -> Option<StringElement> {
    match element {
        Element::String(s) => Some(s.clone()),
        Element::Number(n) => Some(StringElement::new(&n.content.to_string())),
        Element::Boolean(b) => Some(StringElement::new(&b.content.to_string())),
        _ => None,
    }
}

/// Convert element to BooleanElement with type safety
fn convert_to_boolean_element(element: &Element) -> Option<BooleanElement> {
    match element {
        Element::Boolean(b) => Some(b.clone()),
        Element::String(s) => {
            match s.content.to_lowercase().as_str() {
                "true" => Some(BooleanElement::new(true)),
                "false" => Some(BooleanElement::new(false)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fixed-field-{}", field_name),
        SimpleValue::Bool(true)
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.add_class("specification-extension");
    obj.meta.properties.insert(
        "specification-extension".to_string(),
        SimpleValue::String(field_name.to_string())
    );
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        SimpleValue::Bool(true)
    );
}

/// Add metadata for $ref references
fn add_reference_metadata(obj: &mut ObjectElement, ref_path: &str, element_type: &str) {
    obj.add_class("reference");
    obj.meta.properties.insert(
        "referenced-element".to_string(),
        SimpleValue::String(element_type.to_string())
    );
    obj.meta.properties.insert(
        "reference-path".to_string(),
        SimpleValue::String(ref_path.to_string())
    );
}

/// Validate parameter structure
fn validate_parameter(parameter: &ParameterElement) -> Option<()> {
    // If this is a $ref parameter, skip standard validation
    if parameter.object.get("$ref").is_some() {
        return Some(()); // $ref parameters are valid without other required fields
    }
    
    // Check required fields for non-reference parameters
    if parameter.name().is_none() {
        return None; // name is required
    }
    
    if parameter.in_().is_none() {
        return None; // in is required
    }
    
    // Validate 'in' field values
    if let Some(in_value) = parameter.in_() {
        match in_value.content.as_str() {
            "query" | "header" | "path" | "cookie" => {
                // Valid values
            }
            _ => return None, // Invalid 'in' value
        }
    }
    
    // Path parameters must be required
    if let Some(in_value) = parameter.in_() {
        if in_value.content == "path" && !parameter.required() {
            return None; // Path parameters must be required
        }
    }
    
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parameter_builder() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("id")));
        obj.set("in", Element::String(StringElement::new("path")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_parameter(&Element::Object(obj));
        
        assert!(result.is_some());
        let param = result.unwrap();
        assert_eq!(param.object.element, "parameter");
        assert!(param.name().is_some());
        assert!(param.in_().is_some());
        assert!(param.required());
    }

    #[test]
    fn test_enhanced_parameter_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("userId")));
        obj.set("in", Element::String(StringElement::new("path")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        obj.set("description", Element::String(StringElement::new("User identifier")));
        obj.set("deprecated", Element::Boolean(BooleanElement::new(false)));
        
        let result = build_and_decorate_parameter(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let param = result.unwrap();
        
        // Verify fixed field metadata
        assert!(param.object.meta.properties.contains_key("fixed-field-name"));
        assert!(param.object.meta.properties.contains_key("fixed-field-in"));
        assert!(param.object.meta.properties.contains_key("fixed-field-required"));
        assert!(param.object.meta.properties.contains_key("fixed-field-description"));
        assert!(param.object.meta.properties.contains_key("fixed-field-deprecated"));
        
        // Verify element class
        assert!(param.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "parameter"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_parameter_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("api-key")));
        obj.set("in", Element::String(StringElement::new("header")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        obj.set("x-custom-validation", Element::String(StringElement::new("strict")));
        obj.set("x-internal-only", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_parameter(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let param = result.unwrap();
        
        // Verify specification extension metadata
        assert!(param.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(param.object.get("x-custom-validation").is_some());
        assert!(param.object.get("x-internal-only").is_some());
    }

    #[test]
    fn test_parameter_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/parameters/ApiKey")));
        
        let result = build_and_decorate_parameter(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let param = result.unwrap();
        
        // Verify reference metadata
        assert!(param.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            param.object.meta.properties.get("referenced-element"),
            Some(&SimpleValue::String("parameter".to_string()))
        );
        assert_eq!(
            param.object.meta.properties.get("reference-path"),
            Some(&SimpleValue::String("#/components/parameters/ApiKey".to_string()))
        );
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript ParameterVisitor
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("api-version")));
        obj.set("in", Element::String(StringElement::new("header")));
        obj.set("required", Element::Boolean(BooleanElement::new(true)));
        obj.set("description", Element::String(StringElement::new("API version header")));
        
        // Add specification extensions
        obj.set("x-api-gateway", Element::String(StringElement::new("v2")));
        
        // Add $ref for testing
        obj.set("$ref", Element::String(StringElement::new("#/components/parameters/ApiVersion")));
        
        let result = build_and_decorate_parameter(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let param = result.unwrap();
        
        // Verify TypeScript equivalence features:
        
        // 1. Fixed fields processing
        assert!(param.object.meta.properties.contains_key("fixed-field-name"));
        assert!(param.object.meta.properties.contains_key("fixed-field-in"));
        
        // 2. Specification extensions
        assert!(param.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(param.object.get("x-api-gateway").is_some());
        
        // 3. $ref support
        assert!(param.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            param.object.meta.properties.get("referenced-element"),
            Some(&SimpleValue::String("parameter".to_string()))
        );
        
        // 4. Element classification
        assert!(param.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "parameter"
            } else {
                false
            }
        }));
        assert_eq!(
            param.object.meta.properties.get("element-type"),
            Some(&SimpleValue::String("parameter".to_string()))
        );
    }
}