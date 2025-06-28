use apidom_ast::minim_model::*;

/// Represents an OpenAPI 3.1 `requestBody` object.
#[derive(Debug, Clone)]
pub struct RequestBodyElement {
    pub element: String,
    pub meta: MetaElement,
    pub attributes: AttributesElement,
    pub content: Vec<MemberElement>,
    pub classes: ArrayElement,
    pub children: Vec<Element>,
    pub parent: Option<Box<Element>>,
}

impl RequestBodyElement {
    pub fn new() -> Self {
        Self {
            element: "requestBody".to_string(),
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

    pub fn with_required(mut self, required: bool) -> Self {
        self.set_member("required", Element::Boolean(BooleanElement::new(required)));
        self
    }

    pub fn with_content(mut self, content_obj: ObjectElement) -> Self {
        self.set_member("content", Element::Object(content_obj));
        self
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.get_string_member("description")
    }

    pub fn required(&self) -> bool {
        self.get_boolean_member("required").map_or(false, |b| b.content)
    }

    pub fn content_prop(&self) -> Option<&ObjectElement> {
        self.get_object_member("content")
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

    fn get_boolean_member(&self, key: &str) -> Option<&BooleanElement> {
        self.content.iter().find_map(|member| {
            if let Element::String(k) = member.key.as_ref() {
                if k.content == key {
                    if let Element::Boolean(b) = member.value.as_ref() {
                        return Some(b);
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