use apidom_ast::minim_model::{Element, ObjectElement, StringElement};

#[derive(Debug, Clone)]
pub struct JSONReferenceElement {
    pub object: ObjectElement, // 内部结构
}

impl JSONReferenceElement {
    pub fn new(object: ObjectElement) -> Self {
        Self { object }
    }

    pub fn get_ref(&self) -> Option<&StringElement> {
        self.object.get("$ref").and_then(Element::as_string)
    }

    pub fn set_ref(&mut self, value: StringElement) {
        self.object.set("$ref", Element::String(value));
    }
}