use apidom_ast::{Element, ObjectElement};
use apidom_ns_json_schema_draft_7::elements::link_description::LinkDescriptionDraft7Element;

/// JSON Schema Draft 2019-09 的 LinkDescription 扩展实现
#[derive(Debug, Clone)]
pub struct LinkDescription201909Element {
    pub base: LinkDescriptionDraft7Element,
}

impl LinkDescription201909Element {
    pub fn new() -> Self {
        let mut base = LinkDescriptionDraft7Element::new();
        base.base.base.object.set_element_type("LinkDescription201909");
        Self { base }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = LinkDescriptionDraft7Element::with_content(content);
        base.base.base.object.set_element_type("LinkDescription201909");
        Self { base }
    }

    /// targetSchema: JSONSchema or Boolean
    pub fn target_schema(&self) -> Option<&Element> {
        self.base.base.base.object.get("targetSchema")
    }

    pub fn set_target_schema(&mut self, value: Element) {
        self.base.base.base.object.set("targetSchema", value);
    }

    /// hrefSchema
    pub fn href_schema(&self) -> Option<&Element> {
        self.base.base.base.object.get("hrefSchema")
    }

    pub fn set_href_schema(&mut self, value: Element) {
        self.base.base.base.object.set("hrefSchema", value);
    }

    /// headerSchema
    pub fn header_schema(&self) -> Option<&Element> {
        self.base.base.base.object.get("headerSchema")
    }

    pub fn set_header_schema(&mut self, value: Element) {
        self.base.base.base.object.set("headerSchema", value);
    }

    /// submissionSchema
    pub fn submission_schema(&self) -> Option<&Element> {
        self.base.base.base.object.get("submissionSchema")
    }

    pub fn set_submission_schema(&mut self, value: Element) {
        self.base.base.base.object.set("submissionSchema", value);
    }
}