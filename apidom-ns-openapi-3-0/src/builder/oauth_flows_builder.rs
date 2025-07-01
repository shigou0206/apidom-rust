 /*!
 * OpenAPI 3.0 OAuth Flows Element Builder
 * 
 * This module provides comprehensive OAuth Flows element building functionality
 * equivalent to the TypeScript OAuthFlowsVisitor. It supports:
 * - Fixed fields support (implicit, password, clientCredentials, authorizationCode)
 * - Specification extensions support (x-* fields)
 * - Reference support ($ref handling)
 * - Fallback behavior for unknown fields
 * - Type conversion and validation
 * - Recursive folding support
 * - Metadata injection with classes and spec path
 * - OAuth flow validation and constraints
 */

use apidom_ast::*;
use crate::elements::oauth_flows::OAuthFlowsElement;
use crate::builder::oauth_flow_builder::{build_oauth_flow, build_and_decorate_oauth_flow};
use serde_json::Value;

/// Build a basic OAuthFlowsElement from a generic Element
pub fn build_oauth_flows(element: &Element) -> Option<OAuthFlowsElement> {
    let obj = element.as_object()?;
    let mut flows = OAuthFlowsElement::new();

    for member in &obj.content {
        if let Element::String(key) = &*member.key {
            match key.content.as_str() {
                "implicit" | "password" | "clientCredentials" | "authorizationCode" => {
                    if let Some(flow) = build_oauth_flow(&*member.value) {
                        match key.content.as_str() {
                            "implicit" => flows.set_implicit(flow),
                            "password" => flows.set_password(flow),
                            "clientCredentials" => flows.set_client_credentials(flow),
                            "authorizationCode" => flows.set_authorization_code(flow),
                            _ => {}
                        }
                    }
                }
                _ => {
                    // Retain unknown fields
                    flows.object.set(&key.content, (*member.value).clone());
                }
            }
        }
    }

    Some(flows)
}

/// Build and decorate OAuthFlowsElement with enhanced visitor pattern features
/// 
/// This function provides equivalent functionality to the TypeScript OAuthFlowsVisitor:
/// - Fixed fields processing with metadata injection
/// - Specification extensions support (x-* fields)
/// - Reference handling with metadata
/// - Fallback behavior for unknown fields
/// - Type conversion and validation
/// - Recursive folding support
/// - OAuth flows validation and constraints
/// - Metadata injection with classes and spec path
pub fn build_and_decorate_oauth_flows<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<OAuthFlowsElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut flows = OAuthFlowsElement::new();
    
    // Add processing metadata
    add_processing_metadata(&mut flows);
    add_spec_path_metadata(&mut flows);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            flows.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut flows, &ref_str.content);
            return Some(flows);
        }
    }
    
    // Process all object members
    for member in &obj.content {
        if let Element::String(key_str) = member.key.as_ref() {
            let key = &key_str.content;
            let value = member.value.as_ref();
            
            match key.as_str() {
                // Fixed fields (OAuth flow types)
                "implicit" => {
                    if let Some(flow) = build_and_decorate_oauth_flow(value, folder.as_deref_mut()) {
                        flows.set_implicit(flow);
                        add_fixed_field_metadata(&mut flows, "implicit");
                    } else {
                        add_validation_error_metadata(&mut flows, "implicit", "Invalid OAuth flow structure");
                    }
                }
                "password" => {
                    if let Some(flow) = build_and_decorate_oauth_flow(value, folder.as_deref_mut()) {
                        flows.set_password(flow);
                        add_fixed_field_metadata(&mut flows, "password");
                    } else {
                        add_validation_error_metadata(&mut flows, "password", "Invalid OAuth flow structure");
                    }
                }
                "clientCredentials" => {
                    if let Some(flow) = build_and_decorate_oauth_flow(value, folder.as_deref_mut()) {
                        flows.set_client_credentials(flow);
                        add_fixed_field_metadata(&mut flows, "clientCredentials");
                    } else {
                        add_validation_error_metadata(&mut flows, "clientCredentials", "Invalid OAuth flow structure");
                    }
                }
                "authorizationCode" => {
                    if let Some(flow) = build_and_decorate_oauth_flow(value, folder.as_deref_mut()) {
                        flows.set_authorization_code(flow);
                        add_fixed_field_metadata(&mut flows, "authorizationCode");
                    } else {
                        add_validation_error_metadata(&mut flows, "authorizationCode", "Invalid OAuth flow structure");
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
                        
                        flows.object.set(key, processed_value);
                        add_specification_extension_metadata(&mut flows, key);
                    } else {
                        // Fallback field
                        let processed_value = if let Some(ref mut f) = folder {
                            f.fold_element(value.clone())
                        } else {
                            value.clone()
                        };
                        
                        flows.object.set(key, processed_value);
                        add_fallback_metadata(&mut flows, key);
                    }
                }
            }
        }
    }
    
    // Validate OAuth flows constraints
    validate_oauth_flows(&mut flows);
    
    Some(flows)
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(flows: &mut OAuthFlowsElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    flows.object.meta.properties.insert(key, SimpleValue::Bool(true));
    flows.object.classes.content.push(Element::String(StringElement::new("fixed-field")));
}

