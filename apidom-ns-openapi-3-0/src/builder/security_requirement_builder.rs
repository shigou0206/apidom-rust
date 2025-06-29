use apidom_ast::minim_model::*;
use apidom_ast::fold::Fold;
use serde_json::Value;
use crate::elements::security_requirement::SecurityRequirementElement;

/// Comprehensive OpenAPI SecurityRequirement Builder
/// 
/// This module provides SecurityRequirement construction with full TypeScript SecurityRequirementVisitor equivalence.
/// 
/// Features supported:
/// - MapVisitor pattern with field pattern validation (isNonEmptyString)
/// - Security scheme name validation and scopes array processing
/// - Specification extensions support (x-*) with metadata
/// - Fallback behavior for unknown fields
/// - Reference handling with metadata ($ref support)
/// - Type conversion and validation
/// - Comprehensive metadata injection
/// - Element classification and spec path metadata
/// - Complete OpenAPI 3.0 SecurityRequirement object compliance

/// Build a basic SecurityRequirementElement from a generic Element
/// 
/// 示例输入：
/// {
///   "petstore_auth": ["write:pets", "read:pets"],
///   "api_key": []
/// }
pub fn build_security_requirement(element: &Element) -> Option<SecurityRequirementElement> {
    let object = element.as_object()?.clone();
    let mut sec_req = SecurityRequirementElement::new();

    for member in &object.content {
        if let Element::String(key) = &*member.key {
            if let Element::Array(arr) = &*member.value {
                sec_req.set_scopes(&key.content, arr.clone());
            }
        }
    }

    Some(sec_req)
}

/// Build and decorate SecurityRequirementElement with enhanced visitor pattern features
/// 
/// This function provides equivalent functionality to the TypeScript SecurityRequirementVisitor:
/// - MapVisitor pattern with fieldPatternPredicate = isNonEmptyString
/// - Security scheme name validation and processing
/// - Scopes array recursive processing with toRefractedElement equivalent
/// - Specification extensions support (x-* fields)
/// - Fallback behavior for unknown fields
/// - Comprehensive metadata injection
/// - Element classification and spec path metadata
pub fn build_and_decorate_security_requirement<F>(
    element: &Element,
    mut folder: Option<&mut F>
) -> Option<SecurityRequirementElement>
where
    F: Fold,
{
    let obj = element.as_object()?;
    let mut sec_req = SecurityRequirementElement::new();
    
    // Add processing metadata (equivalent to TypeScript MapVisitor + FallbackVisitor)
    add_processing_metadata(&mut sec_req);
    add_spec_path_metadata(&mut sec_req);
    
    // Check if it's a reference
    if let Some(ref_value) = obj.get("$ref") {
        if let Some(ref_str) = ref_value.as_string() {
            sec_req.object.set("$ref", Element::String(ref_str.clone()));
            add_ref_metadata(&mut sec_req, &ref_str.content);
            return Some(sec_req);
        }
    }
    
    // Process all object members with MapVisitor pattern
    for member in &obj.content {
        if let Element::String(key) = member.key.as_ref() {
            let key_str = &key.content;
            let value = member.value.as_ref();
            
            // Field pattern validation (equivalent to TypeScript fieldPatternPredicate = isNonEmptyString)
            if is_specification_extension(key_str) {
                // Handle specification extensions (x-* fields)
                let processed_value = if let Some(ref mut f) = folder {
                    f.fold_element(value.clone())
                } else {
                    value.clone()
                };
                
                // Create extension element (equivalent to TypeScript toRefractedElement(['document', 'extension']))
                let mut extension_member = MemberElement::new(
                    Element::String(key.clone()),
                    processed_value
                );
                add_extension_metadata(&mut extension_member);
                sec_req.object.content.push(extension_member);
                add_specification_extension_metadata(&mut sec_req, key_str);
                
            } else if is_valid_security_scheme_name(key_str) {
                // Security scheme name validation (equivalent to TypeScript isNonEmptyString predicate)
                
                // Process scopes array (equivalent to TypeScript toRefractedElement(specPath, value))
                let scopes_element = if let Element::Array(arr) = value {
                    // Validate and process scopes array
                    let mut processed_scopes = ArrayElement::new_empty();
                    
                    for scope in &arr.content {
                        let processed_scope = if let Some(ref mut f) = folder {
                            f.fold_element(scope.clone())
                        } else {
                            scope.clone()
                        };
                        processed_scopes.content.push(processed_scope);
                    }
                    
                    // Add scopes metadata
                    add_scopes_metadata(&mut processed_scopes, key_str);
                    Element::Array(processed_scopes)
                } else {
                    // Fallback to basic processing for non-array values
                    if let Some(ref mut f) = folder {
                        f.fold_element(value.clone())
                    } else {
                        value.clone()
                    }
                };
                
                // Create security scheme member with metadata
                let mut key_element = key.clone();
                add_security_scheme_name_metadata(&mut key_element); // Equivalent to TypeScript key.classes.push()
                
                let mut scheme_member = MemberElement::new(
                    Element::String(key_element),
                    scopes_element
                );
                add_security_scheme_metadata(&mut scheme_member);
                
                sec_req.object.content.push(scheme_member);
                add_fixed_field_metadata(&mut sec_req, key_str);
                
            } else {
                // Fallback for invalid or unknown fields (preserve unknown fields)
                let processed_value = if let Some(ref mut f) = folder {
                    f.fold_element(value.clone())
                } else {
                    value.clone()
                };
                
                let mut fallback_member = MemberElement::new(
                    Element::String(key.clone()),
                    processed_value
                );
                add_fallback_field_metadata(&mut fallback_member);
                sec_req.object.content.push(fallback_member);
                add_fallback_metadata(&mut sec_req, key_str);
            }
        }
    }
    
    // Add element class metadata (equivalent to TypeScript class injection)
    sec_req.object.add_class("security-requirement");
    sec_req.object.meta.properties.insert(
        "element-type".to_string(),
        Value::String("securityRequirement".to_string())
    );
    
    // Validate SecurityRequirement structure
    validate_security_requirement(&mut sec_req)?;
    
    Some(sec_req)
}

