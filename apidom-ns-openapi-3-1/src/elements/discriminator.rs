use apidom_ast::minim_model::{
    Element, ObjectElement, StringElement, MetaElement, AttributesElement,
};

/// Represents a Discriminator object in OpenAPI 3.1
#[derive(Debug, Clone)]
pub struct DiscriminatorElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<(String, Element)>,
}

impl Default for DiscriminatorElement {
    fn default() -> Self {
        Self {
            element: "discriminator".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: Vec::new(),
        }
    }
}

impl DiscriminatorElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_property_name(&self) -> Option<&Element> {
        self.get("propertyName")
    }

    pub fn set_property_name(&mut self, name: StringElement) {
        self.set("propertyName", Element::String(name));
    }

    pub fn get_mapping(&self) -> Option<&Element> {
        self.get("mapping")
    }

    pub fn set_mapping(&mut self, mapping: ObjectElement) {
        self.set("mapping", Element::Object(mapping));
    }

    pub fn get(&self, key: &str) -> Option<&Element> {
        self.content.iter().find_map(|(k, v)| {
            if k == key {
                Some(v)
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, key: &str, value: Element) {
        if let Some((_, existing)) = self.content.iter_mut().find(|(k, _)| k == key) {
            *existing = value;
        } else {
            self.content.push((key.to_string(), value));
        }
    }
}