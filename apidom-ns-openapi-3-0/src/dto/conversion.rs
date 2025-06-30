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

/// 统一的 Element → JSON 值转换函数（带缓存）
pub fn element_to_json_value_cached(element: &Element) -> Value {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    
    // 计算元素的哈希值作为缓存键
    let mut hasher = DefaultHasher::new();
    match element {
        Element::String(s) => s.content.hash(&mut hasher),
        Element::Number(n) => n.content.to_string().hash(&mut hasher),
        Element::Boolean(b) => b.content.hash(&mut hasher),
        Element::Null(_) => "null".hash(&mut hasher),
        Element::Array(arr) => {
            for elem in &arr.content {
                format!("{:?}", elem).hash(&mut hasher);
            }
        }
        Element::Object(obj) => {
            for member in &obj.content {
                format!("{:?}", member.key).hash(&mut hasher);
                format!("{:?}", member.value).hash(&mut hasher);
            }
        }
        _ => format!("{:?}", element).hash(&mut hasher),
    }
    let cache_key = hasher.finish().to_string();
    
    // 尝试从缓存获取
    JSON_CACHE.with(|cache| {
        if let Some(cached_value) = cache.borrow().get(&cache_key) {
            return cached_value.clone();
        }
        
        // 缓存未命中，执行转换
        let value = match element {
            Element::String(s) => Value::String(s.content.clone()),
            Element::Number(n) => {
                if n.content.fract() == 0.0 {
                    Value::Number(serde_json::Number::from(n.content as i64))
                } else {
                    Value::Number(serde_json::Number::from_f64(n.content).unwrap_or(serde_json::Number::from(0)))
                }
            }
            Element::Boolean(b) => Value::Bool(b.content),
            Element::Null(_) => Value::Null,
            Element::Array(arr) => {
                Value::Array(arr.content.iter().map(element_to_json_value_cached).collect())
            }
            Element::Object(obj) => {
                let mut map = serde_json::Map::new();
                for member in &obj.content {
                    if let Element::String(key) = &*member.key {
                        map.insert(
                            key.content.clone(),
                            element_to_json_value_cached(&member.value)
                        );
                    }
                }
                Value::Object(map)
            }
            _ => Value::Null,
        };
        
        // 存入缓存
        cache.borrow_mut().insert(cache_key, value.clone());
        value
    })
}

// ==================== 自动 Setter Trait ====================

/// 为 DTO 提供自动的 setter 方法
pub trait DtoSetter {
    /// 设置字段值
    fn set_field(&mut self, field_name: &str, value: &Element) -> bool;
    
    /// 获取字段规范
    fn field_specs() -> Vec<FieldSpec> where Self: Sized;
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

// ==================== 字段规范系统 ====================

/// 为 DTO 字段生成 FieldSpec 的派生宏
pub trait DeriveFieldSpec {
    fn field_specs() -> Vec<FieldSpec>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldSpec {
    name: String,
    json_key: String,  // 新增: JSON key alias
    field_type: FieldType,
    is_required: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    String,
    Number,
    NumberAs(NumberType),  // 新增: 支持数值类型转换
    Boolean,
    Json,
    Reference,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberType {
    USize,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

impl FieldSpec {
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        let name_str = name.into();
        Self {
            json_key: name_str.clone(),  // 默认使用相同的 key
            name: name_str,
            field_type,
            is_required: false,
        }
    }

    pub fn with_json_key(mut self, json_key: impl Into<String>) -> Self {
        self.json_key = json_key.into();
        self
    }

    pub fn required(mut self) -> Self {
        self.is_required = true;
        self
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn json_key(&self) -> &str {
        &self.json_key
    }
    
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }
    
    pub fn is_required(&self) -> bool {
        self.is_required
    }
}

// 为 FieldType 添加便捷构造函数
impl FieldType {
    pub fn number_as(number_type: NumberType) -> Self {
        Self::NumberAs(number_type)
    }
    
    pub fn usize() -> Self {
        Self::NumberAs(NumberType::USize)
    }
    
    pub fn i32() -> Self {
        Self::NumberAs(NumberType::I32)
    }
    
    pub fn u32() -> Self {
        Self::NumberAs(NumberType::U32)
    }
    
    pub fn i64() -> Self {
        Self::NumberAs(NumberType::I64)
    }
    
    pub fn u64() -> Self {
        Self::NumberAs(NumberType::U64)
    }
    
    pub fn f32() -> Self {
        Self::NumberAs(NumberType::F32)
    }
    
    pub fn f64() -> Self {
        Self::NumberAs(NumberType::F64)
    }
}

#[derive(Debug, Default)]
pub struct FieldSpecs {
    fields: Vec<FieldSpec>,
}

impl FieldSpecs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, spec: FieldSpec) -> Self {
        self.fields.push(spec);
        self
    }