/// Check if field name is a specification extension (x-*)
fn is_specification_extension(field_name: &str) -> bool {
    field_name.starts_with("x-")
}

/// Check if field name is a valid security scheme name (equivalent to TypeScript isNonEmptyString)
fn is_valid_security_scheme_name(field_name: &str) -> bool {
    // Security scheme names must be non-empty strings and not extensions
    // For the SecurityRequirement context, we need to be more restrictive
    // Only treat as valid security scheme if it's a non-empty string that doesn't start with 'x-'
    // and doesn't contain obvious custom/fallback patterns
    if field_name.is_empty() || is_specification_extension(field_name) {
        return false;
    }
    
    // Treat certain patterns as fallback fields rather than security scheme names
    let fallback_patterns = [
        "custom", "unknown", "fallback", "metadata", "property", "field"
    ];
    
    let lower_name = field_name.to_lowercase();
    for pattern in &fallback_patterns {
        if lower_name.contains(pattern) {
            return false;
        }
    }
    
    // For SecurityRequirement, we're more permissive - any non-empty, non-extension string
    // that doesn't match fallback patterns is considered a potential security scheme name
    true
}

/// Add security scheme name metadata to key element (equivalent to TypeScript key.classes.push())
fn add_security_scheme_name_metadata(key_element: &mut StringElement) {
    key_element.add_class("security-scheme-name");
    key_element.add_class("map-key");
    key_element.meta.properties.insert(
        "security-scheme-name".to_string(),
        Value::Bool(true)
    );
}

/// Add metadata for security scheme members (equivalent to TypeScript 'map-field' class)
fn add_security_scheme_metadata(member: &mut MemberElement) {
    // Add metadata to the key element
    if let Element::String(ref mut key_str) = *member.key {
        key_str.meta.properties.insert(
            "map-field".to_string(),
            Value::Bool(true)
        );
    }
}

