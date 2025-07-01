use apidom_ast::*;

/// OpenAPI Reference Element
#[derive(Debug, Clone)]
pub struct ReferenceElement {
    pub object: ObjectElement,
}

impl ReferenceElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("reference");
        obj.add_class("openapi-reference");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("reference");
        content.add_class("openapi-reference");
        Self { object: content }
    }

    pub fn ref_(&self) -> Option<&StringElement> {
        self.object.get("$ref").and_then(Element::as_string)
    }

    pub fn set_ref(&mut self, value: StringElement) {
        self.object.set("$ref", Element::String(value));
    }
}