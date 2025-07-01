use std::collections::HashMap;
pub use apidom_ast::ObjectElement;

/// Field type enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    String,
    Number,
    Reference,
}

/// Field specification
#[derive(Debug, Clone)]
pub struct FieldSpec {
    pub name: String,
    pub field_type: FieldType,
    pub json_key: String,
}

impl FieldSpec {
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        let name_str = name.into();
        Self {
            name: name_str.clone(),
            field_type,
            json_key: name_str,
        }
    }

    pub fn with_json_key(mut self, json_key: impl Into<String>) -> Self {
        self.json_key = json_key.into();
        self
    }
}

/// Field specification trait
pub trait DtoFieldSpecs {
    fn field_specs() -> Vec<FieldSpec>;
}

/// Object element conversion trait
pub trait FromObjectElement {
    fn from_object_element(obj: &ObjectElement) -> Self;
}

/// Object element extension trait
pub trait ObjectElementExt {
    fn get_string(&self, key: &str) -> Option<String>;
    fn get_number(&self, key: &str) -> Option<f64>;
    fn get_extensions(&self) -> HashMap<String, String>;
} 