use apidom_ast::minim_model::{
    Element, ObjectElement, StringElement, BooleanElement, MetaElement, AttributesElement,
};

/// Represents an `Encoding` object in OpenAPI 3.1
#[derive(Debug, Clone)]
pub struct EncodingElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<(String, Element)>,
}

impl Default for EncodingElement {
    fn default() -> Self {
        Self {
            element: "encoding".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: Vec::new(),
        }
    }
}

impl EncodingElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_content_type(&self) -> Option<&Element> {
        self.get("contentType")
    }

    pub fn set_content_type(&mut self, val: StringElement) {
        self.set("contentType", Element::String(val));
    }

    pub fn get_headers(&self) -> Option<&Element> {
        self.get("headers")
    }

    pub fn set_headers(&mut self, val: ObjectElement) {
        self.set("headers", Element::Object(val));
    }

    pub fn get_style(&self) -> Option<&Element> {
        self.get("style")
    }

    pub fn set_style(&mut self, val: StringElement) {
        self.set("style", Element::String(val));
    }

    pub fn get_explode(&self) -> Option<&Element> {
        self.get("explode")
    }

    pub fn set_explode(&mut self, val: BooleanElement) {
        self.set("explode", Element::Boolean(val));
    }

    pub fn get_allowed_reserved(&self) -> Option<&Element> {
        self.get("allowedReserved")
    }

    pub fn set_allowed_reserved(&mut self, val: BooleanElement) {
        self.set("allowedReserved", Element::Boolean(val));
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