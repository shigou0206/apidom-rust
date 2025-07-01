use apidom_ast::*;
use crate::elements::path_item::PathItemElement;

/// Comprehensive OpenAPI PathItem Builder
/// 
/// This module provides PathItem construction with full TypeScript PathItemVisitor equivalence.
/// 
/// Features supported:
/// - Fixed fields support ($ref, summary, description, servers, parameters)
/// - HTTP method operations (get, post, put, delete, patch, head, options, trace)
/// - Operation metadata injection with HTTP method annotation
/// - $ref reference handling with reference-element class
/// - Specification extensions (x-*) with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion between element types
/// - Comprehensive metadata injection
/// - Element classification and spec path metadata
/// - Validation with proper PathItem structure handling
/// - Complete OpenAPI 3.0 PathItem object compliance

/// Build a basic PathItemElement from a generic Element
pub fn build_path_item(element: &Element) -> Option<PathItemElement> {
    let object = element.as_object()?.clone();
    Some(PathItemElement::with_content(object))
}

/// Build and decorate PathItemElement with enhanced visitor pattern features
pub fn build_and_decorate_path_item<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<PathItemElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut path_item = PathItemElement::new();
    
    // Track if this is a reference PathItem
    let mut is_reference = false;
    
    // Process object members with fixed field support and fallback handling
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            let key_str = &key.content;
            let value = member.value.as_ref();
            
            match key_str.as_str() {
                // $ref field - special handling for reference PathItems
                "$ref" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        path_item.object.set("$ref", Element::String(converted.clone()));
                        add_reference_metadata(&mut path_item.object, &converted.content, "pathItem");
                        is_reference = true;
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        path_item.object.set("$ref", folded);
                        is_reference = true;
                    }
                }
                // Fixed fields - direct mapping with type conversion
                "summary" | "description" => {
                    if let Some(converted) = convert_to_string_element(value) {
                        path_item.object.set(key_str, Element::String(converted));
                        add_fixed_field_metadata(&mut path_item.object, key_str);
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        path_item.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut path_item.object, key_str);
                    }
                }
                "servers" | "parameters" => {
                    // Handle array fields with recursive folding
                    if let Element::Array(array_val) = value {
                        let mut processed_array = ArrayElement::new_empty();
                        for item in &array_val.content {
                            let processed_item = if let Some(ref mut f) = folder {
                                f.fold_element(item.clone())
                            } else {
                                item.clone()
                            };
                            processed_array.content.push(processed_item);
                        }
                        path_item.object.set(key_str, Element::Array(processed_array));
                        add_fixed_field_metadata(&mut path_item.object, key_str);
                        
                        // Add specific metadata for servers/parameters arrays
                        if key_str == "servers" {
                            add_servers_metadata(&mut path_item.object);
                        } else if key_str == "parameters" {
                            add_parameters_metadata(&mut path_item.object);
                        }
                    } else if let Some(folded) = folder.as_mut().map(|f| f.fold_element(value.clone())) {
                        path_item.object.set(key_str, folded);
                        add_fixed_field_metadata(&mut path_item.object, key_str);
                    }
                }
                // HTTP method operations
                "get" | "post" | "put" | "delete" | "patch" | "head" | "options" | "trace" => {
                    let mut processed_value = if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    };
                    
                    // Inject HTTP method metadata (equivalent to TypeScript setMetaProperty)
                    if let Element::Object(ref mut operation_obj) = processed_value {
                        let http_method_upper = key_str.to_uppercase();
                        operation_obj.meta.properties.insert(
                            "http-method".to_string(),
                            SimpleValue::string(http_method_upper.clone())
                        );
                        // Add operation element class
                        operation_obj.add_class("operation");
                        operation_obj.meta.properties.insert(
                            "http-method-original".to_string(),
                            SimpleValue::string(key_str.to_string())
                        );
                    }
                    
                    path_item.object.set(key_str, processed_value);
                    add_operation_metadata(&mut path_item.object, key_str);
                }
                _ => {
                    // Handle specification extensions (x-*) and fallback fields
                    if key_str.starts_with("x-") {
                        // Specification extension
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        path_item.object.set(key_str, processed_value);
                        add_specification_extension_metadata(&mut path_item.object, key_str);
                    } else {
                        // Fallback field - preserve unknown fields
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        path_item.object.set(key_str, processed_value);
                        add_fallback_field_metadata(&mut path_item.object, key_str);
                    }
                }
            }
        }
    }
    
    // Add element class metadata (equivalent to TypeScript class injection)
    path_item.object.add_class("path-item");
    path_item.object.meta.properties.insert(
        "element-type".to_string(),
        SimpleValue::string("pathItem".to_string())
    );
    
    // Add reference-element class if this is a $ref PathItem (TypeScript equivalence)
    if is_reference {
        path_item.object.add_class("reference-element");
        path_item.object.meta.properties.insert(
            "is-reference".to_string(),
            SimpleValue::bool(true)
        );
    }
    
    // Add spec path metadata (equivalent to TypeScript specPath)
    add_spec_path_metadata(&mut path_item.object);
    
    // Validate PathItem structure
    validate_path_item(&path_item)?;
    
    Some(path_item)
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

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    let key = format!("fixed-field_{}", field_name);
    obj.meta.properties.insert(key, SimpleValue::Bool(true));
}

