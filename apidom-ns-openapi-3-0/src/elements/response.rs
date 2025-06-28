use apidom_ast::minim_model::*;

/// OpenAPI Response Element
#[derive(Debug, Clone)]
pub struct ResponseElement {
    pub object: ObjectElement,
}

impl ResponseElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("response");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("response");
        Self { object: content }
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, value: StringElement) {
        self.object.set("description", Element::String(value));
    }

    pub fn headers(&self) -> Option<&ObjectElement> {
        self.object.get("headers").and_then(Element::as_object)
    }

    pub fn set_headers(&mut self, value: ObjectElement) {
        self.object.set("headers", Element::Object(value));
    }

    pub fn content_prop(&self) -> Option<&ObjectElement> {
        self.object.get("content").and_then(Element::as_object)
    }

    pub fn set_content_prop(&mut self, value: ObjectElement) {
        self.object.set("content", Element::Object(value));
    }

    pub fn links(&self) -> Option<&ObjectElement> {
        self.object.get("links").and_then(Element::as_object)
    }

    pub fn set_links(&mut self, value: ObjectElement) {
        self.object.set("links", Element::Object(value));
    }
}