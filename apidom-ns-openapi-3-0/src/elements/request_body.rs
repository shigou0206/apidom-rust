use apidom_ast::*;

/// OpenAPI RequestBody Element
#[derive(Debug, Clone)]
pub struct RequestBodyElement {
    pub object: ObjectElement,
}

impl RequestBodyElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("requestBody");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("requestBody");
        Self { object: content }
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, value: StringElement) {
        self.object.set("description", Element::String(value));
    }

    pub fn content_prop(&self) -> Option<&ObjectElement> {
        self.object.get("content").and_then(Element::as_object)
    }

    pub fn set_content_prop(&mut self, value: ObjectElement) {
        self.object.set("content", Element::Object(value));
    }

    pub fn required(&self) -> bool {
        self.object
            .get("required")
            .and_then(Element::as_boolean)
            .map(|b| b.content)
            .unwrap_or(false)
    }

    pub fn set_required(&mut self, value: bool) {
        self.object
            .set("required", Element::Boolean(BooleanElement::new(value)));
    }
}