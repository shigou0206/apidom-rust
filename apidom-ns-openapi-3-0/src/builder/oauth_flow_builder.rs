/*!
 * OpenAPI 3.0 OAuth Flow Element Builder
 * 
 * This module provides comprehensive OAuth Flow element building functionality
 * equivalent to the TypeScript OAuthFlowVisitor. It supports:
 * - Fixed fields support (authorizationUrl, tokenUrl, refreshUrl, scopes)
 * - Specification extensions support (x-* fields)
 * - Reference support ($ref handling)
 * - Fallback behavior for unknown fields
 * - Type conversion and validation
 * - Recursive folding support
 * - Scopes handling with proper Map visitor pattern
 * - URL format validation
 */

use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use crate::elements::oauth_flow::OAuthFlowElement;
use serde_json::Value;

/// Build a basic OAuthFlowElement from a generic Element
pub fn build_oauth_flow(element: &Element) -> Option<OAuthFlowElement> {
    let obj = element.as_object()?;
    let mut flow = OAuthFlowElement::new();

    for member in &obj.content {
        if let Element::String(key) = &*member.key {
            match key.content.as_str() {
                "authorizationUrl" => {
                    if let Element::String(val) = &*member.value {
                        flow.set_authorization_url(val.clone());
                    }
                }
                "tokenUrl" => {
                    if let Element::String(val) = &*member.value {
                        flow.set_token_url(val.clone());
                    }
                }
                "refreshUrl" => {
                    if let Element::String(val) = &*member.value {
                        flow.set_refresh_url(val.clone());
                    }
                }
                "scopes" => {
                    if let Element::Object(val) = &*member.value {
                        flow.set_scopes(val.clone());
                    }
                }
                _ => {
                    // 保留未知字段（可选）
                    flow.object.set(&key.content, (*member.value).clone());
                }
            }
        }
    }

    Some(flow)
}

