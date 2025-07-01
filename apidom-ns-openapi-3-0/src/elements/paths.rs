use apidom_ast::*;
use crate::elements::path_item::PathItemElement;
/// OpenAPI Paths Element
#[derive(Debug, Clone)]
pub struct PathsElement {
    pub object: ObjectElement,
}

impl PathsElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("paths");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("paths");
        Self { object: content }
    }

    // 示例接口，可按需扩展
    pub fn get_path(&self, path: &str) -> Option<&Element> {
        self.object.get(path)
    }

    pub fn set_path(&mut self, path: &str, value: Element) {
        self.object.set(path, value);
    }

    pub fn paths(&self) -> Vec<(&String, &Element)> {
        self.object
            .content
            .iter()
            .filter_map(|member| {
                let key = member.key.as_string()?;
                Some((&key.content, &*member.value))
            })
            .collect()
    }

    pub fn path_items(&self) -> Vec<(&str, PathItemElement)> {
        self.paths()
            .iter()
            .filter_map(|(k, v)| {
                v.as_object().map(|obj| (k.as_str(), PathItemElement::with_content(obj.clone())))
            })
            .collect()
    }
}