use apidom_ast::minim_model::{Element, ObjectElement};

#[derive(Debug, Clone)]
pub struct ComponentsElement {
    pub base: ObjectElement,
}

impl ComponentsElement {
    pub fn new(base: ObjectElement) -> Self {
        Self { base }
    }

    pub fn element_name(&self) -> &'static str {
        "components"
    }

    fn get_field(&self, name: &str) -> Option<&Element> {
        self.base.get(name)
    }

    fn set_field(&mut self, name: &str, value: Option<ObjectElement>) {
        if let Some(obj) = value {
            self.base.set(name, Element::Object(obj));
        } else {
            // Optional: clear field
            // self.base.remove(name); // if you implement remove()
        }
    }

    // === Getters ===
    pub fn schemas(&self) -> Option<&Element> {
        self.get_field("schemas")
    }

    pub fn responses(&self) -> Option<&Element> {
        self.get_field("responses")
    }

    pub fn parameters(&self) -> Option<&Element> {
        self.get_field("parameters")
    }

    pub fn examples(&self) -> Option<&Element> {
        self.get_field("examples")
    }

    pub fn request_bodies(&self) -> Option<&Element> {
        self.get_field("requestBodies")
    }

    pub fn headers(&self) -> Option<&Element> {
        self.get_field("headers")
    }

    pub fn security_schemes(&self) -> Option<&Element> {
        self.get_field("securitySchemes")
    }

    pub fn links(&self) -> Option<&Element> {
        self.get_field("links")
    }

    pub fn callbacks(&self) -> Option<&Element> {
        self.get_field("callbacks")
    }

    pub fn path_items(&self) -> Option<&Element> {
        self.get_field("pathItems")
    }

    // === Setters ===
    pub fn set_schemas(&mut self, val: Option<ObjectElement>) {
        self.set_field("schemas", val);
    }

    pub fn set_responses(&mut self, val: Option<ObjectElement>) {
        self.set_field("responses", val);
    }

    pub fn set_parameters(&mut self, val: Option<ObjectElement>) {
        self.set_field("parameters", val);
    }

    pub fn set_examples(&mut self, val: Option<ObjectElement>) {
        self.set_field("examples", val);
    }

    pub fn set_request_bodies(&mut self, val: Option<ObjectElement>) {
        self.set_field("requestBodies", val);
    }

    pub fn set_headers(&mut self, val: Option<ObjectElement>) {
        self.set_field("headers", val);
    }

    pub fn set_security_schemes(&mut self, val: Option<ObjectElement>) {
        self.set_field("securitySchemes", val);
    }

    pub fn set_links(&mut self, val: Option<ObjectElement>) {
        self.set_field("links", val);
    }

    pub fn set_callbacks(&mut self, val: Option<ObjectElement>) {
        self.set_field("callbacks", val);
    }

    pub fn set_path_items(&mut self, val: Option<ObjectElement>) {
        self.set_field("pathItems", val);
    }
}