use apidom_ast::minim_model::*;
use crate::elements::operation::{
    OperationElement, OperationParametersElement, OperationSecurityElement,
    OperationServersElement, OperationTagsElement, OperationCallbacksElement
};
use crate::elements::external_documentation::ExternalDocumentationElement;
use crate::elements::request_body::RequestBodyElement;
use crate::elements::responses::ResponsesElement;
use crate::elements::callback::CallbackElement;
use crate::elements::server::ServerElement;
use crate::elements::parameter::ParameterElement;
use crate::elements::security_requirement::SecurityRequirementElement;
use serde_json::Value;

/// Enhanced Operation builder with TypeScript CallbacksVisitor equivalence
/// Provides structured field processing, metadata injection, and validation
pub fn build_operation(element: &Element) -> Option<OperationElement> {
    let obj = element.as_object()?.clone();
    build_and_decorate_operation(obj)
}

/// Core builder function with comprehensive field processing
pub fn build_and_decorate_operation(mut obj: ObjectElement) -> Option<OperationElement> {
    // Set element type and classes
    obj.set_element_type("operation");
    obj.add_class("operation");
    
    // Create operation element
    let mut operation = OperationElement::with_content(obj.clone());
    
    // Process all fields with structured encapsulation
    process_operation_fields(&mut operation, &obj);
    
    // Inject comprehensive metadata
    inject_operation_metadata(&mut operation.object, &obj);
    
    Some(operation)
}

/// Process all operation fields with structured encapsulation
fn process_operation_fields(operation: &mut OperationElement, source: &ObjectElement) {
    // Process fixed fields
    process_fixed_fields(operation, source);
    
    // Process structured array/object fields
    process_structured_fields(operation, source);
    
    // Process reference fields
    process_reference_fields(operation, source);
    
    // Process specification extensions
    process_specification_extensions(operation, source);
    
    // Process fallback fields
    process_fallback_fields(operation, source);
}

/// Process fixed scalar fields
fn process_fixed_fields(operation: &mut OperationElement, source: &ObjectElement) {
    // tags: string[]
    if let Some(tags_elem) = source.get("tags") {
        if let Some(tags_array) = tags_elem.as_array() {
            let tags_element = build_operation_tags(tags_array.clone());
            operation.set_operation_tags(tags_element);
            add_fixed_field_metadata(&mut operation.object, "tags");
        }
    }
    
    // summary: string
    if let Some(summary_elem) = source.get("summary") {
        if let Some(summary_str) = summary_elem.as_string() {
            operation.set_summary(summary_str.clone());
            add_fixed_field_metadata(&mut operation.object, "summary");
        }
    }
    
    // description: string
    if let Some(desc_elem) = source.get("description") {
        if let Some(desc_str) = desc_elem.as_string() {
            operation.set_description(desc_str.clone());
            add_fixed_field_metadata(&mut operation.object, "description");
        }
    }
    
    // operationId: string
    if let Some(op_id_elem) = source.get("operationId") {
        if let Some(op_id_str) = op_id_elem.as_string() {
            operation.set_operation_id(op_id_str.clone());
            add_fixed_field_metadata(&mut operation.object, "operationId");
        }
    }
    
    // deprecated: boolean
    if let Some(deprecated_elem) = source.get("deprecated") {
        if let Some(deprecated_bool) = deprecated_elem.as_boolean() {
            operation.set_deprecated(deprecated_bool.content);
            add_fixed_field_metadata(&mut operation.object, "deprecated");
        }
    }
}