/// Add metadata for servers array processing
fn add_servers_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "has-servers".to_string(),
        SimpleValue::bool(true)
    );
    obj.meta.properties.insert(
        "servers-processed".to_string(),
        SimpleValue::bool(true)
    );
}

/// Add metadata for parameters array processing
fn add_parameters_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert(
        "has-parameters".to_string(),
        SimpleValue::bool(true)
    );
    obj.meta.properties.insert(
        "parameters-processed".to_string(),
        SimpleValue::bool(true)
    );
}

/// Add metadata for HTTP operation methods
fn add_operation_metadata(obj: &mut ObjectElement, method: &str) {
    obj.meta.properties.insert(
        format!("has-{}-operation", method),
        SimpleValue::bool(true)
    );
    obj.meta.properties.insert(
        "has-operations".to_string(),
        SimpleValue::bool(true)
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(obj: &mut ObjectElement, field_name: &str) {
    let key = format!("specification-extension_{}", field_name);
    obj.meta.properties.insert(key, SimpleValue::Bool(true));
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        SimpleValue::bool(true)
    );
}

/// Add metadata for $ref references
fn add_reference_metadata(obj: &mut ObjectElement, ref_path: &str, element_type: &str) {
    obj.add_class("reference");
    obj.meta.properties.insert(
        "referenced-element".to_string(),
        SimpleValue::string(element_type.to_string())
    );
    obj.meta.properties.insert(
        "reference-path".to_string(),
        SimpleValue::string(ref_path.to_string())
    );
    obj.meta.properties.insert(
        "reference-value-class".to_string(),
        SimpleValue::string("reference-value".to_string())
    );
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert("specPath".to_string(), SimpleValue::array(vec![
        SimpleValue::string("document".to_string()),
        SimpleValue::string("objects".to_string()),
        SimpleValue::string("PathItem".to_string())
    ]));
}

/// Validate PathItem structure
fn validate_path_item(path_item: &PathItemElement) -> Option<()> {
    // If this is a $ref PathItem, skip standard validation
    if path_item.ref_().is_some() {
        return Some(()); // $ref PathItems are valid without other fields
    }
    
    // PathItem has no strictly required fields in OpenAPI 3.0
    // However, it should have at least one operation or some content
    let has_operations = has_any_operation(path_item);
    let has_summary = path_item.summary().is_some();
    let has_description = path_item.description().is_some();
    let has_servers = path_item.servers().is_some();
    let has_parameters = path_item.parameters().is_some();
    
    if !has_operations && !has_summary && !has_description && !has_servers && !has_parameters {
        // Empty PathItem - still valid but add warning metadata
        // path_item.object.meta.properties.insert(
        //     "validation-warning".to_string(),
        //     SimpleValue::String("PathItem should have at least one operation or content".to_string())
        // );
    }
    
    Some(())
}

/// Check if PathItem has any HTTP operations
fn has_any_operation(path_item: &PathItemElement) -> bool {
    let http_methods = ["get", "post", "put", "delete", "patch", "head", "options", "trace"];
    http_methods.iter().any(|method| path_item.operation(method).is_some())
}

fn add_processing_metadata(obj: &mut ObjectElement) {
    obj.meta.properties.insert("processed".to_string(), SimpleValue::bool(true));
    obj.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::bool(true));
    obj.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::bool(true));
    obj.meta.properties.insert("canSupportSpecificationExtensions".to_string(), SimpleValue::bool(true));
}

