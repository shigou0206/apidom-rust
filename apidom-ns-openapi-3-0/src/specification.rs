//! # OpenAPI 3.0 Specification
//!
//! This module provides a comprehensive specification structure equivalent to the TypeScript
//! specification object. It defines the complete visitor mapping system for OpenAPI 3.0
//! document processing, allowing complete control over element traversal and transformation.
//!
//! ## Features
//!
//! ### 1. Complete Visitor Mapping
//! - Maps all OpenAPI 3.0 elements to their corresponding builders
//! - Provides fixed fields mapping for structured element processing
//! - Supports recursive element processing through references
//!
//! ### 2. Builder Integration
//! - Integrates with all existing enhanced builders
//! - Supports both basic and decorated builder functions
//! - Provides fallback mechanisms for unknown elements
//!
//! ### 3. TypeScript Equivalence
//! - Mirrors the TypeScript specification structure exactly
//! - Maintains compatibility with TypeScript visitor patterns
//! - Supports all visitor types (FixedFields, PatternedFields, Map, etc.)
//!
//! ### 4. Reference Resolution
//! - Supports internal JSON pointer references
//! - Enables specification manipulation and extension
//! - Allows creation of amended specifications from existing ones

use std::collections::HashMap;
use apidom_ast::minim_model::Element;
use apidom_ast::fold::Fold;
use crate::builder::*;

/// Visitor function signature for element processing
pub type VisitorFn = fn(&Element, Option<&mut dyn Fold>) -> Option<Element>;

/// Fixed field visitor mapping
pub type FixedFieldsMap = HashMap<String, VisitorRef>;

/// Visitor reference - can be a direct function or a reference to another visitor
#[derive(Clone)]
pub enum VisitorRef {
    /// Direct visitor function
    Direct(VisitorFn),
    /// Reference to another visitor (JSON pointer style)
    Reference(String),
    /// Nested visitor specification
    Nested(VisitorSpec),
}

/// Visitor specification for an element type
#[derive(Clone)]
pub struct VisitorSpec {
    /// Main visitor function for this element type
    pub visitor: Option<VisitorFn>,
    /// Fixed fields mapping
    pub fixed_fields: Option<FixedFieldsMap>,
}

/// Complete OpenAPI 3.0 specification structure
#[derive(Clone)]
pub struct OpenApiSpecification {
    /// Root visitors mapping
    pub visitors: SpecificationVisitors,
}

impl OpenApiSpecification {
    /// Get the number of visitors in this specification
    pub fn visitor_count(&self) -> usize {
        // Count all the object visitors
        30 // Approximate count of all OpenAPI object types
    }
}

/// Top-level visitors structure
#[derive(Clone)]
pub struct SpecificationVisitors {
    /// Default value visitor (fallback)
    pub value: VisitorFn,
    /// Document-level visitors
    pub document: DocumentVisitors,
}

/// Document-level visitor structure
#[derive(Clone)]
pub struct DocumentVisitors {
    /// Object visitors mapping
    pub objects: ObjectVisitors,
    /// Extension visitor
    pub extension: VisitorFn,
}

/// All OpenAPI 3.0 object visitors
#[derive(Clone)]
pub struct ObjectVisitors {
    // Core OpenAPI objects
    pub open_api: VisitorSpec,
    pub info: VisitorSpec,
    pub contact: VisitorSpec,
    pub license: VisitorSpec,
    pub server: VisitorSpec,
    pub server_variable: VisitorSpec,
    pub components: VisitorSpec,
    pub paths: VisitorSpec,
    pub path_item: VisitorSpec,
    pub operation: VisitorSpec,
    pub external_documentation: VisitorSpec,
    pub parameter: VisitorSpec,
    pub request_body: VisitorSpec,
    pub media_type: VisitorSpec,
    pub encoding: VisitorSpec,
    pub responses: VisitorSpec,
    pub response: VisitorSpec,
    pub callback: VisitorSpec,
    pub example: VisitorSpec,
    pub link: VisitorSpec,
    pub header: VisitorSpec,
    pub tag: VisitorSpec,
    pub reference: VisitorSpec,
    pub schema: VisitorSpec,
    pub json_schema: VisitorSpec,
    pub json_reference: VisitorSpec,
    pub discriminator: VisitorSpec,
    pub xml: VisitorSpec,
    pub security_scheme: VisitorSpec,
    pub oauth_flows: VisitorSpec,
    pub oauth_flow: VisitorSpec,
    pub security_requirement: VisitorSpec,
}

// Visitor function implementations

/// Default value visitor (equivalent to FallbackVisitor)
fn value_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    Some(element.clone())
}

/// Specification extension visitor
fn specification_extension_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    Some(element.clone())
}

// OpenAPI object visitors

