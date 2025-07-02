use std::collections::HashMap;
use validator::ValidationError;
use crate::openapi3_1_spec::MediaType;
use regex::Regex;
use crate::openapi3_1_spec::{
    License, Parameter, ServerVariable, 
    Server, Example, ExampleOrRef, Header, 
    SecurityScheme, Encoding, Contact, Info, 
    Tag, ExternalDocumentation, OAuthFlow, OAuthFlows, Components, OpenApiSpec
};

// Primitive field validators -------------------------------------------------

pub fn validate_parameter_in(location: &str) -> Result<(), ValidationError> {
    match location {
        "query" | "header" | "path" | "cookie" => Ok(()),
        _ => Err(ValidationError::new("invalid_parameter_location")),
    }
}

pub fn validate_parameter_style(style: &str) -> Result<(), ValidationError> {
    match style {
        "matrix" | "label" | "simple" | "form" | "spaceDelimited" | "pipeDelimited" | "deepObject" => Ok(()),
        _ => Err(ValidationError::new("invalid_parameter_style")),
    }
}

pub fn validate_content_size(content: &HashMap<String, MediaType>) -> Result<(), ValidationError> {
    if content.len() > 1 {
        Err(ValidationError::new("content_size_exceeded"))
    } else {
        Ok(())
    }
}

// Struct-level validators -----------------------------------------------------

pub fn validate_parameter_struct(param: &Parameter) -> Result<(), ValidationError> {
    // XOR rule: exactly one of schema or (non-empty) content must be present
    let has_schema = param.schema.is_some();
    let has_content = !param.content.is_empty();
    if has_schema == has_content {
        return Err(ValidationError::new("parameter_schema_content_xor"));
    }

    // allow_empty_value only valid for query parameters
    if param.allow_empty_value && param.location != "query" {
        return Err(ValidationError::new("allow_empty_value_not_allowed"));
    }

    // in == path ⇒ required must be true and style limited
    if param.location == "path" {
        if !param.required {
            return Err(ValidationError::new("path_parameter_must_be_required"));
        }
        if let Some(style) = &param.style {
            match style.as_str() {
                "matrix" | "label" | "simple" => {}
                _ => return Err(ValidationError::new("invalid_style_for_path")),
            }
        }
    }

    // in == header ⇒ style must be simple (if specified)
    if param.location == "header" {
        if let Some(style) = &param.style {
            if style != "simple" {
                return Err(ValidationError::new("invalid_style_for_header"));
            }
        }
    }

    // in == query ⇒ style allowed set
    if param.location == "query" {
        if let Some(style) = &param.style {
            match style.as_str() {
                "form" | "spaceDelimited" | "pipeDelimited" | "deepObject" => {},
                _ => return Err(ValidationError::new("invalid_style_for_query")),
            }
        }
    }

    // in == cookie ⇒ style must be form (if specified)
    if param.location == "cookie" {
        if let Some(style) = &param.style {
            if style != "form" {
                return Err(ValidationError::new("invalid_style_for_cookie"));
            }
        }
    }

    // Additional checks --------------------------------------------------
    // Path parameter name must match pattern (no '/', '#', '?')
    if param.location == "path" {
        let re = Regex::new(r"[/#?]").unwrap();
        if re.is_match(&param.name) {
            return Err(ValidationError::new("invalid_path_parameter_name"));
        }
    }

    // example and examples are mutually exclusive
    if param.example.is_some() && !param.examples.is_empty() {
        return Err(ValidationError::new("example_examples_mutex"));
    }

    // Extension keys must start with x-
    for key in param.extensions.keys() {
        if !key.starts_with("x-") {
            return Err(ValidationError::new("invalid_extension_key"));
        }
    }

    Ok(())
}

pub fn validate_license_struct(lic: &License) -> Result<(), ValidationError> {
    if lic.identifier.is_some() && lic.url.is_some() {
        return Err(ValidationError::new("license_identifier_with_url"));
    }
    Ok(())
}

