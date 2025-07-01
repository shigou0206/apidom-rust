use apidom_ast::{Element, ObjectElement, StringElement};

#[derive(Debug, Clone)]
pub struct MediaElement {
    pub object: ObjectElement,
}

impl MediaElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("media");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("media");
        Self { object: content }
    }

    pub fn binary_encoding(&self) -> Option<&StringElement> {
        self.object.get("binaryEncoding").and_then(Element::as_string)
    }

    pub fn set_binary_encoding(&mut self, val: StringElement) {
        self.object.set("binaryEncoding", Element::String(val));
    }

    pub fn media_type(&self) -> Option<&StringElement> {
        self.object.get("type").and_then(Element::as_string)
    }

    pub fn set_media_type(&mut self, val: StringElement) {
        self.object.set("type", Element::String(val));
    }

    pub fn inner(&self) -> &ObjectElement {
        &self.object
    }

    pub fn inner_mut(&mut self) -> &mut ObjectElement {
        &mut self.object
    }
}