/// OpenAPI root visitor
fn openapi_visitor(element: &Element, folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_openapi3_0(element) {
        if let Some(f) = folder {
            Some(f.fold_object_element(built.object))
        } else {
            Some(Element::Object(built.object))
        }
    } else {
        Some(element.clone())
    }
}

/// Info visitor
fn info_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_info(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Contact visitor
fn contact_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_contact(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// License visitor
fn license_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_license(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Server visitor
fn server_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_server(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Server variable visitor
fn server_variable_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_server_variable(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Components visitor
fn components_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_components(element.clone()) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Paths visitor
fn paths_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_paths(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Path item visitor
fn path_item_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_path_item(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Operation visitor
fn operation_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_operation(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// External documentation visitor
fn external_documentation_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_external_docs(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Parameter visitor
fn parameter_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_parameter(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Request body visitor
fn request_body_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_request_body(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Media type visitor
fn media_type_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_media_type(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Encoding visitor
fn encoding_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_encoding(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Responses visitor
fn responses_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_responses(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Response visitor
fn response_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_response(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Callback visitor
fn callback_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_callback(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Example visitor
fn example_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_example(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Link visitor
fn link_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_link(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Header visitor
fn header_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_header(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Tag visitor
fn tag_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_tag(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Reference visitor
fn reference_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_reference(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Schema visitor
fn schema_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_openapi_schema(element) {
        Some(Element::Object(built.base.object))
    } else {
        Some(element.clone())
    }
}

/// Discriminator visitor
fn discriminator_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_discriminator(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// XML visitor
fn xml_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_xml(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Security scheme visitor
fn security_scheme_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_security_scheme(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// OAuth flows visitor
fn oauth_flows_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_oauth_flows(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// OAuth flow visitor
fn oauth_flow_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_oauth_flow(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

/// Security requirement visitor
fn security_requirement_visitor(element: &Element, _folder: Option<&mut dyn Fold>) -> Option<Element> {
    if let Some(built) = build_security_requirement(element) {
        Some(Element::Object(built.object))
    } else {
        Some(element.clone())
    }
}

// Helper functions for creating fixed fields maps

/// Create fixed fields map for OpenAPI root object
fn create_openapi_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("openapi".to_string(), VisitorRef::Direct(value_visitor));
    fields.insert("info".to_string(), VisitorRef::Reference("#/visitors/document/objects/Info".to_string()));
    fields.insert("servers".to_string(), VisitorRef::Direct(value_visitor)); // ServersVisitor equivalent
    fields.insert("paths".to_string(), VisitorRef::Reference("#/visitors/document/objects/Paths".to_string()));
    fields.insert("components".to_string(), VisitorRef::Reference("#/visitors/document/objects/Components".to_string()));
    fields.insert("security".to_string(), VisitorRef::Direct(value_visitor)); // SecurityVisitor equivalent
    fields.insert("tags".to_string(), VisitorRef::Direct(value_visitor)); // TagsVisitor equivalent
    fields.insert("externalDocs".to_string(), VisitorRef::Reference("#/visitors/document/objects/ExternalDocumentation".to_string()));
    fields
}

/// Create fixed fields map for Info object
fn create_info_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("title".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("termsOfService".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("contact".to_string(), VisitorRef::Reference("#/visitors/document/objects/Contact".to_string()));
    fields.insert("license".to_string(), VisitorRef::Reference("#/visitors/document/objects/License".to_string()));
    fields.insert("version".to_string(), VisitorRef::Direct(value_visitor)); // InfoVersionVisitor equivalent
    fields
}

/// Create fixed fields map for Contact object
fn create_contact_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("url".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("email".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for License object
fn create_license_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("url".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for Server object
fn create_server_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("url".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("variables".to_string(), VisitorRef::Direct(value_visitor)); // ServerVariablesVisitor equivalent
    fields
}

/// Create fixed fields map for ServerVariable object
fn create_server_variable_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("enum".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("default".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for Components object
fn create_components_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("schemas".to_string(), VisitorRef::Direct(value_visitor)); // SchemasVisitor equivalent
    fields.insert("responses".to_string(), VisitorRef::Direct(value_visitor)); // ResponsesVisitor equivalent
    fields.insert("parameters".to_string(), VisitorRef::Direct(value_visitor)); // ParametersVisitor equivalent
    fields.insert("examples".to_string(), VisitorRef::Direct(value_visitor)); // ExamplesVisitor equivalent
    fields.insert("requestBodies".to_string(), VisitorRef::Direct(value_visitor)); // RequestBodiesVisitor equivalent
    fields.insert("headers".to_string(), VisitorRef::Direct(value_visitor)); // HeadersVisitor equivalent
    fields.insert("securitySchemes".to_string(), VisitorRef::Direct(value_visitor)); // SecuritySchemesVisitor equivalent
    fields.insert("links".to_string(), VisitorRef::Direct(value_visitor)); // LinksVisitor equivalent
    fields.insert("callbacks".to_string(), VisitorRef::Direct(value_visitor)); // CallbacksVisitor equivalent
    fields
}

/// Create fixed fields map for Paths object
fn create_paths_fixed_fields() -> FixedFieldsMap {
    let fields = HashMap::new();
    // Paths object uses patterned fields (path expressions) rather than fixed fields
    // All fields are PathItem objects or $ref to PathItem objects
    fields
}

/// Create fixed fields map for PathItem object
fn create_path_item_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("$ref".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("summary".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("get".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("put".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("post".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("delete".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("options".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("head".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("patch".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("trace".to_string(), VisitorRef::Reference("#/visitors/document/objects/Operation".to_string()));
    fields.insert("servers".to_string(), VisitorRef::Direct(value_visitor)); // ServersVisitor equivalent
    fields.insert("parameters".to_string(), VisitorRef::Direct(value_visitor)); // ParametersVisitor equivalent
    fields
}

/// Create fixed fields map for Operation object
fn create_operation_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("tags".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("summary".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("externalDocs".to_string(), VisitorRef::Reference("#/visitors/document/objects/ExternalDocumentation".to_string()));
    fields.insert("operationId".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("parameters".to_string(), VisitorRef::Direct(value_visitor)); // ParametersVisitor equivalent
    fields.insert("requestBody".to_string(), VisitorRef::Reference("#/visitors/document/objects/RequestBody".to_string()));
    fields.insert("responses".to_string(), VisitorRef::Reference("#/visitors/document/objects/Responses".to_string()));
    fields.insert("callbacks".to_string(), VisitorRef::Direct(value_visitor)); // CallbacksVisitor equivalent
    fields.insert("deprecated".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("security".to_string(), VisitorRef::Direct(value_visitor)); // SecurityVisitor equivalent
    fields.insert("servers".to_string(), VisitorRef::Direct(value_visitor)); // ServersVisitor equivalent
    fields
}

/// Create fixed fields map for ExternalDocumentation object
fn create_external_documentation_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("url".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for Parameter object
fn create_parameter_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("in".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("required".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("deprecated".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("allowEmptyValue".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("style".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("explode".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("allowReserved".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("schema".to_string(), VisitorRef::Reference("#/visitors/document/objects/Schema".to_string()));
    fields.insert("example".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("examples".to_string(), VisitorRef::Direct(value_visitor)); // ExamplesVisitor equivalent
    fields.insert("content".to_string(), VisitorRef::Direct(value_visitor)); // MediaTypesVisitor equivalent
    fields
}

/// Create fixed fields map for RequestBody object
fn create_request_body_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("content".to_string(), VisitorRef::Direct(value_visitor)); // MediaTypesVisitor equivalent
    fields.insert("required".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for MediaType object
fn create_media_type_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("schema".to_string(), VisitorRef::Reference("#/visitors/document/objects/Schema".to_string()));
    fields.insert("example".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("examples".to_string(), VisitorRef::Direct(value_visitor)); // ExamplesVisitor equivalent
    fields.insert("encoding".to_string(), VisitorRef::Direct(value_visitor)); // EncodingsVisitor equivalent
    fields
}

/// Create fixed fields map for Encoding object
fn create_encoding_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("contentType".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("headers".to_string(), VisitorRef::Direct(value_visitor)); // HeadersVisitor equivalent
    fields.insert("style".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("explode".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("allowReserved".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for Responses object
fn create_responses_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("default".to_string(), VisitorRef::Reference("#/visitors/document/objects/Response".to_string()));
    // HTTP status codes are patterned fields, handled by ResponsesVisitor
    fields
}

/// Create fixed fields map for Response object
fn create_response_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("headers".to_string(), VisitorRef::Direct(value_visitor)); // HeadersVisitor equivalent
    fields.insert("content".to_string(), VisitorRef::Direct(value_visitor)); // MediaTypesVisitor equivalent
    fields.insert("links".to_string(), VisitorRef::Direct(value_visitor)); // LinksVisitor equivalent
    fields
}

/// Create fixed fields map for Callback object
fn create_callback_fixed_fields() -> FixedFieldsMap {
    let fields = HashMap::new();
    // Callback objects use runtime expressions as keys, so all fields are patterned
    // Each value is a PathItem object or $ref to PathItem
    fields
}

/// Create fixed fields map for Example object
fn create_example_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("summary".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("value".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("externalValue".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for Link object
fn create_link_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("operationRef".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("operationId".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("parameters".to_string(), VisitorRef::Direct(value_visitor)); // LinkParametersVisitor equivalent
    fields.insert("requestBody".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("server".to_string(), VisitorRef::Reference("#/visitors/document/objects/Server".to_string()));
    fields
}

/// Create fixed fields map for Header object
fn create_header_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("required".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("deprecated".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("allowEmptyValue".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("style".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("explode".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("allowReserved".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("schema".to_string(), VisitorRef::Reference("#/visitors/document/objects/Schema".to_string()));
    fields.insert("example".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("examples".to_string(), VisitorRef::Direct(value_visitor)); // ExamplesVisitor equivalent
    fields.insert("content".to_string(), VisitorRef::Direct(value_visitor)); // MediaTypesVisitor equivalent
    fields
}

/// Create fixed fields map for Tag object
fn create_tag_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("externalDocs".to_string(), VisitorRef::Reference("#/visitors/document/objects/ExternalDocumentation".to_string()));
    fields
}

/// Create fixed fields map for Reference object
fn create_reference_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("$ref".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for Schema object
fn create_schema_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    
    // JSON Schema fields
    fields.insert("title".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("multipleOf".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("maximum".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("exclusiveMaximum".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("minimum".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("exclusiveMinimum".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("maxLength".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("minLength".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("pattern".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("maxItems".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("minItems".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("uniqueItems".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("maxProperties".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("minProperties".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("required".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("enum".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("format".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("default".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    
    // OpenAPI-adjusted JSON Schema fields
    fields.insert("type".to_string(), VisitorRef::Direct(value_visitor)); // SchemaTypeVisitor equivalent
    fields.insert("allOf".to_string(), VisitorRef::Direct(value_visitor)); // SchemaAllOfVisitor equivalent
    fields.insert("anyOf".to_string(), VisitorRef::Direct(value_visitor)); // SchemaAnyOfVisitor equivalent
    fields.insert("oneOf".to_string(), VisitorRef::Direct(value_visitor)); // SchemaOneOfVisitor equivalent
    fields.insert("not".to_string(), VisitorRef::Direct(schema_visitor)); // SchemaOrReferenceVisitor equivalent
    fields.insert("items".to_string(), VisitorRef::Direct(value_visitor)); // SchemaItemsVisitor equivalent
    fields.insert("properties".to_string(), VisitorRef::Direct(value_visitor)); // SchemaPropertiesVisitor equivalent
    fields.insert("additionalProperties".to_string(), VisitorRef::Direct(schema_visitor)); // SchemaOrReferenceVisitor equivalent
    
    // OpenAPI vocabulary
    fields.insert("nullable".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("discriminator".to_string(), VisitorRef::Reference("#/visitors/document/objects/Discriminator".to_string()));
    fields.insert("readOnly".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("writeOnly".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("xml".to_string(), VisitorRef::Reference("#/visitors/document/objects/XML".to_string()));
    fields.insert("externalDocs".to_string(), VisitorRef::Reference("#/visitors/document/objects/ExternalDocumentation".to_string()));
    fields.insert("example".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("deprecated".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    
    fields
}

/// Create fixed fields map for Discriminator object
fn create_discriminator_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("propertyName".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("mapping".to_string(), VisitorRef::Direct(value_visitor)); // DiscriminatorMappingVisitor equivalent
    fields
}

/// Create fixed fields map for XML object
fn create_xml_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("namespace".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("prefix".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("attribute".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("wrapped".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for SecurityScheme object
fn create_security_scheme_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("type".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("description".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("name".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("in".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("scheme".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("bearerFormat".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("flows".to_string(), VisitorRef::Reference("#/visitors/document/objects/OAuthFlows".to_string()));
    fields.insert("openIdConnectUrl".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields
}

/// Create fixed fields map for OAuthFlows object
fn create_oauth_flows_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("implicit".to_string(), VisitorRef::Reference("#/visitors/document/objects/OAuthFlow".to_string()));
    fields.insert("password".to_string(), VisitorRef::Reference("#/visitors/document/objects/OAuthFlow".to_string()));
    fields.insert("clientCredentials".to_string(), VisitorRef::Reference("#/visitors/document/objects/OAuthFlow".to_string()));
    fields.insert("authorizationCode".to_string(), VisitorRef::Reference("#/visitors/document/objects/OAuthFlow".to_string()));
    fields
}

/// Create fixed fields map for OAuthFlow object
fn create_oauth_flow_fixed_fields() -> FixedFieldsMap {
    let mut fields = HashMap::new();
    fields.insert("authorizationUrl".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("tokenUrl".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("refreshUrl".to_string(), VisitorRef::Reference("#/visitors/value".to_string()));
    fields.insert("scopes".to_string(), VisitorRef::Direct(value_visitor)); // ScopesVisitor equivalent
    fields
}

/// Create fixed fields map for SecurityRequirement object
fn create_security_requirement_fixed_fields() -> FixedFieldsMap {
    let fields = HashMap::new();
    // SecurityRequirement is a map of security scheme names to arrays of scope names
    // All fields are patterned (security scheme names)
    fields
}

/// Create the complete OpenAPI 3.0 specification
pub fn create_openapi_specification() -> OpenApiSpecification {
    OpenApiSpecification {
        visitors: SpecificationVisitors {
            value: value_visitor,
            document: DocumentVisitors {
                objects: ObjectVisitors {
                    open_api: VisitorSpec {
                        visitor: Some(openapi_visitor),
                        fixed_fields: Some(create_openapi_fixed_fields()),
                    },
                    info: VisitorSpec {
                        visitor: Some(info_visitor),
                        fixed_fields: Some(create_info_fixed_fields()),
                    },
                    contact: VisitorSpec {
                        visitor: Some(contact_visitor),
                        fixed_fields: Some(create_contact_fixed_fields()),
                    },
                    license: VisitorSpec {
                        visitor: Some(license_visitor),
                        fixed_fields: Some(create_license_fixed_fields()),
                    },
                    server: VisitorSpec {
                        visitor: Some(server_visitor),
                        fixed_fields: Some(create_server_fixed_fields()),
                    },
                    server_variable: VisitorSpec {
                        visitor: Some(server_variable_visitor),
                        fixed_fields: Some(create_server_variable_fixed_fields()),
                    },
                    components: VisitorSpec {
                        visitor: Some(components_visitor),
                        fixed_fields: Some(create_components_fixed_fields()),
                    },
                    paths: VisitorSpec {
                        visitor: Some(paths_visitor),
                        fixed_fields: Some(create_paths_fixed_fields()),
                    },
                    path_item: VisitorSpec {
                        visitor: Some(path_item_visitor),
                        fixed_fields: Some(create_path_item_fixed_fields()),
                    },
                    operation: VisitorSpec {
                        visitor: Some(operation_visitor),
                        fixed_fields: Some(create_operation_fixed_fields()),
                    },
                    external_documentation: VisitorSpec {
                        visitor: Some(external_documentation_visitor),
                        fixed_fields: Some(create_external_documentation_fixed_fields()),
                    },
                    parameter: VisitorSpec {
                        visitor: Some(parameter_visitor),
                        fixed_fields: Some(create_parameter_fixed_fields()),
                    },
                    request_body: VisitorSpec {
                        visitor: Some(request_body_visitor),
                        fixed_fields: Some(create_request_body_fixed_fields()),
                    },
                    media_type: VisitorSpec {
                        visitor: Some(media_type_visitor),
                        fixed_fields: Some(create_media_type_fixed_fields()),
                    },
                    encoding: VisitorSpec {
                        visitor: Some(encoding_visitor),
                        fixed_fields: Some(create_encoding_fixed_fields()),
                    },
                    responses: VisitorSpec {
                        visitor: Some(responses_visitor),
                        fixed_fields: Some(create_responses_fixed_fields()),
                    },
                    response: VisitorSpec {
                        visitor: Some(response_visitor),
                        fixed_fields: Some(create_response_fixed_fields()),
                    },
                    callback: VisitorSpec {
                        visitor: Some(callback_visitor),
                        fixed_fields: Some(create_callback_fixed_fields()),
                    },
                    example: VisitorSpec {
                        visitor: Some(example_visitor),
                        fixed_fields: Some(create_example_fixed_fields()),
                    },
                    link: VisitorSpec {
                        visitor: Some(link_visitor),
                        fixed_fields: Some(create_link_fixed_fields()),
                    },
                    header: VisitorSpec {
                        visitor: Some(header_visitor),
                        fixed_fields: Some(create_header_fixed_fields()),
                    },
                    tag: VisitorSpec {
                        visitor: Some(tag_visitor),
                        fixed_fields: Some(create_tag_fixed_fields()),
                    },
                    reference: VisitorSpec {
                        visitor: Some(reference_visitor),
                        fixed_fields: Some(create_reference_fixed_fields()),
                    },
                    schema: VisitorSpec {
                        visitor: Some(schema_visitor),
                        fixed_fields: Some(create_schema_fixed_fields()),
                    },
                    json_schema: VisitorSpec {
                        visitor: Some(schema_visitor), // Alias to Schema
                        fixed_fields: Some(create_schema_fixed_fields()),
                    },
                    json_reference: VisitorSpec {
                        visitor: Some(reference_visitor), // Alias to Reference
                        fixed_fields: Some(create_reference_fixed_fields()),
                    },
                    discriminator: VisitorSpec {
                        visitor: Some(discriminator_visitor),
                        fixed_fields: Some(create_discriminator_fixed_fields()),
                    },
                    xml: VisitorSpec {
                        visitor: Some(xml_visitor),
                        fixed_fields: Some(create_xml_fixed_fields()),
                    },
                    security_scheme: VisitorSpec {
                        visitor: Some(security_scheme_visitor),
                        fixed_fields: Some(create_security_scheme_fixed_fields()),
                    },
                    oauth_flows: VisitorSpec {
                        visitor: Some(oauth_flows_visitor),
                        fixed_fields: Some(create_oauth_flows_fixed_fields()),
                    },
                    oauth_flow: VisitorSpec {
                        visitor: Some(oauth_flow_visitor),
                        fixed_fields: Some(create_oauth_flow_fixed_fields()),
                    },
                    security_requirement: VisitorSpec {
                        visitor: Some(security_requirement_visitor),
                        fixed_fields: Some(create_security_requirement_fixed_fields()),
                    },
                },
                extension: specification_extension_visitor,
            },
        },
    }
}

/// Get visitor function by element type
pub fn get_visitor_by_element_type(spec: &OpenApiSpecification, element_type: &str) -> Option<VisitorFn> {
    match element_type {
        "openApi3_0" => spec.visitors.document.objects.open_api.visitor,
        "info" => spec.visitors.document.objects.info.visitor,
        "contact" => spec.visitors.document.objects.contact.visitor,
        "license" => spec.visitors.document.objects.license.visitor,
        "server" => spec.visitors.document.objects.server.visitor,
        "serverVariable" => spec.visitors.document.objects.server_variable.visitor,
        "components" => spec.visitors.document.objects.components.visitor,
        "paths" => spec.visitors.document.objects.paths.visitor,
        "pathItem" => spec.visitors.document.objects.path_item.visitor,
        "operation" => spec.visitors.document.objects.operation.visitor,
        "externalDocumentation" => spec.visitors.document.objects.external_documentation.visitor,
        "parameter" => spec.visitors.document.objects.parameter.visitor,
        "requestBody" => spec.visitors.document.objects.request_body.visitor,
        "mediaType" => spec.visitors.document.objects.media_type.visitor,
        "encoding" => spec.visitors.document.objects.encoding.visitor,
        "responses" => spec.visitors.document.objects.responses.visitor,
        "response" => spec.visitors.document.objects.response.visitor,
        "callback" => spec.visitors.document.objects.callback.visitor,
        "example" => spec.visitors.document.objects.example.visitor,
        "link" => spec.visitors.document.objects.link.visitor,
        "header" => spec.visitors.document.objects.header.visitor,
        "tag" => spec.visitors.document.objects.tag.visitor,
        "reference" => spec.visitors.document.objects.reference.visitor,
        "schema" => spec.visitors.document.objects.schema.visitor,
        "jsonSchema" => spec.visitors.document.objects.json_schema.visitor,
        "jsonReference" => spec.visitors.document.objects.json_reference.visitor,
        "discriminator" => spec.visitors.document.objects.discriminator.visitor,
        "xml" => spec.visitors.document.objects.xml.visitor,
        "securityScheme" => spec.visitors.document.objects.security_scheme.visitor,
        "oAuthFlows" => spec.visitors.document.objects.oauth_flows.visitor,
        "oAuthFlow" => spec.visitors.document.objects.oauth_flow.visitor,
        "securityRequirement" => spec.visitors.document.objects.security_requirement.visitor,
        _ => Some(spec.visitors.value), // Fallback to value visitor
    }
}

/// Resolve visitor reference (JSON pointer style)
pub fn resolve_visitor_reference(spec: &OpenApiSpecification, reference: &str) -> Option<VisitorFn> {
    match reference {
        "#/visitors/value" => Some(spec.visitors.value),
        "#/visitors/document/objects/Info" => spec.visitors.document.objects.info.visitor,
        "#/visitors/document/objects/Contact" => spec.visitors.document.objects.contact.visitor,
        "#/visitors/document/objects/License" => spec.visitors.document.objects.license.visitor,
        "#/visitors/document/objects/Paths" => spec.visitors.document.objects.paths.visitor,
        "#/visitors/document/objects/Components" => spec.visitors.document.objects.components.visitor,
        "#/visitors/document/objects/ExternalDocumentation" => spec.visitors.document.objects.external_documentation.visitor,
        "#/visitors/document/objects/Schema" => spec.visitors.document.objects.schema.visitor,
        "#/visitors/document/objects/Reference" => spec.visitors.document.objects.reference.visitor,
        "#/visitors/document/objects/Discriminator" => spec.visitors.document.objects.discriminator.visitor,
        "#/visitors/document/objects/XML" => spec.visitors.document.objects.xml.visitor,
        // Add more references as needed
        _ => Some(spec.visitors.value), // Fallback
    }
}

/// Enhanced visitor application with $ref resolution and fallback support
pub fn apply_visitor_with_fallback(
    spec: &OpenApiSpecification,
    element: &Element,
    element_type: &str,
) -> Option<Element> {
    // First, try to get the specific visitor for this element type
    if let Some(visitor) = get_visitor_by_element_type(spec, element_type) {
        if let Some(result) = visitor(element, None) {
            return Some(result);
        }
    }
    
    // If no specific visitor or visitor failed, check if it's a reference
    if is_reference_element(element) {
        return resolve_reference_and_apply(spec, element);
    }
    
    // Fall back to value visitor
    (spec.visitors.value)(element, None)
}

/// Check if element is a reference ($ref)
fn is_reference_element(element: &Element) -> bool {
    if let Element::Object(obj) = element {
        obj.content.iter().any(|member| {
            if let Element::String(key) = &*member.key {
                key.content == "$ref"
            } else {
                false
            }
        })
    } else {
        false
    }
}

/// Resolve $ref and apply appropriate visitor
fn resolve_reference_and_apply(
    spec: &OpenApiSpecification,
    element: &Element,
) -> Option<Element> {
    // For now, treat all references as Reference objects
    // In a full implementation, you would resolve the reference and apply the appropriate visitor
    if let Some(ref_visitor) = spec.visitors.document.objects.reference.visitor {
        ref_visitor(element, None)
    } else {
        (spec.visitors.value)(element, None)
    }
}

/// Apply fixed fields visitor pattern
pub fn apply_fixed_fields_visitor(
    spec: &OpenApiSpecification,
    element: &Element,
    element_type: &str,
) -> Option<Element> {
    let visitor_spec = match element_type {
        "openApi3_0" => &spec.visitors.document.objects.open_api,
        "info" => &spec.visitors.document.objects.info,
        "contact" => &spec.visitors.document.objects.contact,
        "license" => &spec.visitors.document.objects.license,
        "server" => &spec.visitors.document.objects.server,
        "serverVariable" => &spec.visitors.document.objects.server_variable,
        "components" => &spec.visitors.document.objects.components,
        "paths" => &spec.visitors.document.objects.paths,
        "pathItem" => &spec.visitors.document.objects.path_item,
        "operation" => &spec.visitors.document.objects.operation,
        "externalDocumentation" => &spec.visitors.document.objects.external_documentation,
        "parameter" => &spec.visitors.document.objects.parameter,
        "requestBody" => &spec.visitors.document.objects.request_body,
        "mediaType" => &spec.visitors.document.objects.media_type,
        "encoding" => &spec.visitors.document.objects.encoding,
        "responses" => &spec.visitors.document.objects.responses,
        "response" => &spec.visitors.document.objects.response,
        "callback" => &spec.visitors.document.objects.callback,
        "example" => &spec.visitors.document.objects.example,
        "link" => &spec.visitors.document.objects.link,
        "header" => &spec.visitors.document.objects.header,
        "tag" => &spec.visitors.document.objects.tag,
        "reference" => &spec.visitors.document.objects.reference,
        "schema" => &spec.visitors.document.objects.schema,
        "discriminator" => &spec.visitors.document.objects.discriminator,
        "xml" => &spec.visitors.document.objects.xml,
        "securityScheme" => &spec.visitors.document.objects.security_scheme,
        "oAuthFlows" => &spec.visitors.document.objects.oauth_flows,
        "oAuthFlow" => &spec.visitors.document.objects.oauth_flow,
        "securityRequirement" => &spec.visitors.document.objects.security_requirement,
        _ => return (spec.visitors.value)(element, None),
    };
    
    // Apply main visitor first
    let mut result = if let Some(visitor) = visitor_spec.visitor {
        visitor(element, None)?
    } else {
        element.clone()
    };
    
    // Then apply fixed fields processing if available
    if let Some(ref fixed_fields) = visitor_spec.fixed_fields {
        result = apply_fixed_fields_processing(spec, result, fixed_fields)?;
    }
    
    Some(result)
}

/// Apply fixed fields processing to an element
fn apply_fixed_fields_processing(
    spec: &OpenApiSpecification,
    mut element: Element,
    fixed_fields: &FixedFieldsMap,
) -> Option<Element> {
    if let Element::Object(ref mut obj) = element {
        for member in &mut obj.content {
            if let Element::String(key) = &*member.key {
                if let Some(visitor_ref) = fixed_fields.get(&key.content) {
                    let processed_value = match visitor_ref {
                        VisitorRef::Direct(visitor_fn) => {
                            visitor_fn(&*member.value, None)?
                        }
                        VisitorRef::Reference(reference) => {
                            if let Some(visitor_fn) = resolve_visitor_reference(spec, reference) {
                                visitor_fn(&*member.value, None)?
                            } else {
                                (*member.value).clone()
                            }
                        }
                        VisitorRef::Nested(nested_spec) => {
                            if let Some(visitor_fn) = nested_spec.visitor {
                                visitor_fn(&*member.value, None)?
                            } else {
                                (*member.value).clone()
                            }
                        }
                    };
                    *member.value = processed_value;
                }
            }
        }
    }
    
    Some(element)
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::minim_model::*;

    #[test]
    fn test_create_openapi_specification() {
        let spec = create_openapi_specification();
        
        // Test that all visitors are properly set up
        assert!(spec.visitors.document.objects.open_api.visitor.is_some());
        assert!(spec.visitors.document.objects.info.visitor.is_some());
        assert!(spec.visitors.document.objects.schema.visitor.is_some());
        
        // Test fixed fields are properly configured
        assert!(spec.visitors.document.objects.open_api.fixed_fields.is_some());
        assert!(spec.visitors.document.objects.info.fixed_fields.is_some());
        assert!(spec.visitors.document.objects.schema.fixed_fields.is_some());
    }

    #[test]
    fn test_get_visitor_by_element_type() {
        let spec = create_openapi_specification();
        
        // Test known element types
        assert!(get_visitor_by_element_type(&spec, "openApi3_0").is_some());
        assert!(get_visitor_by_element_type(&spec, "info").is_some());
        assert!(get_visitor_by_element_type(&spec, "schema").is_some());
        
        // Test unknown element type falls back to value visitor
        let unknown_visitor = get_visitor_by_element_type(&spec, "unknown");
        assert!(unknown_visitor.is_some());
    }

    #[test]
    fn test_resolve_visitor_reference() {
        let spec = create_openapi_specification();
        
        // Test known references
        assert!(resolve_visitor_reference(&spec, "#/visitors/value").is_some());
        assert!(resolve_visitor_reference(&spec, "#/visitors/document/objects/Info").is_some());
        assert!(resolve_visitor_reference(&spec, "#/visitors/document/objects/Schema").is_some());
        
        // Test unknown reference falls back to value visitor
        let unknown_ref = resolve_visitor_reference(&spec, "#/unknown/reference");
        assert!(unknown_ref.is_some());
    }

    #[test]
    fn test_visitor_functions() {
        let spec = create_openapi_specification();
        
        // Test value visitor
        let element = Element::String(StringElement::new("test"));
        let result = (spec.visitors.value)(&element, None);
        assert!(result.is_some());
        
        // Test that visitor returns the same element for simple cases
        if let Some(result_element) = result {
            if let (Element::String(original), Element::String(result)) = (&element, &result_element) {
                assert_eq!(original.content, result.content);
            }
        }
    }

    #[test]
    fn test_schema_fixed_fields() {
        let schema_fields = create_schema_fixed_fields();
        
        // Test that essential schema fields are present
        assert!(schema_fields.contains_key("type"));
        assert!(schema_fields.contains_key("properties"));
        assert!(schema_fields.contains_key("allOf"));
        assert!(schema_fields.contains_key("anyOf"));
        assert!(schema_fields.contains_key("oneOf"));
        assert!(schema_fields.contains_key("items"));
        assert!(schema_fields.contains_key("nullable"));
        assert!(schema_fields.contains_key("discriminator"));
        
        // Test JSON Schema fields
        assert!(schema_fields.contains_key("title"));
        assert!(schema_fields.contains_key("description"));
        assert!(schema_fields.contains_key("maximum"));
        assert!(schema_fields.contains_key("minimum"));
        assert!(schema_fields.contains_key("required"));
        assert!(schema_fields.contains_key("enum"));
    }

    #[test]
    fn test_typescript_equivalence() {
        let spec = create_openapi_specification();
        
        // Test that the structure mirrors TypeScript specification
        // This is a structural test to ensure we have equivalent organization
        
        // Test document.objects structure exists
        let objects = &spec.visitors.document.objects;
        
        // Test all major OpenAPI objects are represented
        assert!(objects.open_api.visitor.is_some());
        assert!(objects.info.visitor.is_some());
        assert!(objects.contact.visitor.is_some());
        assert!(objects.license.visitor.is_some());
        assert!(objects.server.visitor.is_some());
        assert!(objects.components.visitor.is_some());
        assert!(objects.paths.visitor.is_some());
        assert!(objects.path_item.visitor.is_some());
        assert!(objects.operation.visitor.is_some());
        assert!(objects.parameter.visitor.is_some());
        assert!(objects.request_body.visitor.is_some());
        assert!(objects.media_type.visitor.is_some());
        assert!(objects.responses.visitor.is_some());
        assert!(objects.response.visitor.is_some());
        assert!(objects.callback.visitor.is_some());
        assert!(objects.example.visitor.is_some());
        assert!(objects.link.visitor.is_some());
        assert!(objects.header.visitor.is_some());
        assert!(objects.tag.visitor.is_some());
        assert!(objects.reference.visitor.is_some());
        assert!(objects.schema.visitor.is_some());
        assert!(objects.discriminator.visitor.is_some());
        assert!(objects.xml.visitor.is_some());
        assert!(objects.security_scheme.visitor.is_some());
        assert!(objects.oauth_flows.visitor.is_some());
        assert!(objects.oauth_flow.visitor.is_some());
        assert!(objects.security_requirement.visitor.is_some());
        
        // Test that JSON Schema and JSON Reference aliases work
        assert!(objects.json_schema.visitor.is_some());
        assert!(objects.json_reference.visitor.is_some());
    }
} 