pub fn validate_server_variable_struct(var: &ServerVariable) -> Result<(), ValidationError> {
    // enum must have at least 1 item if provided
    if let Some(list) = &var.allowed_values {
        if list.is_empty() {
            return Err(ValidationError::new("server_variable_enum_empty"));
        }
        // default must be in enum
        if !list.contains(&var.default) {
            return Err(ValidationError::new("default_not_in_enum"));
        }
    }
    Ok(())
}

pub fn validate_server_struct(srv: &Server) -> Result<(), ValidationError> {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();
    let mut placeholders: Vec<String> = Vec::new();
    for cap in re.captures_iter(&srv.url) {
        placeholders.push(cap[1].to_string());
        if !srv.variables.contains_key(&cap[1]) {
            return Err(ValidationError::new("undefined_server_variable"));
        }
    }
    // extra variables defined but not used in url
    for key in srv.variables.keys() {
        if !placeholders.contains(key) {
            return Err(ValidationError::new("unused_server_variable"));
        }
    }
    Ok(())
}

pub fn validate_header_struct(header: &Header) -> Result<(), ValidationError> {
    // Exactly one of schema or content
    let has_schema = header.schema.is_some();
    let has_content = !header.content.is_empty();
    if has_schema == has_content {
        return Err(ValidationError::new("header_schema_content_xor"));
    }

    // style must be simple (spec default)
    if header.style != "simple" {
        return Err(ValidationError::new("invalid_header_style"));
    }

    // content maximum 1 property ensured by validate_content_size already (Header uses same helper)
    if header.content.len() > 1 {
        return Err(ValidationError::new("content_size_exceeded"));
    }

    Ok(())
}

pub fn validate_security_scheme_struct(ss: &SecurityScheme) -> Result<(), ValidationError> {
    match ss.r#type.as_str() {
        "apiKey" => {
            if ss.name.is_none() || ss.r#in.is_none() {
                return Err(ValidationError::new("apikey_missing_fields"));
            }
        }
        "http" => {
            if ss.scheme.is_none() { return Err(ValidationError::new("http_missing_scheme")); }
            if let Some(scheme) = &ss.scheme {
                if scheme.to_lowercase() == "bearer" {
                    // bearerFormat optional, nothing extra
                }
            }
        }
        "oauth2" => {
            if ss.flows.is_none() {
                return Err(ValidationError::new("oauth2_missing_flows"));
            }
        }
        "openIdConnect" => {
            if ss.open_id_connect_url.is_none() {
                return Err(ValidationError::new("oidc_missing_url"));
            }
        }
        _ => return Err(ValidationError::new("invalid_security_type")),
    }
    Ok(())
}

pub fn validate_example_struct(ex: &Example) -> Result<(), ValidationError> {
    if ex.value.is_some() && ex.external_value.is_some() {
        return Err(ValidationError::new("value_externalValue_mutex"));
    }
    Ok(())
}

pub fn validate_encoding_struct(enc: &Encoding) -> Result<(), ValidationError> {
    // style allowed
    match enc.style.as_str() {
        "form" | "spaceDelimited" | "pipeDelimited" | "deepObject" => {}
        _ => return Err(ValidationError::new("invalid_encoding_style")),
    }

    // contentType (if present) must be media-range e.g. application/json or application/*
    if let Some(ct) = &enc.content_type {
        let re = Regex::new(r"^[\w!#$&^_.+-]+/[\w!#$&^_.+*-]+$").unwrap();
        if !re.is_match(ct) {
            return Err(ValidationError::new("invalid_content_type"));
        }
    }

    // headers key pattern
    let hdr_key_re = Regex::new(r"^[A-Za-z0-9-]+$").unwrap();
    for key in enc.headers.keys() {
        if !hdr_key_re.is_match(key) {
            return Err(ValidationError::new("invalid_encoding_header_key"));
        }
    }

    // explode defaults based on style
    if enc.style == "form" {
        if !enc.explode {
            return Err(ValidationError::new("explode_should_default_true"));
        }
    } else {
        if enc.explode {
            return Err(ValidationError::new("explode_should_default_false"));
        }
    }
    Ok(())
}

