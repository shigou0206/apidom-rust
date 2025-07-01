use apidom_ast::{Element, ObjectElement, StringElement, BooleanElement, NumberElement};
use apidom_ns_json_schema_draft_7::elements::json_schema::JSONSchemaDraft7Element;

/// JSON Schema 2019-09 Element
#[derive(Debug, Clone)]
pub struct JSONSchema201909Element {
    pub base: JSONSchemaDraft7Element,
}

impl JSONSchema201909Element {
    pub fn new() -> Self {
        let mut base = JSONSchemaDraft7Element::new();
        base.base.base.object.set_element_type("JSONSchema201909");
        Self { base }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = JSONSchemaDraft7Element::with_content(content);
        base.base.base.object.set_element_type("JSONSchema201909");
        Self { base }
    }

    // --- Core Vocabulary ---

    pub fn vocabulary(&self) -> Option<&ObjectElement> {
        self.base.base.base.object.get("$vocabulary").and_then(Element::as_object)
    }

    pub fn set_vocabulary(&mut self, val: ObjectElement) {
        self.base.base.base.object.set("$vocabulary", Element::Object(val));
    }

    pub fn anchor(&self) -> Option<&StringElement> {
        self.base.base.base.object.get("$anchor").and_then(Element::as_string)
    }

    pub fn set_anchor(&mut self, val: StringElement) {
        self.base.base.base.object.set("$anchor", Element::String(val));
    }

    pub fn recursive_anchor(&self) -> Option<&BooleanElement> {
        self.base.base.base.object.get("$recursiveAnchor").and_then(Element::as_boolean)
    }

    pub fn set_recursive_anchor(&mut self, val: BooleanElement) {
        self.base.base.base.object.set("$recursiveAnchor", Element::Boolean(val));
    }

    pub fn recursive_ref(&self) -> Option<&StringElement> {
        self.base.base.base.object.get("$recursiveRef").and_then(Element::as_string)
    }

    pub fn set_recursive_ref(&mut self, val: StringElement) {
        self.base.base.base.object.set("$recursiveRef", Element::String(val));
    }

    pub fn defs(&self) -> Option<&ObjectElement> {
        self.base.base.base.object.get("$defs").and_then(Element::as_object)
    }

    pub fn set_defs(&mut self, val: ObjectElement) {
        self.base.base.base.object.set("$defs", Element::Object(val));
    }

    // --- Validation for Arrays ---

    pub fn max_contains(&self) -> Option<&NumberElement> {
        self.base.base.base.object.get("maxContains").and_then(Element::as_number)
    }

    pub fn set_max_contains(&mut self, val: NumberElement) {
        self.base.base.base.object.set("maxContains", Element::Number(val));
    }

    pub fn min_contains(&self) -> Option<&NumberElement> {
        self.base.base.base.object.get("minContains").and_then(Element::as_number)
    }

    pub fn set_min_contains(&mut self, val: NumberElement) {
        self.base.base.base.object.set("minContains", Element::Number(val));
    }

    // --- Validation for Objects ---

    pub fn dependent_required(&self) -> Option<&ObjectElement> {
        self.base.base.base.object.get("dependentRequired").and_then(Element::as_object)
    }

    pub fn set_dependent_required(&mut self, val: ObjectElement) {
        self.base.base.base.object.set("dependentRequired", Element::Object(val));
    }

    // --- Metadata Vocabulary ---

    pub fn deprecated(&self) -> Option<&BooleanElement> {
        self.base.base.base.object.get("deprecated").and_then(Element::as_boolean)
    }

    pub fn set_deprecated(&mut self, val: BooleanElement) {
        self.base.base.base.object.set("deprecated", Element::Boolean(val));
    }

    // --- Content Vocabulary ---

    pub fn content_schema(&self) -> Option<&Element> {
        self.base.base.base.object.get("contentSchema")
    }

    pub fn set_content_schema(&mut self, val: Element) {
        self.base.base.base.object.set("contentSchema", val);
    }
}