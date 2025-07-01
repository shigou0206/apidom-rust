use apidom_ast::*;

/// Represents the `license` object in OpenAPI 3.1 specification.
#[derive(Debug, Clone)]
pub struct LicenseElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<MemberElement>,
    pub classes: ArrayElement,
    pub children: Vec<Element>,
    pub parent: Option<Box<Element>>,
}

impl LicenseElement {
    pub fn new() -> Self {
        Self {
            element: "license".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: vec![],
            classes: ArrayElement::new_empty(),
            children: vec![],
            parent: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.set_member("name", Element::String(StringElement::new(name)));
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.set_member("url", Element::String(StringElement::new(url)));
        self
    }

    pub fn with_identifier(mut self, identifier: &str) -> Self {
        self.set_member("identifier", Element::String(StringElement::new(identifier)));
        self
    }

    pub fn name(&self) -> Option<&StringElement> {
        self.get_string_member("name")
    }

    pub fn url(&self) -> Option<&StringElement> {
        self.get_string_member("url")
    }

    pub fn identifier(&self) -> Option<&StringElement> {
        self.get_string_member("identifier")
    }

    fn set_member(&mut self, key: &str, value: Element) {
        let member = MemberElement::new(Element::String(StringElement::new(key)), value);
        self.content.push(member);
    }

    fn get_string_member(&self, key: &str) -> Option<&StringElement> {
        self.content.iter().find_map(|member| {
            if let Element::String(k) = member.key.as_ref() {
                if k.content == key {
                    match member.value.as_ref() {
                        Element::String(s) => Some(s),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}