use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use crate::elements::components::ComponentsElement;
use serde_json::Value;

/// Enhanced Components builder with full TypeScript visitor equivalence
/// 
/// This function provides comprehensive functionality equivalent to the TypeScript ComponentsVisitor:
/// - Individual component field processing (schemas, responses, parameters, etc.)
/// - Reference detection and metadata injection for each component type
/// - Key-name metadata injection (schema-name, response-name, etc.)
/// - Recursive processing of component sub-elements
/// - Specification extensions support
/// - Complete metadata annotation system
/// - Fallback behavior for unknown fields
pub fn build_and_decorate_components<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<ComponentsElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut components = ComponentsElement::new();
    
    // Add processing metadata
    add_processing_metadata(&mut components);
    add_spec_path_metadata(&mut components);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            components.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut components, &ref_str.content);
            return Some(components);
        }
    }

    // Process all object members with enhanced visitor pattern
    for member in &obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            match key.as_str() {
                "schemas" => {
                    if let Some(processed) = process_schemas_field(value, folder.as_deref_mut()) {
                        components.set_schemas(processed);
                        add_field_metadata(&mut components, "schemas", "ComponentsSchemasElement");
                    }
                }
                "responses" => {
                    if let Some(processed) = process_responses_field(value, folder.as_deref_mut()) {
                        components.set_responses(processed);
                        add_field_metadata(&mut components, "responses", "ComponentsResponsesElement");
                    }
                }
                "parameters" => {
                    if let Some(processed) = process_parameters_field(value, folder.as_deref_mut()) {
                        components.set_parameters(processed);
                        add_field_metadata(&mut components, "parameters", "ComponentsParametersElement");
                    }
                }
                "examples" => {
                    if let Some(processed) = process_examples_field(value, folder.as_deref_mut()) {
                        components.set_examples(processed);
                        add_field_metadata(&mut components, "examples", "ComponentsExamplesElement");
                    }
                }
                "requestBodies" => {
                    if let Some(processed) = process_request_bodies_field(value, folder.as_deref_mut()) {
                        components.set_request_bodies(processed);
                        add_field_metadata(&mut components, "requestBodies", "ComponentsRequestBodiesElement");
                    }
                }
                "headers" => {
                    if let Some(processed) = process_headers_field(value, folder.as_deref_mut()) {
                        components.set_headers(processed);
                        add_field_metadata(&mut components, "headers", "ComponentsHeadersElement");
                    }
                }
                "securitySchemes" => {
                    if let Some(processed) = process_security_schemes_field(value, folder.as_deref_mut()) {
                        components.set_security_schemes(processed);
                        add_field_metadata(&mut components, "securitySchemes", "ComponentsSecuritySchemesElement");
                    }
                }
                "links" => {
                    if let Some(processed) = process_links_field(value, folder.as_deref_mut()) {
                        components.set_links(processed);
                        add_field_metadata(&mut components, "links", "ComponentsLinksElement");
                    }
                }
                "callbacks" => {
                    if let Some(processed) = process_callbacks_field(value, folder.as_deref_mut()) {
                        components.set_callbacks(processed);
                        add_field_metadata(&mut components, "callbacks", "ComponentsCallbacksElement");
                    }
                }
                _ if key.starts_with("x-") => {
                    // Handle specification extensions
                    components.object.set(key, value.clone());
                    add_specification_extension_metadata(&mut components, key);
                }
                _ => {
                    // Unknown field - add with fallback metadata
                    components.object.set(key, value.clone());
                    add_unknown_field_metadata(&mut components, key);
                }
            }
        }
    }

    Some(components)
}

/// Process schemas field equivalent to ComponentsSchemasVisitor
fn process_schemas_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut schemas_obj = obj.clone();
    
    // Process each schema entry
    for member in &mut schemas_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let schema_name = &key_str.content;
            
            if let Element::Object(schema_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = schema_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        schema_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("schema".to_string())
                        );
                        schema_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        schema_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        schema_obj.classes.content.push(Element::String(StringElement::new("schema-reference")));
                    }
                } else {
                    // Process as actual schema using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        schema_obj.set_element_type("schema");
                        let folded = f.fold_object_element(schema_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *schema_obj = folded_obj;
                        }
                    }
                }
                
                // Add schema name metadata
                schema_obj.meta.properties.insert(
                    "schema-name".to_string(),
                    Value::String(schema_name.clone())
                );
                schema_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("schema".to_string())
                );
            }
        }
    }
    
    Some(schemas_obj)
}

