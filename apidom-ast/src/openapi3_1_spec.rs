use crate::minim_model::Element;
use crate::simple_value::SimpleValue;
use crate::validators::{
    validate_content_size, validate_license_struct, validate_parameter_in,
    validate_parameter_struct, validate_parameter_style,
};
use apidom_derive::BuildFromElement;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use validator::{Validate, ValidationError};

fn default_false() -> bool {
    false
}

#[allow(dead_code)]
fn default_empty_vec<T>() -> Vec<T> {
    Vec::new()
}

fn default_empty_map<K, V>() -> HashMap<K, V> {
    HashMap::new()
}

fn default_simple_style() -> String {
    "simple".to_string()
}

fn default_form_style() -> String {
    "form".to_string()
}

/// Reference Object
/// https://spec.openapis.org/oas/v3.1.0#reference-object
/// Represents a reference to another component in the OpenAPI document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub ref_field: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Helper enum to represent either a direct value or a reference to it
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrReference<T> {
    Item(T),
    Ref(Reference),
}

/// Schema Object
/// https://spec.openapis.org/oas/v3.1.0#schema-object
/// Represents a JSON Schema definition compatible with OpenAPI 3.1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Boolean(bool),
    Object(SchemaObject),
}

/// Schema Object properties when it's an object
/// https://spec.openapis.org/oas/v3.1.0#schema-object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaObject {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub multiple_of: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(default)]
    pub exclusive_maximum: Option<bool>,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub exclusive_minimum: Option<bool>,
    #[serde(default)]
    pub max_length: Option<u64>,
    #[serde(default)]
    pub min_length: Option<u64>,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub max_items: Option<u64>,
    #[serde(default)]
    pub min_items: Option<u64>,
    #[serde(default)]
    pub unique_items: Option<bool>,
    #[serde(default)]
    pub max_properties: Option<u64>,
    #[serde(default)]
    pub min_properties: Option<u64>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub r#enum: Vec<serde_json::Value>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub all_of: Vec<SchemaOrRef>,
    #[serde(default)]
    pub one_of: Vec<SchemaOrRef>,
    #[serde(default)]
    pub any_of: Vec<SchemaOrRef>,
    #[serde(default)]
    pub not: Option<Box<SchemaOrRef>>,
    #[serde(default)]
    pub items: Option<Box<SchemaOrRef>>,
    #[serde(default)]
    pub properties: HashMap<String, SchemaOrRef>,
    #[serde(default)]
    pub additional_properties: Option<Box<SchemaOrRef>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub nullable: Option<bool>,
    #[serde(default)]
    pub discriminator: Option<Discriminator>,
    #[serde(default)]
    pub read_only: Option<bool>,
    #[serde(default)]
    pub write_only: Option<bool>,
    #[serde(default)]
    pub example: Option<serde_json::Value>,
    #[serde(default)]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(default)]
    pub deprecated: Option<bool>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Discriminator Object
/// https://spec.openapis.org/oas/v3.1.0#discriminator-object
/// Adds support for polymorphism in schemas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discriminator {
    pub property_name: String,
    #[serde(default)]
    pub mapping: HashMap<String, String>,
}

/// Response Object
/// https://spec.openapis.org/oas/v3.1.0#response-object
/// Describes a single response from an API Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    #[serde(default)]
    pub headers: HashMap<String, HeaderOrRef>,
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
    #[serde(default)]
    pub links: HashMap<String, LinkOrRef>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Request Body Object
/// https://spec.openapis.org/oas/v3.1.0#request-body-object
/// Describes a single request body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(default)]
    pub description: Option<String>,
    pub content: HashMap<String, MediaType>,
    #[serde(default)]
    pub required: bool,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Header Object
