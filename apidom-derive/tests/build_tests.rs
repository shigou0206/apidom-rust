pub use std::collections::HashMap;
use apidom_derive::BuildFromElement;

// Mock core types ---------------------------------------------------------
#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    data: ElementData,
}

#[derive(Debug, PartialEq, Clone)]
enum ElementData {
    Object(Object),
    Array(Vec<Element>),
    String(String),
    Number(u64),
    Boolean(bool),
    Float(f64),
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub content: Vec<Member>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Member {
    pub key: Key,
    pub value: Element,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Key {
    pub content: String,
}

impl Key {
    fn new(s: &str) -> Self {
        Self { content: s.to_string() }
    }
    
    fn as_string(&self) -> Option<&Self> {
        Some(self)
    }
}

impl Element {
    fn object(content: Vec<Member>) -> Self {
        Self { data: ElementData::Object(Object { content }) }
    }
    
    fn string(s: &str) -> Self {
        Self { data: ElementData::String(s.to_string()) }
    }
    
    fn number(n: u64) -> Self {
        Self { data: ElementData::Number(n) }
    }
    
    fn boolean(b: bool) -> Self {
        Self { data: ElementData::Boolean(b) }
    }
    
    fn float(f: f64) -> Self {
        Self { data: ElementData::Float(f) }
    }
    
    fn array(items: Vec<Element>) -> Self {
        Self { data: ElementData::Array(items) }
    }

    fn as_object(&self) -> Option<&Object> {
        match &self.data {
            ElementData::Object(obj) => Some(obj),
            _ => None,
        }
    }

    fn get_str(&self, key: &str) -> Option<String> {
        self.as_object()?.get_str(key)
    }

    fn get_u64(&self, key: &str) -> Option<u64> {
        self.as_object()?.get_u64(key)
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        self.as_object()?.get_bool(key)
    }

    fn get_f64(&self, key: &str) -> Option<f64> {
        self.as_object()?.get_f64(key)
    }

    fn get_value(&self, key: &str) -> Option<&Element> {
        self.as_object()?.get_value(key)
    }

    fn get_array(&self, key: &str) -> Option<Vec<Element>> {
        self.as_object()?.get_array(key)
    }

    fn get_object(&self, key: &str) -> Option<&Object> {
        self.as_object()?.get_object(key)
    }

    fn get(&self, key: &str) -> Option<&Element> {
        self.as_object()?.get(key)
    }

    fn as_ref_string(&self) -> Option<String> {
        match &self.data {
            ElementData::String(s) if s.starts_with('#') => Some(s.clone()),
            _ => None,
        }
    }
}

impl Object {
    fn find_member(&self, key: &str) -> Option<&Member> {
        self.content.iter().find(|m| m.key.content == key)
    }

    fn get_str(&self, key: &str) -> Option<String> {
        match &self.find_member(key)?.value.data {
            ElementData::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    fn get_u64(&self, key: &str) -> Option<u64> {
        match &self.find_member(key)?.value.data {
            ElementData::Number(n) => Some(*n),
            _ => None,
        }
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        match &self.find_member(key)?.value.data {
            ElementData::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    fn get_f64(&self, key: &str) -> Option<f64> {
        match &self.find_member(key)?.value.data {
            ElementData::Float(f) => Some(*f),
            _ => None,
        }
    }

    fn get_value(&self, key: &str) -> Option<&Element> {
        Some(&self.find_member(key)?.value)
    }

    fn get_array(&self, key: &str) -> Option<Vec<Element>> {
        match &self.find_member(key)?.value.data {
            ElementData::Array(arr) => Some(arr.clone()),
            _ => None,
        }
    }

    fn get_object(&self, key: &str) -> Option<&Object> {
        self.find_member(key)?.value.as_object()
    }

    fn get(&self, key: &str) -> Option<&Element> {
        Some(&self.find_member(key)?.value)
    }
}

// Core trait
pub trait BuildFromElement: Sized {
    fn build_from_element(el: &Element) -> Option<Self>;
}

// Helper types
#[derive(Debug, PartialEq)]
struct SimpleValue {
    data: String,
}

impl SimpleValue {
    fn from_element(e: &Element) -> Self {
        match &e.data {
            ElementData::String(s) => SimpleValue { data: s.clone() },
            ElementData::Number(n) => SimpleValue { data: n.to_string() },
            ElementData::Boolean(b) => SimpleValue { data: b.to_string() },
            ElementData::Float(f) => SimpleValue { data: f.to_string() },
            _ => SimpleValue { data: "null".to_string() },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Reference {
    ref_field: String,
}

impl Reference {
    fn new(reference: String) -> Self {
        Self { ref_field: reference }
    }
}

#[derive(Debug, PartialEq)]
enum OrReference<T> {
    Item(T),
    Ref(Reference),
}

// Test structs -------------------------------------------------------------
#[derive(BuildFromElement, Debug, PartialEq)]
struct Tag {
    name: Option<String>,
}

#[derive(BuildFromElement, Debug, PartialEq)]
struct Category {
    name: Option<String>,
    description: Option<String>,
}

#[derive(BuildFromElement, Debug, PartialEq)]
struct EmptyStruct {}

#[derive(BuildFromElement, Debug, PartialEq)]
struct Pet {
    name: Option<String>,
    #[element(default)]
    age: u64,
    tags: Option<Vec<Tag>>,
    category: Option<OrReference<Category>>,
}

#[derive(BuildFromElement, Debug, PartialEq)]
struct ExtensibleStruct {
    name: Option<String>,
    #[element(extension)]
    extensions: HashMap<String, SimpleValue>,
}

#[derive(BuildFromElement, Debug, PartialEq)]
struct TypesStruct {
    name: Option<String>,
    active: Option<bool>,
    score: Option<f64>,
    count: u64,
    enabled: bool,
    rating: f64,
}

impl Default for TypesStruct {
    fn default() -> Self {
        Self {
            name: None,
            active: None,
            score: None,
            count: 0,
            enabled: false,
            rating: 0.0,
        }
    }
}

#[derive(BuildFromElement, Debug, PartialEq)]
struct BaseInfo {
    title: Option<String>,
    version: Option<String>,
}

impl Default for BaseInfo {
    fn default() -> Self {
        Self {
            title: None,
            version: None,
        }
    }
}

#[derive(BuildFromElement, Debug, PartialEq)]
struct FlattenedStruct {
    description: Option<String>,
    #[element(flatten)]
    info: BaseInfo,
}

impl Default for FlattenedStruct {
    fn default() -> Self {
        Self {
            description: None,
            info: BaseInfo::default(),
        }
    }
}

// Tests --------------------------------------------------------------------
#[test]
fn test_empty_struct() {
    let element = Element::object(vec![]);
    let result = EmptyStruct::build_from_element(&element);
    assert_eq!(result, Some(EmptyStruct {}));
}

#[test]
fn test_simple_fields() {
    let element = Element::object(vec![
        Member {
            key: Key::new("name"),
            value: Element::string("Fluffy"),
        },
        Member {
            key: Key::new("age"),
            value: Element::number(3),
        },
    ]);

    let result = Pet::build_from_element(&element).unwrap();
    assert_eq!(result.name, Some("Fluffy".to_string()));
    assert_eq!(result.age, 3);
}

#[test]
fn test_default_field() {
    let element = Element::object(vec![
        Member {
            key: Key::new("name"),
            value: Element::string("Buddy"),
        },
        // age field missing - should use default
    ]);

    let result = Pet::build_from_element(&element).unwrap();
    assert_eq!(result.name, Some("Buddy".to_string()));
    assert_eq!(result.age, 0); // default u64
}

#[test]
fn test_nested_vec() {
    let element = Element::object(vec![
        Member {
            key: Key::new("name"),
            value: Element::string("Rex"),
        },
        Member {
            key: Key::new("tags"),
            value: Element::array(vec![
                Element::object(vec![
                    Member {
                        key: Key::new("name"),
                        value: Element::string("friendly"),
                    }
                ]),
                Element::object(vec![
                    Member {
                        key: Key::new("name"),
                        value: Element::string("playful"),
                    }
                ]),
            ]),
        },
    ]);

    let result = Pet::build_from_element(&element).unwrap();
    assert_eq!(result.name, Some("Rex".to_string()));
    assert_eq!(result.tags, Some(vec![
        Tag { name: Some("friendly".to_string()) },
        Tag { name: Some("playful".to_string()) },
    ]));
}

#[test]
fn test_or_reference_item() {
    let element = Element::object(vec![
        Member {
            key: Key::new("category"),
            value: Element::object(vec![
                Member {
                    key: Key::new("name"),
                    value: Element::string("Dog"),
                },
            ]),
        },
    ]);

    let result = Pet::build_from_element(&element).unwrap();
    match result.category {
        Some(OrReference::Item(cat)) => {
            assert_eq!(cat.name, Some("Dog".to_string()));
        }
        _ => panic!("Expected OrReference::Item"),
    }
}

#[test]
fn test_or_reference_ref() {
    let element = Element::object(vec![
        Member {
            key: Key::new("category"),
            value: Element::string("#/components/schemas/Dog"),
        },
    ]);

    let result = Pet::build_from_element(&element).unwrap();
    match result.category {
        Some(OrReference::Ref(reference)) => {
            assert_eq!(reference.ref_field, "#/components/schemas/Dog");
        }
        _ => panic!("Expected OrReference::Ref"),
    }
}

#[test]
fn test_extensions() {
    let element = Element::object(vec![
        Member {
            key: Key::new("name"),
            value: Element::string("Test"),
        },
        Member {
            key: Key::new("x-custom"),
            value: Element::string("custom-value"),
        },
        Member {
            key: Key::new("x-another"),
            value: Element::number(42),
        },
        Member {
            key: Key::new("regular"),
            value: Element::string("ignored"),
        },
    ]);

    let result = ExtensibleStruct::build_from_element(&element).unwrap();
    assert_eq!(result.name, Some("Test".to_string()));
    assert_eq!(result.extensions.len(), 2);
    assert_eq!(result.extensions.get("x-custom").unwrap().data, "custom-value");
    assert_eq!(result.extensions.get("x-another").unwrap().data, "42");
    assert!(!result.extensions.contains_key("regular"));
}

#[test]
fn test_missing_object() {
    let element = Element::string("not an object");
    let result = Pet::build_from_element(&element);
    assert_eq!(result, None);
} 