/// Process responses field equivalent to ComponentsResponsesVisitor
fn process_responses_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut responses_obj = obj.clone();
    
    // Process each response entry
    for member in &mut responses_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let response_name = &key_str.content;
            
            if let Element::Object(response_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = response_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        response_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("response".to_string())
                        );
                        response_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        response_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        response_obj.classes.content.push(Element::String(StringElement::new("response-reference")));
                    }
                } else {
                    // Process as actual response using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        response_obj.set_element_type("response");
                        let folded = f.fold_object_element(response_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *response_obj = folded_obj;
                        }
                    }
                }
                
                // Add response name metadata (equivalent to http-status-code)
                response_obj.meta.properties.insert(
                    "response-name".to_string(),
                    Value::String(response_name.clone())
                );
                response_obj.meta.properties.insert(
                    "http-status-code".to_string(),
                    Value::String(response_name.clone())
                );
                response_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("response".to_string())
                );
            }
        }
    }
    
    Some(responses_obj)
}

/// Process parameters field equivalent to ComponentsParametersVisitor
fn process_parameters_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut parameters_obj = obj.clone();
    
    // Process each parameter entry
    for member in &mut parameters_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let param_name = &key_str.content;
            
            if let Element::Object(param_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = param_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        param_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("parameter".to_string())
                        );
                        param_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        param_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        param_obj.classes.content.push(Element::String(StringElement::new("parameter-reference")));
                    }
                } else {
                    // Process as actual parameter using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        param_obj.set_element_type("parameter");
                        let folded = f.fold_object_element(param_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *param_obj = folded_obj;
                        }
                    }
                }
                
                // Add parameter name metadata
                param_obj.meta.properties.insert(
                    "parameter-name".to_string(),
                    Value::String(param_name.clone())
                );
                param_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("parameter".to_string())
                );
            }
        }
    }
    
    Some(parameters_obj)
}

/// Process examples field equivalent to ComponentsExamplesVisitor
fn process_examples_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut examples_obj = obj.clone();
    
    // Process each example entry
    for member in &mut examples_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let example_name = &key_str.content;
            
            if let Element::Object(example_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = example_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        example_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("example".to_string())
                        );
                        example_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        example_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        example_obj.classes.content.push(Element::String(StringElement::new("example-reference")));
                    }
                } else {
                    // Process as actual example using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        example_obj.set_element_type("example");
                        let folded = f.fold_object_element(example_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *example_obj = folded_obj;
                        }
                    }
                }
                
                // Add example name metadata
                example_obj.meta.properties.insert(
                    "example-name".to_string(),
                    Value::String(example_name.clone())
                );
                example_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("example".to_string())
                );
            }
        }
    }
    
    Some(examples_obj)
}

/// Process requestBodies field equivalent to ComponentsRequestBodiesVisitor
fn process_request_bodies_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut request_bodies_obj = obj.clone();
    
    // Process each request body entry
    for member in &mut request_bodies_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let request_body_name = &key_str.content;
            
            if let Element::Object(request_body_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = request_body_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        request_body_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("requestBody".to_string())
                        );
                        request_body_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        request_body_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        request_body_obj.classes.content.push(Element::String(StringElement::new("request-body-reference")));
                    }
                } else {
                    // Process as actual request body using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        request_body_obj.set_element_type("requestBody");
                        let folded = f.fold_object_element(request_body_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *request_body_obj = folded_obj;
                        }
                    }
                }
                
                // Add request body name metadata
                request_body_obj.meta.properties.insert(
                    "request-body-name".to_string(),
                    Value::String(request_body_name.clone())
                );
                request_body_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("requestBody".to_string())
                );
            }
        }
    }
    
    Some(request_bodies_obj)
}