/// https://spec.openapis.org/oas/v3.1.0#header-object
/// Describes a single operation parameter that appears in a header
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_header_struct"))]
pub struct Header {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_false")]
    pub required: bool,
    #[serde(default = "default_false")]
    pub deprecated: bool,
    #[serde(default)]
    pub schema: Option<SchemaOrRef>,
    #[serde(default = "default_simple_style")]
    pub style: String, // "simple"
    #[serde(default = "default_false")]
    pub explode: bool,
    #[serde(default)]
    pub example: Option<serde_json::Value>,
    #[serde(default = "default_empty_map")]
    pub examples: HashMap<String, ExampleOrRef>,
    #[serde(default = "default_empty_map")]
    pub content: HashMap<String, MediaType>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Security Scheme Object
/// https://spec.openapis.org/oas/v3.1.0#security-scheme-object
/// Defines a security scheme that can be used by the operations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_security_scheme_struct"))]
pub struct SecurityScheme {
    pub r#type: String,
    #[serde(default)]
    pub description: Option<String>,
    // apiKey 类型特有字段
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub r#in: Option<String>,
    // http 类型特有字段
    #[serde(default)]
    pub scheme: Option<String>,
    #[serde(default)]
    pub bearer_format: Option<String>,
    // oauth2 类型特有字段
    #[serde(default)]
    pub flows: Option<OAuthFlows>,
    // openIdConnect 类型特有字段
    #[serde(default)]
    pub open_id_connect_url: Option<String>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// OAuth Flows Object
/// https://spec.openapis.org/oas/v3.1.0#oauth-flows-object
/// Allows configuration of the supported OAuth Flows
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_oauth_flows_struct"))]
pub struct OAuthFlows {
    #[serde(default)]
    pub implicit: Option<OAuthFlow>,
    #[serde(default)]
    pub password: Option<OAuthFlow>,
    #[serde(default)]
    pub client_credentials: Option<OAuthFlow>,
    #[serde(default)]
    pub authorization_code: Option<OAuthFlow>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// OAuth Flow Object
/// https://spec.openapis.org/oas/v3.1.0#oauth-flow-object
/// Configuration details for a supported OAuth Flow
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_oauth_flow_struct"))]
pub struct OAuthFlow {
    #[serde(default)]
    pub authorization_url: Option<String>,
    #[serde(default)]
    pub token_url: Option<String>,
    #[serde(default)]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Example Object
/// https://spec.openapis.org/oas/v3.1.0#example-object
/// Represents an example that can be used to demonstrate the use of a schema
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_example_struct"))]
pub struct Example {
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub external_value: Option<String>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Media Type Object
/// https://spec.openapis.org/oas/v3.1.0#media-type-object
/// Provides schema and examples for a particular media type
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_media_type_struct"))]
pub struct MediaType {
    #[serde(default)]
    pub schema: Option<SchemaOrRef>,
    #[serde(default)]
    pub example: Option<serde_json::Value>,
    #[serde(default)]
    pub examples: HashMap<String, ExampleOrRef>,
    #[serde(default)]
    pub encoding: HashMap<String, Encoding>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Encoding Object
/// https://spec.openapis.org/oas/v3.1.0#encoding-object
/// A single encoding definition applied to a single schema property
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_encoding_struct"))]
pub struct Encoding {
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default = "default_empty_map")]
    pub headers: HashMap<String, HeaderOrRef>,
    #[serde(default = "default_form_style")]
    pub style: String, // "form", "spaceDelimited", "pipeDelimited", "deepObject"
    #[serde(default = "default_false")]
    pub explode: bool,
    #[serde(default = "default_false")]
    pub allow_reserved: bool,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Path Item Object
/// https://spec.openapis.org/oas/v3.1.0#path-item-object
/// Describes the operations available on a single path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub get: Option<Operation>,
    #[serde(default)]
    pub put: Option<Operation>,
    #[serde(default)]
    pub post: Option<Operation>,
    #[serde(default)]
    pub delete: Option<Operation>,
    #[serde(default)]
    pub options: Option<Operation>,
    #[serde(default)]
    pub head: Option<Operation>,
    #[serde(default)]
    pub patch: Option<Operation>,
    #[serde(default)]
    pub trace: Option<Operation>,
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(default)]
    pub parameters: Vec<ParameterOrRef>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Parameter Object
/// https://spec.openapis.org/oas/v3.1.0#parameter-object
/// Describes a single operation parameter
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_parameter_struct"))]
pub struct Parameter {
    #[validate(length(min = 1))]
    pub name: String,
    #[serde(rename = "in")]
    #[validate(custom(function = "validate_parameter_in"))]
    pub location: String, // "query", "header", "path", "cookie"
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_false")]
    pub required: bool,
    #[serde(default = "default_false")]
    pub deprecated: bool,
    #[serde(default = "default_false")]
    pub allow_empty_value: bool,
    #[serde(default)]
    #[validate(custom(function = "validate_parameter_style"))]
    pub style: Option<String>, // "matrix", "label", "simple", "form", "spaceDelimited", "pipeDelimited", "deepObject"
    #[serde(default = "default_false")]
    pub explode: bool,
    #[serde(default = "default_false")]
    pub allow_reserved: bool,
    #[serde(default)]
    pub schema: Option<SchemaOrRef>,
    #[serde(default)]
    pub example: Option<serde_json::Value>,
    #[serde(default = "default_empty_map")]
    pub examples: HashMap<String, ExampleOrRef>,
    #[serde(default = "default_empty_map")]
    #[validate(custom(function = "validate_content_size"))]
    pub content: HashMap<String, MediaType>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Operation Object
/// https://spec.openapis.org/oas/v3.1.0#operation-object
/// Describes a single API operation on a path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(default)]
    pub operation_id: Option<String>,
    #[serde(default)]
    pub parameters: Vec<ParameterOrRef>,
    #[serde(default)]
    pub request_body: Option<RequestBodyOrRef>,
    pub responses: Responses,
    #[serde(default)]
    pub callbacks: HashMap<String, CallbackOrRef>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub security: Vec<SecurityRequirement>,
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Server Object
/// https://spec.openapis.org/oas/v3.1.0#server-object
/// An object representing a Server
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_server_struct"))]
pub struct Server {
    #[validate(url)]
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_empty_map")]
    pub variables: HashMap<String, ServerVariable>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Server Variable Object
/// https://spec.openapis.org/oas/v3.1.0#server-variable-object
/// An object representing a Server Variable for server URL template substitution
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_server_variable_struct"))]
pub struct ServerVariable {
    #[serde(rename = "enum")]
    pub allowed_values: Option<Vec<String>>,
    pub default: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Components Object
/// https://spec.openapis.org/oas/v3.1.0#components-object
/// Holds a set of reusable objects for different aspects of the OAS
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_components_struct"))]
pub struct Components {
    #[serde(default)]
    pub schemas: HashMap<String, SchemaOrRef>,
    #[serde(default)]
    pub responses: HashMap<String, ResponseOrRef>,
    #[serde(default)]
    pub parameters: HashMap<String, ParameterOrRef>,
    #[serde(default)]
    pub examples: HashMap<String, ExampleOrRef>,
    #[serde(default)]
    pub request_bodies: HashMap<String, RequestBodyOrRef>,
    #[serde(default)]
    pub headers: HashMap<String, HeaderOrRef>,
    #[serde(default)]
    pub security_schemes: HashMap<String, SecuritySchemeOrRef>,
    #[serde(default)]
    pub links: HashMap<String, LinkOrRef>,
    #[serde(default)]
    pub callbacks: HashMap<String, CallbackOrRef>,
    #[serde(default)]
    pub path_items: HashMap<String, PathItemOrRef>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Info Object
/// https://spec.openapis.org/oas/v3.1.0#info-object
/// The object provides metadata about the API
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_info_struct"))]
pub struct Info {
    #[validate(length(min = 1))]
    pub title: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub terms_of_service: Option<String>,
    #[serde(default)]
    pub contact: Option<Contact>,
    #[serde(default)]
    pub license: Option<License>,
    #[validate(length(min = 1))]
    pub version: String,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Contact Object
/// https://spec.openapis.org/oas/v3.1.0#contact-object
/// Contact information for the exposed API
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_contact_struct"))]
pub struct Contact {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    #[validate(url)]
    pub url: Option<String>,
    #[serde(default)]
    #[validate(email)]
    pub email: Option<String>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// License Object
/// https://spec.openapis.org/oas/v3.1.0#license-object
/// License information for the exposed API
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_license_struct"))]
pub struct License {
    #[validate(length(min = 1))]
    pub name: String,
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default)]
    #[validate(url)]
    pub url: Option<String>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Tag Object
/// https://spec.openapis.org/oas/v3.1.0#tag-object
/// Adds metadata to a single tag that is used by Operation Object
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_tag_struct"))]
pub struct Tag {
    #[validate(length(min = 1))]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// External Documentation Object
/// https://spec.openapis.org/oas/v3.1.0#external-documentation-object
/// Allows referencing an external resource for extended documentation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_external_docs_struct"))]
pub struct ExternalDocumentation {
    #[serde(default)]
    pub description: Option<String>,
    #[validate(url)]
    pub url: String,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Link Object
/// https://spec.openapis.org/oas/v3.1.0#link-object
/// Represents a possible design-time link for a response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    #[serde(default)]
    pub operation_ref: Option<String>,
    #[serde(default)]
    pub operation_id: Option<String>,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub request_body: Option<serde_json::Value>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub server: Option<Server>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Security Requirement Object
/// https://spec.openapis.org/oas/v3.1.0#security-requirement-object
/// Lists the required security schemes to execute this operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirement(pub HashMap<String, Vec<String>>);

/// Callback Object
pub type Callback = HashMap<String, PathItemOrRef>;

// Type aliases
pub type PathItemOrRef = OrReference<PathItem>;
pub type ParameterOrRef = OrReference<Parameter>;
pub type SchemaOrRef = OrReference<Schema>;
pub type ResponseOrRef = OrReference<Response>;
pub type RequestBodyOrRef = OrReference<RequestBody>;
pub type HeaderOrRef = OrReference<Header>;
pub type SecuritySchemeOrRef = OrReference<SecurityScheme>;
pub type ExampleOrRef = OrReference<Example>;
pub type LinkOrRef = OrReference<Link>;
pub type CallbackOrRef = OrReference<Callback>;

// 定义顶层结构
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "crate::validators::validate_openapi_spec"))]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: Info,
    #[serde(default)]
    pub json_schema_dialect: Option<String>,
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(default)]
    pub paths: HashMap<String, PathItemOrRef>,
    #[serde(default)]
    pub webhooks: HashMap<String, PathItemOrRef>,
    #[serde(default)]
    pub components: Option<Components>,
    #[serde(default)]
    pub security: Vec<SecurityRequirement>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(flatten)]
    pub extensions: HashMap<String, SimpleValue>,
}

/// Responses Object
/// https://spec.openapis.org/oas/v3.1.0#responses-object
/// A container for the expected responses of an operation
#[derive(Debug, Clone)]
pub struct Responses {
    pub default: Option<ResponseOrRef>,
    pub status_codes: HashMap<String, ResponseOrRef>,
    pub extensions: HashMap<String, SimpleValue>,
}

impl Responses {
    pub fn new() -> Self {
        Self {
            default: None,
            status_codes: HashMap::new(),
            extensions: HashMap::new(),
        }
    }
}

impl Validate for Responses {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        // Must have at least one response
        if self.default.is_none() && self.status_codes.is_empty() {
            errors.add("responses", ValidationError::new("responses_empty"));
        }