/// Process structured array/object fields
fn process_structured_fields(operation: &mut OperationElement, source: &ObjectElement) {
    // parameters: Parameter[]
    if let Some(params_elem) = source.get("parameters") {
        if let Some(params_array) = params_elem.as_array() {
            let params_element = build_operation_parameters(params_array.clone());
            operation.set_operation_parameters(params_element);
            add_fixed_field_metadata(&mut operation.object, "parameters");
        }
    }
    
    // requestBody: RequestBody
    if let Some(req_body_elem) = source.get("requestBody") {
        if let Some(req_body_obj) = req_body_elem.as_object() {
            if let Some(request_body) = build_request_body(req_body_obj.clone()) {
                operation.set_request_body(Element::Object(request_body.object));
                add_fixed_field_metadata(&mut operation.object, "requestBody");
            }
        }
    }
    
    // responses: Responses (required)
    if let Some(responses_elem) = source.get("responses") {
        if let Some(responses_obj) = responses_elem.as_object() {
            if let Some(responses) = build_responses(responses_obj.clone()) {
                operation.set_responses(Element::Object(responses.object));
                add_fixed_field_metadata(&mut operation.object, "responses");
            }
        }
    }
    
    // callbacks: Map[string, Callback]
    if let Some(callbacks_elem) = source.get("callbacks") {
        if let Some(callbacks_obj) = callbacks_elem.as_object() {
            let callbacks_element = build_operation_callbacks(callbacks_obj.clone());
            operation.set_operation_callbacks(callbacks_element);
            add_fixed_field_metadata(&mut operation.object, "callbacks");
        }
    }
    
    // security: SecurityRequirement[]
    if let Some(security_elem) = source.get("security") {
        if let Some(security_array) = security_elem.as_array() {
            let security_element = build_operation_security(security_array.clone());
            operation.set_operation_security(security_element);
            add_fixed_field_metadata(&mut operation.object, "security");
        }
    }
    
    // servers: Server[]
    if let Some(servers_elem) = source.get("servers") {
        if let Some(servers_array) = servers_elem.as_array() {
            let servers_element = build_operation_servers(servers_array.clone());
            operation.set_operation_servers(servers_element);
            add_fixed_field_metadata(&mut operation.object, "servers");
        }
    }
    
    // externalDocs: ExternalDocumentation
    if let Some(ext_docs_elem) = source.get("externalDocs") {
        if let Some(ext_docs_obj) = ext_docs_elem.as_object() {
            if let Some(external_docs) = build_external_documentation(ext_docs_obj.clone()) {
                operation.set_external_docs(external_docs.object);
                add_fixed_field_metadata(&mut operation.object, "externalDocs");
            }
        }
    }
}

/// Process reference fields ($ref handling)
fn process_reference_fields(operation: &mut OperationElement, source: &ObjectElement) {
    if let Some(ref_elem) = source.get("$ref") {
        if let Some(ref_str) = ref_elem.as_string() {
            // Mark as reference and add metadata
            operation.object.add_class("reference");
            add_reference_metadata(&mut operation.object, &ref_str.content, "operation");
        }
    }
}

/// Process specification extensions (x-* fields)
fn process_specification_extensions(operation: &mut OperationElement, source: &ObjectElement) {
    for member in &source.content {
        if let Element::String(key_str) = &*member.key {
            if key_str.content.starts_with("x-") {
                // Add specification extension
                operation.object.set(&key_str.content, (*member.value).clone());
                add_specification_extension_metadata(&mut operation.object, &key_str.content);
            }
        }
    }
}

/// Process fallback fields (unknown fields)
fn process_fallback_fields(operation: &mut OperationElement, source: &ObjectElement) {
    let known_fields = [
        "tags", "summary", "description", "operationId", "deprecated",
        "parameters", "requestBody", "responses", "callbacks", "security",
        "servers", "externalDocs", "$ref"
    ];
    
    for member in &source.content {
        if let Element::String(key_str) = &*member.key {
            let field_name = &key_str.content;
            
            // Skip known fields and spec extensions
            if !known_fields.contains(&field_name.as_str()) && !field_name.starts_with("x-") {
                // Add as fallback field
                operation.object.set(field_name, (*member.value).clone());
                add_fallback_field_metadata(&mut operation.object, field_name);
            }
        }
    }
}

/// Build operation-specific parameters element
pub fn build_operation_parameters(array: ArrayElement) -> OperationParametersElement {
    let mut params_element = OperationParametersElement::new();
    
    for element in array.content {
        if let Some(param_obj) = element.as_object() {
            if let Some(param) = build_parameter(param_obj.clone()) {
                params_element.push(Element::Object(param.object));
            }
        } else {
            // Handle non-object elements (e.g., references)
            params_element.push(element);
        }
    }
    
    params_element
}

/// Build operation-specific security element
pub fn build_operation_security(array: ArrayElement) -> OperationSecurityElement {
    let mut security_element = OperationSecurityElement::new();
    
    for element in array.content {
        if let Some(security_obj) = element.as_object() {
            if let Some(security_req) = build_security_requirement(security_obj.clone()) {
                security_element.push(Element::Object(security_req.object));
            }
        } else {
            // Handle non-object elements
            security_element.push(element);
        }
    }
    
    security_element
}

/// Build operation-specific servers element
pub fn build_operation_servers(array: ArrayElement) -> OperationServersElement {
    let mut servers_element = OperationServersElement::new();
    
    for element in array.content {
        if let Some(server_obj) = element.as_object() {
            if let Some(server) = build_server(server_obj.clone()) {
                servers_element.push(Element::Object(server.object));
            }
        } else {
            // Handle non-object elements
            servers_element.push(element);
        }
    }
    
    servers_element
}

/// Build operation-specific tags element
pub fn build_operation_tags(array: ArrayElement) -> OperationTagsElement {
    let mut tags_element = OperationTagsElement::new();
    
    for element in array.content {
        // Tags are typically strings, but can also be Tag objects
        tags_element.push(element);
    }
    
    tags_element
}

