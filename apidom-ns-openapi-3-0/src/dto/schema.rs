//! Schema DTO implementation
//! 
//! Provides DTO for OpenAPI/JSON Schema objects, handling both simple types
//! and complex nested schemas with references.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::dto::{
    Extensions, IntoDto, json_value_to_extension_string,
    ObjectElementExt, ExtensionExtractor, extract_string_array,
};
use crate::extract_field;
use crate::elements::schema::OpenApiSchemaElement;
use apidom_ast::minim_model::*;
use serde_json;
use apidom_dto_derive::{DtoSpec, FromObjectElement, IntoFrbDto};

/// Schema 类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
    Null,
}

/// ExternalDocs DTO
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DtoSpec, FromObjectElement, IntoFrbDto)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    pub url: String,
    
    #[serde(flatten, skip_serializing_if = "Extensions::is_empty")]
    #[dto(extensions)]
    pub extensions: Extensions,
}

/// Schema DTO - JSON Schema/OpenAPI Schema 对象
#[derive(Debug, Clone, Serialize, Deserialize, DtoSpec, FromObjectElement, IntoFrbDto)]
pub struct SchemaDto {
    // 核心字段
    #[serde(rename = "type")]
    #[dto(rename = "type")]
    pub schema_type: Option<SchemaType>,
    
    pub title: Option<String>,
    pub description: Option<String>,
    
    #[dto(json)]
    pub default: Option<String>,
    
    #[dto(json)]
    pub example: Option<String>,
    
    // 数值约束
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    
    #[dto(rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<f64>,
    
    #[dto(rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<f64>,
    
    #[dto(rename = "multipleOf")]
    pub multiple_of: Option<f64>,
    
    // 字符串约束
    #[dto(usize, rename = "minLength")]
    pub min_length: Option<usize>,
    
    #[dto(usize, rename = "maxLength")]
    pub max_length: Option<usize>,
    
    pub pattern: Option<String>,
    
    // 数组约束
    #[dto(usize, rename = "minItems")]
    pub min_items: Option<usize>,
    
    #[dto(usize, rename = "maxItems")]
    pub max_items: Option<usize>,
    
    #[dto(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    
    pub items: Option<Box<SchemaDto>>,
    
    // 对象约束
    #[dto(usize, rename = "minProperties")]
    pub min_properties: Option<usize>,
    
    #[dto(usize, rename = "maxProperties")]
    pub max_properties: Option<usize>,
    
    pub required: Option<Vec<String>>,
    pub properties: Option<HashMap<String, SchemaDto>>,
    
    #[dto(rename = "additionalProperties")]
    pub additional_properties: Option<Box<SchemaDto>>,
    
    // 枚举和组合
    #[serde(rename = "enum")]
    #[dto(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    
    #[dto(rename = "allOf")]
    pub all_of: Option<Vec<SchemaDto>>,
    
    #[dto(rename = "anyOf")]
    pub any_of: Option<Vec<SchemaDto>>,
    
    #[dto(rename = "oneOf")]
    pub one_of: Option<Vec<SchemaDto>>,
    
    pub not: Option<Box<SchemaDto>>,
    
    // OpenAPI 特有字段
    pub format: Option<String>,
    pub nullable: Option<bool>,
    
    #[dto(rename = "readOnly")]
    pub read_only: Option<bool>,
    
    #[dto(rename = "writeOnly")]
    pub write_only: Option<bool>,
    
    pub deprecated: Option<bool>,
    
    #[dto(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocsDto>,
    
    // 引用
    #[serde(rename = "$ref")]
    #[dto(reference, rename = "$ref")]
    pub reference: Option<String>,
    
    // 扩展字段
    #[serde(flatten)]
    #[dto(extensions)]
    pub extensions: Extensions,
}

