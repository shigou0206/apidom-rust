use apidom_ast::{Element, ObjectElement, StringElement, ArrayElement};
use apidom_ns_json_schema_draft_6::elements::link_description::LinkDescriptionDraft6Element;

/// JSON Schema Draft-7 LinkDescription
#[derive(Debug, Clone)]
pub struct LinkDescriptionDraft7Element {
    pub base: LinkDescriptionDraft6Element,
}

impl LinkDescriptionDraft7Element {
    pub fn new() -> Self {
        let mut base = LinkDescriptionDraft6Element::new();
        base.base.object.set_element_type("LinkDescriptionDraft7");
        Self { base }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = LinkDescriptionDraft6Element::with_content(content);
        base.base.object.set_element_type("LinkDescriptionDraft7");
        Self { base }
    }

    pub fn anchor(&self) -> Option<&StringElement> {
        self.base.base.object.get("anchor").and_then(Element::as_string)
    }

    pub fn set_anchor(&mut self, value: StringElement) {
        self.base.base.object.set("anchor", Element::String(value));
    }

    pub fn anchor_pointer(&self) -> Option<&StringElement> {
        self.base.base.object.get("anchorPointer").and_then(Element::as_string)
    }

    pub fn set_anchor_pointer(&mut self, value: StringElement) {
        self.base.base.object.set("anchorPointer", Element::String(value));
    }

    pub fn template_pointers(&self) -> Option<&ObjectElement> {
        self.base.base.object.get("templatePointers").and_then(Element::as_object)
    }

    pub fn set_template_pointers(&mut self, value: ObjectElement) {
        self.base.base.object.set("templatePointers", Element::Object(value));
    }

    pub fn template_required(&self) -> Option<&ArrayElement> {
        self.base.base.object.get("templateRequired").and_then(Element::as_array)
    }

    pub fn set_template_required(&mut self, value: ArrayElement) {
        self.base.base.object.set("templateRequired", Element::Array(value));
    }

    pub fn target_schema(&self) -> Option<&Element> {
        self.base.base.object.get("targetSchema")
    }

    pub fn set_target_schema(&mut self, value: Element) {
        self.base.base.object.set("targetSchema", value);
    }

    pub fn target_media_type(&self) -> Option<&StringElement> {
        self.base.base.object.get("targetMediaType").and_then(Element::as_string)
    }

    pub fn set_target_media_type(&mut self, value: StringElement) {
        self.base.base.object.set("targetMediaType", Element::String(value));
    }

    pub fn target_hints(&self) -> Option<&ObjectElement> {
        self.base.base.object.get("targetHints").and_then(Element::as_object)
    }

    pub fn set_target_hints(&mut self, value: ObjectElement) {
        self.base.base.object.set("targetHints", Element::Object(value));
    }

    pub fn description(&self) -> Option<&StringElement> {
        self.base.base.object.get("description").and_then(Element::as_string)
    }

    pub fn set_description(&mut self, value: StringElement) {
        self.base.base.object.set("description", Element::String(value));
    }

    pub fn comment(&self) -> Option<&StringElement> {
        self.base.base.object.get("$comment").and_then(Element::as_string)
    }

    pub fn set_comment(&mut self, value: StringElement) {
        self.base.base.object.set("$comment", Element::String(value));
    }

    pub fn href_schema(&self) -> Option<&Element> {
        self.base.base.object.get("hrefSchema")
    }

    pub fn set_href_schema(&mut self, value: Element) {
        self.base.base.object.set("hrefSchema", value);
    }

    pub fn header_schema(&self) -> Option<&Element> {
        self.base.base.object.get("headerSchema")
    }

    pub fn set_header_schema(&mut self, value: Element) {
        self.base.base.object.set("headerSchema", value);
    }

    pub fn submission_schema(&self) -> Option<&Element> {
        self.base.base.object.get("submissionSchema")
    }

    pub fn set_submission_schema(&mut self, value: Element) {
        self.base.base.object.set("submissionSchema", value);
    }

    pub fn submission_media_type(&self) -> Option<&StringElement> {
        self.base.base.object.get("submissionMediaType").and_then(Element::as_string)
    }

    pub fn set_submission_media_type(&mut self, value: StringElement) {
        self.base.base.object.set("submissionMediaType", Element::String(value));
    }
}
