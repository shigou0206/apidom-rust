use apidom_ast::minim_model::*;

#[derive(Debug, Clone)]
pub struct JSONSchemaDraft4Element {
    pub object: ObjectElement,
}

impl JSONSchemaDraft4Element {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("JSONSchemaDraft4");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("JSONSchemaDraft4");
        Self { object: content }
    }

    // --- 示例字段接口 ---

    pub fn title(&self) -> Option<&StringElement> {
        self.object.get("title").and_then(Element::as_string)
    }

    pub fn set_title(&mut self, title: StringElement) {
        self.object.set("title", Element::String(title));
    }

    pub fn properties(&self) -> Option<&ObjectElement> {
        self.object.get("properties").and_then(Element::as_object)
    }

    pub fn set_properties(&mut self, props: ObjectElement) {
        self.object.set("properties", Element::Object(props));
    }

    pub fn default_value(&self) -> Option<&Element> {
        self.object.get("default")
    }

    pub fn set_default_value(&mut self, value: Element) {
        self.object.set("default", value);
    }

    pub fn type_(&self) -> Option<&Element> {
        self.object.get("type")
    }

    pub fn set_type(&mut self, value: Element) {
        self.object.set("type", value);
    }

    // 等等……
}