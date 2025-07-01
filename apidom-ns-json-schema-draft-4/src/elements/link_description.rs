use apidom_ast::{Element, ObjectElement, StringElement};

#[derive(Debug, Clone)]
pub struct LinkDescriptionElement {
    pub object: ObjectElement,
}

impl LinkDescriptionElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("linkDescription");
        Self { object: obj }
    }

    pub fn with_content(obj: ObjectElement) -> Self {
        let mut obj = obj;
        obj.set_element_type("linkDescription");
        Self { object: obj }
    }

    pub fn href(&self) -> Option<&StringElement> {
        self.object.get("href").and_then(Element::as_string)
    }

    pub fn set_href(&mut self, val: StringElement) {
        self.object.set("href", Element::String(val));
    }

    pub fn rel(&self) -> Option<&StringElement> {
        self.object.get("rel").and_then(Element::as_string)
    }

    pub fn set_rel(&mut self, val: StringElement) {
        self.object.set("rel", Element::String(val));
    }

    pub fn title(&self) -> Option<&StringElement> {
        self.object.get("title").and_then(Element::as_string)
    }

    pub fn set_title(&mut self, val: StringElement) {
        self.object.set("title", Element::String(val));
    }

    pub fn method(&self) -> Option<&StringElement> {
        self.object.get("method").and_then(Element::as_string)
    }

    pub fn set_method(&mut self, val: StringElement) {
        self.object.set("method", Element::String(val));
    }

    pub fn media_type(&self) -> Option<&StringElement> {
        self.object.get("mediaType").and_then(Element::as_string)
    }

    pub fn set_media_type(&mut self, val: StringElement) {
        self.object.set("mediaType", Element::String(val));
    }

    pub fn enc_type(&self) -> Option<&StringElement> {
        self.object.get("encType").and_then(Element::as_string)
    }

    pub fn set_enc_type(&mut self, val: StringElement) {
        self.object.set("encType", Element::String(val));
    }

    pub fn target_schema(&self) -> Option<&Element> {
        self.object.get("targetSchema")
    }

    pub fn set_target_schema(&mut self, schema: Element) {
        self.object.set("targetSchema", schema);
    }

    pub fn schema(&self) -> Option<&Element> {
        self.object.get("schema")
    }

    pub fn set_schema(&mut self, schema: Element) {
        self.object.set("schema", schema);
    }
}

impl From<ObjectElement> for LinkDescriptionElement {
    fn from(mut obj: ObjectElement) -> Self {
        obj.set_element_type("linkDescription");
        Self { object: obj }
    }
}