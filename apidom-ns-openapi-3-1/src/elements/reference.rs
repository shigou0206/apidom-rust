use apidom_ast::*;

/// Represents the `reference` element in OpenAPI 3.x/3.1.
/// Typically used to `$ref` other components or schema fragments.
#[derive(Debug, Clone)]
pub struct ReferenceElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<MemberElement>,
    pub classes: ArrayElement,
    pub children: Vec<Element>,
    pub parent: Option<Box<Element>>,
}

impl ReferenceElement {
    pub fn new() -> Self {
        let mut classes = ArrayElement::new_empty();
        classes.content.push(Element::String(StringElement::new("openapi-reference")));

        Self {
            element: "reference".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: vec![],
            classes,
            children: vec![],
            parent: None,
        }
    }

    pub fn with_ref(mut self, reference: &str) -> Self {
        self.set_member("$ref", Element::String(StringElement::new(reference)));
        self
    }

    pub fn with_summary(mut self, summary: &str) -> Self {
        self.set_member("summary", Element::String(StringElement::new(summary)));
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.set_member("description", Element::String(StringElement::new(description)));
        self
    }

    pub fn r#ref(&self) -> Option<&StringElement> {
        self.get_string_member("$ref")
    }

    pub fn summary(&self) -> Option<&StringElement> {
        self.get_string_member("summary")
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.get_string_member("description")
    }

    fn set_member(&mut self, key: &str, value: Element) {
        let member = MemberElement::new(Element::String(StringElement::new(key)), value);
        self.content.push(member);
    }

    fn get_string_member(&self, key: &str) -> Option<&StringElement> {
        self.content.iter().find_map(|member| {
            if let Element::String(k) = member.key.as_ref() {
                if k.content == key {
                    if let Element::String(s) = member.value.as_ref() {
                        return Some(s);
                    }
                }
            }
            None
        })
    }
}