/// Add metadata for references
fn add_ref_metadata(flows: &mut OAuthFlowsElement, ref_path: &str) {
    flows.object.meta.properties.insert("referenced-element".to_string(), SimpleValue::String("oAuthFlows".to_string()));
    flows.object.meta.properties.insert("reference-path".to_string(), SimpleValue::String(ref_path.to_string()));
    flows.object.classes.content.push(Element::String(StringElement::new("reference-element")));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(flows: &mut OAuthFlowsElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    flows.object.meta.properties.insert(key, SimpleValue::Bool(true));
    flows.object.classes.content.push(Element::String(StringElement::new("specification-extension")));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(flows: &mut OAuthFlowsElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    flows.object.meta.properties.insert(key, SimpleValue::Bool(true));
    flows.object.classes.content.push(Element::String(StringElement::new("fallback-field")));
}

/// Add metadata for validation errors
fn add_validation_error_metadata(flows: &mut OAuthFlowsElement, field_name: &str, error_msg: &str) {
    let key = format!("validationError_{}", field_name);
    flows.object.meta.properties.insert(key, SimpleValue::String(error_msg.to_string()));
}

/// Add overall processing metadata
fn add_processing_metadata(flows: &mut OAuthFlowsElement) {
    flows.object.meta.properties.insert("processed".to_string(), SimpleValue::Bool(true));
    flows.object.meta.properties.insert("fixedFieldsVisitor".to_string(), SimpleValue::Bool(true));
    flows.object.meta.properties.insert("fallbackVisitor".to_string(), SimpleValue::Bool(true));
    flows.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), SimpleValue::Bool(true));
    
    // Add OAuth flows specific classes
    flows.object.classes.content.push(Element::String(StringElement::new("oauth-flows")));
}

/// Add spec path metadata
fn add_spec_path_metadata(flows: &mut OAuthFlowsElement) {
    flows.object.meta.properties.insert("specPath".to_string(), SimpleValue::Array(vec![
        SimpleValue::String("document".to_string()),
        SimpleValue::String("objects".to_string()),
        SimpleValue::String("OAuthFlows".to_string())
    ]));
}

