use apidom_ast::{
    Element, StringElement, MetaElement, AttributesElement,
};

/// Represents an `externalDocumentation` object in OpenAPI 3.1
#[derive(Debug, Clone)]
pub struct ExternalDocumentationElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<(String, Element)>,
}

impl Default for ExternalDocumentationElement {
    fn default() -> Self {
        Self {
            element: "externalDocumentation".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: Vec::new(),
        }
    }
}

impl ExternalDocumentationElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_description(&self) -> Option<&Element> {
        self.get("description")
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.set("description", Element::String(val));
    }

    pub fn get_url(&self) -> Option<&Element> {
        self.get("url")
    }

    pub fn set_url(&mut self, val: StringElement) {
        self.set("url", Element::String(val));
    }

    fn get(&self, key: &str) -> Option<&Element> {
        self.content.iter().find_map(|(k, v)| {
            if k == key {
                Some(v)
            } else {
                None
            }
        })
    }

    fn set(&mut self, key: &str, value: Element) {
        if let Some((_, existing)) = self.content.iter_mut().find(|(k, _)| k == key) {
            *existing = value;
        } else {
            self.content.push((key.to_string(), value));
        }
    }
}