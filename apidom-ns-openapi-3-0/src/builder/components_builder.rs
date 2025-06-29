use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::components::ComponentsElement;

/// Basic components builder (fallback)
pub fn build_components(element: Element) -> Option<ComponentsElement> {
    if let Element::Object(obj) = element {
        Some(ComponentsElement::with_content(obj))
    } else {
        None
    }
}

/// Enhanced components builder with metadata injection and reference handling
/// Fully equivalent to TypeScript ComponentsVisitor with all 9 component visitors
pub fn build_and_decorate_components(
    element: Element,
    folder: Option<&mut dyn Fold>,
) -> Option<ComponentsElement> {
    if let Element::Object(obj) = element {
        let mut components_element = ComponentsElement::with_content(obj);
        
        // Collect extension fields first to avoid borrow conflicts
        let mut has_extensions = false;
        
        // Process individual component fields with recursive folding
        if let Some(folder) = folder {
            for member in &mut components_element.object.content {
                if let Element::String(key_elem) = &*member.key {
                    let field_name = &key_elem.content;
                    
                    match field_name.as_str() {
                "schemas" => {
                            process_schemas_field(&mut *member.value, folder);
                        }
                        "responses" => {
                            process_responses_field(&mut *member.value, folder);
                        }
                        "parameters" => {
                            process_parameters_field(&mut *member.value, folder);
                        }
                        "examples" => {
                            process_examples_field(&mut *member.value, folder);
                        }
                        "requestBodies" => {
                            process_request_bodies_field(&mut *member.value, folder);
                        }
                        "headers" => {
                            process_headers_field(&mut *member.value, folder);
                        }
                        "securitySchemes" => {
                            process_security_schemes_field(&mut *member.value, folder);
                        }
                        "links" => {
                            process_links_field(&mut *member.value, folder);
                        }
                        "callbacks" => {
                            process_callbacks_field(&mut *member.value, folder);
                        }
                        _ => {
                            // Handle specification extensions or unknown fields
                            if field_name.starts_with("x-") {
                                has_extensions = true;
                            }
                        }
                    }
                }
            }
        }
        
        // Add metadata after processing
        add_processing_metadata(&mut components_element);
        if has_extensions {
            add_specification_extension_metadata(&mut components_element);
        }
        
        Some(components_element)
    } else {
        None
    }
}

// Individual component field processors (equivalent to TypeScript Visitors)

/// Process schemas field (equivalent to SchemasVisitor)
fn process_schemas_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "schema", 
        &["document", "objects", "Schema"],
        &["document", "objects", "Reference"],
        folder
    );
}

/// Process responses field (equivalent to ResponsesVisitor)
fn process_responses_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "response", 
        &["document", "objects", "Response"],
        &["document", "objects", "Reference"],
        folder
    );
    inject_response_status_codes(field_element);
}

/// Process parameters field (equivalent to ParametersVisitor)
fn process_parameters_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "parameter", 
        &["document", "objects", "Parameter"],
        &["document", "objects", "Reference"],
        folder
    );
    inject_parameter_names(field_element);
}

/// Process examples field (equivalent to ExamplesVisitor)
fn process_examples_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "example", 
        &["document", "objects", "Example"],
        &["document", "objects", "Reference"],
        folder
    );
    inject_example_names(field_element);
}

/// Process requestBodies field (equivalent to RequestBodiesVisitor)
fn process_request_bodies_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "requestBody", 
        &["document", "objects", "RequestBody"],
        &["document", "objects", "Reference"],
        folder
    );
}

/// Process headers field (equivalent to HeadersVisitor)
fn process_headers_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "header", 
        &["document", "objects", "Header"],
        &["document", "objects", "Reference"],
        folder
    );
    inject_header_names(field_element);
}

/// Process securitySchemes field (equivalent to SecuritySchemesVisitor)
fn process_security_schemes_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "securityScheme", 
        &["document", "objects", "SecurityScheme"],
        &["document", "objects", "Reference"],
        folder
    );
}

/// Process links field (equivalent to LinksVisitor)
fn process_links_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "link", 
        &["document", "objects", "Link"],
        &["document", "objects", "Reference"],
        folder
    );
    inject_link_names(field_element);
}

/// Process callbacks field (equivalent to CallbacksVisitor)
fn process_callbacks_field(field_element: &mut Element, folder: &mut dyn Fold) {
    process_component_field_with_spec_path(
        field_element, 
        "callback", 
        &["document", "objects", "Callback"],
        &["document", "objects", "Reference"],
        folder
    );
}

