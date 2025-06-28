use apidom_ast::minim_model::*;

/// OpenAPI Encoding Element
#[derive(Debug, Clone)]
pub struct EncodingElement {
    pub object: ObjectElement,
}

impl EncodingElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("encoding");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("encoding");
        Self { object: content }
    }

    pub fn content_type(&self) -> Option<&StringElement> {
        self.object.get("contentType").and_then(Element::as_string)
    }

    pub fn set_content_type(&mut self, value: StringElement) {
        self.object.set("contentType", Element::String(value));
    }

    pub fn headers(&self) -> Option<&ObjectElement> {
        self.object.get("headers").and_then(Element::as_object)
    }

    pub fn set_headers(&mut self, value: ObjectElement) {
        self.object.set("headers", Element::Object(value));
    }

    pub fn style(&self) -> Option<&StringElement> {
        self.object.get("style").and_then(Element::as_string)
    }

    pub fn set_style(&mut self, value: StringElement) {
        self.object.set("style", Element::String(value));
    }

    pub fn explode(&self) -> Option<&BooleanElement> {
        self.object.get("explode").and_then(Element::as_boolean)
    }

    pub fn set_explode(&mut self, value: BooleanElement) {
        self.object.set("explode", Element::Boolean(value));
    }

    pub fn allowed_reserved(&self) -> Option<&BooleanElement> {
        self.object.get("allowedReserved").and_then(Element::as_boolean)
    }

    pub fn set_allowed_reserved(&mut self, value: BooleanElement) {
        self.object.set("allowedReserved", Element::Boolean(value));
    }
}