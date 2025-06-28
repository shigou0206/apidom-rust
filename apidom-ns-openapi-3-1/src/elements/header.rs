use apidom_ast::minim_model::*;

/// Represents an OpenAPI 3.1 `header` object.
#[derive(Debug, Clone)]
pub struct HeaderElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<(String, Element)>,
}

impl Default for HeaderElement {
    fn default() -> Self {
        Self {
            element: "header".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: vec![],
        }
    }
}

impl HeaderElement {
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

    pub fn get_required(&self) -> BooleanElement {
        self.get("required")
            .and_then(|e| match e {
                Element::Boolean(b) => Some(b.clone()),
                _ => None,
            })
            .unwrap_or(BooleanElement::new(false))
    }

    pub fn set_required(&mut self, val: BooleanElement) {
        self.set("required", Element::Boolean(val));
    }

    pub fn get_deprecated(&self) -> BooleanElement {
        self.get("deprecated")
            .and_then(|e| match e {
                Element::Boolean(b) => Some(b.clone()),
                _ => None,
            })
            .unwrap_or(BooleanElement::new(false))
    }

    pub fn set_deprecated(&mut self, val: BooleanElement) {
        self.set("deprecated", Element::Boolean(val));
    }

    pub fn get_allow_empty_value(&self) -> Option<&Element> {
        self.get("allowEmptyValue")
    }

    pub fn set_allow_empty_value(&mut self, val: BooleanElement) {
        self.set("allowEmptyValue", Element::Boolean(val));
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

    pub fn get_allow_reserved(&self) -> Option<&Element> {
        self.get("allowReserved")
    }

    pub fn set_allow_reserved(&mut self, val: BooleanElement) {
        self.set("allowReserved", Element::Boolean(val));
    }

    pub fn get_schema(&self) -> Option<&Element> {
        self.get("schema")
    }

    pub fn set_schema(&mut self, val: Element) {
        self.set("schema", val);
    }

    pub fn get_example(&self) -> Option<&Element> {
        self.get("example")
    }

    pub fn set_example(&mut self, val: Element) {
        self.set("example", val);
    }

    pub fn get_examples(&self) -> Option<&Element> {
        self.get("examples")
    }

    pub fn set_examples(&mut self, val: ObjectElement) {
        self.set("examples", Element::Object(val));
    }

    pub fn get_content(&self) -> Option<&Element> {
        self.get("content")
    }

    pub fn set_content(&mut self, val: ObjectElement) {
        self.set("content", Element::Object(val));
    }

    pub fn get_description(&self) -> Option<&Element> {
        self.get("description")
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.set("description", Element::String(val));
    }
}