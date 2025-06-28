use apidom_ast::minim_model::{
    Element, StringElement, MetaElement, AttributesElement,
};

/// Represents an `Example` object in OpenAPI 3.1
#[derive(Debug, Clone)]
pub struct ExampleElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<(String, Element)>,
}

impl Default for ExampleElement {
    fn default() -> Self {
        Self {
            element: "example".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: Vec::new(),
        }
    }
}

impl ExampleElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_summary(&self) -> Option<&Element> {
        self.get("summary")
    }

    pub fn set_summary(&mut self, val: StringElement) {
        self.set("summary", Element::String(val));
    }

    pub fn get_description(&self) -> Option<&Element> {
        self.get("description")
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.set("description", Element::String(val));
    }

    pub fn get_value(&self) -> Option<&Element> {
        self.get("value")
    }

    pub fn set_value(&mut self, val: Element) {
        self.set("value", val);
    }

    pub fn get_external_value(&self) -> Option<&Element> {
        self.get("externalValue")
    }

    pub fn set_external_value(&mut self, val: StringElement) {
        self.set("externalValue", Element::String(val));
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