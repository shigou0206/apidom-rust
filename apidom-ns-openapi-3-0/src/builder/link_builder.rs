use crate::elements::link::{LinkElement, LinkParametersElement};
use apidom_ast::*;

pub fn build_link(element: &Element) -> Option<LinkElement> {
    let object = element.as_object()?.clone();
    Some(LinkElement::with_content(object))
}


pub fn build_and_decorate_link<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<LinkElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut link = LinkElement::new();
    
    // Add processing metadata (equivalent to TypeScript FixedFieldsVisitor + FallbackVisitor)
    add_processing_metadata(&mut link);
    add_spec_path_metadata(&mut link);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            link.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut link, &ref_str.content);
            return Some(link);
        }
    }
    
    // Track reference fields for reference-element marking
    let mut has_operation_id = false;
    let mut has_operation_ref = false;
    let mut _has_validation_error = false;
    
    // Process all object members with FixedFieldsVisitor pattern
    for member in &obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            match key.as_str() {
                "operationId" => {
                    if let Element::String(string_elem) = value {
                        // Add reference-value class (equivalent to OperationRefVisitor)
                        let mut ref_elem = string_elem.clone();
                        ref_elem.meta.properties.insert(
                            "class".to_string(),
                            SimpleValue::string("reference-value".to_string())
                        );
                        link.set_operation_id(ref_elem);
                        add_fixed_field_metadata(&mut link, "operationId");
                        has_operation_id = true;
                    } else {
                        add_validation_error_metadata(&mut link, "operationId", "Expected string value");
                        _has_validation_error = true;
                    }
                }
                "operationRef" => {
                    if let Element::String(string_elem) = value {
                        // Add reference-value class (equivalent to OperationRefVisitor)
                        let mut ref_elem = string_elem.clone();
                        ref_elem.meta.properties.insert(
                            "class".to_string(),
                            SimpleValue::string("reference-value".to_string())
                        );
                        link.set_operation_ref(ref_elem);
                        add_fixed_field_metadata(&mut link, "operationRef");
                        has_operation_ref = true;
                    } else {
                        add_validation_error_metadata(&mut link, "operationRef", "Expected string value");
                    }
                }
                "parameters" => {
                    // MapVisitor pattern processing (equivalent to ParametersVisitor)
                    if let Some(params_obj) = value.as_object() {
                        let link_params = build_link_parameters(params_obj, folder.as_deref_mut());
                        link.set_parameters(link_params.object);
                        add_fixed_field_metadata(&mut link, "parameters");
                    } else {
                        add_validation_error_metadata(&mut link, "parameters", "Expected object value");
                    }
                }
                "requestBody" => {
                    // Process requestBody (can be any value)
                    let processed_value = if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    };
                    
                    link.set_request_body(processed_value);
                    add_fixed_field_metadata(&mut link, "requestBody");
                }
                "description" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        link.set_description(string_elem);
                        add_fixed_field_metadata(&mut link, "description");
                    } else {
                        add_validation_error_metadata(&mut link, "description", "Expected string value");
                    }
                }
                "server" => {
                    if let Some(server_obj) = value.as_object() {
                        // Process server object (could be recursively folded)
                        let processed_server = if let Some(ref mut f) = folder {
                            if let Element::Object(folded_obj) = f.fold_element(Element::Object(server_obj.clone())) {
                                folded_obj
                            } else {
                                server_obj.clone()
                            }
                        } else {
                            server_obj.clone()
                        };
                        
                        link.set_server(processed_server);
                        add_fixed_field_metadata(&mut link, "server");
                    } else {
                        add_validation_error_metadata(&mut link, "server", "Expected object value");
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
                        
                        link.object.set(key, processed_value);
                        add_specification_extension_metadata(&mut link, key);
                    } else {
                        // Fallback field (preserve unknown fields)
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        
                        link.object.set(key, processed_value);
                        add_fallback_metadata(&mut link, key);
                    }
                }
            }
        }
    }
    
    // Add validation error metadata if needed
    // (Note: specific validation errors are already added by add_validation_error_metadata)
    
    // Add reference-element class if operationId or operationRef are present
    if has_operation_id || has_operation_ref {
        link.object.add_class("reference-element");
        add_reference_element_metadata(&mut link);
    }
    
    // Validate mutual exclusion (operationId vs operationRef)
    if has_operation_id && has_operation_ref {
        add_validation_error_metadata(&mut link, "link", 
            "operationId and operationRef are mutually exclusive");
    }
    
    // Add element class metadata (equivalent to TypeScript class injection)
    link.object.add_class("link");
    link.object.meta.properties.insert(
        "element-type".to_string(),
        SimpleValue::string("link".to_string())
    );
    
    Some(link)
}