impl Default for SchemaDto {
    fn default() -> Self {
        Self {
            schema_type: None,
            title: None,
            description: None,
            default: None,
            example: None,
            minimum: None,
            maximum: None,
            exclusive_minimum: None,
            exclusive_maximum: None,
            multiple_of: None,
            min_length: None,
            max_length: None,
            pattern: None,
            min_items: None,
            max_items: None,
            unique_items: None,
            items: None,
            min_properties: None,
            max_properties: None,
            required: None,
            properties: None,
            additional_properties: None,
            enum_values: None,
            all_of: None,
            any_of: None,
            one_of: None,
            not: None,
            format: None,
            nullable: None,
            read_only: None,
            write_only: None,
            deprecated: None,
            external_docs: None,
            reference: None,
            extensions: Extensions::new(),
        }
    }
}

impl SchemaDto {
    /// 创建新的 SchemaDto
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 创建简单类型的 schema
    pub fn with_type(schema_type: SchemaType) -> Self {
        Self {
            schema_type: Some(schema_type),
            ..Default::default()
        }
    }
    
    /// 创建引用 schema
    pub fn with_reference(reference: impl Into<String>) -> Self {
        Self {
            reference: Some(reference.into()),
            ..Default::default()
        }
    }
    
    /// 创建字符串 schema
    pub fn string() -> Self {
        Self::with_type(SchemaType::String)
    }
    
    /// 创建数字 schema
    pub fn number() -> Self {
        Self::with_type(SchemaType::Number)
    }
    
    /// 创建整数 schema
    pub fn integer() -> Self {
        Self::with_type(SchemaType::Integer)
    }
    
    /// 创建布尔 schema
    pub fn boolean() -> Self {
        Self::with_type(SchemaType::Boolean)
    }
    
    /// 创建数组 schema
    pub fn array(items: SchemaDto) -> Self {
        Self {
            schema_type: Some(SchemaType::Array),
            items: Some(Box::new(items)),
            ..Default::default()
        }
    }
    
    /// 创建对象 schema
    pub fn object() -> Self {
        Self::with_type(SchemaType::Object)
    }
    
    /// 添加属性
    pub fn with_property(mut self, name: impl Into<String>, schema: SchemaDto) -> Self {
        if self.properties.is_none() {
            self.properties = Some(HashMap::new());
        }
        self.properties.as_mut().unwrap().insert(name.into(), schema);
        self
    }
    
    /// 添加必填字段
    pub fn with_required(mut self, fields: Vec<impl Into<String>>) -> Self {
        self.required = Some(fields.into_iter().map(|f| f.into()).collect());
        self
    }
    
    /// 设置描述
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// 检查是否为引用
    pub fn is_reference(&self) -> bool {
        self.reference.is_some()
    }
    
    /// 检查是否为基本类型
    pub fn is_primitive(&self) -> bool {
        matches!(self.schema_type, Some(SchemaType::String | SchemaType::Number | SchemaType::Integer | SchemaType::Boolean | SchemaType::Null))
    }
    
    /// 检查是否为容器类型
    pub fn is_container(&self) -> bool {
        matches!(self.schema_type, Some(SchemaType::Array | SchemaType::Object))
    }
}