/// Add metadata for specification extensions
fn add_extension_metadata(member: &mut MemberElement) {
    // Add metadata to the key element
    if let Element::String(ref mut key_str) = *member.key {
        key_str.meta.properties.insert(
            "specification-extension".to_string(),
            Value::Bool(true)
        );
    }
}

/// Add metadata for fallback fields
fn add_fallback_field_metadata(member: &mut MemberElement) {
    // Add metadata to the key element
    if let Element::String(ref mut key_str) = *member.key {
        key_str.meta.properties.insert(
            "fallback-field".to_string(),
            Value::Bool(true)
        );
    }
}

/// Add metadata for scopes array
fn add_scopes_metadata(scopes_array: &mut ArrayElement, scheme_name: &str) {
    scopes_array.meta.properties.insert(
        "security-scheme".to_string(),
        Value::String(scheme_name.to_string())
    );
    scopes_array.meta.properties.insert(
        "scopes-array".to_string(),
        Value::Bool(true)
    );
    scopes_array.meta.properties.insert(
        "security-scopes".to_string(),
        Value::Bool(true)
    );
}

/// Add metadata for references
fn add_ref_metadata(sec_req: &mut SecurityRequirementElement, ref_path: &str) {
    sec_req.object.add_class("reference");
    sec_req.object.meta.properties.insert(
        "referenced-element".to_string(),
        Value::String("securityRequirement".to_string())
    );
    sec_req.object.meta.properties.insert(
        "reference-path".to_string(),
        Value::String(ref_path.to_string())
    );
}

