use apidom_ast::*;

/// Represents an OpenAPI 3.1 Responses object.
#[derive(Debug, Clone)]
pub struct ResponsesElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<MemberElement>,
    pub classes: ArrayElement,
    pub children: Vec<Element>,
    pub parent: Option<Box<Element>>,
}

impl ResponsesElement {
    pub fn new() -> Self {
        Self {
            element: "responses".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: vec![],
            classes: ArrayElement::new_empty(),
            children: vec![],
            parent: None,
        }
    }

    pub fn with_default(mut self, response: Element) -> Self {
        self.set_member("default", response);
        self
    }

    pub fn default(&self) -> Option<&Element> {
        self.get_member("default")
    }

    pub fn insert_status(&mut self, status_code: &str, response: Element) {
        self.set_member(status_code, response);
    }

    pub fn get_status(&self, status_code: &str) -> Option<&Element> {
        self.get_member(status_code)
    }

    fn set_member(&mut self, key: &str, value: Element) {
        let member = MemberElement::new(Element::String(StringElement::new(key)), value);
        self.content.push(member);
    }

    fn get_member(&self, key: &str) -> Option<&Element> {
        self.content.iter().find_map(|member| {
            if let Element::String(k) = member.key.as_ref() {
                if k.content == key {
                    Some(member.value.as_ref())
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}