impl ExternalDocsDto {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            description: None,
            url: url.into(),
            extensions: Extensions::new(),
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// 从 OpenApiSchemaElement 转换为 SchemaDto
impl IntoDto<SchemaDto> for &OpenApiSchemaElement {
    fn into_dto(self) -> SchemaDto {
        element_to_schema_dto(&Element::Object(self.base.object.clone()))
    }
}

/// 从 OpenApiSchemaElement 转换为 SchemaDto（拥有所有权版本）
impl IntoDto<SchemaDto> for OpenApiSchemaElement {
    fn into_dto(self) -> SchemaDto {
        element_to_schema_dto(&Element::Object(self.base.object))
    }
}

/// 从 Element 转换为 SchemaDto
impl IntoDto<SchemaDto> for &Element {
    fn into_dto(self) -> SchemaDto {
        element_to_schema_dto(self)
    }
}

/// 从 ObjectElement 转换为 SchemaDto
impl IntoDto<SchemaDto> for &ObjectElement {
    fn into_dto(self) -> SchemaDto {
        element_to_schema_dto(&Element::Object(self.clone()))
    }
}

/// 核心转换函数：从 AST Element 转换为 SchemaDto
fn element_to_schema_dto(element: &Element) -> SchemaDto {
    match element {
        Element::Object(obj) => SchemaDto::from_object_element(obj),
        _ => SchemaDto::default(),
    }
}

/// 从 Element 转换为 ExternalDocsDto
fn element_to_external_docs_dto(element: &Element) -> ExternalDocsDto {
    match element {
        Element::Object(obj) => ExternalDocsDto::from_object_element(obj),
        _ => ExternalDocsDto::new(""),
    }
}

/// 解析字符串为 SchemaType
fn parse_schema_type(type_str: &str) -> Option<SchemaType> {
    match type_str {
        "string" => Some(SchemaType::String),
        "number" => Some(SchemaType::Number),
        "integer" => Some(SchemaType::Integer),
        "boolean" => Some(SchemaType::Boolean),
        "array" => Some(SchemaType::Array),
        "object" => Some(SchemaType::Object),
        "null" => Some(SchemaType::Null),
        _ => None,
    }
}

// ==================== 字段注册 ====================

/// Schema DTO 的字段注册表
#[allow(dead_code)]
fn schema_field_registry() -> Vec<&'static str> {
    vec![
        "type", "title", "description", "default", "example",
        "minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "multipleOf",
        "minLength", "maxLength", "pattern",
        "minItems", "maxItems", "uniqueItems", "items",
        "minProperties", "maxProperties", "required", "properties", "additionalProperties",
        "enum", "allOf", "anyOf", "oneOf", "not",
        "format", "nullable", "readOnly", "writeOnly", "deprecated", "externalDocs",
        "$ref"
    ]
}

