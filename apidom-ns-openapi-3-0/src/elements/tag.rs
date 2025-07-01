use apidom_ast::*;
use crate::elements::external_documentation::ExternalDocumentationElement;

/// OpenAPI Tag Element
#[derive(Debug, Clone)]
pub struct TagElement {
    pub object: ObjectElement,
}

impl TagElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("tag");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("tag");
        Self { object: content }
    }

    pub fn name(&self) -> Option<&StringElement> {
        self.object.get("name").and_then(Element::as_string)
    }

    pub fn set_name(&mut self, value: StringElement) {
        self.object.set("name", Element::String(value));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, value: StringElement) {
        self.object.set("description", Element::String(value));
    }

    pub fn external_docs(&self) -> Option<ExternalDocumentationElement> {
        self.object
            .get("externalDocs")
            .and_then(Element::as_object)
            .map(|obj| ExternalDocumentationElement::with_content(obj.clone()))
    }

    pub fn set_external_docs(&mut self, value: ExternalDocumentationElement) {
        self.object.set("externalDocs", Element::Object(value.object));
    }
}