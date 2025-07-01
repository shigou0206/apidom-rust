use apidom_ast::*;

/// OpenAPI Example Element
#[derive(Debug, Clone)]
pub struct ExampleElement {
    pub object: ObjectElement,
}

impl ExampleElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("example");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("example");
        Self { object: content }
    }

    pub fn summary(&self) -> Option<&StringElement> {
        self.object.get("summary").and_then(Element::as_string)
    }

    pub fn set_summary(&mut self, val: StringElement) {
        self.object.set("summary", Element::String(val));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.object.set("description", Element::String(val));
    }

    pub fn value(&self) -> Option<&Element> {
        self.object.get("value")
    }

    pub fn set_value(&mut self, val: Element) {
        self.object.set("value", val);
    }

    pub fn external_value(&self) -> Option<&StringElement> {
        self.object.get("externalValue").and_then(Element::as_string)
    }

    pub fn set_external_value(&mut self, val: StringElement) {
        self.object.set("externalValue", Element::String(val));
    }
}