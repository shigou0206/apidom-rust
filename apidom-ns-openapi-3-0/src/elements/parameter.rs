use apidom_ast::minim_model::*;

/// OpenAPI Parameter Element
#[derive(Debug, Clone)]
pub struct ParameterElement {
    pub object: ObjectElement,
}

impl ParameterElement {
    pub fn new() -> Self {
        let mut object = ObjectElement::new();
        object.set_element_type("parameter");
        Self { object }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("parameter");
        Self { object: content }
    }

    pub fn name(&self) -> Option<&StringElement> {
        self.object.get("name").and_then(Element::as_string)
    }

    pub fn set_name(&mut self, val: StringElement) {
        self.object.set("name", Element::String(val));
    }

    pub fn in_(&self) -> Option<&StringElement> {
        self.object.get("in").and_then(Element::as_string)
    }

    pub fn set_in(&mut self, val: StringElement) {
        self.object.set("in", Element::String(val));
    }

    pub fn required(&self) -> bool {
        self.object
            .get("required")
            .and_then(Element::as_boolean)
            .map(|b| b.content)
            .unwrap_or(false)
    }

    pub fn set_required(&mut self, val: bool) {
        self.object.set("required", Element::Boolean(BooleanElement::new(val)));
    }

    pub fn deprecated(&self) -> bool {
        self.object
            .get("deprecated")
            .and_then(Element::as_boolean)
            .map(|b| b.content)
            .unwrap_or(false)
    }

    pub fn set_deprecated(&mut self, val: bool) {
        self.object.set("deprecated", Element::Boolean(BooleanElement::new(val)));
    }

    pub fn allow_empty_value(&self) -> Option<&BooleanElement> {
        self.object.get("allowEmptyValue").and_then(Element::as_boolean)
    }

    pub fn set_allow_empty_value(&mut self, val: BooleanElement) {
        self.object.set("allowEmptyValue", Element::Boolean(val));
    }

    pub fn style(&self) -> Option<&StringElement> {
        self.object.get("style").and_then(Element::as_string)
    }

    pub fn set_style(&mut self, val: StringElement) {
        self.object.set("style", Element::String(val));
    }

    pub fn explode(&self) -> Option<&BooleanElement> {
        self.object.get("explode").and_then(Element::as_boolean)
    }

    pub fn set_explode(&mut self, val: BooleanElement) {
        self.object.set("explode", Element::Boolean(val));
    }

    pub fn allow_reserved(&self) -> Option<&BooleanElement> {
        self.object.get("allowReserved").and_then(Element::as_boolean)
    }

    pub fn set_allow_reserved(&mut self, val: BooleanElement) {
        self.object.set("allowReserved", Element::Boolean(val));
    }

    pub fn schema(&self) -> Option<&Element> {
        self.object.get("schema")
    }

    pub fn set_schema(&mut self, val: Element) {
        self.object.set("schema", val);
    }

    pub fn example(&self) -> Option<&Element> {
        self.object.get("example")
    }

    pub fn set_example(&mut self, val: Element) {
        self.object.set("example", val);
    }

    pub fn examples(&self) -> Option<&ObjectElement> {
        self.object.get("examples").and_then(Element::as_object)
    }

    pub fn set_examples(&mut self, val: ObjectElement) {
        self.object.set("examples", Element::Object(val));
    }

    pub fn content_prop(&self) -> Option<&ObjectElement> {
        self.object.get("content").and_then(Element::as_object)
    }

    pub fn set_content_prop(&mut self, val: ObjectElement) {
        self.object.set("content", Element::Object(val));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.object.set("description", Element::String(val));
    }
}