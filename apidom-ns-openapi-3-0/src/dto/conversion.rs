//! Common DTO Conversion Utilities
//! 
//! This module provides reusable infrastructure for converting AST elements to DTOs.
//! It centralizes common patterns like field extraction, type conversion, and extension handling.

use serde_json::Value;
use std::collections::HashMap;
use crate::dto::{Extensions, json_value_to_extension_string};
use apidom_ast::minim_model::*;

// ==================== 通用字段访问 Trait ====================

/// ObjectElement 扩展 trait，提供类型安全的字段访问
/// 这是所有 DTO 转换的基础，确保一致的字段访问方式
pub trait ObjectElementExt {
    fn get_string(&self, key: &str) -> Option<String>;
    fn get_number(&self, key: &str) -> Option<f64>;
    fn get_bool(&self, key: &str) -> Option<bool>;
    fn get_element(&self, key: &str) -> Option<&Element>;
    fn get_array(&self, key: &str) -> Option<&ArrayElement>;
    fn get_object(&self, key: &str) -> Option<&ObjectElement>;
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
    
    fn get_bool(&self, key: &str) -> Option<bool> {
        if let Some(Element::Boolean(b)) = self.get(key) {
            Some(b.content)
        } else {
            None
        }
    }
    
    fn get_element(&self, key: &str) -> Option<&Element> {
        self.get(key)
    }
    
    fn get_array(&self, key: &str) -> Option<&ArrayElement> {
        if let Some(Element::Array(arr)) = self.get(key) {
            Some(arr)
        } else {
            None
        }
    }
    
    fn get_object(&self, key: &str) -> Option<&ObjectElement> {
        if let Some(Element::Object(obj)) = self.get(key) {
            Some(obj)
        } else {
            None
        }
    }
}

// ==================== 通用字段提取宏 ====================

