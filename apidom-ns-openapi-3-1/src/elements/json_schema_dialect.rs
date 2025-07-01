use apidom_ast::{MetaElement, AttributesElement, StringElement, Element};

/// Represents the `jsonSchemaDialect` string element in OpenAPI 3.1.
#[derive(Debug, Clone)]
pub struct JsonSchemaDialectElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: String,
}

impl JsonSchemaDialectElement {
    /// Create a new instance with optional content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            element: "jsonSchemaDialect".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: content.into(),
        }
    }

    /// Default dialect value as defined in OpenAPI 3.1 spec
    pub fn default_value() -> Self {
        Self::new("https://spec.openapis.org/oas/3.1/dialect/base")
    }

    /// Convert into generic Element
    pub fn into_element(self) -> Element {
        Element::String(StringElement {
            element: self.element,
            meta: self.meta,
            attributes: self.attributes,
            content: self.content,
        })
    }
}