/// Process headers field equivalent to ComponentsHeadersVisitor
fn process_headers_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut headers_obj = obj.clone();
    
    // Process each header entry
    for member in &mut headers_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let header_name = &key_str.content;
            
            if let Element::Object(header_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = header_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        header_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("header".to_string())
                        );
                        header_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        header_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        header_obj.classes.content.push(Element::String(StringElement::new("header-reference")));
                    }
                } else {
                    // Process as actual header using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        header_obj.set_element_type("header");
                        let folded = f.fold_object_element(header_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *header_obj = folded_obj;
                        }
                    }
                }
                
                // Add header name metadata (equivalent to TypeScript header-name)
                header_obj.meta.properties.insert(
                    "header-name".to_string(),
                    Value::String(header_name.clone())
                );
                header_obj.meta.properties.insert(
                    "header-name-metadata".to_string(),
                    Value::String(header_name.clone())
                );
                header_obj.meta.properties.insert(
                    "is-header".to_string(),
                    Value::Bool(true)
                );
                header_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("header".to_string())
                );
            }
        }
    }
    
    Some(headers_obj)
}

/// Process securitySchemes field equivalent to ComponentsSecuritySchemesVisitor
fn process_security_schemes_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut security_schemes_obj = obj.clone();
    
    // Process each security scheme entry
    for member in &mut security_schemes_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let scheme_name = &key_str.content;
            
            if let Element::Object(scheme_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = scheme_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        scheme_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("securityScheme".to_string())
                        );
                        scheme_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        scheme_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        scheme_obj.classes.content.push(Element::String(StringElement::new("security-scheme-reference")));
                    }
                } else {
                    // Process as actual security scheme using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        scheme_obj.set_element_type("securityScheme");
                        let folded = f.fold_object_element(scheme_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *scheme_obj = folded_obj;
                        }
                    }
                }
                
                // Add security scheme name metadata
                scheme_obj.meta.properties.insert(
                    "security-scheme-name".to_string(),
                    Value::String(scheme_name.clone())
                );
                scheme_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("securityScheme".to_string())
                );
            }
        }
    }
    
    Some(security_schemes_obj)
}

/// Process links field equivalent to ComponentsLinksVisitor
fn process_links_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut links_obj = obj.clone();
    
    // Process each link entry
    for member in &mut links_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let link_name = &key_str.content;
            
            if let Element::Object(link_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = link_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        link_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("link".to_string())
                        );
                        link_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        link_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        link_obj.classes.content.push(Element::String(StringElement::new("link-reference")));
                    }
                } else {
                    // Process as actual link using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        link_obj.set_element_type("link");
                        let folded = f.fold_object_element(link_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *link_obj = folded_obj;
                        }
                    }
                }
                
                // Add link name metadata
                link_obj.meta.properties.insert(
                    "link-name".to_string(),
                    Value::String(link_name.clone())
                );
                link_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("link".to_string())
                );
            }
        }
    }
    
    Some(links_obj)
}

/// Process callbacks field equivalent to ComponentsCallbacksVisitor
fn process_callbacks_field<F>(value: &Element, mut folder: Option<&mut F>) -> Option<ObjectElement>
where
    F: Fold,
{
    let obj = value.as_object()?;
    let mut callbacks_obj = obj.clone();
    
    // Process each callback entry
    for member in &mut callbacks_obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let callback_name = &key_str.content;
            
            if let Element::Object(callback_obj) = member.value.as_mut() {
                // Check if it's a reference
                if let Some(ref_value) = callback_obj.get("$ref") {
                    if let Some(ref_str) = ref_value.as_string() {
                        // Add reference metadata
                        callback_obj.meta.properties.insert(
                            "referenced-element".to_string(),
                            Value::String("callback".to_string())
                        );
                        callback_obj.meta.properties.insert(
                            "reference-path".to_string(),
                            Value::String(ref_str.content.clone())
                        );
                        callback_obj.classes.content.push(Element::String(StringElement::new("reference")));
                        callback_obj.classes.content.push(Element::String(StringElement::new("callback-reference")));
                    }
                } else {
                    // Process as actual callback using folder if available
                    if let Some(f) = folder.as_deref_mut() {
                        callback_obj.set_element_type("callback");
                        let folded = f.fold_object_element(callback_obj.clone());
                        if let Element::Object(folded_obj) = folded {
                            *callback_obj = folded_obj;
                        }
                    }
                }
                
                // Add callback name metadata
                callback_obj.meta.properties.insert(
                    "callback-name".to_string(),
                    Value::String(callback_name.clone())
                );
                callback_obj.meta.properties.insert(
                    "component-type".to_string(),
                    Value::String("callback".to_string())
                );
            }
        }
    }
    
    Some(callbacks_obj)
}

