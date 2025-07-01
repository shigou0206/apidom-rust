use apidom_ast::{Element, ObjectElement, StringElement, ArrayElement, NumberElement};
use apidom_ns_json_schema_draft_4::elements::json_schema::JSONSchemaDraft4Element;

/// JSON Schema Draft-6 Element
#[derive(Debug, Clone)]
pub struct JSONSchemaDraft6Element {
    pub base: JSONSchemaDraft4Element,
}

impl JSONSchemaDraft6Element {
    /// Create a new empty Draft-6 schema element
    pub fn new() -> Self {
        let mut base = JSONSchemaDraft4Element::new();
        base.object.set_element_type("JSONSchemaDraft6");
        Self { base }
    }

    /// Create from existing content
    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = JSONSchemaDraft4Element::with_content(content);
        base.object.set_element_type("JSONSchemaDraft6");
        Self { base }
    }

    // ------------- Draft-4 继承字段 -------------

    pub fn title(&self) -> Option<&StringElement> {
        self.base.title()
    }

    pub fn set_title(&mut self, val: StringElement) {
        self.base.set_title(val);
    }

    pub fn type_(&self) -> Option<&Element> {
        self.base.type_()
    }

    pub fn set_type(&mut self, val: Element) {
        self.base.set_type(val);
    }

    pub fn default_value(&self) -> Option<&Element> {
        self.base.default_value()
    }

    pub fn set_default_value(&mut self, val: Element) {
        self.base.set_default_value(val);
    }

    // ... 如有需要，可暴露更多 base 方法

    // ------------- Draft-6 新字段 -------------

    /// `$id` keyword (replaces `id` in draft-4)
    pub fn id(&self) -> Option<&StringElement> {
        self.base.object.get("$id").and_then(Element::as_string)
    }

    pub fn set_id(&mut self, value: StringElement) {
        self.base.object.set("$id", Element::String(value));
    }

    /// `const` keyword
    pub fn const_value(&self) -> Option<&Element> {
        self.base.object.get("const")
    }

    pub fn set_const_value(&mut self, value: Element) {
        self.base.object.set("const", value);
    }

    /// `examples` keyword
    pub fn examples(&self) -> Option<&ArrayElement> {
        self.base.object.get("examples").and_then(Element::as_array)
    }

    pub fn set_examples(&mut self, arr: ArrayElement) {
        self.base.object.set("examples", Element::Array(arr));
    }

    /// `exclusiveMaximum` as number
    pub fn exclusive_maximum(&self) -> Option<&NumberElement> {
        self.base.object.get("exclusiveMaximum").and_then(Element::as_number)
    }

    pub fn set_exclusive_maximum(&mut self, val: NumberElement) {
        self.base.object.set("exclusiveMaximum", Element::Number(val));
    }

    /// `exclusiveMinimum` as number
    pub fn exclusive_minimum(&self) -> Option<&NumberElement> {
        self.base.object.get("exclusiveMinimum").and_then(Element::as_number)
    }

    pub fn set_exclusive_minimum(&mut self, val: NumberElement) {
        self.base.object.set("exclusiveMinimum", Element::Number(val));
    }

    /// `propertyNames` keyword
    pub fn property_names(&self) -> Option<&Element> {
        self.base.object.get("propertyNames")
    }

    pub fn set_property_names(&mut self, val: Element) {
        self.base.object.set("propertyNames", val);
    }

    /// `contains` keyword
    pub fn contains(&self) -> Option<&Element> {
        self.base.object.get("contains")
    }

    pub fn set_contains(&mut self, val: Element) {
        self.base.object.set("contains", val);
    }
}