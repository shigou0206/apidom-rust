use apidom_ast::minim_model::*;
use apidom_ns_json_schema_draft_4::elements::link_description::LinkDescriptionElement;
use std::fmt;

/// JSON Schema Draft-6 LinkDescription Element (Hyper-Schema)
#[derive(Debug, Clone)]
pub struct LinkDescriptionDraft6Element {
    pub base: LinkDescriptionElement,
}

impl LinkDescriptionDraft6Element {
    pub fn new() -> Self {
        let mut base = LinkDescriptionElement::new();
        base.object.set_element_type("linkDescription");
        Self { base }
    }

    pub fn with_content(obj: ObjectElement) -> Self {
        let mut base = LinkDescriptionElement::with_content(obj);
        base.object.set_element_type("linkDescription");
        Self { base }
    }

    // -----------------
    // Supported Fields
    // -----------------

    pub fn href_schema(&self) -> Option<&Element> {
        self.base.object.get("hrefSchema")
    }

    pub fn set_href_schema(&mut self, val: Element) {
        self.base.object.set("hrefSchema", val);
    }

    pub fn target_schema(&self) -> Option<&Element> {
        self.base.object.get("targetSchema")
    }

    pub fn set_target_schema(&mut self, val: Element) {
        self.base.object.set("targetSchema", val);
    }

    pub fn submission_schema(&self) -> Option<&Element> {
        self.base.object.get("submissionSchema")
    }

    pub fn set_submission_schema(&mut self, val: Element) {
        self.base.object.set("submissionSchema", val);
    }

    pub fn submission_enc_type(&self) -> Option<&StringElement> {
        self.base.object.get("submissionEncType").and_then(Element::as_string)
    }

    pub fn set_submission_enc_type(&mut self, val: StringElement) {
        self.base.object.set("submissionEncType", Element::String(val));
    }

    // -----------------
    // Unsupported Fields
    // -----------------

    pub fn schema(&self) -> Result<&Element, UnsupportedFieldError> {
        Err(UnsupportedFieldError("schema has been renamed to submissionSchema".into()))
    }

    pub fn method(&self) -> Result<&Element, UnsupportedFieldError> {
        Err(UnsupportedFieldError("method keyword has been removed".into()))
    }

    pub fn enc_type(&self) -> Result<&Element, UnsupportedFieldError> {
        Err(UnsupportedFieldError("encType has been renamed to submissionEncType".into()))
    }
}

/// Custom error type for removed/renamed fields
#[derive(Debug, Clone)]
pub struct UnsupportedFieldError(pub String);

impl fmt::Display for UnsupportedFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsupported field: {}", self.0)
    }
}

impl std::error::Error for UnsupportedFieldError {}