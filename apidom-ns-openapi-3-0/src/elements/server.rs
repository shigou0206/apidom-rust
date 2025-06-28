use apidom_ast::minim_model::*;

/// OpenAPI Server Element
#[derive(Debug, Clone)]
pub struct ServerElement {
    pub object: ObjectElement,
}

impl ServerElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("server");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("server");
        Self { object: content }
    }

    pub fn url(&self) -> Option<&StringElement> {
        self.object.get("url").and_then(Element::as_string)
    }

    pub fn set_url(&mut self, val: StringElement) {
        self.object.set("url", Element::String(val));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.object.set("description", Element::String(val));
    }

    pub fn variables(&self) -> Option<&ObjectElement> {
        self.object.get("variables").and_then(Element::as_object)
    }

    pub fn set_variables(&mut self, val: ObjectElement) {
        self.object.set("variables", Element::Object(val));
    }
}