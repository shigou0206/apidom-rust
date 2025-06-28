use apidom_ast::minim_model::*;

#[derive(Debug, Clone)]
pub struct OperationElement {
    pub object: ObjectElement,
}

impl OperationElement {
    pub fn new() -> Self {
        let mut object = ObjectElement::new();
        object.set_element_type("operation");
        Self { object }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("operation");
        Self { object: content }
    }

    pub fn summary(&self) -> Option<&StringElement> {
        self.object.get("summary").and_then(Element::as_string)
    }

    pub fn set_summary(&mut self, val: StringElement) {
        self.object.set("summary", Element::String(val));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, val: StringElement) {
        self.object.set("description", Element::String(val));
    }

    pub fn operation_id(&self) -> Option<&StringElement> {
        self.object.get("operationId").and_then(Element::as_string)
    }

    pub fn set_operation_id(&mut self, val: StringElement) {
        self.object.set("operationId", Element::String(val));
    }

    pub fn parameters(&self) -> Option<&ArrayElement> {
        self.object.get("parameters").and_then(Element::as_array)
    }

    pub fn set_parameters(&mut self, val: ArrayElement) {
        self.object.set("parameters", Element::Array(val));
    }

    pub fn request_body(&self) -> Option<&Element> {
        self.object.get("requestBody")
    }

    pub fn set_request_body(&mut self, val: Element) {
        self.object.set("requestBody", val);
    }

    pub fn responses(&self) -> Option<&Element> {
        self.object.get("responses")
    }

    pub fn set_responses(&mut self, val: Element) {
        self.object.set("responses", val);
    }

    pub fn callbacks(&self) -> Option<&ObjectElement> {
        self.object.get("callbacks").and_then(Element::as_object)
    }

    pub fn set_callbacks(&mut self, val: ObjectElement) {
        self.object.set("callbacks", Element::Object(val));
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

    pub fn security(&self) -> Option<&ArrayElement> {
        self.object.get("security").and_then(Element::as_array)
    }

    pub fn set_security(&mut self, val: ArrayElement) {
        self.object.set("security", Element::Array(val));
    }

    pub fn servers(&self) -> Option<&ArrayElement> {
        self.object.get("servers").and_then(Element::as_array)
    }

    pub fn set_servers(&mut self, val: ArrayElement) {
        self.object.set("servers", Element::Array(val));
    }

    pub fn tags(&self) -> Option<&ArrayElement> {
        self.object.get("tags").and_then(Element::as_array)
    }

    pub fn set_tags(&mut self, val: ArrayElement) {
        self.object.set("tags", Element::Array(val));
    }

    pub fn external_docs(&self) -> Option<&ObjectElement> {
        self.object.get("externalDocs").and_then(Element::as_object)
    }

    pub fn set_external_docs(&mut self, val: ObjectElement) {
        self.object.set("externalDocs", Element::Object(val));
    }
}