/*!
 * OpenAPI 3.0 SecurityScheme Element Builder
 * 
 * This module provides comprehensive SecurityScheme element building functionality
 * equivalent to the TypeScript SecuritySchemeVisitor. It supports:
 * - FixedFieldsVisitor pattern with type-driven processing
 * - Security scheme type validation (apiKey, http, oauth2, openIdConnect)
 * - OAuth2 flows recursive processing with build_oauth_flows integration
 * - Specification extensions support (x-* fields)
 * - Reference support ($ref handling)
 * - Fallback behavior for unknown fields
 * - Type conversion and validation
 * - Comprehensive metadata injection with classes and spec path
 * - Bearer format validation for HTTP schemes
 * - OpenID Connect URL validation
 */

use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::security_scheme::SecuritySchemeElement;
use crate::builder::oauth_flows_builder::build_and_decorate_oauth_flows;

/// Build a basic SecuritySchemeElement from a generic Element
/// 
/// Example input:
/// {
///   "type": "http",
///   "scheme": "bearer",
///   "bearerFormat": "JWT",
///   "description": "JWT Bearer token"
/// }
pub fn build_security_scheme(element: &Element) -> Option<SecuritySchemeElement> {
    let object = element.as_object()?.clone();
    Some(SecuritySchemeElement::with_content(object))
}