/// ExternalDocs DTO 的字段注册表
#[allow(dead_code)]
fn external_docs_field_registry() -> Vec<&'static str> {
    vec!["url", "description"]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_schema_dto_creation() {
        let schema = SchemaDto::string()
            .with_description("A test string");
        
        assert_eq!(schema.schema_type, Some(SchemaType::String));
        assert_eq!(schema.description, Some("A test string".to_string()));
    }
    
    #[test]
    fn test_schema_dto_object_with_properties() {
        let user_schema = SchemaDto::object()
            .with_property("name", SchemaDto::string())
            .with_property("age", SchemaDto::integer())
            .with_required(vec!["name"]);
        
        assert_eq!(user_schema.schema_type, Some(SchemaType::Object));
        assert!(user_schema.properties.is_some());
        assert_eq!(user_schema.required, Some(vec!["name".to_string()]));
        
        let properties = user_schema.properties.unwrap();
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("age"));
    }
    
    #[test]
    fn test_schema_dto_array() {
        let array_schema = SchemaDto::array(SchemaDto::string());
        
        assert_eq!(array_schema.schema_type, Some(SchemaType::Array));
        assert!(array_schema.items.is_some());
        
        let items = array_schema.items.unwrap();
        assert_eq!(items.schema_type, Some(SchemaType::String));
    }
    
    #[test]
    fn test_schema_dto_reference() {
        let ref_schema = SchemaDto::with_reference("#/components/schemas/User");
        
        assert!(ref_schema.is_reference());
        assert_eq!(ref_schema.reference, Some("#/components/schemas/User".to_string()));
    }
    
    #[test]
    fn test_schema_type_checks() {
        assert!(SchemaDto::string().is_primitive());
        assert!(SchemaDto::array(SchemaDto::string()).is_container());
        assert!(SchemaDto::object().is_container());
        assert!(!SchemaDto::with_reference("#/ref").is_primitive());
    }
    
    #[test]
    fn test_object_element_ext_trait() {
        let mut obj = ObjectElement::new();
        obj.set("title", Element::String(StringElement::new("Test Title")));
        obj.set("minLength", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 5.0,
        }));
        obj.set("nullable", Element::Boolean(BooleanElement::new(true)));
        
        assert_eq!(obj.get_string("title"), Some("Test Title".to_string()));
        assert_eq!(obj.get_number("minLength"), Some(5.0));
        assert_eq!(obj.get_bool("nullable"), Some(true));
        assert!(obj.get_string("nonexistent").is_none());
    }
    
    #[test]
    fn test_schema_dto_conversion() {
        // 创建一个简单的 Schema AST 元素
        let mut schema_obj = ObjectElement::new();
        schema_obj.set("type", Element::String(StringElement::new("string")));
        schema_obj.set("description", Element::String(StringElement::new("A test schema")));
        schema_obj.set("minLength", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 5.0,
        }));
        
        // 转换为 DTO
        let dto: SchemaDto = (&schema_obj).into_dto();
        
        // 验证转换结果
        assert_eq!(dto.schema_type, Some(SchemaType::String));
        assert_eq!(dto.description, Some("A test schema".to_string()));
        assert_eq!(dto.min_length, Some(5));
    }
    
    #[test]
    fn test_schema_dto_with_properties() {
        // 创建一个包含属性的对象 Schema
        let mut schema_obj = ObjectElement::new();
        schema_obj.set("type", Element::String(StringElement::new("object")));
        
        // 创建 properties 对象
        let mut properties_obj = ObjectElement::new();
        
        // 添加 name 属性
        let mut name_schema = ObjectElement::new();
        name_schema.set("type", Element::String(StringElement::new("string")));
        properties_obj.set("name", Element::Object(name_schema));
        
        // 添加 age 属性
        let mut age_schema = ObjectElement::new();
        age_schema.set("type", Element::String(StringElement::new("integer")));
        properties_obj.set("age", Element::Object(age_schema));
        
        schema_obj.set("properties", Element::Object(properties_obj));
        
        // 添加 required 数组
        let mut required_arr = ArrayElement::new_empty();
        required_arr.content.push(Element::String(StringElement::new("name")));
        schema_obj.set("required", Element::Array(required_arr));
        
        // 转换为 DTO
        let dto: SchemaDto = (&schema_obj).into_dto();
        
        // 验证转换结果
        assert_eq!(dto.schema_type, Some(SchemaType::Object));
        assert!(dto.properties.is_some());
        assert_eq!(dto.required, Some(vec!["name".to_string()]));
        
        let properties = dto.properties.unwrap();
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("age"));
        assert_eq!(properties.get("name").unwrap().schema_type, Some(SchemaType::String));
        assert_eq!(properties.get("age").unwrap().schema_type, Some(SchemaType::Integer));
    }
    
    #[test]
    fn test_refactored_extraction_functions() {
        // 测试重构后的字段提取功能
        let mut schema_obj = ObjectElement::new();
        schema_obj.set("type", Element::String(StringElement::new("object")));
        schema_obj.set("title", Element::String(StringElement::new("Test Schema")));
        schema_obj.set("minimum", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 0.0,
        }));
        schema_obj.set("minLength", Element::Number(NumberElement {
            element: "number".to_string(),
            meta: Default::default(),
            attributes: Default::default(),
            content: 1.0,
        }));
        schema_obj.set("nullable", Element::Boolean(BooleanElement::new(true)));
        
        let dto: SchemaDto = (&schema_obj).into_dto();
        
        assert_eq!(dto.schema_type, Some(SchemaType::Object));
        assert_eq!(dto.title, Some("Test Schema".to_string()));
        assert_eq!(dto.minimum, Some(0.0));
        assert_eq!(dto.min_length, Some(1));
        assert_eq!(dto.nullable, Some(true));
    }
} 