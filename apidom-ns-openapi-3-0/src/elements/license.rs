use apidom_ast::minim_model::*;

/// OpenAPI `License` Element
#[derive(Debug, Clone)]
pub struct LicenseElement {
    pub object: ObjectElement,
}

impl LicenseElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("license");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("license");
        Self { object: content }
    }

    pub fn name(&self) -> Option<&StringElement> {
        self.object.get("name").and_then(Element::as_string)
    }

    pub fn set_name(&mut self, value: StringElement) {
        self.object.set("name", Element::String(value));
    }

    pub fn url(&self) -> Option<&StringElement> {
        self.object.get("url").and_then(Element::as_string)
    }

    pub fn set_url(&mut self, value: StringElement) {
        self.object.set("url", Element::String(value));
    }
}