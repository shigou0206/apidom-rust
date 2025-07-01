use apidom_ast::*;

/// Represents an OpenAPI 3.1 Response object.
#[derive(Debug, Clone)]
pub struct ResponseElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<MemberElement>,
    pub classes: ArrayElement,
    pub children: Vec<Element>,
    pub parent: Option<Box<Element>>,
}

impl ResponseElement {
    pub fn new() -> Self {
        Self {
            element: "response".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: vec![],
            classes: ArrayElement::new_empty(),
            children: vec![],
            parent: None,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.set_member("description", Element::String(StringElement::new(desc)));
        self
    }

    pub fn with_headers(mut self, headers: ObjectElement) -> Self {
        self.set_member("headers", Element::Object(headers));
        self
    }

    pub fn with_content(mut self, content: ObjectElement) -> Self {
        self.set_member("content", Element::Object(content));
        self
    }

    pub fn with_links(mut self, links: ObjectElement) -> Self {
        self.set_member("links", Element::Object(links));
        self
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.get_string_member("description")
    }

    pub fn headers(&self) -> Option<&ObjectElement> {
        self.get_object_member("headers")
    }

    pub fn content_prop(&self) -> Option<&ObjectElement> {
        self.get_object_member("content")
    }

    pub fn links(&self) -> Option<&ObjectElement> {
        self.get_object_member("links")
    }

    fn set_member(&mut self, key: &str, value: Element) {
        let member = MemberElement::new(Element::String(StringElement::new(key)), value);
        self.content.push(member);
    }

    fn get_string_member(&self, key: &str) -> Option<&StringElement> {
        self.content.iter().find_map(|member| {
            if let Element::String(k) = member.key.as_ref() {
                if k.content == key {
                    if let Element::String(v) = member.value.as_ref() {
                        return Some(v);
                    }
                }
            }
            None
        })
    }

    fn get_object_member(&self, key: &str) -> Option<&ObjectElement> {
        self.content.iter().find_map(|member| {
            if let Element::String(k) = member.key.as_ref() {
                if k.content == key {
                    if let Element::Object(o) = member.value.as_ref() {
                        return Some(o);
                    }
                }
            }
            None
        })
    }
}