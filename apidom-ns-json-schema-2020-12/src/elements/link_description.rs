use apidom_ast::minim_model::*;
use apidom_ns_json_schema_2019_09::elements::link_description::LinkDescription201909Element;

/// JSON Schema Draft 2020-12 LinkDescription Element
#[derive(Debug, Clone)]
pub struct LinkDescriptionDraft202012Element {
    pub base: LinkDescription201909Element,
}

impl LinkDescriptionDraft202012Element {
    pub fn new() -> Self {
        let mut base = LinkDescription201909Element::new();
        base.base.base.base.object.set_element_type("LinkDescriptionDraft202012");
        Self { base }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = LinkDescription201909Element::with_content(content);
        base.base.base.base.object.set_element_type("LinkDescriptionDraft202012");
        Self { base }
    }

    // ------------------- Inherited Fields -------------------

    pub fn href_schema(&self) -> Option<&Element> {
        self.base.base.base.base.object.get("hrefSchema")
    }

    pub fn set_href_schema(&mut self, value: Element) {
        self.base.base.base.base.object.set("hrefSchema", value);
    }

    pub fn header_schema(&self) -> Option<&Element> {
        self.base.base.base.base.object.get("headerSchema")
    }

    pub fn set_header_schema(&mut self, value: Element) {
        self.base.base.base.base.object.set("headerSchema", value);
    }

    pub fn submission_schema(&self) -> Option<&Element> {
        self.base.base.base.base.object.get("submissionSchema")
    }

    pub fn set_submission_schema(&mut self, value: Element) {
        self.base.base.base.base.object.set("submissionSchema", value);
    }

    pub fn target_schema(&self) -> Option<&Element> {
        self.base.base.base.base.object.get("targetSchema")
    }

    pub fn set_target_schema(&mut self, value: Element) {
        self.base.base.base.base.object.set("targetSchema", value);
    }
}