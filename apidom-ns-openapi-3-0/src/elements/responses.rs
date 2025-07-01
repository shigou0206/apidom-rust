use apidom_ast::*;

/// OpenAPI Responses Element
#[derive(Debug, Clone)]
pub struct ResponsesElement {
    pub object: ObjectElement,
}

impl ResponsesElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("responses");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("responses");
        Self { object: content }
    }

    /// `default` 字段（可能为 ResponseElement 或 ReferenceElement）
    pub fn default(&self) -> Option<&Element> {
        self.object.get("default")
    }

    pub fn set_default(&mut self, value: Element) {
        self.object.set("default", value);
    }

    // 可扩展方法，如 get("200")、get("404") 等
    pub fn get_status_response(&self, status: &str) -> Option<&Element> {
        self.object.get(status)
    }

    pub fn set_status_response(&mut self, status: &str, value: Element) {
        self.object.set(status, value);
    }
}