use apidom_ast::{Element, ObjectElement, StringElement, ArrayElement, SchemaElement};
use apidom_ns_json_schema_2019_09::elements::json_schema::JSONSchema201909Element;

/// JSON Schema Draft 2020-12 Element
#[derive(Debug, Clone)]
pub struct JSONSchema202012Element {
    pub base: JSONSchema201909Element,
}

impl SchemaElement for JSONSchema202012Element {
    fn object(&self) -> &ObjectElement {
        &self.base.base.base.base.object
    }

    fn object_mut(&mut self) -> &mut ObjectElement {
        &mut self.base.base.base.base.object
    }
}

impl JSONSchema202012Element {
    pub fn new() -> Self {
        let mut base = JSONSchema201909Element::new();
        base.base.base.base.object.set_element_type("JSONSchema202012");
        Self { base }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = JSONSchema201909Element::with_content(content);
        base.base.base.base.object.set_element_type("JSONSchema202012");
        Self { base }
    }

    // -------------------- Draft-2019-09 继承字段（可选暴露） --------------------

    pub fn type_(&self) -> Option<&Element> {
        self.base.base.type_()
    }

    pub fn set_type(&mut self, val: Element) {
        self.base.base.set_type(val);
    }

    // -------------------- Draft-2020-12 扩展字段 --------------------

    /// `$dynamicAnchor` keyword
    pub fn dynamic_anchor(&self) -> Option<&StringElement> {
        self.get_string_field("$dynamicAnchor")
    }

    pub fn set_dynamic_anchor(&mut self, value: StringElement) {
        self.set_string_field("$dynamicAnchor", value);
    }

    /// `$dynamicRef` keyword
    pub fn dynamic_ref(&self) -> Option<&StringElement> {
        self.get_string_field("$dynamicRef")
    }

    pub fn set_dynamic_ref(&mut self, value: StringElement) {
        self.set_string_field("$dynamicRef", value);
    }

    /// `prefixItems` keyword
    pub fn prefix_items(&self) -> Option<&ArrayElement> {
        self.get_array_field("prefixItems")
    }

    pub fn set_prefix_items(&mut self, value: ArrayElement) {
        self.set_array_field("prefixItems", value);
    }
}