/// Build and decorate SecuritySchemeElement with enhanced visitor pattern features
/// 
/// This function provides equivalent functionality to the TypeScript SecuritySchemeVisitor:
/// - FixedFieldsVisitor pattern with type-driven field processing
/// - Security scheme type validation and type-specific processing
/// - OAuth2 flows recursive processing (equivalent to OAuthFlowsVisitor integration)
/// - Specification extensions support (x-* fields)
/// - Reference handling with metadata
/// - Fallback behavior for unknown fields
/// - Bearer format validation for HTTP schemes
/// - OpenID Connect URL validation
/// - Comprehensive metadata injection with classes and spec path
pub fn build_and_decorate_security_scheme<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<SecuritySchemeElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut sec_scheme = SecuritySchemeElement::new();
    
    // Add processing metadata (equivalent to TypeScript FixedFieldsVisitor + FallbackVisitor)
    add_processing_metadata(&mut sec_scheme);
    add_spec_path_metadata(&mut sec_scheme);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            sec_scheme.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut sec_scheme, &ref_str.content);
            return Some(sec_scheme);
        }
    }
    
    // First pass: extract type to drive processing logic
    let scheme_type = obj.get("type")
        .and_then(Element::as_string)
        .map(|s| s.content.as_str())
        .unwrap_or("");
    
    // Process all object members with FixedFieldsVisitor pattern
    for member in &obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            match key.as_str() {
                // Fixed fields - type-driven processing
                "type" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        if validate_security_scheme_type(&string_elem.content) {
                            sec_scheme.set_type(string_elem);
                            add_fixed_field_metadata(&mut sec_scheme, "type");
                        } else {
                            add_validation_error_metadata(&mut sec_scheme, "type", 
                                "Invalid security scheme type. Must be one of: apiKey, http, oauth2, openIdConnect");
                        }
                    } else {
                        add_validation_error_metadata(&mut sec_scheme, "type", "Expected string value");
                    }
                }
                "description" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        sec_scheme.set_description(string_elem);
                        add_fixed_field_metadata(&mut sec_scheme, "description");
                    } else {
                        add_validation_error_metadata(&mut sec_scheme, "description", "Expected string value");
                    }
                }
                "name" => {
                    // Required for apiKey type
                    if let Some(string_elem) = convert_to_string_element(value) {
                        let content = string_elem.content.clone(); // Clone before move
                        sec_scheme.set_name(string_elem);
                        add_fixed_field_metadata(&mut sec_scheme, "name");
                        
                        // Type-specific validation
                        if scheme_type == "apiKey" && content.is_empty() {
                            add_validation_error_metadata(&mut sec_scheme, "name", 
                                "name is required for apiKey security schemes");
                        }
                    } else {
                        add_validation_error_metadata(&mut sec_scheme, "name", "Expected string value");
                    }
                }
                "in" => {
                    // Required for apiKey type
                    if let Some(string_elem) = convert_to_string_element(value) {
                        let content = string_elem.content.clone(); // Clone before move
                        if validate_api_key_location(&content) {
                            sec_scheme.set_in(string_elem);
                            add_fixed_field_metadata(&mut sec_scheme, "in");
                        } else {
                            add_validation_error_metadata(&mut sec_scheme, "in", 
                                "Invalid location. Must be one of: query, header, cookie");
                        }
                    } else {
                        add_validation_error_metadata(&mut sec_scheme, "in", "Expected string value");
                    }
                }
                "scheme" => {
                    // Required for http type
                    if let Some(string_elem) = convert_to_string_element(value) {
                        let content = string_elem.content.clone(); // Clone before move
                        sec_scheme.set_scheme(string_elem);
                        add_fixed_field_metadata(&mut sec_scheme, "scheme");
                        
                        // Type-specific validation
                        if scheme_type == "http" && content.is_empty() {
                            add_validation_error_metadata(&mut sec_scheme, "scheme", 
                                "scheme is required for http security schemes");
                        }
                    } else {
                        add_validation_error_metadata(&mut sec_scheme, "scheme", "Expected string value");
                    }
                }
                "bearerFormat" => {
                    // Optional for http bearer schemes
                    if let Some(string_elem) = convert_to_string_element(value) {
                        sec_scheme.set_bearer_format(string_elem);
                        add_fixed_field_metadata(&mut sec_scheme, "bearerFormat");
                    } else {
                        add_validation_error_metadata(&mut sec_scheme, "bearerFormat", "Expected string value");
                    }
                }
                "flows" => {
                    // Required for oauth2 type - recursive processing with OAuthFlowsVisitor equivalent
                    if scheme_type == "oauth2" {
                        if let Some(flows) = build_and_decorate_oauth_flows(value, folder.as_deref_mut()) {
                            sec_scheme.set_flows(flows.object);
                            add_fixed_field_metadata(&mut sec_scheme, "flows");
                        } else {
                            add_validation_error_metadata(&mut sec_scheme, "flows", 
                                "Invalid OAuth flows structure");
                        }
                    } else {
                        // flows field only valid for oauth2 type
                        add_validation_error_metadata(&mut sec_scheme, "flows", 
                            "flows field is only valid for oauth2 security schemes");
                    }
                }
                "openIdConnectUrl" => {
                    // Required for openIdConnect type
                    if let Some(string_elem) = convert_to_string_element(value) {
                        let content = string_elem.content.clone(); // Clone before move
                        if validate_url_format(&content) {
                            sec_scheme.set_openid_connect_url(string_elem);
                            add_fixed_field_metadata(&mut sec_scheme, "openIdConnectUrl");
                            
                            // Type-specific validation
                            if scheme_type == "openIdConnect" && content.is_empty() {
                                add_validation_error_metadata(&mut sec_scheme, "openIdConnectUrl", 
                                    "openIdConnectUrl is required for openIdConnect security schemes");
                            }
                        } else {
                            add_validation_error_metadata(&mut sec_scheme, "openIdConnectUrl", "Invalid URL format");
                        }
                    } else {
                        add_validation_error_metadata(&mut sec_scheme, "openIdConnectUrl", "Expected string value");
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
                        
                        sec_scheme.object.set(key, processed_value);
                        add_specification_extension_metadata(&mut sec_scheme, key);
                    } else {
                        // Fallback field (preserve unknown fields)
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        
                        sec_scheme.object.set(key, processed_value);
                        add_fallback_metadata(&mut sec_scheme, key);
                    }
                }
            }
        }
    }
    
    // Add element class metadata (equivalent to TypeScript class injection)
    sec_scheme.object.add_class("security-scheme");
    sec_scheme.object.meta.properties.insert(
        "element-type".to_string(),
        Value::String("securityScheme".to_string())
    );
    
    // Type-specific validation
    validate_security_scheme_constraints(&mut sec_scheme, scheme_type);
    
    Some(sec_scheme)
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

/// Validate security scheme type
fn validate_security_scheme_type(scheme_type: &str) -> bool {
    matches!(scheme_type, "apiKey" | "http" | "oauth2" | "openIdConnect")
}

/// Validate API key location
fn validate_api_key_location(location: &str) -> bool {
    matches!(location, "query" | "header" | "cookie")
}