/// Enhanced component field processor with SpecPath support
/// Equivalent to TypeScript MapVisitor + FallbackVisitor with specPath
fn process_component_field_with_spec_path(
    field_element: &mut Element, 
    component_type: &str,
    definition_spec_path: &[&str],
    reference_spec_path: &[&str],
    folder: &mut dyn Fold
) {
    if let Element::Object(field_obj) = field_element {
        for field_member in &mut field_obj.content {
            if let Element::String(key_elem) = &*field_member.key {
                let component_key = &key_elem.content;
                
                // Check if it's a reference (equivalent to isReferenceLikeElement)
                if is_reference(&*field_member.value) {
                    // Inject reference metadata
                    inject_reference_metadata(&mut *field_member.value, component_type);
                    // Add reference-element class
                    add_reference_class(&mut *field_member.value);
                    // Add reference spec path
                    inject_spec_path_metadata(&mut *field_member.value, reference_spec_path);
                } else {
                    // Process non-reference components recursively
                    let processed = folder.fold_element((*field_member.value).clone());
                    *field_member.value = processed;
                    
                    // Add component type class
                    add_component_type_class(&mut *field_member.value, component_type);
                    // Add definition spec path
                    inject_spec_path_metadata(&mut *field_member.value, definition_spec_path);
                }
                
                // Inject component key metadata for semantic access
                inject_component_key_metadata(&mut *field_member.value, component_key, component_type);
                
                // Add element type metadata (equivalent to TypeScript element filtering)
                inject_element_type_metadata(&mut *field_member.value, component_type);
            }
        }
    }
}

/// Check if element is a reference (equivalent to isReferenceLikeElement)
fn is_reference(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        for member in &obj.content {
            if let Element::String(key_elem) = &*member.key {
                if key_elem.content == "$ref" {
                    return true;
                }
            }
        }
    }
    false
}

// Enhanced metadata injection functions

/// Inject reference metadata for referenced elements
fn inject_reference_metadata(element: &mut Element, referenced_type: &str) {
    if let Element::Object(obj) = element {
        // Add referenced-element metadata
        obj.meta.properties.insert(
            "referenced-element".to_string(),
            Value::String(referenced_type.to_string())
        );
        
        // Extract and store the reference path
        for member in &obj.content {
            if let Element::String(key_elem) = &*member.key {
                if key_elem.content == "$ref" {
                    if let Element::String(ref_value) = &*member.value {
                        obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_value.content.clone())
                        );
                    }
                }
            }
        }
    }
}

/// Inject SpecPath metadata (equivalent to TypeScript specPath)
fn inject_spec_path_metadata(element: &mut Element, spec_path: &[&str]) {
    if let Element::Object(obj) = element {
        let path_values: Vec<Value> = spec_path.iter()
            .map(|s| Value::String(s.to_string()))
            .collect();
        
        obj.meta.properties.insert(
            "spec-path".to_string(),
            Value::Array(path_values)
        );
    }
}

/// Inject element type metadata for filtering (equivalent to TypeScript element filtering)
fn inject_element_type_metadata(element: &mut Element, element_type: &str) {
    if let Element::Object(obj) = element {
        obj.meta.properties.insert(
            "element-type".to_string(),
            Value::String(element_type.to_string())
        );
        
        // Add filter metadata (equivalent to TypeScript filter predicates)
        let filter_key = format!("is-{}-element", element_type);
        obj.meta.properties.insert(
            filter_key,
            Value::Bool(true)
        );
    }
}

/// Inject component key metadata for semantic access
fn inject_component_key_metadata(element: &mut Element, key: &str, component_type: &str) {
    if let Element::Object(obj) = element {
        // Inject key-specific metadata based on component type
        let meta_key = match component_type {
            "header" => "header-name",
            "response" => "http-status-code", 
            "parameter" => "parameter-name",
            "example" => "example-name",
            "link" => "link-name",
            "schema" => "schema-name",
            "callback" => "callback-name",
            "securityScheme" => "security-scheme-name",
            "requestBody" => "request-body-name",
            _ => "component-key"
        };
        
        obj.meta.properties.insert(
            meta_key.to_string(),
            Value::String(key.to_string())
        );
        
        // Also add generic component-name for consistency
        obj.meta.properties.insert(
            "component-name".to_string(),
            Value::String(key.to_string())
        );
        
        obj.meta.properties.insert(
            "component-type".to_string(),
            Value::String(component_type.to_string())
        );
    }
}

/// Add reference-element class
fn add_reference_class(element: &mut Element) {
    if let Element::Object(obj) = element {
        obj.classes.content.push(Element::String(StringElement::new("reference-element")));
    }
}

/// Add component type class
fn add_component_type_class(element: &mut Element, component_type: &str) {
    if let Element::Object(obj) = element {
        obj.classes.content.push(Element::String(StringElement::new(component_type)));
    }
}

// Specialized key name injection functions