// --- Additional validators ----------------------------------------------------
pub fn validate_contact_struct(c: &Contact) -> Result<(), ValidationError> {
    // nothing extra beyond field-level url/email validation
    // but at least one of name/url/email should appear per good practice
    if c.name.is_none() && c.url.is_none() && c.email.is_none() {
        return Err(ValidationError::new("contact_empty"));
    }
    Ok(())
}

pub fn validate_info_struct(info: &Info) -> Result<(), ValidationError> {
    if info.title.trim().is_empty() {
        return Err(ValidationError::new("empty_title"));
    }
    if info.version.trim().is_empty() {
        return Err(ValidationError::new("empty_version"));
    }
    Ok(())
}

pub fn validate_tag_struct(tag: &Tag) -> Result<(), ValidationError> {
    if tag.name.trim().is_empty() {
        return Err(ValidationError::new("empty_tag_name"));
    }
    Ok(())
}

pub fn validate_external_docs_struct(ed: &ExternalDocumentation) -> Result<(), ValidationError> {
    if ed.url.trim().is_empty() {
        return Err(ValidationError::new("empty_external_url"));
    }
    Ok(())
}

pub fn validate_oauth_flow_struct(flow: &OAuthFlow) -> Result<(), ValidationError> {
    // Determine which flow and validate
    // We'll rely on caller to decide type; here ensure required combos
    let mut err = |msg| Err(ValidationError::new(msg));

    // scopes always required non-empty
    if flow.scopes.is_empty() {
        return err("oauth_flow_empty_scopes");
    }
    Ok(())
}

pub fn validate_oauth_flows_struct(flows: &OAuthFlows) -> Result<(), ValidationError> {
    // Each present flow must have required urls
    if let Some(f) = &flows.implicit {
        if f.authorization_url.is_none() { return Err(ValidationError::new("implicit_missing_auth_url")); }
    }
    if let Some(f) = &flows.password {
        if f.token_url.is_none() { return Err(ValidationError::new("password_missing_token_url")); }
    }
    if let Some(f) = &flows.client_credentials {
        if f.token_url.is_none() { return Err(ValidationError::new("client_credentials_missing_token_url")); }
    }
    if let Some(f) = &flows.authorization_code {
        if f.authorization_url.is_none() || f.token_url.is_none() {
            return Err(ValidationError::new("auth_code_missing_urls"));
        }
    }
    Ok(())
}

pub fn validate_components_struct(c: &Components) -> Result<(), ValidationError> {
    let key_re = Regex::new(r"^[A-Za-z0-9._-]+$").unwrap();
    macro_rules! check_map { ($m:expr) => { for k in $m.keys() { if !key_re.is_match(k) { return Err(ValidationError::new("invalid_component_key")); } } }; }
    check_map!(&c.schemas);
    check_map!(&c.responses);
    check_map!(&c.parameters);
    check_map!(&c.examples);
    check_map!(&c.request_bodies);
    check_map!(&c.headers);
    check_map!(&c.security_schemes);
    check_map!(&c.links);
    check_map!(&c.callbacks);
    check_map!(&c.path_items);
    Ok(())
}

pub fn validate_openapi_spec(spec: &OpenApiSpec) -> Result<(), ValidationError> {
    if spec.paths.is_empty()
        && spec.webhooks.is_empty()
        && spec.components.as_ref().map_or(true, |c| {
            c.schemas.is_empty() && c.responses.is_empty() && c.parameters.is_empty()
        })
    {
        return Err(ValidationError::new("spec_missing_paths_components_webhooks"));
    }
    // path keys must start with '/'
    for k in spec.paths.keys() {
        if !k.starts_with('/') {
            return Err(ValidationError::new("invalid_path_key"));
        }
    }
    Ok(())
}

