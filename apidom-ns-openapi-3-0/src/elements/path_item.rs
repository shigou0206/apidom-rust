use apidom_ast::minim_model::*;

#[derive(Debug, Clone)]
pub struct PathItemElement {
    pub object: ObjectElement,
}

impl PathItemElement {
    pub fn new() -> Self {
        let mut object = ObjectElement::new();
        object.set_element_type("pathItem");
        Self { object }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("pathItem");
        Self { object: content }
    }

    pub fn ref_(&self) -> Option<&StringElement> {
        self.object.get("$ref").and_then(Element::as_string)
    }

    pub fn set_ref(&mut self, val: StringElement) {
        self.object.set("$ref", Element::String(val));
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

    pub fn operation(&self, method: &str) -> Option<&Element> {
        self.object.get(method)
    }

    pub fn set_operation(&mut self, method: &str, op: Element) {
        self.object.set(method, op);
    }

    pub fn servers(&self) -> Option<&ArrayElement> {
        self.object.get("servers").and_then(Element::as_array)
    }

    pub fn set_servers(&mut self, val: ArrayElement) {
        self.object.set("servers", Element::Array(val));
    }

    pub fn parameters(&self) -> Option<&ArrayElement> {
        self.object.get("parameters").and_then(Element::as_array)
    }

    pub fn set_parameters(&mut self, val: ArrayElement) {
        self.object.set("parameters", Element::Array(val));
    }

    // 快捷方法：HTTP 操作
    pub fn get(&self) -> Option<&Element> {
        self.operation("get")
    }

    pub fn post(&self) -> Option<&Element> {
        self.operation("post")
    }

    pub fn put(&self) -> Option<&Element> {
        self.operation("put")
    }

    pub fn delete(&self) -> Option<&Element> {
        self.operation("delete")
    }

    pub fn patch(&self) -> Option<&Element> {
        self.operation("patch")
    }

    pub fn head(&self) -> Option<&Element> {
        self.operation("head")
    }

    pub fn options(&self) -> Option<&Element> {
        self.operation("options")
    }

    pub fn trace(&self) -> Option<&Element> {
        self.operation("trace")
    }
}