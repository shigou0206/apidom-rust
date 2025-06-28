use apidom_ast::minim_model::*;

/// OpenAPI ExternalDocumentation Element
#[derive(Debug, Clone)]
pub struct ExternalDocumentationElement {
    pub object: ObjectElement,
}

impl ExternalDocumentationElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("externalDocumentation");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("externalDocumentation");
        Self { object: content }
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.object.set("description", Element::String(val));
    }

    pub fn url(&self) -> Option<&StringElement> {
        self.object.get("url").and_then(Element::as_string)
    }

    pub fn set_url(&mut self, val: StringElement) {
        self.object.set("url", Element::String(val));
    }
}