pub fn validate_media_type_struct(mt: &MediaType) -> Result<(), ValidationError> {
    if mt.example.is_some() && !mt.examples.is_empty() {
        return Err(ValidationError::new("media_example_examples_mutex"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;
    use crate::simple_value::SimpleValue;
    use crate::openapi3_1_spec::{MediaType, Parameter, License, SchemaOrRef, Schema, Response, ResponseOrRef, Responses, Server, ServerVariable, Header, SecurityScheme, HeaderOrRef};
    use std::collections::HashMap;

    fn base_parameter() -> Parameter {
        Parameter {
            name: "id".to_string(),
            location: "query".to_string(),
            description: None,
            required: false,
            deprecated: false,
            allow_empty_value: false,
            style: None,
            explode: false,
            allow_reserved: false,
            schema: None,
            example: None,
            examples: HashMap::new(),
            content: HashMap::new(),
            extensions: HashMap::new(),
        }
    }

    #[test]
    fn test_parameter_path_name_pattern() {
        let mut p = base_parameter();
        p.location = "path".to_string();
        p.required = true;
        p.schema = Some(SchemaOrRef::Item(Schema::Boolean(true)));
        
        // Valid path parameter names
        p.name = "userId".to_string();
        assert!(p.validate().is_ok());
        
        // Invalid path parameter names
        p.name = "user/id".to_string();
        assert!(p.validate().is_err());
        p.name = "user#id".to_string();
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_parameter_example_content_mutex() {
        let mut p = base_parameter();
        p.schema = Some(SchemaOrRef::Item(Schema::Boolean(true)));
        
        // Can't have both example and examples
        p.example = Some(serde_json::json!(true));
        p.examples = {
            let mut m = HashMap::new();
            m.insert("test".to_string(), ExampleOrRef::Item(Example {
                summary: None,
                description: None,
                value: Some(serde_json::json!(false)),
                external_value: None,
                extensions: HashMap::new(),
            }));
            m
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_responses_validation() {
        // Must have at least one response
        let empty_responses = Responses::new();
        assert!(empty_responses.validate().is_err());
        
        // Valid status code patterns
        let mut responses = Responses::new();
        responses.status_codes.insert("200".to_string(), ResponseOrRef::Item(Response {
            description: "OK".to_string(),
            headers: HashMap::new(),
            content: HashMap::new(),
            links: HashMap::new(),
            extensions: HashMap::new(),
        }));
        assert!(responses.validate().is_ok());
        
        responses.status_codes.insert("2XX".to_string(), ResponseOrRef::Item(Response {
            description: "Success".to_string(),
            headers: HashMap::new(),
            content: HashMap::new(),
            links: HashMap::new(),
            extensions: HashMap::new(),
        }));
        assert!(responses.validate().is_ok());
        
        // Invalid status code patterns
        let mut bad_responses = Responses::new();
        bad_responses.status_codes.insert("2".to_string(), ResponseOrRef::Item(Response {
            description: "Invalid".to_string(),
            headers: HashMap::new(),
            content: HashMap::new(),
            links: HashMap::new(),
            extensions: HashMap::new(),
        }));
        assert!(bad_responses.validate().is_err());
    }

    #[test]
    fn test_server_url_template() {
        // Test URL template validation
        let server = Server {
            url: "https://{env}.api.example.com/v1".to_string(),
            description: None,
            variables: {
                let mut vars = HashMap::new();
                vars.insert("env".to_string(), ServerVariable {
                    allowed_values: Some(vec!["dev".into(), "prod".into()]),
                    default: "dev".into(),
                    description: None,
                    extensions: HashMap::new(),
                });
                vars
            },
            extensions: HashMap::new(),
        };
        assert!(server.validate().is_ok());
        
        // Missing variable definition
        let server_missing_var = Server {
            url: "https://{env}.api.example.com/v1".to_string(),
            description: None,
            variables: HashMap::new(),
            extensions: HashMap::new(),
        };
        assert!(server_missing_var.validate().is_err());
    }

    #[test]
    fn test_extension_validation() {
        // Test x- prefix requirement
        let mut extensions = HashMap::new();
        
        // Valid extension key
        extensions.insert("x-custom-field".to_string(), 
            SimpleValue::String("value".to_string()));
        
        // Invalid extension key (no x- prefix)
        extensions.insert("custom-field".to_string(),
            SimpleValue::String("value".to_string()));
            
        let p = Parameter {
            name: "test".to_string(),
            location: "query".to_string(),
            description: None,
            required: false,
            deprecated: false,
            allow_empty_value: false,
            style: None,
            explode: false,
            allow_reserved: false,
            schema: Some(SchemaOrRef::Item(Schema::Boolean(true))),
            example: None,
            examples: HashMap::new(),
            content: HashMap::new(),
            extensions,
        };
        
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_media_type_validation() {
        // Test valid media type pattern with single item (size check passes)
        let mut content = HashMap::new();
        content.insert("application/json".to_string(), MediaType {
            schema: None,
            example: None,
            examples: HashMap::new(),
            encoding: HashMap::new(),
            extensions: HashMap::new(),
        });
        assert!(validate_content_size(&content).is_ok());

        // Invalid media type format
        let mut invalid_content = HashMap::new();
        invalid_content.insert("invalid".to_string(), MediaType {
            schema: None,
            example: None,
            examples: HashMap::new(),
            encoding: HashMap::new(),
            extensions: HashMap::new(),
        });
        assert!(validate_content_size(&invalid_content).is_ok()); // size ok; format not checked yet
    }

    #[test]
    fn test_header_validation() {
        // Valid header using schema
        let hdr = Header {
            description: None,
            required: false,
            deprecated: false,
            schema: Some(crate::openapi3_1_spec::SchemaOrRef::Item(crate::openapi3_1_spec::Schema::Boolean(true))),
            style: "simple".to_string(),
            explode: false,
            example: None,
            examples: HashMap::new(),
            content: HashMap::new(),
            extensions: HashMap::new(),
        };
        assert!(hdr.validate().is_ok());

        // Invalid header with both schema and content
        let mut hdr2 = hdr.clone();
        hdr2.content.insert("application/json".to_string(), MediaType {
            schema: None,
            example: None,
            examples: HashMap::new(),
            encoding: HashMap::new(),
            extensions: HashMap::new(),
        });
        assert!(hdr2.validate().is_err());

        // Invalid style
        let mut hdr3 = hdr.clone();
        hdr3.style = "form".to_string();
        assert!(hdr3.validate().is_err());
    }

    #[test]
    fn test_security_scheme_validation() {
        // apiKey valid
        let api_key = SecurityScheme {
            r#type: "apiKey".to_string(),
            description: None,
            name: Some("api_key".to_string()),
            r#in: Some("header".to_string()),
            scheme: None,
            bearer_format: None,
            flows: None,
            open_id_connect_url: None,
            extensions: HashMap::new(),
        };
        assert!(api_key.validate().is_ok());

        // apiKey missing name
        let mut api_key_bad = api_key.clone();
        api_key_bad.name = None;
        assert!(api_key_bad.validate().is_err());

        // http missing scheme
        let http_bad = SecurityScheme {
            r#type: "http".to_string(),
            description: None,
            name: None,
            r#in: None,
            scheme: None,
            bearer_format: None,
            flows: None,
            open_id_connect_url: None,
            extensions: HashMap::new(),
        };
        assert!(http_bad.validate().is_err());

        // oauth2 missing flows
        let oauth2_bad = SecurityScheme {
            r#type: "oauth2".to_string(),
            description: None,
            name: None,
            r#in: None,
            scheme: None,
            bearer_format: None,
            flows: None,
            open_id_connect_url: None,
            extensions: HashMap::new(),
        };
        assert!(oauth2_bad.validate().is_err());
    }

    #[test]
    fn test_example_validation() {
        let ex_ok = Example {
            summary: None,
            description: None,
            value: Some(serde_json::json!({"foo": "bar"})),
            external_value: None,
            extensions: HashMap::new(),
        };
        assert!(ex_ok.validate().is_ok());

        let ex_bad = Example {
            summary: None,
            description: None,
            value: Some(serde_json::json!(true)),
            external_value: Some("https://example.com".to_string()),
            extensions: HashMap::new(),
        };
        assert!(ex_bad.validate().is_err());
    }

    #[test]
    fn test_encoding_validation() {
        // Valid form style with explode=true
        let enc_ok = Encoding {
            content_type: None,
            headers: HashMap::new(),
            style: "form".to_string(),
            explode: true,
            allow_reserved: false,
            extensions: HashMap::new(),
        };
        assert!(enc_ok.validate().is_ok());

        // Invalid: form but explode = false
        let mut enc_bad = enc_ok.clone();
        enc_bad.explode = false;
        assert!(enc_bad.validate().is_err());

        // Invalid: spaceDelimited with explode = true
        let enc_bad2 = Encoding {
            content_type: None,
            headers: HashMap::new(),
            style: "spaceDelimited".to_string(),
            explode: true,
            allow_reserved: false,
            extensions: HashMap::new(),
        };
        assert!(enc_bad2.validate().is_err());
    }

    #[test]
    fn test_contact_validation() {
        let c_good = Contact { name: Some("API Team".into()), url: None, email: None, extensions: HashMap::new() };
        assert!(c_good.validate().is_ok());

        let c_bad = Contact { name: None, url: None, email: None, extensions: HashMap::new() };
        assert!(c_bad.validate().is_err());
    }

    #[test]
    fn test_info_validation() {
        let i_bad = Info { title: "".into(), summary: None, description: None, terms_of_service: None, contact: None, license: None, version: "1.0".into(), extensions: HashMap::new() };
        assert!(i_bad.validate().is_err());
    }

    #[test]
    fn test_tag_validation() {
        let t = Tag { name: "users".into(), description: None, external_docs: None, extensions: HashMap::new() };
        assert!(t.validate().is_ok());
        let t_bad = Tag { name: "".into(), description: None, external_docs: None, extensions: HashMap::new() };
        assert!(t_bad.validate().is_err());
    }

    #[test]
    fn test_external_docs_validation() {
        let ed = ExternalDocumentation { description: None, url: "https://example.com".into(), extensions: HashMap::new() };
        assert!(ed.validate().is_ok());
        let ed_bad = ExternalDocumentation { description: None, url: "".into(), extensions: HashMap::new() };
        assert!(ed_bad.validate().is_err());
    }

    #[test]
    fn test_oauth_flows_validation() {
        let mut scopes = HashMap::new(); scopes.insert("read".into(), "r".into());
        let implicit = OAuthFlow { authorization_url: Some("https://auth".into()), token_url: None, refresh_url: None, scopes: scopes.clone(), extensions: HashMap::new() };
        let flows = OAuthFlows { implicit: Some(implicit), password: None, client_credentials: None, authorization_code: None, extensions: HashMap::new() };
        assert!(flows.validate().is_ok());

        let bad_flows = OAuthFlows { implicit: Some(OAuthFlow { authorization_url: None, token_url: None, refresh_url: None, scopes, extensions: HashMap::new() }), password: None, client_credentials: None, authorization_code: None, extensions: HashMap::new() };
        assert!(bad_flows.validate().is_err());
    }

    #[test]
    fn test_components_key_pattern() {
        let mut c = Components { schemas: HashMap::new(), responses: HashMap::new(), parameters: HashMap::new(), examples: HashMap::new(), request_bodies: HashMap::new(), headers: HashMap::new(), security_schemes: HashMap::new(), links: HashMap::new(), callbacks: HashMap::new(), path_items: HashMap::new(), extensions: HashMap::new() };
        c.schemas.insert("User".into(), crate::openapi3_1_spec::SchemaOrRef::Item(crate::openapi3_1_spec::Schema::Boolean(true)));
        assert!(c.validate().is_ok());

        c.schemas.insert("Bad Key".into(), crate::openapi3_1_spec::SchemaOrRef::Item(crate::openapi3_1_spec::Schema::Boolean(true)));
        assert!(c.validate().is_err());
    }

    #[test]
    fn test_openapi_spec_min_requirement() {
        let spec_empty = OpenApiSpec {
            openapi: "3.1.0".into(),
            info: Info { title: "t".into(), summary: None, description: None, terms_of_service: None, contact: None, license: None, version: "1".into(), extensions: HashMap::new() },
            json_schema_dialect: None,
            servers: vec![],
            paths: HashMap::new(),
            webhooks: HashMap::new(),
            components: None,
            security: vec![],
            tags: vec![],
            external_docs: None,
            extensions: HashMap::new(),
        };
        assert!(spec_empty.validate().is_err());
    }

    #[test]
    fn test_media_type_example_mutex() {
        let mt_ok = MediaType { schema: None, example: Some(serde_json::json!(1)), examples: HashMap::new(), encoding: HashMap::new(), extensions: HashMap::new() };
        assert!(mt_ok.validate().is_ok());

        let mut mt_bad = mt_ok.clone();
        mt_bad.examples.insert("a".into(), crate::openapi3_1_spec::ExampleOrRef::Item(crate::openapi3_1_spec::Example { summary: None, description: None, value: Some(serde_json::json!(1)), external_value: None, extensions: HashMap::new() }));
        assert!(mt_bad.validate().is_err());
    }

    #[test]
    fn test_encoding_header_key_pattern() {
        let mut headers = HashMap::new();
        headers.insert("X-Custom".into(), HeaderOrRef::Ref(crate::openapi3_1_spec::Reference::new("#/components/headers/X".into())));
        let enc = Encoding { content_type: Some("application/json".into()), headers, style: "form".into(), explode: true, allow_reserved: false, extensions: HashMap::new() };
        assert!(enc.validate().is_ok());

        let mut headers_bad = HashMap::new();
        headers_bad.insert("Invalid Header".into(), HeaderOrRef::Ref(crate::openapi3_1_spec::Reference::new("#/components/headers/X".into())));
        let enc_bad = Encoding { content_type: Some("application/json".into()), headers: headers_bad, style: "form".into(), explode: true, allow_reserved: false, extensions: HashMap::new() };
        assert!(enc_bad.validate().is_err());
    }

    #[test]
    fn test_encoding_content_type_regex() {
        let enc_good = Encoding { content_type: Some("application/*".into()), headers: HashMap::new(), style: "form".into(), explode: true, allow_reserved: false, extensions: HashMap::new() };
        assert!(enc_good.validate().is_ok());

        let enc_bad = Encoding { content_type: Some("not a type".into()), headers: HashMap::new(), style: "form".into(), explode: true, allow_reserved: false, extensions: HashMap::new() };
        assert!(enc_bad.validate().is_err());
    }

    #[test]
    fn test_server_variable_unused_detection() {
        let mut vars = HashMap::new();
        vars.insert("env".into(), ServerVariable { allowed_values: None, default: "prod".into(), description: None, extensions: HashMap::new() });
        vars.insert("lang".into(), ServerVariable { allowed_values: None, default: "en".into(), description: None, extensions: HashMap::new() });
        let srv = Server { url: "https://{env}.example.com".into(), description: None, variables: vars, extensions: HashMap::new() };
        assert!(srv.validate().is_err()); // lang unused
    }
} 