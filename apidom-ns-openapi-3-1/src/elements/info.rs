use apidom_ast::minim_model::*;

/// Represents the OpenAPI `info` object.
#[derive(Debug, Clone)]
pub struct InfoElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub classes: ArrayElement,
    pub content: Vec<(String, Element)>,
}

impl Default for InfoElement {
    fn default() -> Self {
        Self {
            element: "info".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            classes: ArrayElement::from_strings(&["info"]),
            content: vec![],
        }
    }
}

impl InfoElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&Element> {
        self.content.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn set(&mut self, key: &str, value: Element) {
        if let Some((_, v)) = self.content.iter_mut().find(|(k, _)| k == key) {
            *v = value;
        } else {
            self.content.push((key.to_string(), value));
        }
    }

    // === Accessors ===

    pub fn get_title(&self) -> Option<&Element> {
        self.get("title")
    }

    pub fn set_title(&mut self, val: StringElement) {
        self.set("title", Element::String(val));
    }

    pub fn get_description(&self) -> Option<&Element> {
        self.get("description")
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.set("description", Element::String(val));
    }

    pub fn get_terms_of_service(&self) -> Option<&Element> {
        self.get("termsOfService")
    }

    pub fn set_terms_of_service(&mut self, val: StringElement) {
        self.set("termsOfService", Element::String(val));
    }

    pub fn get_contact(&self) -> Option<&Element> {
        self.get("contact")
    }

    pub fn set_contact(&mut self, val: Element) {
        self.set("contact", val);
    }

    pub fn get_license(&self) -> Option<&Element> {
        self.get("license")
    }

    pub fn set_license(&mut self, val: Element) {
        self.set("license", val);
    }

    pub fn get_version(&self) -> Option<&Element> {
        self.get("version")
    }

    pub fn set_version(&mut self, val: StringElement) {
        self.set("version", Element::String(val));
    }

    pub fn get_summary(&self) -> Option<&Element> {
        self.get("summary")
    }

    pub fn set_summary(&mut self, val: StringElement) {
        self.set("summary", Element::String(val));
    }
}