/// Build and decorate OAuthFlowElement with enhanced visitor pattern features
/// 
/// This function provides equivalent functionality to the TypeScript OAuthFlowVisitor:
/// - Fixed fields processing with metadata injection
/// - Specification extensions support (x-* fields)
/// - Reference handling with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion and validation
/// - Recursive folding support
/// - Scopes handling with Map visitor pattern
/// - URL format validation
pub fn build_and_decorate_oauth_flow<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<OAuthFlowElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut flow = OAuthFlowElement::new();
    
    // Add processing metadata
    add_processing_metadata(&mut flow);
    add_spec_path_metadata(&mut flow);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            flow.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut flow, &ref_str.content);
            return Some(flow);
        }
    }
    
    // Process all object members
    for member in &obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            match key.as_str() {
                // Fixed fields
                "authorizationUrl" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        // Validate URL format
                        if validate_url_format(&string_elem.content) {
                            flow.set_authorization_url(string_elem);
                            add_fixed_field_metadata(&mut flow, "authorizationUrl");
                        } else {
                            add_validation_error_metadata(&mut flow, "authorizationUrl", "Invalid URL format");
                        }
                    } else {
                        add_validation_error_metadata(&mut flow, "authorizationUrl", "Expected string value");
                    }
                }
                "tokenUrl" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        // Validate URL format
                        if validate_url_format(&string_elem.content) {
                            flow.set_token_url(string_elem);
                            add_fixed_field_metadata(&mut flow, "tokenUrl");
                        } else {
                            add_validation_error_metadata(&mut flow, "tokenUrl", "Invalid URL format");
                        }
                    } else {
                        add_validation_error_metadata(&mut flow, "tokenUrl", "Expected string value");
                    }
                }
                "refreshUrl" => {
                    if let Some(string_elem) = convert_to_string_element(value) {
                        // Validate URL format
                        if validate_url_format(&string_elem.content) {
                            flow.set_refresh_url(string_elem);
                            add_fixed_field_metadata(&mut flow, "refreshUrl");
                        } else {
                            add_validation_error_metadata(&mut flow, "refreshUrl", "Invalid URL format");
                        }
                    } else {
                        add_validation_error_metadata(&mut flow, "refreshUrl", "Expected string value");
                    }
                }
                "scopes" => {
                    // Process scopes with Map visitor pattern (equivalent to ScopesVisitor)
                    let processed_scopes = if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    };
                    
                    if let Some(obj_elem) = processed_scopes.as_object() {
                        let mut scopes_obj = obj_elem.clone();
                        // Add scopes-specific metadata (equivalent to OAuthFlowScopesElement)
                        add_scopes_metadata(&mut scopes_obj);
                        flow.set_scopes(scopes_obj);
                        add_fixed_field_metadata(&mut flow, "scopes");
                    } else {
                        add_validation_error_metadata(&mut flow, "scopes", "Expected object value");
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
                        
                        flow.object.set(key, processed_value);
                        add_specification_extension_metadata(&mut flow, key);
                    } else {
                        // Fallback field
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        
                        flow.object.set(key, processed_value);
                        add_fallback_metadata(&mut flow, key);
                    }
                }
            }
        }
    }
    
    // Validate OAuth flow constraints
    validate_oauth_flow(&mut flow);
    
    Some(flow)
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
fn add_fixed_field_metadata(flow: &mut OAuthFlowElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    flow.object.meta.properties.insert(key, Value::Bool(true));
    flow.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(flow: &mut OAuthFlowElement, ref_path: &str) {
    flow.object.meta.properties.insert("referenced-element".to_string(), Value::String("oAuthFlow".to_string()));
    flow.object.meta.properties.insert("reference-path".to_string(), Value::String(ref_path.to_string()));
    flow.object.classes.content.push(Element::String(StringElement::new("reference-element")));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(flow: &mut OAuthFlowElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    flow.object.meta.properties.insert(key, Value::Bool(true));
    flow.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(flow: &mut OAuthFlowElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    flow.object.meta.properties.insert(key, Value::Bool(true));
    flow.object.classes.content.push(Element::String(StringElement::new("fallback-field")));
}

/// Add metadata for validation errors
fn add_validation_error_metadata(flow: &mut OAuthFlowElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    flow.object.meta.properties.insert(key, Value::String(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(flow: &mut OAuthFlowElement) {
    flow.object.meta.properties.insert("processed".to_string(), Value::Bool(true));
    flow.object.meta.properties.insert("fixedFieldsVisitor".to_string(), Value::Bool(true));
    flow.object.meta.properties.insert("fallbackVisitor".to_string(), Value::Bool(true));
    flow.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), Value::Bool(true));
}

/// Add spec path metadata
fn add_spec_path_metadata(flow: &mut OAuthFlowElement) {
    flow.object.meta.properties.insert("specPath".to_string(), Value::Array(vec![
        Value::String("document".to_string()),
        Value::String("objects".to_string()),
        Value::String("OAuthFlow".to_string())
    ]));
}

/// Add scopes-specific metadata (equivalent to OAuthFlowScopesElement)
fn add_scopes_metadata(scopes: &mut ObjectElement) {
    scopes.meta.properties.insert("mapVisitor".to_string(), Value::Bool(true));
    scopes.meta.properties.insert("scopesElement".to_string(), Value::Bool(true));
    scopes.meta.properties.insert("specPath".to_string(), Value::Array(vec![
        Value::String("value".to_string())
    ]));
    scopes.classes.content.push(Element::String(StringElement::new("oauth-flow-scopes")));
}

/// Validate URL format (basic validation)
fn validate_url_format(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Validate OAuth flow constraints
fn validate_oauth_flow(flow: &mut OAuthFlowElement) {
    let mut has_errors = false;
    
    // Check flow type requirements based on presence of certain fields
    let has_authorization_url = flow.authorization_url().is_some();
    let has_token_url = flow.token_url().is_some();
    let has_scopes = flow.scopes().is_some();
    
    // Basic validation: scopes should always be present
    if !has_scopes {
        add_validation_error_metadata(flow, "oAuthFlow", "Missing required field: scopes");
        has_errors = true;
    }
    
    // Validate flow type consistency
    if has_authorization_url && !has_token_url {
        // Authorization Code flow requires both authorizationUrl and tokenUrl
        add_validation_error_metadata(flow, "oAuthFlow", "Authorization Code flow requires tokenUrl");
        has_errors = true;
    }
    
    // If validation passes
    if !has_errors {
        flow.object.meta.properties.insert("validOAuthFlow".to_string(), Value::Bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::fold::DefaultFolder;

    #[test]
    fn test_basic_oauth_flow_builder() {
        let mut obj = ObjectElement::new();
        obj.set("authorizationUrl", Element::String(StringElement::new("https://example.com/auth")));
        obj.set("tokenUrl", Element::String(StringElement::new("https://example.com/token")));
        obj.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read", Element::String(StringElement::new("Read access")));
            scopes.set("write", Element::String(StringElement::new("Write access")));
            scopes
        }));

        let flow = build_oauth_flow(&Element::Object(obj));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        assert_eq!(flow.authorization_url().unwrap().content, "https://example.com/auth");
        assert_eq!(flow.token_url().unwrap().content, "https://example.com/token");
        assert!(flow.scopes().is_some());
    }

    #[test]
    fn test_enhanced_oauth_flow_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        obj.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        obj.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        obj.set("refreshUrl", Element::String(StringElement::new("https://oauth.example.com/refresh")));
        obj.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read:user", Element::String(StringElement::new("Read user information")));
            scopes.set("write:user", Element::String(StringElement::new("Write user information")));
            scopes
        }));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify basic fields
        assert_eq!(flow.authorization_url().unwrap().content, "https://oauth.example.com/authorize");
        assert_eq!(flow.token_url().unwrap().content, "https://oauth.example.com/token");
        assert_eq!(flow.refresh_url().unwrap().content, "https://oauth.example.com/refresh");
        assert!(flow.scopes().is_some());
        
        // Verify fixed field metadata
        assert!(flow.object.meta.properties.contains_key("fixedField_authorizationUrl"));
        assert!(flow.object.meta.properties.contains_key("fixedField_tokenUrl"));
        assert!(flow.object.meta.properties.contains_key("fixedField_refreshUrl"));
        assert!(flow.object.meta.properties.contains_key("fixedField_scopes"));
        
        // Verify processing metadata
        assert!(flow.object.meta.properties.contains_key("processed"));
        assert!(flow.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(flow.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // Verify spec path metadata
        if let Some(Value::Array(spec_path)) = flow.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], Value::String("document".to_string()));
            assert_eq!(spec_path[1], Value::String("objects".to_string()));
            assert_eq!(spec_path[2], Value::String("OAuthFlow".to_string()));
        }
        
        // Verify scopes metadata (equivalent to OAuthFlowScopesElement)
        let scopes = flow.scopes().unwrap();
        assert!(scopes.meta.properties.contains_key("mapVisitor"));
        assert!(scopes.meta.properties.contains_key("scopesElement"));
    }

    #[test]
    fn test_oauth_flow_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("tokenUrl", Element::String(StringElement::new("https://api.example.com/token")));
        obj.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("x-flow-id", Element::String(StringElement::new("client-credentials")));
        obj.set("x-rate-limit", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 1000.0,
        }));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify specification extensions are preserved
        assert!(flow.object.get("x-flow-id").is_some());
        assert!(flow.object.get("x-rate-limit").is_some());
        
        // Verify specification extension metadata
        assert!(flow.object.meta.properties.contains_key("specificationExtension_x-flow-id"));
        assert!(flow.object.meta.properties.contains_key("specificationExtension_x-rate-limit"));
        
        // Verify specification extension classes
        assert!(flow.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_oauth_flow_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/securitySchemes/oauth2/flows/authorizationCode")));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify reference is preserved
        if let Some(Element::String(ref_str)) = flow.object.get("$ref") {
            assert_eq!(ref_str.content, "#/components/securitySchemes/oauth2/flows/authorizationCode");
        }
        
        // Verify reference metadata
        assert!(flow.object.meta.properties.contains_key("referenced-element"));
        assert!(flow.object.meta.properties.contains_key("reference-path"));
        
        if let Some(Value::String(ref_elem)) = flow.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_elem, "oAuthFlow");
        }
    }

    #[test]
    fn test_oauth_flow_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        obj.set("tokenUrl", Element::String(StringElement::new("https://api.example.com/token")));
        obj.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("anotherField", Element::Boolean(BooleanElement::new(true)));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify fallback fields are preserved
        assert!(flow.object.get("customField").is_some());
        assert!(flow.object.get("anotherField").is_some());
        
        // Verify fallback metadata
        assert!(flow.object.meta.properties.contains_key("fallback_customField"));
        assert!(flow.object.meta.properties.contains_key("fallback_anotherField"));
        
        // Verify fallback classes
        assert!(flow.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "fallback-field"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_oauth_flow_url_validation() {
        let mut obj = ObjectElement::new();
        obj.set("authorizationUrl", Element::String(StringElement::new("invalid-url")));
        obj.set("tokenUrl", Element::String(StringElement::new("https://valid.example.com/token")));
        obj.set("scopes", Element::Object(ObjectElement::new()));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify URL validation error for invalid URL
        assert!(flow.object.meta.properties.contains_key("validationError_authorizationUrl"));
        
        // Verify valid URL was processed correctly
        assert!(flow.object.meta.properties.contains_key("fixedField_tokenUrl"));
    }

    #[test]
    fn test_oauth_flow_type_conversion() {
        let mut obj = ObjectElement::new();
        obj.set("tokenUrl", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 123.0,
        })); // Should convert to string but fail URL validation
        obj.set("scopes", Element::Object(ObjectElement::new()));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify type conversion happened but URL validation failed
        assert!(flow.object.meta.properties.contains_key("validationError_tokenUrl"));
    }

    #[test]
    fn test_oauth_flow_validation_errors() {
        let mut obj = ObjectElement::new();
        obj.set("authorizationUrl", Element::String(StringElement::new("https://example.com/auth")));
        // Missing tokenUrl for authorization code flow
        // Missing scopes

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify validation errors
        assert!(flow.object.meta.properties.contains_key("validationError_oAuthFlow"));
    }

    #[test]
    fn test_oauth_flow_scopes_map_visitor() {
        let mut obj = ObjectElement::new();
        obj.set("tokenUrl", Element::String(StringElement::new("https://api.example.com/token")));
        obj.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read:pets", Element::String(StringElement::new("Read pets")));
            scopes.set("write:pets", Element::String(StringElement::new("Write pets")));
            scopes.set("admin", Element::String(StringElement::new("Admin access")));
            scopes
        }));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify scopes are processed with Map visitor pattern
        let scopes = flow.scopes().unwrap();
        assert!(scopes.meta.properties.contains_key("mapVisitor"));
        assert!(scopes.meta.properties.contains_key("scopesElement"));
        
        // Verify scopes content
        assert!(scopes.get("read:pets").is_some());
        assert!(scopes.get("write:pets").is_some());
        assert!(scopes.get("admin").is_some());
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript OAuthFlowVisitor
        let mut obj = ObjectElement::new();
        obj.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        obj.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        obj.set("refreshUrl", Element::String(StringElement::new("https://oauth.example.com/refresh")));
        obj.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read", Element::String(StringElement::new("Read access")));
            scopes.set("write", Element::String(StringElement::new("Write access")));
            scopes
        }));
        
        // Add specification extensions
        obj.set("x-flow-type", Element::String(StringElement::new("authorization_code")));
        obj.set("x-pkce-required", Element::Boolean(BooleanElement::new(true)));
        
        // Add fallback field
        obj.set("customField", Element::String(StringElement::new("custom value")));

        let mut folder = DefaultFolder;
        let flow = build_and_decorate_oauth_flow(&Element::Object(obj), Some(&mut folder));
        assert!(flow.is_some());
        
        let flow = flow.unwrap();
        
        // Verify all TypeScript OAuthFlowVisitor features are present:
        
        // 1. Fixed fields processing
        assert!(flow.object.meta.properties.contains_key("fixedField_authorizationUrl"));
        assert!(flow.object.meta.properties.contains_key("fixedField_tokenUrl"));
        assert!(flow.object.meta.properties.contains_key("fixedField_refreshUrl"));
        assert!(flow.object.meta.properties.contains_key("fixedField_scopes"));
        
        // 2. Scopes Map visitor pattern (equivalent to ScopesVisitor)
        let scopes = flow.scopes().unwrap();
        assert!(scopes.meta.properties.contains_key("mapVisitor"));
        assert!(scopes.meta.properties.contains_key("scopesElement"));
        if let Some(Value::Array(scopes_spec_path)) = scopes.meta.properties.get("specPath") {
            assert_eq!(scopes_spec_path[0], Value::String("value".to_string()));
        }
        
        // 3. Specification extensions support
        assert!(flow.object.meta.properties.contains_key("specificationExtension_x-flow-type"));
        assert!(flow.object.meta.properties.contains_key("specificationExtension_x-pkce-required"));
        
        // 4. Fallback field handling
        assert!(flow.object.meta.properties.contains_key("fallback_customField"));
        
        // 5. Spec path metadata
        if let Some(Value::Array(spec_path)) = flow.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert_eq!(spec_path[0], Value::String("document".to_string()));
            assert_eq!(spec_path[1], Value::String("objects".to_string()));
            assert_eq!(spec_path[2], Value::String("OAuthFlow".to_string()));
        }
        
        // 6. Overall processing metadata
        assert!(flow.object.meta.properties.contains_key("processed"));
        assert!(flow.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(flow.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(flow.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // 7. Validation status
        assert!(flow.object.meta.properties.contains_key("validOAuthFlow"));
        
        // 8. URL validation
        assert_eq!(flow.authorization_url().unwrap().content, "https://oauth.example.com/authorize");
        assert_eq!(flow.token_url().unwrap().content, "https://oauth.example.com/token");
        assert_eq!(flow.refresh_url().unwrap().content, "https://oauth.example.com/refresh");
    }
}