use std::collections::HashMap;
use apidom_dto_derive::{DtoSpec, FromObjectElement};
use apidom_ast::*;

pub trait DtoFieldSpecs {
    fn field_specs() -> Vec<FieldSpec>;
}

pub trait FromObjectElement {
    fn from_object_element(obj: &ObjectElement) -> Self;
}

pub trait ObjectElementExt {
    fn get_string(&self, key: &str) -> Option<String>;
    fn get_number(&self, key: &str) -> Option<f64>;
    fn get_extensions(&self) -> HashMap<String, String>;
}

impl ObjectElementExt for ObjectElement {
    fn get_string(&self, key: &str) -> Option<String> {
        if let Some(Element::String(s)) = self.get(key) {
            Some(s.content.clone())
        } else {
            None
        }
    }
    
    fn get_number(&self, key: &str) -> Option<f64> {
        if let Some(Element::Number(n)) = self.get(key) {
            Some(n.content)
        } else {
            None
        }
    }
    
    fn get_extensions(&self) -> HashMap<String, String> {
        let mut extensions = HashMap::new();
        for member in &self.content {
            if let Element::String(key) = member.key.as_ref() {
                if key.content.starts_with("x-") {
                    if let Element::String(value) = member.value.as_ref() {
                        extensions.insert(key.content.clone(), value.content.clone());
                    }
                }
            }
        }
        extensions
    }
}

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

#[derive(Debug, PartialEq)]
pub enum FieldType {
    String,
    Number,
    Reference,
}

// #[derive(Debug, Clone, Default, DtoSpec, FromObjectElement)]
// pub struct ExtensionsDto {
//     pub title: Option<String>,
//     #[dto(extensions)]
//     pub extensions: HashMap<String, String>,
// }

#[test]
fn test_extensions_conversion() {
    // Create test data
    let mut obj = ObjectElement::new();
    obj.set("title", Element::String(StringElement::new("Test")));
    obj.set("x-test1", Element::String(StringElement::new("value1")));
    obj.set("x-test2", Element::String(StringElement::new("value2")));
    obj.set("normal-field", Element::String(StringElement::new("normal")));
    
    // Test conversion
    let dto = ExtensionsDto::from_object_element(&obj);
    
    // Verify results
    assert_eq!(dto.title, Some("Test".to_string()));
    assert_eq!(dto.extensions.len(), 2);
    assert_eq!(dto.extensions.get("x-test1"), Some(&"value1".to_string()));
    assert_eq!(dto.extensions.get("x-test2"), Some(&"value2".to_string()));
    assert!(dto.extensions.get("normal-field").is_none());
}

#[test]
fn test_field_specs_with_extensions() {
    let specs = ExtensionsDto::field_specs();
    assert_eq!(specs.len(), 1);
    
    assert_eq!(specs[0].name, "title");
    assert_eq!(specs[0].json_key, "title");
    assert_eq!(specs[0].field_type, FieldType::String);
} 