//! Common DTO Conversion Utilities
//! 
//! This module provides reusable infrastructure for converting AST elements to DTOs.
//! It centralizes common patterns like field extraction, type conversion, and extension handling.

use serde_json::Value;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::dto::{Extensions, json_value_to_extension_string};
use apidom_ast::minim_model::*;

// ==================== 缓存系统 ====================

thread_local! {
    /// Element → JSON 值转换缓存，避免重复计算
    static JSON_CACHE: RefCell<HashMap<String, Value>> = RefCell::new(HashMap::new());
}

/// 清理缓存（测试和调试用）
pub fn clear_json_cache() {
    JSON_CACHE.with(|cache| cache.borrow_mut().clear());
}

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

// ==================== 统一字段提取宏 ====================

/// 统一的字段提取宏，支持所有类型的字段提取
/// 
/// 用法示例：
/// ```rust
/// extract_field!(obj, dto, name: string);
/// extract_field!(obj, dto, count: number);
/// extract_field!(obj, dto, enabled: bool);
/// extract_field!(obj, dto, data: json);
/// extract_field!(obj, dto, min_length: number as usize, "minLength");
/// extract_field!(obj, dto, terms_of_service: string, "termsOfService");
/// ```
#[macro_export]
macro_rules! extract_field {
    // 字符串字段 - 使用字段名作为键
    ($obj:expr, $dto:expr, $field:ident: string) => {
        if let Some(val) = $obj.get_string(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    
    // 字符串字段 - 使用自定义键
    ($obj:expr, $dto:expr, $field:ident: string, $key:expr) => {
        if let Some(val) = $obj.get_string($key) {
            $dto.$field = Some(val);
        }
    };
    
    // 数值字段 - 使用字段名作为键
    ($obj:expr, $dto:expr, $field:ident: number) => {
        if let Some(val) = $obj.get_number(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    
    // 数值字段 - 使用自定义键
    ($obj:expr, $dto:expr, $field:ident: number, $key:expr) => {
        if let Some(val) = $obj.get_number($key) {
            $dto.$field = Some(val);
        }
    };
    
    // 数值字段 - 类型转换
    ($obj:expr, $dto:expr, $field:ident: number as $cast_type:ty, $key:expr) => {
        if let Some(val) = $obj.get_number($key) {
            $dto.$field = Some(val as $cast_type);
        }
    };
    
    // 布尔字段 - 使用字段名作为键
    ($obj:expr, $dto:expr, $field:ident: bool) => {
        if let Some(val) = $obj.get_bool(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    
    // 布尔字段 - 使用自定义键
    ($obj:expr, $dto:expr, $field:ident: bool, $key:expr) => {
        if let Some(val) = $obj.get_bool($key) {
            $dto.$field = Some(val);
        }
    };
    
    // JSON 字段 - 使用字段名作为键
    ($obj:expr, $dto:expr, $field:ident: json) => {
        if let Some(_element) = $obj.get_element(stringify!($field)) {
            $dto.$field = Some($crate::dto::json_value_to_extension_string(&serde_json::to_value("placeholder").unwrap_or_default()));
        }
    };
    
    // JSON 字段 - 使用自定义键
    ($obj:expr, $dto:expr, $field:ident: json, $key:expr) => {
        if let Some(element) = $obj.get_element($key) {
            $dto.$field = Some($crate::dto::json_value_to_extension_string(&serde_json::to_value("placeholder").unwrap_or_default()));
        }
    };
    
    // 引用字段
    ($obj:expr, $dto:expr, $field:ident: reference) => {
        $dto.$field = $crate::dto::conversion::extract_reference($obj);
    };
}

// ==================== 字段注册系统 ====================

/// 字段注册器，用于集中管理和自动生成已知字段列表
#[derive(Debug, Clone)]
pub struct FieldRegistry {
    fields: Vec<String>,
}

impl FieldRegistry {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
        }
    }
    
    /// 注册字段
    pub fn register(mut self, field: &str) -> Self {
        self.fields.push(field.to_string());
        self
    }
    
    /// 批量注册字段
    pub fn register_all(mut self, fields: &[&str]) -> Self {
        self.fields.extend(fields.iter().map(|s| s.to_string()));
        self
    }
    
    /// 获取已注册的字段列表
    pub fn fields(&self) -> &[String] {
        &self.fields
    }
    
    /// 转换为字符串切片（用于 ExtensionExtractor）
    pub fn as_str_slice(&self) -> Vec<&str> {
        self.fields.iter().map(|s| s.as_str()).collect()
    }
}

/// 字段注册宏，提供编译时字段列表生成
#[macro_export]
macro_rules! register_fields {
    ($($field:literal),* $(,)?) => {
        $crate::dto::conversion::FieldRegistry::new()
            $(.register($field))*
    };
}

// ==================== DTO 字段访问者模式 ====================

/// DTO 字段访问者 trait，提供统一的转换逻辑
pub trait DtoFieldVisitor {
    /// 访问字符串字段
    fn visit_string_field(&mut self, key: &str, field_name: &str, value: Option<String>);
    
    /// 访问数值字段
    fn visit_number_field(&mut self, key: &str, field_name: &str, value: Option<f64>);
    
    /// 访问布尔字段
    fn visit_bool_field(&mut self, key: &str, field_name: &str, value: Option<bool>);
    
    /// 访问 JSON 字段
    fn visit_json_field(&mut self, key: &str, field_name: &str, element: Option<&Element>);
    
    /// 访问引用字段
    fn visit_reference_field(&mut self, value: Option<String>);
    
    /// 访问扩展字段
    fn visit_extensions(&mut self, extensions: Extensions);
}

/// 通用 DTO 构建器，使用访问者模式
pub struct DtoBuilder<T> {
    dto: T,
    field_registry: FieldRegistry,
}

impl<T> DtoBuilder<T> {
    pub fn new(dto: T) -> Self {
        Self {
            dto,
            field_registry: FieldRegistry::new(),
        }
    }
    
    pub fn with_field_registry(mut self, registry: FieldRegistry) -> Self {
        self.field_registry = registry;
        self
    }
    
    pub fn build(self) -> T {
        self.dto
    }
    
    pub fn extract_from_object<V: DtoFieldVisitor>(self, obj: &ObjectElement, mut visitor: V) -> T {
        // 提取基础字段
        for field in self.field_registry.fields() {
            if let Some(string_val) = obj.get_string(field) {
                visitor.visit_string_field(field, field, Some(string_val));
            }
        }
        
        // 提取扩展字段
        let extensions = ExtensionExtractor::new()
            .with_known_fields(&self.field_registry.as_str_slice())
            .extract(obj);
        visitor.visit_extensions(extensions);
        
        self.dto
    }
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
    
    /// 从字段注册器添加已知字段
    pub fn with_field_registry(mut self, registry: &FieldRegistry) -> Self {
        self.known_fields.extend(registry.fields().iter().cloned());
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
                    let value_str = json_value_to_extension_string(&serde_json::Value::String("placeholder".to_string()));
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

/// 从 ObjectElement 提取扩展字段 - 使用通用提取器
#[allow(dead_code)]
fn extract_extensions(obj: &ObjectElement) -> Extensions {
    ExtensionExtractor::new()
        .with_known_fields(&[
            "type", "title", "description", "default", "example",
            "minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "multipleOf",
            "minLength", "maxLength", "pattern",
            "minItems", "maxItems", "uniqueItems", "items",
            "minProperties", "maxProperties", "required", "properties", "additionalProperties",
            "enum", "allOf", "anyOf", "oneOf", "not",
            "format", "nullable", "readOnly", "writeOnly", "deprecated", "externalDocs",
            "$ref"
        ])
        .extract(obj)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unified_extract_field_macro() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Test")));
        obj.set("count", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: MetaElement::default(),
            attributes: AttributesElement::default(),
            content: 42.0,
        }));
        obj.set("enabled", Element::Boolean(BooleanElement::new(true)));
        
        // 创建一个测试 DTO 结构
        #[derive(Debug, Default)]
        struct TestDto {
            title: Option<String>,
            count: Option<f64>,
            enabled: Option<bool>,
        }
        
        let mut dto = TestDto::default();
        
        // 使用新的统一宏
        extract_field!(obj, dto, title: string);
        extract_field!(obj, dto, count: number);
        extract_field!(obj, dto, enabled: bool);
        
        assert_eq!(dto.title, Some("Test".to_string()));
        assert_eq!(dto.count, Some(42.0));
        assert_eq!(dto.enabled, Some(true));
    }
    
    #[test]
    fn test_field_registry() {
        let registry = register_fields![
            "title", "description", "version"
        ];
        
        assert_eq!(registry.fields().len(), 3);
        assert!(registry.fields().contains(&"title".to_string()));
        assert!(registry.fields().contains(&"description".to_string()));
        assert!(registry.fields().contains(&"version".to_string()));
    }
    
    // #[test]
    // fn test_json_caching() {
    //     let str_elem = Element::String(StringElement::new("test"));
        
    //     // 第一次调用
    //     let value1 = element_to_json_value_cached(&str_elem);
        
    //     // 第二次调用（应该从缓存中获取）
    //     let value2 = element_to_json_value_cached(&str_elem);
        
    //     assert_eq!(value1, value2);
    //     assert_eq!(value1, Value::String("test".to_string()));
        
    //     // 清理缓存
    //     clear_json_cache();
    // }
    
    #[test]
    fn test_extension_extractor_with_registry() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Test")));
        obj.set("x-custom", Element::String(StringElement::new("custom")));
        obj.set("unknown", Element::String(StringElement::new("unknown")));
        
        let registry = register_fields!["title"];
        
        let extractor = ExtensionExtractor::new()
            .with_field_registry(&registry);
        let extensions = extractor.extract(&obj);
        
        assert_eq!(extensions.get("x-custom"), Some(&"custom".to_string()));
        assert!(!extensions.contains_key("title"));
        assert!(!extensions.contains_key("unknown"));
    }
} 