/// 字符串字段提取宏
/// 
/// 支持两种用法：
/// - `extract_string_field!(obj, dto, field_name)` - 使用字段名作为键
/// - `extract_string_field!(obj, dto, field_name, "customKey")` - 使用自定义键
#[macro_export]
macro_rules! extract_string_field {
    ($obj:expr, $dto:expr, $field:ident) => {
        if let Some(val) = $obj.get_string(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    ($obj:expr, $dto:expr, $field:ident, $key:expr) => {
        if let Some(val) = $obj.get_string($key) {
            $dto.$field = Some(val);
        }
    };
}

/// 数值字段提取宏
/// 
/// 支持多种用法：
/// - `extract_number_field!(obj, dto, field_name)` - 直接提取 f64
/// - `extract_number_field!(obj, dto, field_name, "customKey")` - 使用自定义键
/// - `extract_number_field!(obj, dto, field_name, "customKey", as usize)` - 类型转换
#[macro_export]
macro_rules! extract_number_field {
    ($obj:expr, $dto:expr, $field:ident) => {
        if let Some(val) = $obj.get_number(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    ($obj:expr, $dto:expr, $field:ident, $key:expr) => {
        if let Some(val) = $obj.get_number($key) {
            $dto.$field = Some(val);
        }
    };
    ($obj:expr, $dto:expr, $field:ident, $key:expr, as usize) => {
        if let Some(val) = $obj.get_number($key) {
            $dto.$field = Some(val as usize);
        }
    };
    ($obj:expr, $dto:expr, $field:ident, $key:expr, as i32) => {
        if let Some(val) = $obj.get_number($key) {
            $dto.$field = Some(val as i32);
        }
    };
}

/// 布尔字段提取宏
/// 
/// 支持两种用法：
/// - `extract_bool_field!(obj, dto, field_name)` - 使用字段名作为键
/// - `extract_bool_field!(obj, dto, field_name, "customKey")` - 使用自定义键
#[macro_export]
macro_rules! extract_bool_field {
    ($obj:expr, $dto:expr, $field:ident) => {
        if let Some(val) = $obj.get_bool(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    ($obj:expr, $dto:expr, $field:ident, $key:expr) => {
        if let Some(val) = $obj.get_bool($key) {
            $dto.$field = Some(val);
        }
    };
}

/// JSON 值字段提取宏
/// 
/// 将 Element 转换为 JSON 字符串存储在 DTO 中
#[macro_export]
macro_rules! extract_json_field {
    ($obj:expr, $dto:expr, $field:ident) => {
        if let Some(element) = $obj.get_element(stringify!($field)) {
            $dto.$field = Some($crate::dto::conversion::element_to_json_string(element));
        }
    };
    ($obj:expr, $dto:expr, $field:ident, $key:expr) => {
        if let Some(element) = $obj.get_element($key) {
            $dto.$field = Some($crate::dto::conversion::element_to_json_string(element));
        }
    };
}

// ==================== Element 转换函数 ====================

/// 将 Element 转换为 JSON Value
/// 这是所有 JSON 转换的基础函数
pub fn element_to_json_value(element: &Element) -> serde_json::Value {
    match element {
        Element::Null(_) => Value::Null,
        Element::Boolean(b) => Value::Bool(b.content),
        Element::Number(n) => {
            if n.content.fract() == 0.0 {
                Value::Number(serde_json::Number::from(n.content as i64))
            } else {
                Value::Number(serde_json::Number::from_f64(n.content).unwrap_or_else(|| serde_json::Number::from(0)))
            }
        },
        Element::String(s) => Value::String(s.content.clone()),
        Element::Array(arr) => {
            let values: Vec<Value> = arr.content.iter()
                .map(element_to_json_value)
                .collect();
            Value::Array(values)
        },
        Element::Object(obj) => {
            let mut map = serde_json::Map::new();
            for member in &obj.content {
                if let Element::String(key) = &*member.key {
                    map.insert(key.content.clone(), element_to_json_value(&member.value));
                }
            }
            Value::Object(map)
        },
        // 处理其他 Element 变体
        Element::Member(_) => Value::Null,
        Element::Ref(_) => Value::Null,
        Element::Link(_) => Value::Null,
        _ => Value::Null,
    }
}

/// 将 Element 转换为 JSON 字符串
/// 用于需要字符串存储 JSON 的场景（如 Flutter Rust Bridge 兼容性）
pub fn element_to_json_string(element: &Element) -> String {
    let json_value = element_to_json_value(element);
    serde_json::to_string(&json_value).unwrap_or_default()
}

// ==================== 扩展字段提取 ====================

/// 通用扩展字段提取器
/// 
/// 这个结构体可以配置哪些字段应该被忽略，哪些应该被包含为扩展字段
pub struct ExtensionExtractor {
    /// 已知字段列表，这些字段不会被当作扩展字段
    known_fields: Vec<String>,
    /// 是否包含非 x- 开头的未知字段
    include_unknown_fields: bool,
}

impl ExtensionExtractor {
    /// 创建新的扩展字段提取器
    pub fn new() -> Self {
        Self {
            known_fields: Vec::new(),
            include_unknown_fields: false,
        }
    }
    
    /// 添加已知字段
    pub fn with_known_fields(mut self, fields: &[&str]) -> Self {
        self.known_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }
    
    /// 设置是否包含未知字段
    pub fn include_unknown_fields(mut self, include: bool) -> Self {
        self.include_unknown_fields = include;
        self
    }
    
    /// 从 ObjectElement 提取扩展字段
    pub fn extract(&self, obj: &ObjectElement) -> Extensions {
        let mut extensions = Extensions::new();
        
        for member in &obj.content {
            if let Element::String(key) = &*member.key {
                let key_str = &key.content;
                
                // 检查是否应该包含这个字段
                let should_include = if key_str.starts_with("x-") {
                    // 所有 x- 字段都包含
                    true
                } else if self.include_unknown_fields && !self.known_fields.contains(key_str) {
                    // 包含未知字段，且这个字段不在已知字段列表中
                    true
                } else {
                    false
                };
                
                if should_include {
                    let value_str = json_value_to_extension_string(&element_to_json_value(&member.value));
                    extensions.insert(key_str.clone(), value_str);
                }
            }
        }
        
        extensions
    }
}

impl Default for ExtensionExtractor {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== 通用转换辅助函数 ====================

/// 提取字符串数组（如 required 字段）
pub fn extract_string_array(obj: &ObjectElement, key: &str) -> Option<Vec<String>> {
    if let Some(arr) = obj.get_array(key) {
        let mut strings = Vec::new();
        for item in &arr.content {
            if let Element::String(s) = item {
                strings.push(s.content.clone());
            }
        }
        if !strings.is_empty() {
            Some(strings)
        } else {
            None
        }
    } else {
        None
    }
}

/// 提取字符串映射（如 properties）
pub fn extract_string_map(obj: &ObjectElement, key: &str) -> Option<HashMap<String, String>> {
    if let Some(map_obj) = obj.get_object(key) {
        let mut map = HashMap::new();
        for member in &map_obj.content {
            if let Element::String(prop_key) = &*member.key {
                if let Element::String(prop_value) = &*member.value {
                    map.insert(prop_key.content.clone(), prop_value.content.clone());
                }
            }
        }
        if !map.is_empty() {
            Some(map)
        } else {
            None
        }
    } else {
        None
    }
}

/// 提取引用字段
pub fn extract_reference(obj: &ObjectElement) -> Option<String> {
    obj.get_string("$ref")
}

// ==================== DTO 转换 Trait 扩展 ====================

/// DTO 字段提取器 trait
/// 提供标准化的字段提取方法
pub trait DtoFieldExtractor {
    /// 提取基础字符串字段
    fn extract_basic_strings(&self, obj: &ObjectElement, known_fields: &[&str]);
    
    /// 提取扩展字段
    fn extract_extensions(&mut self, obj: &ObjectElement, known_fields: &[&str]);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_object_element_ext_trait() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Test Title")));
        obj.set("count", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 42.0,
        }));
        obj.set("enabled", Element::Boolean(BooleanElement::new(true)));
        
        assert_eq!(obj.get_string("title"), Some("Test Title".to_string()));
        assert_eq!(obj.get_number("count"), Some(42.0));
        assert_eq!(obj.get_bool("enabled"), Some(true));
        assert!(obj.get_string("nonexistent").is_none());
    }
    
    #[test]
    fn test_element_to_json_value() {
        // 测试字符串
        let str_elem = Element::String(StringElement::new("test"));
        assert_eq!(element_to_json_value(&str_elem), Value::String("test".to_string()));
        
        // 测试数字
        let num_elem = Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 3.14,
        });
        if let Value::Number(n) = element_to_json_value(&num_elem) {
            assert_eq!(n.as_f64().unwrap(), 3.14);
        } else {
            panic!("Expected number");
        }
        
        // 测试布尔值
        let bool_elem = Element::Boolean(BooleanElement::new(true));
        assert_eq!(element_to_json_value(&bool_elem), Value::Bool(true));
    }
    
    #[test]
    fn test_extension_extractor() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Test")));
        obj.set("x-custom", Element::String(StringElement::new("custom")));
        obj.set("x-flag", Element::Boolean(BooleanElement::new(true)));
        obj.set("unknown", Element::String(StringElement::new("unknown")));
        
        // 只提取 x- 字段
        let extractor = ExtensionExtractor::new()
            .with_known_fields(&["title"]);
        let extensions = extractor.extract(&obj);
        
        assert_eq!(extensions.get("x-custom"), Some(&"custom".to_string()));
        assert_eq!(extensions.get("x-flag"), Some(&"true".to_string()));
        assert!(!extensions.contains_key("title"));
        assert!(!extensions.contains_key("unknown"));
        
        // 包含未知字段
        let extractor_with_unknown = ExtensionExtractor::new()
            .with_known_fields(&["title"])
            .include_unknown_fields(true);
        let extensions_with_unknown = extractor_with_unknown.extract(&obj);
        
        assert_eq!(extensions_with_unknown.get("x-custom"), Some(&"custom".to_string()));
        assert_eq!(extensions_with_unknown.get("unknown"), Some(&"unknown".to_string()));
        assert!(!extensions_with_unknown.contains_key("title"));
    }
    
    #[test]
    fn test_extract_string_array() {
        let mut obj = ObjectElement::new();
        
        // 创建字符串数组
        let mut arr = ArrayElement::new_empty();
        arr.content.push(Element::String(StringElement::new("field1")));
        arr.content.push(Element::String(StringElement::new("field2")));
        obj.set("required", Element::Array(arr));
        
        let result = extract_string_array(&obj, "required");
        assert_eq!(result, Some(vec!["field1".to_string(), "field2".to_string()]));
        
        // 测试不存在的字段
        let result = extract_string_array(&obj, "nonexistent");
        assert_eq!(result, None);
    }
    
    #[test] 
    fn test_extract_reference() {
        let mut obj = ObjectElement::new();
        obj.set("$ref", Element::String(StringElement::new("#/components/schemas/User")));
        
        let result = extract_reference(&obj);
        assert_eq!(result, Some("#/components/schemas/User".to_string()));
    }
} 