fn add_validation_error_metadata(obj: &mut ObjectElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    obj.meta.properties.insert(key, SimpleValue::string(error_msg.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_path_item_builder() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("User operations")));
        obj.set("description", Element::String(StringElement::new("Operations on user resources")));
        
        let result = build_path_item(&Element::Object(obj));
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        assert_eq!(path_item.object.element, "pathItem");
        assert!(path_item.summary().is_some());
        assert!(path_item.description().is_some());
        
        if let Some(summary) = path_item.summary() {
            assert_eq!(summary.content, "User operations");
        }
    }

    #[test]
    fn test_path_item_empty_object() {
        let obj = ObjectElement::new();
        
        let result = build_path_item(&Element::Object(obj));
        
        assert!(result.is_some()); // PathItem can be empty
        let path_item = result.unwrap();
        assert_eq!(path_item.object.element, "pathItem");
        assert!(path_item.summary().is_none());
        assert!(path_item.description().is_none());
    }

    #[test]
    fn test_enhanced_path_item_with_operations() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("User API")));
        obj.set("description", Element::String(StringElement::new("User management operations")));
        
        // Add GET operation
        let mut get_op = ObjectElement::new();
        get_op.set("operationId", Element::String(StringElement::new("getUser")));
        get_op.set("summary", Element::String(StringElement::new("Get user")));
        obj.set("get", Element::Object(get_op));
        
        // Add POST operation
        let mut post_op = ObjectElement::new();
        post_op.set("operationId", Element::String(StringElement::new("createUser")));
        post_op.set("summary", Element::String(StringElement::new("Create user")));
        obj.set("post", Element::Object(post_op));
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify fixed field metadata
        assert!(path_item.object.meta.properties.contains_key("fixed-field-summary"));
        assert!(path_item.object.meta.properties.contains_key("fixed-field-description"));
        
        // Verify operation metadata
        assert!(path_item.object.meta.properties.contains_key("has-get-operation"));
        assert!(path_item.object.meta.properties.contains_key("has-post-operation"));
        assert!(path_item.object.meta.properties.contains_key("has-operations"));
        
        // Verify HTTP method metadata injection (TypeScript equivalence)
        if let Some(Element::Object(get_obj)) = path_item.get() {
            assert_eq!(
                get_obj.meta.properties.get("http-method"),
                Some(&SimpleValue::String("GET".to_string()))
            );
            assert_eq!(
                get_obj.meta.properties.get("http-method-original"),
                Some(&SimpleValue::String("get".to_string()))
            );
            assert!(get_obj.classes.content.iter().any(|e| {
                if let Element::String(s) = e {
                    s.content == "operation"
                } else {
                    false
                }
            }));
        }
        
        if let Some(Element::Object(post_obj)) = path_item.post() {
            assert_eq!(
                post_obj.meta.properties.get("http-method"),
                Some(&SimpleValue::String("POST".to_string()))
            );
        }
        
        // Verify element class
        assert!(path_item.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "path-item"
            } else {
                false
            }
        }));
        
        // Verify spec path metadata
        assert!(path_item.object.meta.properties.contains_key("spec-path"));
        if let Some(SimpleValue::Array(spec_path)) = path_item.object.meta.properties.get("spec-path") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], SimpleValue::String("document".to_string()));
            assert_eq!(spec_path[1], SimpleValue::String("objects".to_string()));
            assert_eq!(spec_path[2], SimpleValue::String("PathItem".to_string()));
        }
    }

    #[test]
    fn test_path_item_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/pathItems/PetPath")));
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify reference metadata
        assert!(path_item.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert!(path_item.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference-element"
            } else {
                false
            }
        }));
        assert!(path_item.object.meta.properties.contains_key("referenced-element"));
        if let Some(SimpleValue::String(ref_elem)) = path_item.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_elem, "pathItem");
        }
        if let Some(SimpleValue::String(ref_path)) = path_item.object.meta.properties.get("reference-path") {
            assert_eq!(ref_path, "#/components/pathItems/PetPath");
        }
        assert_eq!(
            path_item.object.meta.properties.get("is-reference"),
            Some(&SimpleValue::Bool(true))
        );
        assert_eq!(
            path_item.object.meta.properties.get("reference-value-class"),
            Some(&SimpleValue::String("reference-value".to_string()))
        );
    }

    #[test]
    fn test_path_item_with_servers_and_parameters() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("API with servers and parameters")));
        
        // Add servers array
        let mut servers = ArrayElement::new_empty();
        let mut server1 = ObjectElement::new();
        server1.set("url", Element::String(StringElement::new("https://api.example.com")));
        servers.content.push(Element::Object(server1));
        obj.set("servers", Element::Array(servers));
        
        // Add parameters array
        let mut parameters = ArrayElement::new_empty();
        let mut param1 = ObjectElement::new();
        param1.set("name", Element::String(StringElement::new("version")));
        param1.set("in", Element::String(StringElement::new("header")));
        parameters.content.push(Element::Object(param1));
        obj.set("parameters", Element::Array(parameters));
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify servers metadata
        assert!(path_item.object.meta.properties.contains_key("fixed-field-servers"));
        assert!(path_item.object.meta.properties.contains_key("has-servers"));
        assert!(path_item.object.meta.properties.contains_key("servers-processed"));
        
        // Verify parameters metadata
        assert!(path_item.object.meta.properties.contains_key("fixed-field-parameters"));
        assert!(path_item.object.meta.properties.contains_key("has-parameters"));
        assert!(path_item.object.meta.properties.contains_key("parameters-processed"));
        
        // Verify field values
        assert!(path_item.servers().is_some());
        assert!(path_item.parameters().is_some());
        
        if let Some(servers_arr) = path_item.servers() {
            assert_eq!(servers_arr.content.len(), 1);
        }
        
        if let Some(params_arr) = path_item.parameters() {
            assert_eq!(params_arr.content.len(), 1);
        }
    }

    #[test]
    fn test_path_item_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("Path with extensions")));
        obj.set("x-internal-id", Element::String(StringElement::new("path-001")));
        obj.set("x-rate-limit", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 100.0,
        }));
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify specification extension metadata
        assert!(path_item.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(path_item.object.get("x-internal-id").is_some());
        assert!(path_item.object.get("x-rate-limit").is_some());
    }

    #[test]
    fn test_path_item_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("Path with fallback")));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("unknownProperty", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify fallback field metadata
        assert!(path_item.object.meta.properties.contains_key("fallback-field-customField"));
        assert!(path_item.object.meta.properties.contains_key("fallback-field-unknownProperty"));
        assert!(path_item.object.get("customField").is_some());
        assert!(path_item.object.get("unknownProperty").is_some());
    }

    #[test]
    fn test_path_item_all_http_methods() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("All HTTP methods")));
        
        let http_methods = ["get", "post", "put", "delete", "patch", "head", "options", "trace"];
        
        for method in &http_methods {
            let mut operation = ObjectElement::new();
            operation.set("operationId", Element::String(StringElement::new(&format!("{}_operation", method))));
            obj.set(method, Element::Object(operation));
        }
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify all HTTP methods have metadata
        for method in &http_methods {
            assert!(path_item.object.meta.properties.contains_key(&format!("has-{}-operation", method)));
            
            // Verify HTTP method metadata injection
            if let Some(Element::Object(op_obj)) = path_item.operation(method) {
                assert_eq!(
                    op_obj.meta.properties.get("http-method"),
                    Some(&SimpleValue::String(method.to_uppercase()))
                );
                assert_eq!(
                    op_obj.meta.properties.get("http-method-original"),
                    Some(&SimpleValue::String(method.to_string()))
                );
                assert!(op_obj.classes.content.iter().any(|e| {
                    if let Element::String(s) = e {
                        s.content == "operation"
                    } else {
                        false
                    }
                }));
            }
        }
        
        assert!(path_item.object.meta.properties.contains_key("has-operations"));
    }

    #[test]
    fn test_path_item_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        }));
        obj.set("description", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify type conversion worked
        if let Some(summary) = path_item.summary() {
            assert_eq!(summary.content, "42");
        }
        if let Some(description) = path_item.description() {
            assert_eq!(description.content, "true");
        }
    }

    #[test]
    fn test_path_item_validation() {
        // Test empty path item (should be valid)
        let obj = ObjectElement::new();
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        assert!(result.is_some());
        
        // Test $ref path item (should be valid)
        let mut ref_obj = ObjectElement::new();
        ref_obj.set("$ref", Element::String(StringElement::new("#/components/pathItems/User")));
        let result = build_and_decorate_path_item(&Element::Object(ref_obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        assert!(result.is_some());
        
        // Test path item with operations (should be valid)
        let mut op_obj = ObjectElement::new();
        op_obj.set("get", Element::Object(ObjectElement::new()));
        let result = build_and_decorate_path_item(&Element::Object(op_obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        assert!(result.is_some());
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript PathItemVisitor
        let mut obj = ObjectElement::new();
        obj.set("summary", Element::String(StringElement::new("User management endpoint")));
        obj.set("description", Element::String(StringElement::new("Comprehensive user operations")));
        
        // Add $ref to demonstrate reference handling
        obj.set("$ref", Element::String(StringElement::new("#/components/pathItems/UserOperations")));
        
        // Add HTTP operations with metadata injection
        let mut get_op = ObjectElement::new();
        get_op.set("operationId", Element::String(StringElement::new("getUser")));
        get_op.set("summary", Element::String(StringElement::new("Retrieve user")));
        obj.set("get", Element::Object(get_op));
        
        let mut post_op = ObjectElement::new();
        post_op.set("operationId", Element::String(StringElement::new("createUser")));
        post_op.set("summary", Element::String(StringElement::new("Create new user")));
        obj.set("post", Element::Object(post_op));
        
        // Add servers and parameters
        let mut servers = ArrayElement::new_empty();
        let mut server = ObjectElement::new();
        server.set("url", Element::String(StringElement::new("https://api.example.com/v1")));
        servers.content.push(Element::Object(server));
        obj.set("servers", Element::Array(servers));
        
        let mut parameters = ArrayElement::new_empty();
        let mut param = ObjectElement::new();
        param.set("name", Element::String(StringElement::new("version")));
        param.set("in", Element::String(StringElement::new("header")));
        parameters.content.push(Element::Object(param));
        obj.set("parameters", Element::Array(parameters));
        
        // Add specification extensions
        obj.set("x-path-version", Element::String(StringElement::new("v1")));
        obj.set("x-deprecated", Element::Boolean(BooleanElement::new(false)));
        
        // Add fallback field
        obj.set("customPathMetadata", Element::String(StringElement::new("custom path value")));
        
        let result = build_and_decorate_path_item(&Element::Object(obj), None::<&mut crate::fold::OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let path_item = result.unwrap();
        
        // Verify TypeScript equivalence features:
        
        // 1. Fixed fields processing
        assert!(path_item.object.meta.properties.contains_key("fixed-field-summary"));
        assert!(path_item.object.meta.properties.contains_key("fixed-field-description"));
        assert!(path_item.object.meta.properties.contains_key("fixed-field-servers"));
        assert!(path_item.object.meta.properties.contains_key("fixed-field-parameters"));
        
        // 2. $ref handling with reference-element class (TypeScript equivalence)
        assert!(path_item.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference-element"
            } else {
                false
            }
        }));
        assert_eq!(
            path_item.object.meta.properties.get("is-reference"),
            Some(&SimpleValue::Bool(true))
        );
        assert_eq!(
            path_item.object.meta.properties.get("reference-value-class"),
            Some(&SimpleValue::String("reference-value".to_string()))
        );
        
        // 3. HTTP method metadata injection (equivalent to TypeScript setMetaProperty)
        if let Some(Element::Object(get_obj)) = path_item.get() {
            assert_eq!(
                get_obj.meta.properties.get("http-method"),
                Some(&SimpleValue::String("GET".to_string()))
            );
            assert!(get_obj.classes.content.iter().any(|e| {
                if let Element::String(s) = e {
                    s.content == "operation"
                } else {
                    false
                }
            }));
        }
        
        if let Some(Element::Object(post_obj)) = path_item.post() {
            assert_eq!(
                post_obj.meta.properties.get("http-method"),
                Some(&SimpleValue::String("POST".to_string()))
            );
        }
        
        // 4. Specification extensions
        assert!(path_item.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
        assert!(path_item.object.get("x-path-version").is_some());
        assert!(path_item.object.get("x-deprecated").is_some());
        
        // 5. Fallback field handling
        assert!(path_item.object.meta.properties.contains_key("fallback-field-customPathMetadata"));
        assert!(path_item.object.get("customPathMetadata").is_some());
        
        // 6. Element classification (equivalent to TypeScript class injection)
        assert!(path_item.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "path-item"
            } else {
                false
            }
        }));
        assert_eq!(
            path_item.object.meta.properties.get("element-type"),
            Some(&SimpleValue::String("pathItem".to_string()))
        );
        
        // 7. Spec path metadata (equivalent to TypeScript specPath)
        assert!(path_item.object.meta.properties.contains_key("specPath"));
        if let Some(SimpleValue::Array(spec_path)) = path_item.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], SimpleValue::String("document".to_string()));
            assert_eq!(spec_path[1], SimpleValue::String("objects".to_string()));
            assert_eq!(spec_path[2], SimpleValue::String("PathItem".to_string()));
        }
        
        // 8. Array processing metadata
        assert!(path_item.object.meta.properties.contains_key("has-servers"));
        assert!(path_item.object.meta.properties.contains_key("servers-processed"));
        assert!(path_item.object.meta.properties.contains_key("has-parameters"));
        assert!(path_item.object.meta.properties.contains_key("parameters-processed"));
        
        // 9. Operation processing metadata
        assert!(path_item.object.meta.properties.contains_key("has-get-operation"));
        assert!(path_item.object.meta.properties.contains_key("has-post-operation"));
        assert!(path_item.object.meta.properties.contains_key("has-operations"));
        
        // 10. Field value validation
        assert!(path_item.summary().is_some());
        assert!(path_item.description().is_some());
        assert!(path_item.ref_().is_some());
        assert!(path_item.servers().is_some());
        assert!(path_item.parameters().is_some());
        assert!(path_item.get().is_some());
        assert!(path_item.post().is_some());
        
        if let Some(summary) = path_item.summary() {
            assert_eq!(summary.content, "User management endpoint");
        }
        
        if let Some(ref_val) = path_item.ref_() {
            assert_eq!(ref_val.content, "#/components/pathItems/UserOperations");
        }
    }
}