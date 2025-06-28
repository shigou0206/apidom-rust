use apidom_ast::minim_model::{Element, ObjectElement, StringElement};

#[derive(Debug, Clone)]
pub struct ContactElement {
    pub base: ObjectElement,
}

impl ContactElement {
    pub fn new(base: ObjectElement) -> Self {
        Self { base }
    }

    pub fn element_name(&self) -> &'static str {
        "contact"
    }

    fn get_str_field(&self, key: &str) -> Option<&Element> {
        self.base.get(key)
    }

    fn set_str_field(&mut self, key: &str, value: Option<StringElement>) {
        if let Some(s) = value {
            self.base.set(key, Element::String(s));
        }
    }

    // === Getters ===
    pub fn name(&self) -> Option<&Element> {
        self.get_str_field("name")
    }

    pub fn url(&self) -> Option<&Element> {
        self.get_str_field("url")
    }

    pub fn email(&self) -> Option<&Element> {
        self.get_str_field("email")
    }

    // === Setters ===
    pub fn set_name(&mut self, name: Option<StringElement>) {
        self.set_str_field("name", name);
    }

    pub fn set_url(&mut self, url: Option<StringElement>) {
        self.set_str_field("url", url);
    }

    pub fn set_email(&mut self, email: Option<StringElement>) {
        self.set_str_field("email", email);
    }
}