/// Build operation-specific callbacks element
pub fn build_operation_callbacks(obj: ObjectElement) -> OperationCallbacksElement {
    let mut callbacks_element = OperationCallbacksElement::new();
    
    for member in obj.content {
        if let Element::String(key_str) = &*member.key {
            let callback_name = &key_str.content;
            
            if let Some(callback_obj) = member.value.as_object() {
                if let Some(callback) = build_callback(callback_obj.clone()) {
                    callbacks_element.set(callback_name, Element::Object(callback.object));
                }
            } else {
                // Handle non-object values (e.g., references)
                callbacks_element.set(callback_name, (*member.value).clone());
            }
        }
    }
    
    callbacks_element
}

/// Inject comprehensive metadata for operation
fn inject_operation_metadata(obj: &mut ObjectElement, source: &ObjectElement) {
    // Add element type metadata
    obj.meta.properties.insert(
        "element-type".to_string(),
        Value::String("operation".to_string())
    );
    
    // Add field count metadata
    obj.meta.properties.insert(
        "field-count".to_string(),
        Value::from(source.content.len())
    );
    
    // Add validation metadata
    let has_responses = source.has_key("responses");
    obj.meta.properties.insert(
        "has-required-responses".to_string(),
        Value::Bool(has_responses)
    );
    
    if !has_responses {
        obj.meta.properties.insert(
            "validation-error".to_string(),
            Value::String("Missing required 'responses' field".to_string())
        );
    }
    
    // Add processing timestamp
    obj.meta.properties.insert(
        "processed-at".to_string(),
        Value::String(chrono::Utc::now().to_rfc3339())
    );
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fixed-field-{}", field_name),
        Value::Bool(true)
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.add_class("specification-extension");
    obj.meta.properties.insert(
        "specification-extension".to_string(),
        Value::String(field_name.to_string())
    );
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(obj: &mut ObjectElement, field_name: &str) {
    obj.meta.properties.insert(
        format!("fallback-field-{}", field_name),
        Value::Bool(true)
    );
}

/// Add metadata for $ref references
fn add_reference_metadata(obj: &mut ObjectElement, ref_path: &str, element_type: &str) {
    obj.add_class("reference");
    obj.meta.properties.insert(
        "referenced-element".to_string(),
        Value::String(element_type.to_string())
    );
    obj.meta.properties.insert(
        "reference-path".to_string(),
        Value::String(ref_path.to_string())
    );
}

// ========== Placeholder builder functions ==========
// These would be implemented based on existing builders in the codebase

fn build_parameter(obj: ObjectElement) -> Option<ParameterElement> {
    // Placeholder - would use existing parameter builder
    Some(ParameterElement::with_content(obj))
}

fn build_request_body(obj: ObjectElement) -> Option<RequestBodyElement> {
    // Placeholder - would use existing request body builder
    Some(RequestBodyElement::with_content(obj))
}

fn build_responses(obj: ObjectElement) -> Option<ResponsesElement> {
    // Placeholder - would use existing responses builder
    Some(ResponsesElement::with_content(obj))
}

fn build_callback(obj: ObjectElement) -> Option<CallbackElement> {
    // Placeholder - would use existing callback builder
    Some(CallbackElement::with_content(obj))
}

fn build_server(obj: ObjectElement) -> Option<ServerElement> {
    // Placeholder - would use existing server builder
    Some(ServerElement::with_content(obj))
}

fn build_security_requirement(obj: ObjectElement) -> Option<SecurityRequirementElement> {
    // Placeholder - would use existing security requirement builder
    Some(SecurityRequirementElement::with_content(obj))
}

fn build_external_documentation(obj: ObjectElement) -> Option<ExternalDocumentationElement> {
    // Placeholder - would use existing external documentation builder
    Some(ExternalDocumentationElement::with_content(obj))
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
    fn test_build_operation_basic() {
        let obj = create_test_object(json!({
            "summary": "Test operation",
            "description": "A test operation",
            "operationId": "testOp",
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check basic fields
        assert_eq!(operation.summary().unwrap().content, "Test operation");
        assert_eq!(operation.description().unwrap().content, "A test operation");
        assert_eq!(operation.operation_id().unwrap().content, "testOp");

        // Check element type and classes
        assert_eq!(operation.object.element, "operation");
        assert!(operation.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "operation"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_build_operation_with_parameters() {
        let obj = create_test_object(json!({
            "parameters": [
                {
                    "name": "id",
                    "in": "path",
                    "required": true,
                    "schema": {
                        "type": "string"
                    }
                }
            ],
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check parameters
        assert!(operation.parameters().is_some());
        let params = operation.parameters().unwrap();
        assert_eq!(params.content.len(), 1);
    }

    #[test]
    fn test_build_operation_with_tags() {
        let obj = create_test_object(json!({
            "tags": ["users", "admin"],
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check tags
        assert!(operation.tags().is_some());
        let tags = operation.tags().unwrap();
        assert_eq!(tags.content.len(), 2);
    }

    #[test]
    fn test_build_operation_with_security() {
        let obj = create_test_object(json!({
            "security": [
                {
                    "api_key": []
                }
            ],
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check security
        assert!(operation.security().is_some());
        let security = operation.security().unwrap();
        assert_eq!(security.content.len(), 1);
    }

    #[test]
    fn test_build_operation_with_deprecated() {
        let obj = create_test_object(json!({
            "deprecated": true,
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check deprecated
        assert!(operation.deprecated());
    }

    #[test]
    fn test_build_operation_with_reference() {
        let obj = create_test_object(json!({
            "$ref": "#/components/operations/testOp"
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check reference class and metadata
        assert!(operation.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));

        // Check reference metadata
        assert!(operation.object.meta.properties.contains_key("referenced-element"));
        assert!(operation.object.meta.properties.contains_key("reference-path"));
    }

    #[test]
    fn test_build_operation_with_spec_extensions() {
        let obj = create_test_object(json!({
            "x-custom-field": "custom-value",
            "x-another-extension": "another-value",
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check specification extensions
        assert!(operation.object.get("x-custom-field").is_some());
        assert!(operation.object.get("x-another-extension").is_some());

        // Check specification extension metadata
        assert!(operation.object.meta.properties.contains_key("specification-extension"));
    }

    #[test]
    fn test_build_operation_metadata_injection() {
        let obj = create_test_object(json!({
            "summary": "Test",
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check comprehensive metadata
        assert_eq!(
            operation.object.meta.properties.get("element-type"),
            Some(&Value::String("operation".to_string()))
        );
        assert!(operation.object.meta.properties.contains_key("field-count"));
        assert_eq!(
            operation.object.meta.properties.get("has-required-responses"),
            Some(&Value::Bool(true))
        );
        assert!(operation.object.meta.properties.contains_key("processed-at"));

        // Check fixed field metadata
        assert_eq!(
            operation.object.meta.properties.get("fixed-field-summary"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            operation.object.meta.properties.get("fixed-field-responses"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn test_build_operation_validation_error() {
        let obj = create_test_object(json!({
            "summary": "Test without responses"
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check validation error for missing responses
        assert_eq!(
            operation.object.meta.properties.get("has-required-responses"),
            Some(&Value::Bool(false))
        );
        assert!(operation.object.meta.properties.contains_key("validation-error"));
    }

    #[test]
    fn test_build_operation_fallback_fields() {
        let obj = create_test_object(json!({
            "unknownField": "unknown-value",
            "anotherUnknown": "another-unknown",
            "responses": {
                "200": {
                    "description": "Success"
                }
            }
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Check fallback fields
        assert!(operation.object.get("unknownField").is_some());
        assert!(operation.object.get("anotherUnknown").is_some());

        // Check fallback field metadata
        assert_eq!(
            operation.object.meta.properties.get("fallback-field-unknownField"),
            Some(&Value::Bool(true))
        );
        assert_eq!(
            operation.object.meta.properties.get("fallback-field-anotherUnknown"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn test_build_operation_comprehensive() {
        let obj = create_test_object(json!({
            "tags": ["users"],
            "summary": "Get user by ID",
            "description": "Retrieve a user by their unique identifier",
            "operationId": "getUserById",
            "deprecated": false,
            "parameters": [
                {
                    "name": "id",
                    "in": "path",
                    "required": true,
                    "schema": {
                        "type": "string"
                    }
                }
            ],
            "responses": {
                "200": {
                    "description": "User found"
                },
                "404": {
                    "description": "User not found"
                }
            },
            "security": [
                {
                    "api_key": []
                }
            ],
            "x-rate-limit": "100/hour"
        }));

        let operation = build_and_decorate_operation(obj).unwrap();

        // Verify all fields are processed
        assert!(operation.tags().is_some());
        assert_eq!(operation.summary().unwrap().content, "Get user by ID");
        assert_eq!(operation.description().unwrap().content, "Retrieve a user by their unique identifier");
        assert_eq!(operation.operation_id().unwrap().content, "getUserById");
        assert!(!operation.deprecated());
        assert!(operation.parameters().is_some());
        assert!(operation.responses().is_some());
        assert!(operation.security().is_some());
        assert!(operation.object.get("x-rate-limit").is_some());

        // Verify metadata completeness
        assert!(operation.object.meta.properties.len() >= 8); // Should have comprehensive metadata
    }
}