// Metadata helper functions

/// Add processing metadata to components element
fn add_processing_metadata(components: &mut ComponentsElement) {
    components.object.meta.properties.insert(
        "processing-metadata".to_string(),
        Value::Bool(true)
    );
    components.object.meta.properties.insert(
        "visitor-type".to_string(),
        Value::String("ComponentsVisitor".to_string())
    );
    components.object.meta.properties.insert(
        "typescript-equivalent".to_string(),
        Value::Bool(true)
    );
    
    // Add classes
    components.object.classes.content.push(Element::String(StringElement::new("components")));
    components.object.classes.content.push(Element::String(StringElement::new("openapi-components")));
}

/// Add spec path metadata
fn add_spec_path_metadata(components: &mut ComponentsElement) {
    components.object.meta.properties.insert(
        "spec-path".to_string(),
        Value::String("document.objects.Components".to_string())
    );
    components.object.meta.properties.insert(
        "element-type".to_string(),
        Value::String("components".to_string())
    );
}

/// Add reference metadata
fn add_ref_metadata(components: &mut ComponentsElement, ref_path: &str) {
    components.object.meta.properties.insert(
        "referenced-element".to_string(),
        Value::String("components".to_string())
    );
    components.object.meta.properties.insert(
        "reference-path".to_string(),
        Value::String(ref_path.to_string())
    );
    components.object.meta.properties.insert(
        "is-reference".to_string(),
        Value::Bool(true)
    );
    
    // Add reference classes
    components.object.classes.content.push(Element::String(StringElement::new("reference")));
    components.object.classes.content.push(Element::String(StringElement::new("components-reference")));
}

/// Add field-specific metadata
fn add_field_metadata(components: &mut ComponentsElement, field_name: &str, element_type: &str) {
    let field_key = format!("fixed-field-{}", field_name);
    components.object.meta.properties.insert(
        field_key,
        Value::Bool(true)
    );
    
    let element_key = format!("{}-element-type", field_name);
    components.object.meta.properties.insert(
        element_key,
        Value::String(element_type.to_string())
    );
    
    let processed_key = format!("{}-processed", field_name);
    components.object.meta.properties.insert(
        processed_key,
        Value::Bool(true)
    );
}

/// Add specification extension metadata
fn add_specification_extension_metadata(components: &mut ComponentsElement, field_name: &str) {
    components.object.meta.properties.insert(
        "has-specification-extensions".to_string(),
        Value::Bool(true)
    );
    
    let ext_key = format!("spec-extension-{}", field_name);
    components.object.meta.properties.insert(
        ext_key,
        Value::Bool(true)
    );
    
    // Add extension class
    components.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add unknown field metadata
fn add_unknown_field_metadata(components: &mut ComponentsElement, field_name: &str) {
    components.object.meta.properties.insert(
        "has-unknown-fields".to_string(),
        Value::Bool(true)
    );
    
    let unknown_key = format!("unknown-field-{}", field_name);
    components.object.meta.properties.insert(
        unknown_key,
        Value::Bool(true)
    );
}

/// Legacy build_components function for backward compatibility
pub fn build_components(element: &Element) -> Option<ComponentsElement> {
    build_and_decorate_components::<crate::fold::OpenApiBuilderFolder>(element, None)
}