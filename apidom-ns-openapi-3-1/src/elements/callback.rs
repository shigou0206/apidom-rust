use apidom_ast::{Element, ObjectElement};

/// A semantic element representing an OpenAPI 3.1.x "Callback" object.
#[derive(Debug, Clone)]
pub struct CallbackElement {
    pub base: ObjectElement,
}

impl CallbackElement {
    pub fn new(base: ObjectElement) -> Self {
        Self { base }
    }

    /// Optional: helper to get by key (e.g. expressions like `$request.body#/url`)
    pub fn get(&self, key: &str) -> Option<&Element> {
        self.base.get(key)
    }

    /// Optional: element name for runtime identification
    pub fn element_name(&self) -> &'static str {
        "callback"
    }
}