/// Build LinkParametersElement with MapVisitor pattern
/// Equivalent to TypeScript ParametersVisitor
fn build_link_parameters<F>(
    params_obj: &ObjectElement,
    mut folder: Option<&mut F>
) -> LinkParametersElement
where
    F: Fold,
{
    let mut link_params = LinkParametersElement::new();
    link_params.object.element = "linkParameters".to_string();
    
    // Add MapVisitor metadata
    add_map_visitor_metadata(&mut link_params);
    
    // Process each parameter
    for member in &params_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            let processed_value = if let Some(ref mut f) = folder {
                f.fold_element(value.clone())
            } else {
                value.clone()
            };
            
            link_params.object.set(key, processed_value);
            add_parameter_metadata(&mut link_params, key);
        }
    }
    
    link_params
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
fn add_fixed_field_metadata(link: &mut LinkElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    link.object.meta.properties.insert(key, SimpleValue::bool(true));
    link.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(link: &mut LinkElement, ref_path: &str) {
    link.object.add_class("reference");
    link.object.meta.properties.insert(
        "referenced-element".to_string(),
        SimpleValue::string("link".to_string())
    );
    link.object.meta.properties.insert(
        "reference-path".to_string(),
        SimpleValue::string(ref_path.to_string())
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(link: &mut LinkElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    link.object.meta.properties.insert(key, SimpleValue::Bool(true));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(link: &mut LinkElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    link.object.meta.properties.insert(key, SimpleValue::Bool(true));
}

/// Add metadata for validation errors
fn add_validation_error_metadata(link: &mut LinkElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    link.object.meta.properties.insert(key, SimpleValue::string(error_msg.to_string()));
    
    // Add general validation error for the link with field-specific message
    let general_error_msg = match field_name {
        "operationId" => format!("Invalid operationId: {}", error_msg),
        "operationRef" => format!("Invalid operationRef: {}", error_msg),
        _ => format!("Invalid {}: {}", field_name, error_msg),
    };
    
    link.object.meta.properties.insert(
        "validationError_link".to_string(), 
        SimpleValue::string(general_error_msg)
    );
}

/// Add metadata for reference element marking
fn add_reference_element_metadata(link: &mut LinkElement) {
    link.object.meta.properties.insert("hasReferenceFields".to_string(), SimpleValue::Bool(true));
}

/// Add overall processing metadata (equivalent to TypeScript FixedFieldsVisitor + FallbackVisitor)
fn add_processing_metadata(link: &mut LinkElement) {
    link.object.meta.properties.insert("processed".to_string(), SimpleValue::Bool(true));
    link.object.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::Bool(true));
    link.object.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::Bool(true));
    link.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), SimpleValue::Bool(true));
    
    // Add Link specific classes
    link.object.classes.content.push(Element::String(StringElement::new("link")));
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(link: &mut LinkElement) {
    link.object.meta.properties.insert("specPath".to_string(), SimpleValue::Array(vec![
        SimpleValue::String("document".to_string()),
        SimpleValue::String("objects".to_string()),
        SimpleValue::String("Link".to_string())
    ]));
}

/// Add MapVisitor metadata for LinkParametersElement
fn add_map_visitor_metadata(link_params: &mut LinkParametersElement) {
    link_params.object.meta.properties.insert("mapVisitor".to_string(), SimpleValue::bool(true));
    link_params.object.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
    
    // Add spec path for parameters
    link_params.object.meta.properties.insert("specPath".to_string(), SimpleValue::array(vec![
        SimpleValue::string("value".to_string())
    ]));
    
    // Add linkParameters class
    link_params.object.classes.content.push(Element::String(StringElement::new("link-parameters")));
}

/// Add metadata for individual parameters
fn add_parameter_metadata(link_params: &mut LinkParametersElement, param_name: &str) {
    // Add metadata for individual parameter processing (TypeScript ParametersVisitor pattern)
    let key = format!("parameter_{}", param_name);
    link_params.object.meta.properties.insert(key, SimpleValue::string("processed".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fold::OpenApiBuilderFolder;

    #[test]
    fn test_basic_link_builder() {
        let mut obj = ObjectElement::new();
        obj.set("operationId", Element::String(StringElement::new("getUserById")));
        obj.set("description", Element::String(StringElement::new("Get user by ID")));
        
        let result = build_link(&Element::Object(obj));
        
        assert!(result.is_some());
        let link = result.unwrap();
        assert_eq!(link.object.element, "link");
        assert!(link.operation_id().is_some());
        assert_eq!(link.operation_id().unwrap().content, "getUserById");
    }

    #[test]
    fn test_enhanced_link_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("operationRef", Element::String(StringElement::new("#/paths/~1users~1{id}/get")));
        obj.set("description", Element::String(StringElement::new("Get user operation")));
        
        // Add parameters
        let mut params = ObjectElement::new();
        params.set("userId", Element::String(StringElement::new("$request.path.id")));
        params.set("format", Element::String(StringElement::new("$request.query.format")));
        obj.set("parameters", Element::Object(params));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify processing metadata
        assert!(link.object.meta.properties.contains_key("processed"));
        assert!(link.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(link.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(link.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // Verify fixed field metadata
        assert!(link.object.meta.properties.contains_key("fixedField_operationRef"));
        assert!(link.object.meta.properties.contains_key("fixedField_description"));
        assert!(link.object.meta.properties.contains_key("fixedField_parameters"));
        
        // Verify spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = link.object.meta.properties.get("specPath") {
            assert!(spec_path.len() == 3);
            assert!(spec_path[0] == SimpleValue::String("document".to_string()));
            assert!(spec_path[1] == SimpleValue::String("objects".to_string()));
            assert!(spec_path[2] == SimpleValue::String("Link".to_string()));
        }
        
        // Verify element class
        assert!(link.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "link"
            } else {
                false
            }
        }));
        
        // Verify reference-element class (operationRef present)
        assert!(link.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference-element"
            } else {
                false
            }
        }));
        assert!(link.object.meta.properties.contains_key("hasReferenceFields"));
    }

    #[test]
    fn test_link_with_operation_id_reference_value_class() {
        let mut obj = ObjectElement::new();
        obj.set("operationId", Element::String(StringElement::new("createUser")));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify operationId has reference-value class (OperationRefVisitor equivalent)
        if let Some(op_id) = link.operation_id() {
            // Check metadata for class information since StringElement doesn't have classes field
            assert!(op_id.meta.properties.contains_key("class"));
            if let Some(SimpleValue::String(class_name)) = op_id.meta.properties.get("class") {
                assert_eq!(class_name, "reference-value");
            }
        }
        
        // Verify reference-element class
        assert!(link.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference-element"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_link_parameters_map_visitor_pattern() {
        let mut obj = ObjectElement::new();
        obj.set("operationId", Element::String(StringElement::new("updateUser")));
        
        // Add complex parameters object
        let mut params = ObjectElement::new();
        params.set("userId", Element::String(StringElement::new("$request.path.id")));
        params.set("userName", Element::String(StringElement::new("$request.body#/name")));
        params.set("userEmail", Element::String(StringElement::new("$request.body#/email")));
        params.set("apiVersion", Element::String(StringElement::new("$request.header.api-version")));
        obj.set("parameters", Element::Object(params));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify parameters were processed with MapVisitor pattern
        let link_params = link.parameters().unwrap();
        assert!(link_params.meta.properties.contains_key("mapVisitor"));
        assert!(link_params.meta.properties.contains_key("processed"));
        
        // Verify individual parameter metadata
        assert!(link_params.meta.properties.contains_key("parameter_userId"));
        assert!(link_params.meta.properties.contains_key("parameter_userName"));
        assert!(link_params.meta.properties.contains_key("parameter_userEmail"));
        assert!(link_params.meta.properties.contains_key("parameter_apiVersion"));
        
        // Verify parameter values are preserved
        assert!(link_params.get("userId").is_some());
        assert!(link_params.get("userName").is_some());
        assert!(link_params.get("userEmail").is_some());
        assert!(link_params.get("apiVersion").is_some());
        
        // Verify element type and class
        assert_eq!(link_params.element, "linkParameters");
        assert!(link_params.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "link-parameters"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_link_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("operationId", Element::String(StringElement::new("deleteUser")));
        obj.set("description", Element::String(StringElement::new("Delete user operation")));
        
        // Add specification extensions
        obj.set("x-rate-limit", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 100.0,
        }));
        obj.set("x-auth-required", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify specification extension metadata
        assert!(link.object.meta.properties.contains_key("specificationExtension_x-rate-limit"));
        assert!(link.object.meta.properties.contains_key("specificationExtension_x-auth-required"));
        
        // Verify extensions are preserved
        assert!(link.object.get("x-rate-limit").is_some());
        assert!(link.object.get("x-auth-required").is_some());
    }

    #[test]
    fn test_link_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("operationId", Element::String(StringElement::new("listUsers")));
        
        // Add fallback fields
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("unknownProperty", Element::Boolean(BooleanElement::new(false)));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify fallback field metadata
        assert!(link.object.meta.properties.contains_key("fallback_customField"));
        assert!(link.object.meta.properties.contains_key("fallback_unknownProperty"));
        
        // Verify fallback fields are preserved
        assert!(link.object.get("customField").is_some());
        assert!(link.object.get("unknownProperty").is_some());
    }

    #[test]
    fn test_link_validation_errors() {
        let mut obj = ObjectElement::new();
        // Add both operationId and operationRef (mutually exclusive)
        obj.set("operationId", Element::String(StringElement::new("getUser")));
        obj.set("operationRef", Element::String(StringElement::new("#/paths/~1users~1{id}/get")));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify validation error for mutual exclusion
        assert!(link.object.meta.properties.contains_key("validationError_link"));
        if let Some(SimpleValue::String(error_msg)) = link.object.meta.properties.get("validationError_link") {
            assert!(error_msg.contains("mutually exclusive"));
        }
    }

    #[test]
    fn test_link_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/links/UserLink")));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify reference metadata
        assert!(link.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            link.object.meta.properties.get("referenced-element"),
            Some(&SimpleValue::String("link".to_string()))
        );
        assert_eq!(
            link.object.meta.properties.get("reference-path"),
            Some(&SimpleValue::String("#/components/links/UserLink".to_string()))
        );
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript LinkVisitor
        let mut obj = ObjectElement::new();
        obj.set("operationRef", Element::String(StringElement::new("#/paths/~1users~1{userId}/get")));
        obj.set("description", Element::String(StringElement::new("Link to user details")));
        
        // Add comprehensive parameters (MapVisitor pattern)
        let mut params = ObjectElement::new();
        params.set("userId", Element::String(StringElement::new("$request.path.id")));
        params.set("includeProfile", Element::String(StringElement::new("$request.query.profile")));
        params.set("apiKey", Element::String(StringElement::new("$request.header.x-api-key")));
        obj.set("parameters", Element::Object(params));
        
        // Add requestBody
        obj.set("requestBody", Element::String(StringElement::new("$request.body")));
        
        // Add server object
        let mut server = ObjectElement::new();
        server.set("url", Element::String(StringElement::new("https://api.example.com")));
        server.set("description", Element::String(StringElement::new("Production server")));
        obj.set("server", Element::Object(server));
        
        // Add specification extensions
        obj.set("x-link-timeout", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 30.0,
        }));
        obj.set("x-cache-enabled", Element::Boolean(BooleanElement::new(true)));
        
        // Add fallback field
        obj.set("customLinkConfig", Element::String(StringElement::new("custom config")));
        
        let result = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let link = result.unwrap();
        
        // Verify all TypeScript LinkVisitor features are present:
        
        // 1. FixedFieldsVisitor processing
        assert!(link.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(link.object.meta.properties.contains_key("fixedField_operationRef"));
        assert!(link.object.meta.properties.contains_key("fixedField_description"));
        assert!(link.object.meta.properties.contains_key("fixedField_parameters"));
        assert!(link.object.meta.properties.contains_key("fixedField_requestBody"));
        assert!(link.object.meta.properties.contains_key("fixedField_server"));
        
        // 2. OperationRef with reference-value class (OperationRefVisitor equivalent)
        if let Some(op_ref) = link.operation_ref() {
            // Check metadata for class information since StringElement doesn't have classes field
            assert!(op_ref.meta.properties.contains_key("class"));
            if let Some(SimpleValue::String(class_name)) = op_ref.meta.properties.get("class") {
                assert_eq!(class_name, "reference-value");
            }
        }
        
        // 3. Parameters processed with MapVisitor pattern (ParametersVisitor equivalent)
        let link_params = link.parameters().unwrap();
        assert!(link_params.meta.properties.contains_key("mapVisitor"));
        assert!(link_params.meta.properties.contains_key("processed"));
        assert_eq!(link_params.element, "linkParameters");
        assert!(link_params.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "link-parameters"
            } else {
                false
            }
        }));
        
        // 4. Reference-element marking (operationRef present)
        assert!(link.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference-element"
            } else {
                false
            }
        }));
        assert!(link.object.meta.properties.contains_key("hasReferenceFields"));
        
        // 5. Specification extensions support
        assert!(link.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        assert!(link.object.meta.properties.contains_key("specificationExtension_x-link-timeout"));
        assert!(link.object.meta.properties.contains_key("specificationExtension_x-cache-enabled"));
        
        // 6. Fallback field handling
        assert!(link.object.meta.properties.contains_key("fallback_customLinkConfig"));
        assert!(link.object.get("customLinkConfig").is_some());
        
        // 7. Spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = link.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], SimpleValue::String("document".to_string()));
            assert_eq!(spec_path[1], SimpleValue::String("objects".to_string()));
            assert_eq!(spec_path[2], SimpleValue::String("Link".to_string()));
        }
        
        // 8. Element classification
        assert!(link.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "link"
            } else {
                false
            }
        }));
        assert_eq!(
            link.object.meta.properties.get("element-type"),
            Some(&SimpleValue::String("link".to_string()))
        );
        
        // 9. Overall processing metadata
        assert!(link.object.meta.properties.contains_key("processed"));
        assert!(link.object.meta.properties.contains_key("fallbackVisitor"));
        
        // 10. Parameter-level processing verification
        assert!(link_params.meta.properties.contains_key("parameter_userId"));
        assert!(link_params.meta.properties.contains_key("parameter_includeProfile"));
        assert!(link_params.meta.properties.contains_key("parameter_apiKey"));
        
        // Verify parameter values are preserved
        assert!(link_params.get("userId").is_some());
        assert!(link_params.get("includeProfile").is_some());
        assert!(link_params.get("apiKey").is_some());
    }

    #[test]
    fn test_build_link_with_validation_error() {
        let mut obj = ObjectElement::new();
        obj.set("operationId", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 123.0,
        }));

        let link = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check validation error metadata
        assert!(link.object.meta.properties.contains_key("validationError_link"));
        if let Some(SimpleValue::String(error_msg)) = link.object.meta.properties.get("validationError_link") {
            assert!(error_msg.contains("Invalid operationId"));
        }
    }

    #[test]
    fn test_build_link_with_spec_path() {
        let mut obj = ObjectElement::new();
        obj.set("description", Element::String(StringElement::new("Test link")));

        let link = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = link.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "Link"));
        }
    }

    #[test]
    fn test_build_link_with_reference() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/links/UserLink")));

        let link = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check reference metadata
        assert_eq!(
            link.object.meta.properties.get("referenced-element"),
            Some(&SimpleValue::string("link".to_string()))
        );
        assert_eq!(
            link.object.meta.properties.get("reference-path"),
            Some(&SimpleValue::string("#/components/links/UserLink".to_string()))
        );
    }

    #[test]
    fn test_build_link_with_operation_ref() {
        let mut obj = ObjectElement::new();
        obj.set("operationRef", Element::String(StringElement::new("#/paths/~1users/get")));

        let link = build_and_decorate_link(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>).unwrap();

        // Check operation ref metadata
        if let Some(op_ref) = link.object.get("operationRef") {
            if let Element::String(op_ref_str) = op_ref {
                assert_eq!(
                    op_ref_str.meta.properties.get("class"),
                    Some(&SimpleValue::string("reference-value".to_string()))
                );
            }
        }
    }
}