use apidom_ast::*;
use super::contact::ContactElement;
use super::license::LicenseElement;

/// OpenAPI `Info` Element
#[derive(Debug, Clone)]
pub struct InfoElement {
    pub object: ObjectElement,
}

impl InfoElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("info");
        obj.add_class("info");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("info");
        content.add_class("info");
        Self { object: content }
    }

    pub fn title(&self) -> Option<&StringElement> {
        self.object.get("title").and_then(Element::as_string)
    }

    pub fn set_title(&mut self, val: StringElement) {
        self.object.set("title", Element::String(val));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.object.set("description", Element::String(val));
    }

    pub fn terms_of_service(&self) -> Option<&StringElement> {
        self.object.get("termsOfService").and_then(Element::as_string)
    }

    pub fn set_terms_of_service(&mut self, val: StringElement) {
        self.object.set("termsOfService", Element::String(val));
    }

    pub fn contact(&self) -> Option<&ObjectElement> {
        self.object.get("contact").and_then(Element::as_object)
    }

    pub fn set_contact(&mut self, val: ContactElement) {
        self.object.set("contact", Element::Object(val.object));
    }

    pub fn license(&self) -> Option<&ObjectElement> {
        self.object.get("license").and_then(Element::as_object)
    }

    pub fn set_license(&mut self, val: LicenseElement) {
        self.object.set("license", Element::Object(val.object));
    }

    pub fn version(&self) -> Option<&StringElement> {
        self.object.get("version").and_then(Element::as_string)
    }

    pub fn set_version(&mut self, val: StringElement) {
        self.object.set("version", Element::String(val));
    }
}