        // Status code pattern validation (collectively reported)
        let re = Regex::new(r"^[1-5](?:[0-9]{2}|XX)$").unwrap();
        let invalid_found = self.status_codes.keys().any(|key| !re.is_match(key));

        if invalid_found {
            errors.add(
                "status_codes",
                ValidationError::new("invalid_status_code_key"),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<'de> Deserialize<'de> for Responses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct ResponsesVisitor;

        impl<'de> Visitor<'de> for ResponsesVisitor {
            type Value = Responses;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a responses object")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut default = None;
                let mut status_codes = HashMap::new();
                let mut extensions = HashMap::new();

                while let Some(key) = map.next_key::<String>()? {
                    if key == "default" {
                        default = Some(map.next_value()?);
                    } else if key.starts_with("x-") {
                        extensions.insert(key, map.next_value()?);
                    } else if key.chars().all(|c| c.is_ascii_digit()) || key.ends_with("XX") {
                        status_codes.insert(key, map.next_value()?);
                    }
                }

                Ok(Responses {
                    default,
                    status_codes,
                    extensions,
                })
            }
        }

        deserializer.deserialize_map(ResponsesVisitor)
    }
}

impl Serialize for Responses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;

        if let Some(ref default) = self.default {
            map.serialize_entry("default", default)?;
        }

