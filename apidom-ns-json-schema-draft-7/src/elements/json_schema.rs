use apidom_ast::minim_model::*;
use apidom_ns_json_schema_draft_6::elements::json_schema::JSONSchemaDraft6Element;

/// JSON Schema Draft-7 Element
#[derive(Debug, Clone)]
pub struct JSONSchemaDraft7Element {
    pub base: JSONSchemaDraft6Element,
}

impl JSONSchemaDraft7Element {
    pub fn new() -> Self {
        let mut base = JSONSchemaDraft6Element::new();
        base.base.object.set_element_type("JSONSchemaDraft7");
        Self { base }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = JSONSchemaDraft6Element::with_content(content);
        base.base.object.set_element_type("JSONSchemaDraft7");
        Self { base }
    }

    // ------------------
    // Inherited fields
    // ------------------
    pub fn title(&self) -> Option<&StringElement> {
        self.base.title()
    }

    pub fn set_title(&mut self, val: StringElement) {
        self.base.set_title(val);
    }

    pub fn type_(&self) -> Option<&Element> {
        self.base.type_()
    }

    pub fn set_type(&mut self, val: Element) {
        self.base.set_type(val);
    }

    // ------------------
    // Draft-7 New Fields
    // ------------------

    /// `$comment`
    pub fn comment(&self) -> Option<&StringElement> {
        self.base.base.object.get("$comment").and_then(Element::as_string)
    }

    pub fn set_comment(&mut self, val: StringElement) {
        self.base.base.object.set("$comment", Element::String(val));
    }

    /// `if` subschema
    pub fn if_schema(&self) -> Option<&Element> {
        self.base.base.object.get("if")
    }

    pub fn set_if_schema(&mut self, val: Element) {
        self.base.base.object.set("if", val);
    }

    /// `then` subschema
    pub fn then_schema(&self) -> Option<&Element> {
        self.base.base.object.get("then")
    }

    pub fn set_then_schema(&mut self, val: Element) {
        self.base.base.object.set("then", val);
    }

    /// `else` subschema
    pub fn else_schema(&self) -> Option<&Element> {
        self.base.base.object.get("else")
    }

    pub fn set_else_schema(&mut self, val: Element) {
        self.base.base.object.set("else", val);
    }

    /// `contentEncoding`
    pub fn content_encoding(&self) -> Option<&StringElement> {
        self.base.base.object.get("contentEncoding").and_then(Element::as_string)
    }

    pub fn set_content_encoding(&mut self, val: StringElement) {
        self.base.base.object.set("contentEncoding", Element::String(val));
    }

    /// `contentMediaType`
    pub fn content_media_type(&self) -> Option<&StringElement> {
        self.base.base.object.get("contentMediaType").and_then(Element::as_string)
    }

    pub fn set_content_media_type(&mut self, val: StringElement) {
        self.base.base.object.set("contentMediaType", Element::String(val));
    }

    /// `writeOnly`
    pub fn write_only(&self) -> Option<&BooleanElement> {
        self.base.base.object.get("writeOnly").and_then(Element::as_boolean)
    }

    pub fn set_write_only(&mut self, val: BooleanElement) {
        self.base.base.object.set("writeOnly", Element::Boolean(val));
    }

    /// Deprecated `media`
    pub fn media(&self) -> Result<&Element, UnsupportedFieldError> {
        Err(UnsupportedFieldError("`media` has been removed; use contentMediaType/contentEncoding".into()))
    }
}

#[derive(Debug, Clone)]
pub struct UnsupportedFieldError(pub String);

impl std::fmt::Display for UnsupportedFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported field: {}", self.0)
    }
}

impl std::error::Error for UnsupportedFieldError {}