/// Inject HTTP status codes for response components
fn inject_response_status_codes(field_element: &mut Element) {
    if let Element::Object(field_obj) = field_element {
        for field_member in &mut field_obj.content {
            if let Element::String(key_elem) = &*field_member.key {
                let status_code = &key_elem.content;
                
                // Check if it's a valid HTTP status code pattern
                if is_http_status_code(status_code) {
                    if let Element::Object(response_obj) = &mut *field_member.value {
                        response_obj.meta.properties.insert(
                            "http-status-code".to_string(),
                            Value::String(status_code.clone())
                        );
                        
                        // Add status code category
                        let category = get_status_code_category(status_code);
                        response_obj.meta.properties.insert(
                            "status-code-category".to_string(),
                            Value::String(category.to_string())
                        );
                    }
                }
            }
        }
    }
}

/// Inject header names
fn inject_header_names(field_element: &mut Element) {
    if let Element::Object(field_obj) = field_element {
        for field_member in &mut field_obj.content {
            if let Element::String(key_elem) = &*field_member.key {
                let header_name = &key_elem.content;
                
                if let Element::Object(header_obj) = &mut *field_member.value {
                    header_obj.meta.properties.insert(
                        "header-name".to_string(),
                        Value::String(header_name.clone())
                    );
                    
                    // Add header type classification
                    let header_type = classify_header_type(header_name);
                    header_obj.meta.properties.insert(
                        "header-type".to_string(),
                        Value::String(header_type.to_string())
                    );
                }
            }
        }
    }
}

/// Inject parameter names
fn inject_parameter_names(field_element: &mut Element) {
    if let Element::Object(field_obj) = field_element {
        for field_member in &mut field_obj.content {
            if let Element::String(key_elem) = &*field_member.key {
                let param_name = &key_elem.content;
                
                if let Element::Object(param_obj) = &mut *field_member.value {
                    param_obj.meta.properties.insert(
                        "parameter-name".to_string(),
                        Value::String(param_name.clone())
                    );
                }
            }
        }
    }
}

/// Inject example names
fn inject_example_names(field_element: &mut Element) {
    if let Element::Object(field_obj) = field_element {
        for field_member in &mut field_obj.content {
            if let Element::String(key_elem) = &*field_member.key {
                let example_name = &key_elem.content;
                
                if let Element::Object(example_obj) = &mut *field_member.value {
                    example_obj.meta.properties.insert(
                        "example-name".to_string(),
                        Value::String(example_name.clone())
                    );
                }
            }
        }
    }
}

/// Inject link names
fn inject_link_names(field_element: &mut Element) {
    if let Element::Object(field_obj) = field_element {
        for field_member in &mut field_obj.content {
            if let Element::String(key_elem) = &*field_member.key {
                let link_name = &key_elem.content;
                
                if let Element::Object(link_obj) = &mut *field_member.value {
                    link_obj.meta.properties.insert(
                        "link-name".to_string(),
                        Value::String(link_name.clone())
                    );
                }
            }
        }
    }
}

// Helper functions

/// Check if string is a valid HTTP status code
fn is_http_status_code(code: &str) -> bool {
    // Check for exact status codes (100-599) or patterns like "2XX", "4XX"
    if code.len() == 3 {
        if let Ok(num) = code.parse::<u16>() {
            return (100..=599).contains(&num);
        }
        // Check for patterns like "2XX"
        if code.ends_with("XX") && code.len() == 3 {
            if let Ok(first_digit) = code.chars().next().unwrap().to_string().parse::<u8>() {
                return (1..=5).contains(&first_digit);
            }
        }
    }
    code == "default"
}

/// Get status code category
fn get_status_code_category(code: &str) -> &'static str {
    if code == "default" {
        return "default";
    }
    
    if code.ends_with("XX") {
        match code.chars().next().unwrap() {
            '1' => "informational",
            '2' => "success", 
            '3' => "redirection",
            '4' => "client-error",
            '5' => "server-error",
            _ => "unknown"
        }
    } else if let Ok(num) = code.parse::<u16>() {
        match num {
            100..=199 => "informational",
            200..=299 => "success",
            300..=399 => "redirection", 
            400..=499 => "client-error",
            500..=599 => "server-error",
            _ => "unknown"
        }
    } else {
        "unknown"
    }
}

/// Classify header type
fn classify_header_type(header_name: &str) -> &'static str {
    let lower_name = header_name.to_lowercase();
    match lower_name.as_str() {
        name if name.starts_with("x-") => "custom",
        "content-type" | "accept" | "content-length" | "content-encoding" => "content",
        "authorization" | "www-authenticate" => "authentication",
        "cache-control" | "expires" | "etag" | "last-modified" => "caching",
        "location" | "referer" | "origin" => "navigation",
        _ => "standard"
    }
}

// Metadata helper functions
fn add_processing_metadata(components: &mut ComponentsElement) {
    // Add metadata indicating this was processed by the enhanced builder
    components.object.classes.content.push(Element::String(StringElement::new("components")));
    components.object.classes.content.push(Element::String(StringElement::new("openapi-components")));
}

fn add_specification_extension_metadata(components: &mut ComponentsElement) {
    // Add specification extension metadata
    components.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}