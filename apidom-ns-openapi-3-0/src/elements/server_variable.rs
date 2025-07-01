use apidom_ast::*;

/// OpenAPI ServerVariable Element
#[derive(Debug, Clone)]
pub struct ServerVariableElement {
    pub object: ObjectElement,
}

impl ServerVariableElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("serverVariable");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("serverVariable");
        Self { object: content }
    }

    pub fn enum_values(&self) -> Option<&ArrayElement> {
        self.object.get("enum").and_then(Element::as_array)
    }

    pub fn set_enum_values(&mut self, value: ArrayElement) {
        self.object.set("enum", Element::Array(value));
    }

    pub fn default_value(&self) -> Option<&StringElement> {
        self.object.get("default").and_then(Element::as_string)
    }

    pub fn set_default_value(&mut self, value: StringElement) {
        self.object.set("default", Element::String(value));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, value: StringElement) {
        self.object.set("description", Element::String(value));
    }
}