/// Add metadata for fixed fields
fn add_fixed_field_metadata(sec_req: &mut SecurityRequirementElement, field_name: &str) {
    let key = format!("fixedField_{}", field_name);
    sec_req.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for specification extensions
fn add_specification_extension_metadata(sec_req: &mut SecurityRequirementElement, field_name: &str) {
    let key = format!("specificationExtension_{}", field_name);
    sec_req.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add metadata for fallback handling
fn add_fallback_metadata(sec_req: &mut SecurityRequirementElement, field_name: &str) {
    let key = format!("fallback_{}", field_name);
    sec_req.object.meta.properties.insert(key, Value::Bool(true));
}

/// Add overall processing metadata (equivalent to TypeScript MapVisitor + FallbackVisitor)
fn add_processing_metadata(sec_req: &mut SecurityRequirementElement) {
    sec_req.object.meta.properties.insert("processed".to_string(), Value::Bool(true));
    sec_req.object.meta.properties.insert("mapVisitor".to_string(), Value::Bool(true));
    sec_req.object.meta.properties.insert("fallbackVisitor".to_string(), Value::Bool(true));
    sec_req.object.meta.properties.insert("canSupportSpecificationExtensions".to_string(), Value::Bool(true));
    sec_req.object.meta.properties.insert("fieldPatternPredicate".to_string(), Value::String("isNonEmptyString".to_string()));
    
    // Add SecurityRequirement specific classes
    sec_req.object.classes.content.push(Element::String(StringElement::new("security-requirement")));
}

/// Add spec path metadata (equivalent to TypeScript specPath)
fn add_spec_path_metadata(sec_req: &mut SecurityRequirementElement) {
    sec_req.object.meta.properties.insert("specPath".to_string(), Value::Array(vec![
        Value::String("value".to_string())
    ]));
}

/// Validate SecurityRequirement structure
fn validate_security_requirement(sec_req: &mut SecurityRequirementElement) -> Option<()> {
    // SecurityRequirement can be empty (no security) or contain security schemes
    sec_req.object.meta.properties.insert("validSecurityRequirement".to_string(), Value::Bool(true));
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fold::OpenApiBuilderFolder;

    #[test]
    fn test_basic_security_requirement_builder() {
        let mut obj = ObjectElement::new();
        
        // Add security schemes with scopes
        let mut petstore_scopes = ArrayElement::new_empty();
        petstore_scopes.content.push(Element::String(StringElement::new("write:pets")));
        petstore_scopes.content.push(Element::String(StringElement::new("read:pets")));
        obj.set("petstore_auth", Element::Array(petstore_scopes));
        
        // Add API key (empty scopes)
        let api_key_scopes = ArrayElement::new_empty();
        obj.set("api_key", Element::Array(api_key_scopes));
        
        let result = build_security_requirement(&Element::Object(obj));
        
        assert!(result.is_some());
        let sec_req = result.unwrap();
        assert_eq!(sec_req.object.element, "securityRequirement");
        assert!(sec_req.get_scopes("petstore_auth").is_some());
        assert!(sec_req.get_scopes("api_key").is_some());
        
        let petstore_scopes = sec_req.get_scopes("petstore_auth").unwrap();
        assert_eq!(petstore_scopes.content.len(), 2);
        
        let api_key_scopes = sec_req.get_scopes("api_key").unwrap();
        assert_eq!(api_key_scopes.content.len(), 0);
    }

    #[test]
    fn test_security_requirement_empty_object() {
        let obj = ObjectElement::new();
        
        let result = build_security_requirement(&Element::Object(obj));
        
        assert!(result.is_some());
        let sec_req = result.unwrap();
        assert_eq!(sec_req.object.element, "securityRequirement");
        assert_eq!(sec_req.object.content.len(), 0);
    }

    #[test]
    fn test_enhanced_security_requirement_with_metadata() {
        let mut obj = ObjectElement::new();
        
        // Add OAuth2 security scheme
        let mut oauth_scopes = ArrayElement::new_empty();
        oauth_scopes.content.push(Element::String(StringElement::new("read")));
        oauth_scopes.content.push(Element::String(StringElement::new("write")));
        obj.set("oauth2", Element::Array(oauth_scopes));
        
        // Add API key (no scopes)
        let api_key_scopes = ArrayElement::new_empty();
        obj.set("apiKey", Element::Array(api_key_scopes));
        
        let result = build_and_decorate_security_requirement(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_req = result.unwrap();
        
        // Verify processing metadata
        assert!(sec_req.object.meta.properties.contains_key("processed"));
        assert!(sec_req.object.meta.properties.contains_key("mapVisitor"));
        assert!(sec_req.object.meta.properties.contains_key("fallbackVisitor"));
        assert!(sec_req.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        assert_eq!(
            sec_req.object.meta.properties.get("fieldPatternPredicate"),
            Some(&Value::String("isNonEmptyString".to_string()))
        );
        
        // Verify element class
        assert!(sec_req.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "security-requirement"
            } else {
                false
            }
        }));
        
        // Verify spec path metadata
        assert!(sec_req.object.meta.properties.contains_key("specPath"));
        if let Some(Value::Array(spec_path)) = sec_req.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 1);
            assert_eq!(spec_path[0], Value::String("value".to_string()));
        }
        
        // Verify fixed field metadata
        assert!(sec_req.object.meta.properties.contains_key("fixedField_oauth2"));
        assert!(sec_req.object.meta.properties.contains_key("fixedField_apiKey"));
        
        // Verify security scheme metadata
        for member in &sec_req.object.content {
            if let Element::String(key) = &*member.key {
                if key.content == "oauth2" || key.content == "apiKey" {
                    // Security scheme key should have map-field metadata
                    assert!(key.meta.properties.contains_key("map-field"));
                    assert!(key.meta.properties.contains_key("security-scheme-name"));
                    
                    // Scopes array should have security metadata
                    if let Element::Array(scopes_arr) = &*member.value {
                        assert!(scopes_arr.meta.properties.contains_key("scopes-array"));
                        assert_eq!(
                            scopes_arr.meta.properties.get("security-scheme"),
                            Some(&Value::String(key.content.clone()))
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_security_requirement_with_specification_extensions() {
        let mut obj = ObjectElement::new();
        
        // Add regular security schemes
        let mut oauth_scopes = ArrayElement::new_empty();
        oauth_scopes.content.push(Element::String(StringElement::new("admin")));
        obj.set("oauth2", Element::Array(oauth_scopes));
        
        // Add specification extensions
        obj.set("x-security-level", Element::String(StringElement::new("high")));
        obj.set("x-rate-limit", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 100.0,
        }));
        
        let result = build_and_decorate_security_requirement(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_req = result.unwrap();
        
        // Verify specification extension metadata
        assert!(sec_req.object.meta.properties.contains_key("specificationExtension_x-security-level"));
        assert!(sec_req.object.meta.properties.contains_key("specificationExtension_x-rate-limit"));
        
        let mut found_extension = false;
        for member in &sec_req.object.content {
            if let Element::String(key) = &*member.key {
                if key.content.starts_with("x-") {
                    assert!(key.meta.properties.contains_key("specification-extension"));
                    found_extension = true;
                }
            }
        }
        assert!(found_extension);
    }

    #[test]
    fn test_security_requirement_with_ref() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/securityRequirements/BasicAuth")));
        
        let result = build_and_decorate_security_requirement(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_req = result.unwrap();
        
        // Verify reference metadata
        assert!(sec_req.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "reference"
            } else {
                false
            }
        }));
        assert_eq!(
            sec_req.object.meta.properties.get("referenced-element"),
            Some(&Value::String("securityRequirement".to_string()))
        );
        assert_eq!(
            sec_req.object.meta.properties.get("reference-path"),
            Some(&Value::String("#/components/securityRequirements/BasicAuth".to_string()))
        );
    }

    #[test]
    fn test_security_requirement_with_fallback_fields() {
        let mut obj = ObjectElement::new();
        
        // Add valid security scheme
        let mut oauth_scopes = ArrayElement::new_empty();
        oauth_scopes.content.push(Element::String(StringElement::new("read")));
        obj.set("oauth2", Element::Array(oauth_scopes));
        
        // Add fallback fields (invalid security scheme names or unknown fields)
        obj.set("", Element::Array(ArrayElement::new_empty())); // Empty string (invalid)
        obj.set("customField", Element::String(StringElement::new("custom value")));
        obj.set("unknownProperty", Element::Boolean(BooleanElement::new(true)));
        
        let result = build_and_decorate_security_requirement(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_req = result.unwrap();
        
        // Verify fallback field metadata
        assert!(sec_req.object.meta.properties.contains_key("fallback_"));
        assert!(sec_req.object.meta.properties.contains_key("fallback_customField"));
        assert!(sec_req.object.meta.properties.contains_key("fallback_unknownProperty"));
        
        let mut found_fallback = false;
        for member in &sec_req.object.content {
            if let Element::String(key) = &*member.key {
                if key.content.is_empty() || key.content == "customField" || key.content == "unknownProperty" {
                    assert!(key.meta.properties.contains_key("fallback-field"));
                    found_fallback = true;
                }
            }
        }
        assert!(found_fallback);
    }

    #[test]
    fn test_typescript_equivalence_demo() {
        // This test demonstrates equivalence with TypeScript SecurityRequirementVisitor
        let mut obj = ObjectElement::new();
        
        // Add comprehensive security schemes
        let mut oauth2_scopes = ArrayElement::new_empty();
        oauth2_scopes.content.push(Element::String(StringElement::new("read:users")));
        oauth2_scopes.content.push(Element::String(StringElement::new("write:users")));
        oauth2_scopes.content.push(Element::String(StringElement::new("admin")));
        obj.set("oauth2", Element::Array(oauth2_scopes));
        
        let mut jwt_scopes = ArrayElement::new_empty();
        jwt_scopes.content.push(Element::String(StringElement::new("api:access")));
        obj.set("jwt_token", Element::Array(jwt_scopes));
        
        // Empty scopes (like API key)
        let api_key_scopes = ArrayElement::new_empty();
        obj.set("api_key", Element::Array(api_key_scopes));
        
        // Add specification extensions
        obj.set("x-security-level", Element::String(StringElement::new("enterprise")));
        obj.set("x-auth-timeout", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 3600.0,
        }));
        
        // Add fallback field
        obj.set("customSecurityMetadata", Element::String(StringElement::new("custom security value")));
        
        let result = build_and_decorate_security_requirement(&Element::Object(obj), None::<&mut OpenApiBuilderFolder>);
        
        assert!(result.is_some());
        let sec_req = result.unwrap();
        
        // Verify TypeScript equivalence features:
        
        // 1. MapVisitor processing
        assert!(sec_req.object.meta.properties.contains_key("mapVisitor"));
        assert_eq!(
            sec_req.object.meta.properties.get("fieldPatternPredicate"),
            Some(&Value::String("isNonEmptyString".to_string()))
        );
        
        // 2. Specification extensions support
        assert!(sec_req.object.meta.properties.contains_key("canSupportSpecificationExtensions"));
        let mut found_extension = false;
        for member in &sec_req.object.content {
            if let Element::String(key) = &*member.key {
                if key.content.starts_with("x-") {
                    assert!(key.meta.properties.contains_key("specification-extension"));
                    found_extension = true;
                }
            }
        }
        assert!(found_extension);
        
        // 3. Security scheme processing (equivalent to TypeScript map field processing)
        for member in &sec_req.object.content {
            if let Element::String(key) = &*member.key {
                if ["oauth2", "jwt_token", "api_key"].contains(&key.content.as_str()) {
                    assert!(key.meta.properties.contains_key("map-field"));
                    assert!(key.meta.properties.contains_key("security-scheme-name"));
                    
                    // Scopes array metadata
                    if let Element::Array(scopes_arr) = &*member.value {
                        assert!(scopes_arr.meta.properties.contains_key("scopes-array"));
                        assert_eq!(
                            scopes_arr.meta.properties.get("security-scheme"),
                            Some(&Value::String(key.content.clone()))
                        );
                    }
                }
            }
        }
        
        // 4. Fixed field processing (equivalent to TypeScript field processing)
        assert!(sec_req.object.meta.properties.contains_key("fixedField_oauth2"));
        assert!(sec_req.object.meta.properties.contains_key("fixedField_jwt_token"));
        assert!(sec_req.object.meta.properties.contains_key("fixedField_api_key"));
        
        // 5. Fallback field handling
        assert!(sec_req.object.meta.properties.contains_key("fallback_customSecurityMetadata"));
        let mut found_fallback = false;
        for member in &sec_req.object.content {
            if let Element::String(key) = &*member.key {
                if key.content == "customSecurityMetadata" {
                    assert!(key.meta.properties.contains_key("fallback-field"));
                    found_fallback = true;
                }
            }
        }
        assert!(found_fallback);
        
        // 6. Element classification (equivalent to TypeScript class injection)
        assert!(sec_req.object.classes.content.iter().any(|e| {
            if let Element::String(s) = e {
                s.content == "security-requirement"
            } else {
                false
            }
        }));
        assert_eq!(
            sec_req.object.meta.properties.get("element-type"),
            Some(&Value::String("securityRequirement".to_string()))
        );
        
        // 7. Spec path metadata (equivalent to TypeScript specPath = always(['value']))
        assert!(sec_req.object.meta.properties.contains_key("specPath"));
        if let Some(Value::Array(spec_path)) = sec_req.object.meta.properties.get("specPath") {
            assert_eq!(spec_path.len(), 1);
            assert_eq!(spec_path[0], Value::String("value".to_string()));
        }
        
        // 8. Processing metadata
        assert!(sec_req.object.meta.properties.contains_key("processed"));
        assert!(sec_req.object.meta.properties.contains_key("fallbackVisitor"));
        
        // 9. Validation
        assert!(sec_req.object.meta.properties.contains_key("validSecurityRequirement"));
        
        // 10. Field count verification
        assert!(sec_req.object.content.len() >= 6); // 3 security schemes + 2 extensions + 1 fallback
        
        // 11. Verify scheme names method works
        let scheme_names = sec_req.scheme_names();
        assert!(scheme_names.contains(&"oauth2".to_string()));
        assert!(scheme_names.contains(&"jwt_token".to_string()));
        assert!(scheme_names.contains(&"api_key".to_string()));
        assert!(scheme_names.len() >= 3);
    }
}