use apidom_ast::*;

#[derive(Debug, Clone)]
pub struct MediaTypeElement {
    pub object: ObjectElement,
}

impl MediaTypeElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("mediaType");
        Self { object: obj }
    }

    pub fn with_content(mut content: ObjectElement) -> Self {
        content.set_element_type("mediaType");
        Self { object: content }
    }

    // schema: SchemaElement or ReferenceElement (ObjectElement)
    pub fn schema(&self) -> Option<&Element> {
        self.object.get("schema")
    }

    pub fn set_schema(&mut self, value: Element) {
        self.object.set("schema", value);
    }

    // example: any Element
    pub fn example(&self) -> Option<&Element> {
        self.object.get("example")
    }

    pub fn set_example(&mut self, value: Element) {
        self.object.set("example", value);
    }

    // examples: ObjectElement
    pub fn examples(&self) -> Option<&ObjectElement> {
        self.object.get("examples").and_then(Element::as_object)
    }

    pub fn set_examples(&mut self, value: ObjectElement) {
        self.object.set("examples", Element::Object(value));
    }

    // encoding: ObjectElement
    pub fn encoding(&self) -> Option<&ObjectElement> {
        self.object.get("encoding").and_then(Element::as_object)
    }

    pub fn set_encoding(&mut self, value: ObjectElement) {
        self.object.set("encoding", Element::Object(value));
    }
}