/// Validate OAuth flows constraints
fn validate_oauth_flows(flows: &mut OAuthFlowsElement) {
    let mut has_errors = false;
    let mut flow_count = 0;
    
    // Check that at least one flow type is defined
    if flows.implicit().is_some() {
        flow_count += 1;
    }
    if flows.password().is_some() {
        flow_count += 1;
    }
    if flows.client_credentials().is_some() {
        flow_count += 1;
    }
    if flows.authorization_code().is_some() {
        flow_count += 1;
    }
    
    if flow_count == 0 {
        add_validation_error_metadata(flows, "oAuthFlows", "At least one OAuth flow must be defined");
        has_errors = true;
    }
    
    // Validate individual flows
    if let Some(implicit_flow) = flows.implicit() {
        if implicit_flow.authorization_url().is_none() {
            add_validation_error_metadata(flows, "implicit", "Implicit flow requires authorizationUrl");
            has_errors = true;
        }
    }
    
    if let Some(password_flow) = flows.password() {
        if password_flow.token_url().is_none() {
            add_validation_error_metadata(flows, "password", "Password flow requires tokenUrl");
            has_errors = true;
        }
    }
    
    if let Some(client_creds_flow) = flows.client_credentials() {
        if client_creds_flow.token_url().is_none() {
            add_validation_error_metadata(flows, "clientCredentials", "Client credentials flow requires tokenUrl");
            has_errors = true;
        }
    }
    
    if let Some(auth_code_flow) = flows.authorization_code() {
        if auth_code_flow.authorization_url().is_none() || auth_code_flow.token_url().is_none() {
            add_validation_error_metadata(flows, "authorizationCode", "Authorization code flow requires both authorizationUrl and tokenUrl");
            has_errors = true;
        }
    }
    
    // If validation passes
    if !has_errors {
        flows.object.meta.properties.insert("validOAuthFlows".to_string(), SimpleValue::Bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_oauth_flows_builder() {
        let mut obj = ObjectElement::new();
        
        // Add implicit flow
        let mut implicit_flow = ObjectElement::new();
        implicit_flow.set("authorizationUrl", Element::String(StringElement::new("https://example.com/auth")));
        implicit_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("implicit", Element::Object(implicit_flow));
        
        // Add password flow
        let mut password_flow = ObjectElement::new();
        password_flow.set("tokenUrl", Element::String(StringElement::new("https://example.com/token")));
        password_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("password", Element::Object(password_flow));

        let flows = build_oauth_flows(&Element::Object(obj));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        assert!(flows.implicit().is_some());
        assert!(flows.password().is_some());
        assert!(flows.client_credentials().is_none());
        assert!(flows.authorization_code().is_none());
    }

    #[test]
    fn test_enhanced_oauth_flows_with_fixed_fields() {
        let mut obj = ObjectElement::new();
        
        // Add all four flow types
        let mut implicit_flow = ObjectElement::new();
        implicit_flow.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        implicit_flow.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read", Element::String(StringElement::new("Read access")));
            scopes
        }));
        obj.set("implicit", Element::Object(implicit_flow));
        
        let mut password_flow = ObjectElement::new();
        password_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        password_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("password", Element::Object(password_flow));
        
        let mut client_creds_flow = ObjectElement::new();
        client_creds_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        client_creds_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("clientCredentials", Element::Object(client_creds_flow));
        
        let mut auth_code_flow = ObjectElement::new();
        auth_code_flow.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        auth_code_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        auth_code_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("authorizationCode", Element::Object(auth_code_flow));

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify all flows are present
        assert!(flows.implicit().is_some());
        assert!(flows.password().is_some());
        assert!(flows.client_credentials().is_some());
        assert!(flows.authorization_code().is_some());
        
        // Verify fixed field metadata
        assert!(flows.object.meta.properties.contains_key("fixedField_implicit"));
        assert!(flows.object.meta.properties.contains_key("fixedField_password"));
        assert!(flows.object.meta.properties.contains_key("fixedField_clientCredentials"));
        assert!(flows.object.meta.properties.contains_key("fixedField_authorizationCode"));
        
        // Verify processing metadata
        assert!(flows.object.meta.properties.contains_key("processed"));
        assert!(flows.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(flows.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // Verify spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = flows.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "OAuthFlows"));
        }
        
        // Verify OAuth flows classes
        assert!(flows.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "oauth-flows"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_oauth_flows_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        
        // Add a basic flow
        let mut implicit_flow = ObjectElement::new();
        implicit_flow.set("authorizationUrl", Element::String(StringElement::new("https://example.com/auth")));
        implicit_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("implicit", Element::Object(implicit_flow));
        
        // Add specification extensions
        obj.set("x-flows-version", Element::String(StringElement::new("2.0")));
        obj.set("x-custom-config", Element::Object({
            let mut config = ObjectElement::new();
            config.set("timeout", Element::Number(NumberElement {
                element: "number".to_string(),
                meta: MetaElement::default(),
                attributes: AttributesElement::default(),
                content: 30.0,
            }));
            config
        }));

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify specification extensions are preserved
        assert!(flows.object.get("x-flows-version").is_some());
        assert!(flows.object.get("x-custom-config").is_some());
        
        // Verify specification extension metadata
        assert!(flows.object.meta.properties.contains_key("specificationExtension_x-flows-version"));
        assert!(flows.object.meta.properties.contains_key("specificationExtension_x-custom-config"));
        
        // Verify specification extension classes
        assert!(flows.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "specification-extension"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_oauth_flows_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/securitySchemes/oauth2/flows")));

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify reference is preserved
        if let Some(Element::String(ref_str)) = flows.object.get("$ref") {
            assert_eq!(ref_str.content, "#/components/securitySchemes/oauth2/flows");
        }
        
        // Verify reference metadata
        assert!(flows.object.meta.properties.contains_key("referenced-element"));
        assert!(flows.object.meta.properties.contains_key("reference-path"));
        
        if let Some(SimpleValue::String(ref_elem)) = flows.object.meta.properties.get("referenced-element") {
            assert_eq!(ref_elem, "oAuthFlows");
        }
    }

    #[test]
    fn test_oauth_flows_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        
        // Add a basic flow
        let mut password_flow = ObjectElement::new();
        password_flow.set("tokenUrl", Element::String(StringElement::new("https://api.example.com/token")));
        password_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("password", Element::Object(password_flow));
        
        // Add fallback fields
        obj.set("customFlowType", Element::String(StringElement::new("custom")));
        obj.set("additionalConfig", Element::Object({
            let mut config = ObjectElement::new();
            config.set("enabled", Element::Boolean(BooleanElement::new(true)));
            config
        }));

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify fallback fields are preserved
        assert!(flows.object.get("customFlowType").is_some());
        assert!(flows.object.get("additionalConfig").is_some());
        
        // Verify fallback metadata
        assert!(flows.object.meta.properties.contains_key("fallback_customFlowType"));
        assert!(flows.object.meta.properties.contains_key("fallback_additionalConfig"));
        
        // Verify fallback classes
        assert!(flows.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "fallback-field"
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_oauth_flows_validation_errors() {
        let obj = ObjectElement::new();
        // Empty object - should trigger validation error

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify validation error for missing flows
        assert!(flows.object.meta.properties.contains_key("validationError_oAuthFlows"));
    }

    #[test]
    fn test_oauth_flows_individual_flow_validation() {
        let mut obj = ObjectElement::new();
        
        // Add implicit flow without required authorizationUrl
        let mut implicit_flow = ObjectElement::new();
        implicit_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("implicit", Element::Object(implicit_flow));
        
        // Add password flow without required tokenUrl
        let mut password_flow = ObjectElement::new();
        password_flow.set("scopes", Element::Object(ObjectElement::new()));
        obj.set("password", Element::Object(password_flow));

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify individual flow validation errors
        assert!(flows.object.meta.properties.contains_key("validationError_implicit"));
        assert!(flows.object.meta.properties.contains_key("validationError_password"));
    }

    #[test]
    fn test_oauth_flows_recursive_folding() {
        let mut obj = ObjectElement::new();
        
        // Add authorization code flow with nested structure
        let mut auth_code_flow = ObjectElement::new();
        auth_code_flow.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        auth_code_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        auth_code_flow.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read:user", Element::String(StringElement::new("Read user information")));
            scopes.set("write:user", Element::String(StringElement::new("Write user information")));
            scopes
        }));
        
        // Add specification extension to the flow
        auth_code_flow.set("x-flow-config", Element::Object({
            let mut config = ObjectElement::new();
            config.set("pkce", Element::Boolean(BooleanElement::new(true)));
            config
        }));
        
        obj.set("authorizationCode", Element::Object(auth_code_flow));

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify recursive folding worked
        let auth_code = flows.authorization_code().unwrap();
        assert!(auth_code.authorization_url().is_some());
        assert!(auth_code.token_url().is_some());
        assert!(auth_code.scopes().is_some());
        
        // Verify the nested flow has proper metadata (from recursive folding)
        assert!(auth_code.object.meta.properties.contains_key("processed"));
        assert!(auth_code.object.meta.properties.contains_key("fixedFieldsVisitor"));
        
        // Verify scopes have Map visitor metadata
        let scopes = auth_code.scopes().unwrap();
        assert!(scopes.meta.properties.contains_key("mapVisitor"));
        assert!(scopes.meta.properties.contains_key("scopesElement"));
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript OAuthFlowsVisitor
        let mut obj = ObjectElement::new();
        
        // Add all OAuth flow types with comprehensive configuration
        let mut implicit_flow = ObjectElement::new();
        implicit_flow.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        implicit_flow.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("read", Element::String(StringElement::new("Read access")));
            scopes.set("write", Element::String(StringElement::new("Write access")));
            scopes
        }));
        obj.set("implicit", Element::Object(implicit_flow));
        
        let mut auth_code_flow = ObjectElement::new();
        auth_code_flow.set("authorizationUrl", Element::String(StringElement::new("https://oauth.example.com/authorize")));
        auth_code_flow.set("tokenUrl", Element::String(StringElement::new("https://oauth.example.com/token")));
        auth_code_flow.set("refreshUrl", Element::String(StringElement::new("https://oauth.example.com/refresh")));
        auth_code_flow.set("scopes", Element::Object({
            let mut scopes = ObjectElement::new();
            scopes.set("admin", Element::String(StringElement::new("Admin access")));
            scopes
        }));
        obj.set("authorizationCode", Element::Object(auth_code_flow));
        
        // Add specification extensions
        obj.set("x-flows-version", Element::String(StringElement::new("2.1")));
        obj.set("x-provider", Element::String(StringElement::new("custom-oauth")));
        
        // Add fallback field
        obj.set("customField", Element::String(StringElement::new("custom value")));

        let mut folder = DefaultFolder;
        let flows = build_and_decorate_oauth_flows(&Element::Object(obj), Some(&mut folder));
        assert!(flows.is_some());
        
        let flows = flows.unwrap();
        
        // Verify all TypeScript OAuthFlowsVisitor features are present:
        
        // 1. Fixed fields processing
        assert!(flows.object.meta.properties.contains_key("fixedField_implicit"));
        assert!(flows.object.meta.properties.contains_key("fixedField_authorizationCode"));
        
        // 2. Recursive folding - individual flows have their own metadata
        let implicit = flows.implicit().unwrap();
        assert!(implicit.object.meta.properties.contains_key("processed"));
        assert!(implicit.object.meta.properties.contains_key("fixedFieldsVisitor"));
        
        let auth_code = flows.authorization_code().unwrap();
        assert!(auth_code.object.meta.properties.contains_key("processed"));
        assert!(auth_code.object.meta.properties.contains_key("validOAuthFlow"));
        
        // 3. Scopes Map visitor pattern (equivalent to OAuthFlowScopesElement)
        let implicit_scopes = implicit.scopes().unwrap();
        assert!(implicit_scopes.meta.properties.contains_key("mapVisitor"));
        assert!(implicit_scopes.meta.properties.contains_key("scopesElement"));
        
        // 4. Specification extensions support
        assert!(flows.object.meta.properties.contains_key("specificationExtension_x-flows-version"));
        assert!(flows.object.meta.properties.contains_key("specificationExtension_x-provider"));
        
        // 5. Fallback field handling
        assert!(flows.object.meta.properties.contains_key("fallback_customField"));
        
        // 6. Spec path metadata
        if let Some(SimpleValue::Array(spec_path)) = flows.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 3);
            assert!(matches!(&spec_path[0], SimpleValue::String(s) if s == "document"));
            assert!(matches!(&spec_path[1], SimpleValue::String(s) if s == "objects"));
            assert!(matches!(&spec_path[2], SimpleValue::String(s) if s == "OAuthFlows"));
        }
        
        // 7. Overall processing metadata
        assert!(flows.object.meta.properties.contains_key("processed"));
        assert!(flows.object.meta.properties.contains_key("fixedFieldsVisitor"));
        assert!(flows.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(flows.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        
        // 8. OAuth flows specific classes
        assert!(flows.object.classes.content.iter().any(|c| {
            if let Element::String(s) = c {
                s.content == "oauth-flows"
            } else {
                false
            }
        }));
        
        // 9. Validation status
        assert!(flows.object.meta.properties.contains_key("validOAuthFlows"));
        
        // 10. Individual flow validation and structure
        assert_eq!(implicit.authorization_url().unwrap().content, "https://oauth.example.com/authorize");
        assert_eq!(auth_code.authorization_url().unwrap().content, "https://oauth.example.com/authorize");
        assert_eq!(auth_code.token_url().unwrap().content, "https://oauth.example.com/token");
        assert_eq!(auth_code.refresh_url().unwrap().content, "https://oauth.example.com/refresh");
    }
}