use apidom_ast::{Element, ObjectElement, StringElement};

/// OpenAPI 3.1 Info Element
#[derive(Debug, Clone)]
pub struct InfoElement {
    pub inner: ObjectElement,
}

impl InfoElement {
    pub fn new() -> Self {
        let mut inner = ObjectElement::new();
        inner.set_element_type("info");
        Self { inner }
    }

    pub fn title(&self) -> Option<&StringElement> {
        self.inner.get("title").and_then(Element::as_string)
    }

    pub fn set_title(&mut self, title: StringElement) {
        self.inner.set("title", Element::String(title));
    }

    pub fn version(&self) -> Option<&StringElement> {
        self.inner.get("version").and_then(Element::as_string)
    }

    pub fn set_version(&mut self, version: StringElement) {
        self.inner.set("version", Element::String(version));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.inner.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, description: StringElement) {
        self.inner.set("description", Element::String(description));
    }
}

impl Default for InfoElement {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenAPI 3.1 Server Element
#[derive(Debug, Clone)]
pub struct ServerElement {
    pub inner: ObjectElement,
}

impl ServerElement {
    pub fn new() -> Self {
        let mut inner = ObjectElement::new();
        inner.set_element_type("server");
        Self { inner }
    }

    pub fn url(&self) -> Option<&StringElement> {
        self.inner.get("url").and_then(Element::as_string)
    }

    pub fn set_url(&mut self, url: StringElement) {
        self.inner.set("url", Element::String(url));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.inner.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, description: StringElement) {
        self.inner.set("description", Element::String(description));
    }
}

impl Default for ServerElement {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenAPI 3.1 Path Item Element
#[derive(Debug, Clone)]
pub struct PathItemElement {
    pub inner: ObjectElement,
}

impl PathItemElement {
    pub fn new() -> Self {
        let mut inner = ObjectElement::new();
        inner.set_element_type("pathItem");
        Self { inner }
    }

    pub fn summary(&self) -> Option<&StringElement> {
        self.inner.get("summary").and_then(Element::as_string)
    }

    pub fn set_summary(&mut self, summary: StringElement) {
        self.inner.set("summary", Element::String(summary));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.inner.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, description: StringElement) {
        self.inner.set("description", Element::String(description));
    }
}

impl Default for PathItemElement {
    fn default() -> Self {
        Self::new()
    }
} 