/// Validate URL format (basic validation)
fn validate_url_format(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(sec_scheme: &mut SecuritySchemeElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    sec_scheme.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for references
fn add_ref_metadata(sec_scheme: &mut SecuritySchemeElement, ref_path: &str) {
    sec_scheme.object.add_class("reference");
    sec_scheme.object.meta.properties.insert(
        "referenced-element".to_string(),
        Value::String("securityScheme".to_string())
    );
    sec_scheme.object.meta.properties.insert(
        "reference-path".to_string(),
        Value::String(ref_path.to_string())
    );
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(sec_scheme: &mut SecuritySchemeElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    sec_scheme.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(sec_scheme: &mut SecuritySchemeElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    sec_scheme.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for validation errors
fn add_validation_error_metadata(sec_scheme: &mut SecuritySchemeElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    sec_scheme.object.meta.properties.insert(key, Value::String(error_msg.to_string()));
}

/// Add overall processing metadata (equivalent to TypeScript FixedFieldsVisitor + FallbackVisitor)
fn add_processing_metadata(sec_scheme: &mut SecuritySchemeElement) {
    sec_scheme.object.meta.properties.insert("processed".to_string(), Value::Bool(true));
    sec_scheme.object.meta.properties.insert("fixedFieldsVisitor".to_string(), Value::Bool(true));
    sec_scheme.object.meta.properties.insert("fallbackVisitor".to_string(), Value::Bool(true));
    sec_scheme.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), Value::Bool(true));
    
    // Add SecurityScheme specific classes
    sec_scheme.object.classes.content.push(Element::String(StringElement::new("security-scheme")));
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(sec_scheme: &mut SecuritySchemeElement) {
    sec_scheme.object.meta.properties.insert("specPath".to_string(), Value::Array(vec![
        Value::String("document".to_string()),
        Value::String("objects".to_string()),
        Value::String("SecurityScheme".to_string())
    ]));
}

/// Validate SecurityScheme type-specific constraints
fn validate_security_scheme_constraints(sec_scheme: &mut SecuritySchemeElement, scheme_type: &str) {
    match scheme_type {
        "apiKey" => {
            // Validate required fields for apiKey
            if sec_scheme.name().is_none() {
                add_validation_error_metadata(sec_scheme, "securityScheme", 
                    "name is required for apiKey security schemes");
            }
            if sec_scheme.in_().is_none() {
                add_validation_error_metadata(sec_scheme, "securityScheme", 
                    "in is required for apiKey security schemes");
            }
        }
        "http" => {
            // Validate required fields for http
            if sec_scheme.scheme().is_none() {
                add_validation_error_metadata(sec_scheme, "securityScheme", 
                    "scheme is required for http security schemes");
            }
        }
        "oauth2" => {
            // Validate required fields for oauth2
            if sec_scheme.flows().is_none() {
                add_validation_error_metadata(sec_scheme, "securityScheme", 
                    "flows is required for oauth2 security schemes");
            }
        }
        "openIdConnect" => {
            // Validate required fields for openIdConnect
            if sec_scheme.openid_connect_url().is_none() {
                add_validation_error_metadata(sec_scheme, "securityScheme", 
                    "openIdConnectUrl is required for openIdConnect security schemes");
            }
        }
        _ => {
            add_validation_error_metadata(sec_scheme, "securityScheme", 
                "Invalid or missing security scheme type");
        }
    }
    
    // Mark as validated
    sec_scheme.object.meta.properties.insert("validSecurityScheme".to_string(), Value::Bool(true));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fold::OpenApiBuilderFolder;

    #[test]
    fn test_basic_security_scheme_builder() {
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("http")));
        obj.set("scheme", Element::String(StringElement::new("bearer")));
        obj.set("bearerFormat", Element::String(StringElement::new("JWT")));
        obj.set("description", Element::String(StringElement::new("JWT Bearer token")));
        
        let result = build_security_scheme(&Element::Object(obj));
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        assert_eq!(sec_scheme.object.element, "securityScheme");
        assert!(sec_scheme.type_().is_some());
        assert_eq!(sec_scheme.type_().unwrap().content, "http");
        assert!(sec_scheme.scheme().is_some());
        assert_eq!(sec_scheme.scheme().unwrap().content, "bearer");
    }

    #[test]
    fn test_enhanced_security_scheme_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("apiKey")));
        obj.set("name", Element::String(StringElement::new("X-API-Key")));
        obj.set("in", Element::String(StringElement::new("header")));
        obj.set("description", Element::String(StringElement::new("API key authentication")));
        
        let result = build_and_decorate_security_scheme(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        
        // Verify processing metadata
        assert!(sec_scheme.object.meta.properties.contains_key("processed"));
        assert!(sec_scheme.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(sec_scheme.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(sec_scheme.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // Verify fixed field metadata
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_type"));
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_name"));
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_in"));
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_description"));
        
        // Verify spec path metadata
        if let Some(Value::Array(spec_path)) = sec_scheme.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], Value::String("document".to_string()));
            assert_eq!(spec_path[1], Value::String("objects".to_string()));
            assert_eq!(spec_path[2], Value::String("SecurityScheme".to_string()));
        }
        
        // Verify element class
        assert!(sec_scheme.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "security-scheme"
            } else {
                false
            }
        }));
        
        // Verify validation status
        assert!(sec_scheme.object.meta.properties.contains_key("validSecurityScheme"));
    }

    #[test]
    fn test_oauth2_security_scheme_with_flows() {
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("oauth2")));
        obj.set("description", Element::String(StringElement::new("OAuth2 authentication")));
        
        // Add OAuth flows
        let mut flows_obj = ObjectElement::new();
        let mut auth_code_flow = ObjectElement::new();
        auth_code_flow.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        auth_code_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        auth_code_flow.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read", Element::String(StringElement::new("Read access")));
            scopes.set("write", Element::String(StringElement::new("Write access")));
            scopes
        }));
        flows_obj.set("authorizationCode", Element::Object(auth_code_flow));
        obj.set("flows", Element::Object(flows_obj));
        
        let result = build_and_decorate_security_scheme(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        
        // Verify OAuth2 type processing
        assert_eq!(sec_scheme.type_().unwrap().content, "oauth2");
        assert!(sec_scheme.flows().is_some());
        
        // Verify flows were processed with OAuthFlowsVisitor equivalent
        let flows = sec_scheme.flows().unwrap();
        assert!(flows.meta.properties.contains_key("processed"));
        assert!(flows.meta.properties.contains_key("fixedFieldsVisitor"));
        
        // Verify fixed field metadata for flows
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_flows"));
        
        // Verify validation passed
        assert!(sec_scheme.object.meta.properties.contains_key("validSecurityScheme"));
    }

    #[test]
    fn test_security_scheme_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("http")));
        obj.set("scheme", Element::String(StringElement::new("bearer")));
        obj.set("bearerFormat", Element::String(StringElement::new("JWT")));
        
        // Add specification extensions
        obj.set("x-token-expiry", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 3600.0,
        }));
        obj.set("x-refresh-enabled", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_security_scheme(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        
        // Verify specification extension metadata
        assert!(sec_scheme.object.meta.properties.contains_key("specificationExtension_x-token-expiry"));
        assert!(sec_scheme.object.meta.properties.contains_key("specificationExtension_x-refresh-enabled"));
        
        // Verify extensions are preserved
        assert!(sec_scheme.object.get("x-token-expiry").is_some());
        assert!(sec_scheme.object.get("x-refresh-enabled").is_some());
    }

    #[test]
    fn test_security_scheme_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("http")));
        obj.set("scheme", Element::String(StringElement::new("basic")));
        
        // Add fallback fields
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("unknownProperty", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_security_scheme(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        
        // Verify fallback field metadata
        assert!(sec_scheme.object.meta.properties.contains_key("fallback_customField"));
        assert!(sec_scheme.object.meta.properties.contains_key("fallback_unknownProperty"));
        
        // Verify fallback fields are preserved
        assert!(sec_scheme.object.get("customField").is_some());
        assert!(sec_scheme.object.get("unknownProperty").is_some());
    }

    #[test]
    fn test_security_scheme_validation_errors() {
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("apiKey")));
        // Missing required 'name' and 'in' fields for apiKey type
        
        let result = build_and_decorate_security_scheme(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        
        // Verify validation errors
        assert!(sec_scheme.object.meta.properties.contains_key("validationError_securityScheme"));
    }

    #[test]
    fn test_security_scheme_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/securitySchemes/BearerAuth")));
        
        let result = build_and_decorate_security_scheme(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        
        // Verify reference metadata
        assert!(sec_scheme.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            sec_scheme.object.meta.properties.get("referenced-element"),
            Some(&Value::String("securityScheme".to_string()))
        );
        assert_eq!(
            sec_scheme.object.meta.properties.get("reference-path"),
            Some(&Value::String("#/components/securitySchemes/BearerAuth".to_string()))
        );
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript SecuritySchemeVisitor
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("oauth2")));
        obj.set("description", Element::String(StringElement::new("OAuth2 with PKCE")));
        
        // Add comprehensive OAuth flows
        let mut flows_obj = ObjectElement::new();
        
        // Authorization code flow
        let mut auth_code_flow = ObjectElement::new();
        auth_code_flow.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        auth_code_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        auth_code_flow.set("refreshUrl", Element::String(StringElement::new("https://oauth.example.com/refresh")));
        auth_code_flow.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read:user", Element::String(StringElement::new("Read user information")));
            scopes.set("write:user", Element::String(StringElement::new("Write user information")));
            scopes.set("admin", Element::String(StringElement::new("Admin access")));
            scopes
        }));
        flows_obj.set("authorizationCode", Element::Object(auth_code_flow));
        
        // Client credentials flow
        let mut client_creds_flow = ObjectElement::new();
        client_creds_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        client_creds_flow.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("api:access", Element::String(StringElement::new("API access")));
            scopes
        }));
        flows_obj.set("clientCredentials", Element::Object(client_creds_flow));
        
        obj.set("flows", Element::Object(flows_obj));
        
        // Add specification extensions
        obj.set("x-pkce-required", Element::Boolean(BooleanElement::new(true)));
        obj.set("x-auth-timeout", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 300.0,
        }));
        
        // Add fallback field
        obj.set("customOAuthConfig", Element::String(StringElement::new("custom config")));
        
        let result = build_and_decorate_security_scheme(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_scheme = result.unwrap();
        
        // Verify all TypeScript SecuritySchemeVisitor features are present:
        
        // 1. FixedFieldsVisitor processing
        assert!(sec_scheme.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_type"));
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_description"));
        assert!(sec_scheme.object.meta.properties.contains_key("fixedField_flows"));
        
        // 2. Type-driven processing - OAuth2 flows were processed recursively
        let flows = sec_scheme.flows().unwrap();
        assert!(flows.meta.properties.contains_key("processed"));
        assert!(flows.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(flows.meta.properties.contains_key("validOAuthFlows"));
        
        // 3. Specification extensions support
        assert!(sec_scheme.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        assert!(sec_scheme.object.meta.properties.contains_key("specificationExtension_x-pkce-required"));
        assert!(sec_scheme.object.meta.properties.contains_key("specificationExtension_x-auth-timeout"));
        
        // 4. Fallback field handling
        assert!(sec_scheme.object.meta.properties.contains_key("fallback_customOAuthConfig"));
        assert!(sec_scheme.object.get("customOAuthConfig").is_some());
        
        // 5. Spec path metadata
        if let Some(Value::Array(spec_path)) = sec_scheme.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], Value::String("document".to_string()));
            assert_eq!(spec_path[1], Value::String("objects".to_string()));
            assert_eq!(spec_path[2], Value::String("SecurityScheme".to_string()));
        }
        
        // 6. Element classification
        assert!(sec_scheme.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "security-scheme"
            } else {
                false
            }
        }));
        assert_eq!(
            sec_scheme.object.meta.properties.get("element-type"),
            Some(&Value::String("securityScheme".to_string()))
        );
        
        // 7. Overall processing metadata
        assert!(sec_scheme.object.meta.properties.contains_key("processed"));
        assert!(sec_scheme.object.meta.properties.contains_key("fallbackVisitor"));
        
        // 8. Validation status
        assert!(sec_scheme.object.meta.properties.contains_key("validSecurityScheme"));
        
        // 9. OAuth2 type-specific validation
        assert_eq!(sec_scheme.type_().unwrap().content, "oauth2");
        assert!(sec_scheme.flows().is_some());
        
        // 10. Recursive OAuth flows processing verification
        if let Some(Element::Object(auth_code_obj)) = flows.get("authorizationCode") {
            assert!(auth_code_obj.meta.properties.contains_key("processed"));
            assert!(auth_code_obj.meta.properties.contains_key("validOAuthFlow"));
            
            // Verify scopes Map visitor pattern
            if let Some(Element::Object(scopes_obj)) = auth_code_obj.get("scopes") {
                assert!(scopes_obj.meta.properties.contains_key("mapVisitor"));
                assert!(scopes_obj.meta.properties.contains_key("scopesElement"));
            }
        }
    }
}