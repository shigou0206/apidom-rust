use std::collections::HashMap;
use apidom_dto_derive::{DtoSpec, FromObjectElement, IntoFrbDto};
use serde::{Serialize, Deserialize};
use apidom_ast::minim_model::*;

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

#[derive(Debug, PartialEq)]
pub enum FieldType {
    String,
    Number,
    Reference,
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

// #[derive(Debug, Clone, Default, DtoSpec, FromObjectElement, IntoFrbDto)]
// pub struct TestDto {
//     pub title: Option<String>,
//     pub description: Option<String>,
//     #[dto(usize)]
//     pub count: Option<usize>,
//     #[dto(extensions)]
//     pub extensions: HashMap<String, String>,
// }

#[test]
fn test_frb_conversion() {
    // Create test DTO
    let dto = TestDto {
        title: Some("Test".to_string()),
        description: Some("A test DTO".to_string()),
        count: Some(42),
        extensions: {
            let mut map = HashMap::new();
            map.insert("x-test".to_string(), "value".to_string());
            map
        },
    };
    
    // Convert to FRB DTO
    let frb_dto: TestDtoFrb = dto.clone().into();
    
    // Verify FRB DTO fields
    assert_eq!(frb_dto.title, dto.title);
    assert_eq!(frb_dto.description, dto.description);
    assert_eq!(frb_dto.count.map(|v| v as usize), dto.count);
    assert_eq!(frb_dto.extensions, dto.extensions);
    
    // Convert back to original DTO
    let converted_dto: TestDto = frb_dto.into();
    
    // Verify conversion back
    assert_eq!(converted_dto.title, dto.title);
    assert_eq!(converted_dto.description, dto.description);
    assert_eq!(converted_dto.count, dto.count);
    assert_eq!(converted_dto.extensions, dto.extensions);
} 