        for (status_code, response) in &self.status_codes {
            map.serialize_entry(status_code, response)?;
        }

        for (key, value) in &self.extensions {
            map.serialize_entry(key, value)?;
        }

        map.end()
    }
}

// Reference 实现
impl Reference {
    pub fn new(reference: String) -> Self {
        Self {
            ref_field: reference,
            summary: None,
            description: None,
        }
    }

    pub fn with_summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn validate_uri_reference(&self) -> Result<(), String> {
        let ref_str = &self.ref_field;

        if ref_str.starts_with('#') {
            return Self::validate_internal_ref(ref_str);
        }

        if let Ok(_) = Url::parse(ref_str) {
            return Ok(());
        }

        if ref_str.starts_with("./") || ref_str.starts_with("../") {
            return Ok(());
        }

        Err(format!("Invalid URI reference: {}", ref_str))
    }

    fn validate_internal_ref(ref_str: &str) -> Result<(), String> {
        let parts: Vec<&str> = ref_str[1..].trim_start_matches('/').split('/').collect();
        if parts.is_empty() {
            return Err("Empty reference path".to_string());
        }

        for part in parts {
            if part.is_empty() {
                return Err("Empty path segment".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_validation() {
        let valid_refs = vec![
            "#/components/schemas/User",
            "https://example.com/schemas/user",
            "./schemas/user",
            "../common/schemas/user",
        ];

        for ref_str in valid_refs {
            let reference = Reference::new(ref_str.to_string());
            assert!(reference.validate_uri_reference().is_ok());
        }

        let invalid_refs = vec![
            "", "#",
            "#/",
            // NOTE: `url` crate considers `http:invalid` a valid URI reference (path-absolute);
            // keep truly invalid examples only.
        ];

        for ref_str in invalid_refs {
            let reference = Reference::new(ref_str.to_string());
            assert!(reference.validate_uri_reference().is_err());
        }
    }
}
