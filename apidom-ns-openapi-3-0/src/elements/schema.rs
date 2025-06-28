use apidom_ast::minim_model::*;
use apidom_ns_json_schema_draft_4::elements::json_schema::JSONSchemaDraft4Element;

/// OpenAPI Schema Element based on JSON Schema Draft-4
#[derive(Debug, Clone)]
pub struct OpenApiSchemaElement {
    pub base: JSONSchemaDraft4Element,
}

impl OpenApiSchemaElement {
    pub fn new() -> Self {
        let mut base = JSONSchemaDraft4Element::new();
        base.object.set_element_type("schema");
        base.object.add_class("json-schema-draft-4");
        Self { base }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut base = JSONSchemaDraft4Element::with_content(content);
        base.object.set_element_type("schema");
        base.object.add_class("json-schema-draft-4");
        Self { base }
    }

    // ========== JSON Schema 部分字段可复用 base ==========
    pub fn set_type(&mut self, val: StringElement) {
        self.base.set_type(Element::String(val));
    }

    pub fn type_(&self) -> Option<&StringElement> {
        self.base.type_().and_then(Element::as_string)
    }

    // ========== OpenAPI 扩展字段 ==========

    pub fn nullable(&self) -> Option<&BooleanElement> {
        self.base.object.get("nullable").and_then(Element::as_boolean)
    }

    pub fn set_nullable(&mut self, val: BooleanElement) {
        self.base.object.set("nullable", Element::Boolean(val));
    }

    pub fn discriminator(&self) -> Option<&ObjectElement> {
        self.base.object.get("discriminator").and_then(Element::as_object)
    }

    pub fn set_discriminator(&mut self, val: ObjectElement) {
        self.base.object.set("discriminator", Element::Object(val));
    }

    pub fn xml(&self) -> Option<&ObjectElement> {
        self.base.object.get("xml").and_then(Element::as_object)
    }

    pub fn set_xml(&mut self, val: ObjectElement) {
        self.base.object.set("xml", Element::Object(val));
    }

    pub fn external_docs(&self) -> Option<&ObjectElement> {
        self.base.object.get("externalDocs").and_then(Element::as_object)
    }

    pub fn set_external_docs(&mut self, val: ObjectElement) {
        self.base.object.set("externalDocs", Element::Object(val));
    }

    pub fn example(&self) -> Option<&Element> {
        self.base.object.get("example")
    }

    pub fn set_example(&mut self, val: Element) {
        self.base.object.set("example", val);
    }

    pub fn deprecated(&self) -> Option<&BooleanElement> {
        self.base.object.get("deprecated").and_then(Element::as_boolean)
    }

    pub fn set_deprecated(&mut self, val: BooleanElement) {
        self.base.object.set("deprecated", Element::Boolean(val));
    }

    pub fn write_only(&self) -> Option<&BooleanElement> {
        self.base.object.get("writeOnly").and_then(Element::as_boolean)
    }

    pub fn set_write_only(&mut self, val: BooleanElement) {
        self.base.object.set("writeOnly", Element::Boolean(val));
    }
}