    pub fn known_field_names(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.name.as_str()).collect()
    }
}

// ==================== 统一字段提取宏 ====================

/// Extract fields from an object element into a DTO.
/// Supports the following types:
/// - string: String fields
/// - number: Number fields
/// - usize: Number fields converted to usize
/// - bool: Boolean fields
/// - json: JSON fields
/// - reference: Reference fields
///
/// # Examples
///
/// ```rust
/// use apidom_ns_openapi_3_0::extract_field;
/// use apidom_ast::minim_model::ObjectElement;
/// use apidom_ns_openapi_3_0::dto::ObjectElementExt;
/// # struct SchemaDto { min_length: Option<usize>, pattern: Option<String> }
/// # impl Default for SchemaDto { fn default() -> Self { Self { min_length: None, pattern: None } } }
/// # let obj = ObjectElement::new();
/// # let mut dto = SchemaDto::default();
/// extract_field!(obj => dto.min_length: usize, "minLength");
/// extract_field!(obj => dto.pattern: string);
/// ```
#[macro_export]
macro_rules! extract_field {
    // 数值字段 - 类型转换 (使用 usize)
    ($obj:expr => $dto:ident.$field:ident: usize, $key:expr) => {
        if let Some(val) = $obj.get_number($key) {
            $dto.$field = Some(val as usize);
        }
    };
    
    // 字符串字段 - 使用字段名作为键
    ($obj:expr => $dto:ident.$field:ident: string) => {
        if let Some(val) = $obj.get_string(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    
    // 字符串字段 - 使用自定义键
    ($obj:expr => $dto:ident.$field:ident: string, $key:expr) => {
        if let Some(val) = $obj.get_string($key) {
            $dto.$field = Some(val);
        }
    };
    
    // 数值字段 - 使用字段名作为键
    ($obj:expr => $dto:ident.$field:ident: number) => {
        if let Some(val) = $obj.get_number(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    
    // 数值字段 - 使用自定义键
    ($obj:expr => $dto:ident.$field:ident: number, $key:expr) => {
        if let Some(val) = $obj.get_number($key) {
            $dto.$field = Some(val);
        }
    };
    
    // 布尔字段 - 使用字段名作为键
    ($obj:expr => $dto:ident.$field:ident: bool) => {
        if let Some(val) = $obj.get_bool(stringify!($field)) {
            $dto.$field = Some(val);
        }
    };
    
    // 布尔字段 - 使用自定义键
    ($obj:expr => $dto:ident.$field:ident: bool, $key:expr) => {
        if let Some(val) = $obj.get_bool($key) {
            $dto.$field = Some(val);
        }
    };
    
    // JSON 字段 - 使用字段名作为键
    ($obj:expr => $dto:ident.$field:ident: json) => {
        if let Some(element) = $obj.get_element(stringify!($field)) {
            $dto.$field = Some($crate::dto::json_value_to_extension_string(
                &$crate::dto::conversion::element_to_json_value_cached(element)
            ));
        }
    };
    
    // JSON 字段 - 使用自定义键
    ($obj:expr => $dto:ident.$field:ident: json, $key:expr) => {
        if let Some(element) = $obj.get_element($key) {
            $dto.$field = Some($crate::dto::json_value_to_extension_string(
                &$crate::dto::conversion::element_to_json_value_cached(element)
            ));
        }
    };
    
    // 引用字段
    ($obj:expr => $dto:ident.$field:ident: reference) => {
        $dto.$field = $crate::dto::conversion::extract_reference($obj);
    };
}

// ==================== 字段注册系统 ====================

/// 字段注册器，用于集中管理和自动生成已知字段列表
#[derive(Debug, Clone)]
pub struct FieldRegistry {
    fields: Vec<FieldSpec>,
}

impl FieldRegistry {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
        }
    }
    
    /// 注册字段
    pub fn register(mut self, field: FieldSpec) -> Self {
        self.fields.push(field);
        self
    }
    
    /// 批量注册字段
    pub fn register_all(mut self, fields: &[FieldSpec]) -> Self {
        self.fields.extend_from_slice(fields);
        self
    }
    
    /// 获取已注册的字段列表
    pub fn fields(&self) -> &[FieldSpec] {
        &self.fields
    }
    
    /// 转换为字符串切片（用于 ExtensionExtractor）
    pub fn as_str_slice(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.name.as_str()).collect()
    }
}

/// 字段注册宏，提供编译时字段列表生成
#[macro_export]
macro_rules! register_fields {
    ($($field:expr),* $(,)?) => {
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
    
    /// 访问数值字段（带类型转换）
    fn visit_number_as_field(&mut self, key: &str, field_name: &str, value: Option<f64>, number_type: NumberType);
    
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
    
    /// 自动从对象中提取字段并填充 DTO
    pub fn auto<V: DtoFieldVisitor>(self, obj: &ObjectElement, mut visitor: V) -> T {
        // 提取基础字段
        for field in self.field_registry.fields() {
            match &field.field_type {
                FieldType::String => {
                    if let Some(string_val) = obj.get_string(&field.name) {
                        visitor.visit_string_field(&field.name, &field.name, Some(string_val));
                    }
                }
                FieldType::Number => {
                    if let Some(number_val) = obj.get_number(&field.name) {
                        visitor.visit_number_field(&field.name, &field.name, Some(number_val));
                    }
                }
                FieldType::NumberAs(number_type) => {
                    if let Some(number_val) = obj.get_number(&field.name) {
                        visitor.visit_number_as_field(&field.name, &field.name, Some(number_val), number_type.clone());
                    }
                }
                FieldType::Boolean => {
                    if let Some(bool_val) = obj.get_bool(&field.name) {
                        visitor.visit_bool_field(&field.name, &field.name, Some(bool_val));
                    }
                }
                FieldType::Json => {
                    if let Some(element) = obj.get_element(&field.name) {
                        visitor.visit_json_field(&field.name, &field.name, Some(element));
                    }
                }
                FieldType::Reference => {
                    visitor.visit_reference_field(obj.get_string(&field.name));
                }
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

/// 扩展字段提取器
pub struct ExtensionExtractor {
    known_fields: Vec<String>,
    include_unknown_fields: bool,
}

impl ExtensionExtractor {
    pub fn new() -> Self {
        Self {
            known_fields: Vec::new(),
            include_unknown_fields: false,
        }
    }
    
    pub fn with_known_fields(mut self, fields: &[&str]) -> Self {
        self.known_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }
    
    pub fn with_field_registry(mut self, registry: &FieldRegistry) -> Self {
        self.known_fields = registry.as_str_slice().into_iter().map(String::from).collect();
        self
    }
    
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
                    let value_str = json_value_to_extension_string(&member.value.to_value());
                    extensions.insert(key_str.clone(), value_str);
                }
            }
        }
        
        extensions
    }

    pub fn with_field_specs(mut self, specs: &FieldSpecs) -> Self {
        self.known_fields = specs.known_field_names().into_iter().map(String::from).collect();
        self
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
    
    // 测试用的示例 DTO
    #[derive(Debug, Default)]
    struct TestDto {
        title: Option<String>,
        count: Option<f64>,
        size: Option<usize>,
        enabled: Option<bool>,
        data: Option<String>,
    }
    
    // 实现 DtoSetter trait
    impl DtoSetter for TestDto {
        fn set_field(&mut self, field_name: &str, value: &Element) -> bool {
            match field_name {
                "title" => {
                    if let Element::String(s) = value {
                        self.title = Some(s.content.clone());
                        true
                    } else {
                        false
                    }
                }
                "count" => {
                    if let Element::Number(n) = value {
                        self.count = Some(n.content);
                        true
                    } else {
                        false
                    }
                }
                "size" => {
                    if let Element::Number(n) = value {
                        self.size = Some(n.content as usize);
                        true
                    } else {
                        false
                    }
                }
                "enabled" => {
                    if let Element::Boolean(b) = value {
                        self.enabled = Some(b.content);
                        true
                    } else {
                        false
                    }
                }
                "data" => {
                    self.data = Some(json_value_to_extension_string(&element_to_json_value_cached(value)));
                    true
                }
                _ => false,
            }
        }
        
        fn field_specs() -> Vec<FieldSpec> {
            vec![
                FieldSpec::new("title", FieldType::String),
                FieldSpec::new("count", FieldType::Number),
                FieldSpec::new("size", FieldType::usize()),
                FieldSpec::new("enabled", FieldType::Boolean),
                FieldSpec::new("data", FieldType::Json),
            ]
        }
    }
    
    #[test]
    fn test_field_spec_with_json_key() {
        let spec = FieldSpec::new("minLength", FieldType::usize())
            .with_json_key("min_length");
        
        assert_eq!(spec.name(), "minLength");
        assert_eq!(spec.json_key(), "min_length");
        assert!(matches!(spec.field_type(), FieldType::NumberAs(NumberType::USize)));
    }
    
    #[test]
    fn test_dto_setter() {
        let mut dto = TestDto::default();
        
        // 测试设置字符串字段
        let title = Element::String(StringElement::new("Test Title"));
        assert!(dto.set_field("title", &title));
        assert_eq!(dto.title, Some("Test Title".to_string()));
        
        // 测试设置数值字段
        let count = Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        });
        assert!(dto.set_field("count", &count));
        assert_eq!(dto.count, Some(42.0));
        
        // 测试设置 usize 字段
        let size = Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 100.0,
        });
        assert!(dto.set_field("size", &size));
        assert_eq!(dto.size, Some(100));
    }
    
    #[test]
    fn test_element_to_json_value_cached() {
        // 测试字符串元素
        let str_elem = Element::String(StringElement::new("test"));
        let json_str = element_to_json_value_cached(&str_elem);
        assert_eq!(json_str, Value::String("test".to_string()));
        
        // 测试数值元素
        let num_elem = Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 42.0,
        });
        let json_num = element_to_json_value_cached(&num_elem);
        assert_eq!(json_num, Value::Number(serde_json::Number::from(42)));
        
        // 测试缓存命中
        let cached_value = element_to_json_value_cached(&str_elem);
        assert_eq!(cached_value, Value::String("test".to_string()));
    }
    
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
        let mut dto = TestDto::default();
        
        // 使用新的统一宏
        extract_field!(obj => dto.title: string);
        extract_field!(obj => dto.count: number);
        extract_field!(obj => dto.enabled: bool);
        
        assert_eq!(dto.title, Some("Test".to_string()));
        assert_eq!(dto.count, Some(42.0));
        assert_eq!(dto.enabled, Some(true));
    }
    
    #[test]
    fn test_field_registry() {
        let registry = FieldRegistry::new()
            .register(FieldSpec::new("title", FieldType::String))
            .register(FieldSpec::new("description", FieldType::String))
            .register(FieldSpec::new("version", FieldType::String));
        
        assert_eq!(registry.fields().len(), 3);
        assert!(registry.fields().contains(&FieldSpec::new("title", FieldType::String)));
        assert!(registry.fields().contains(&FieldSpec::new("description", FieldType::String)));
        assert!(registry.fields().contains(&FieldSpec::new("version", FieldType::String)));
    }
    
    #[test]
    fn test_extension_extractor_with_registry() {
        let registry = FieldRegistry::new()
            .register(FieldSpec::new("custom", FieldType::String));
        let extractor = ExtensionExtractor::new()
            .with_field_registry(&registry);

        let mut obj = ObjectElement::new();
        obj.set("x-custom-extension", Element::String(StringElement::new("custom")));

        let extracted_extensions = extractor.extract(&obj);

        assert_eq!(extracted_extensions.get("x-custom-extension"), Some(&"custom".to_string()));
    }
    
    #[test]
    fn test_field_specs() {
        let specs = FieldSpecs::new()
            .add(FieldSpec::new("title", FieldType::String))
            .add(FieldSpec::new("count", FieldType::Number).required())
            .add(FieldSpec::new("data", FieldType::Json));
            
        let known_fields = specs.known_field_names();
        assert_eq!(known_fields.len(), 3);
        assert!(known_fields.contains(&"title"));
        assert!(known_fields.contains(&"count"));
        assert!(known_fields.contains(&"data"));
        
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Test")));
        obj.set("x-custom", Element::String(StringElement::new("custom")));
        
        let extractor = ExtensionExtractor::new()
            .with_field_specs(&specs);
        let extensions = extractor.extract(&obj);
        
        assert!(extensions.contains_key("x-custom"));
        assert!(!extensions.contains_key("title"));
    }
} 