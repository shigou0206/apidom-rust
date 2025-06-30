//! Schema DTO implementation
//! 
//! Provides DTO for OpenAPI/JSON Schema objects, handling both simple types
//! and complex nested schemas with references.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::dto::{
    Extensions, IntoDto, json_value_to_extension_string,
    ObjectElementExt, ExtensionExtractor, element_to_json_value, extract_string_array
};
use crate::{extract_string_field, extract_number_field, extract_bool_field, extract_json_field};
use crate::elements::schema::OpenApiSchemaElement;
use apidom_ast::minim_model::*;

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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    pub url: String,
    
    #[serde(flatten, skip_serializing_if = "Extensions::is_empty")]
    pub extensions: Extensions,
}

/// Schema DTO - JSON Schema/OpenAPI Schema 对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDto {
    // 核心字段
    #[serde(rename = "type")]
    pub schema_type: Option<SchemaType>,
    
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>, // JSON 序列化字符串
    pub example: Option<String>, // JSON 序列化字符串
    
    // 数值约束
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub multiple_of: Option<f64>,
    
    // 字符串约束
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    
    // 数组约束
    pub min_items: Option<usize>,
    pub max_items: Option<usize>,
    pub unique_items: Option<bool>,
    pub items: Option<Box<SchemaDto>>,
    
    // 对象约束
    pub min_properties: Option<usize>,
    pub max_properties: Option<usize>,
    pub required: Option<Vec<String>>,
    pub properties: Option<HashMap<String, SchemaDto>>,
    pub additional_properties: Option<Box<SchemaDto>>,
    
    // 枚举和组合
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>, // JSON 序列化字符串数组
    pub all_of: Option<Vec<SchemaDto>>,
    pub any_of: Option<Vec<SchemaDto>>,
    pub one_of: Option<Vec<SchemaDto>>,
    pub not: Option<Box<SchemaDto>>,
    
    // OpenAPI 特有字段
    pub format: Option<String>,
    pub nullable: Option<bool>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub deprecated: Option<bool>,
    pub external_docs: Option<ExternalDocsDto>,
    
    // 引用
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    
    // 扩展字段
    #[serde(flatten)]
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

// ==================== 字段提取函数 ====================

/// 提取基础字段（type, title, description 等）
fn extract_basic_fields(obj: &ObjectElement, dto: &mut SchemaDto) {
    // 处理 type 字段
    if let Some(type_str) = obj.get_string("type") {
        dto.schema_type = parse_schema_type(&type_str);
    }
    
    extract_string_field!(obj, dto, title);
    extract_string_field!(obj, dto, description);
    
    // 处理 JSON 值字段（转换为字符串）
    extract_json_field!(obj, dto, default);
    extract_json_field!(obj, dto, example);
    
    // 处理引用
    extract_string_field!(obj, dto, reference, "$ref");
}

/// 提取数值约束字段
fn extract_numeric_constraints(obj: &ObjectElement, dto: &mut SchemaDto) {
    extract_number_field!(obj, dto, minimum);
    extract_number_field!(obj, dto, maximum);
    extract_number_field!(obj, dto, exclusive_minimum, "exclusiveMinimum");
    extract_number_field!(obj, dto, exclusive_maximum, "exclusiveMaximum");
    extract_number_field!(obj, dto, multiple_of, "multipleOf");
}

/// 提取字符串约束字段
fn extract_string_constraints(obj: &ObjectElement, dto: &mut SchemaDto) {
    extract_number_field!(obj, dto, min_length, "minLength", as usize);
    extract_number_field!(obj, dto, max_length, "maxLength", as usize);
    extract_string_field!(obj, dto, pattern);
}

/// 提取数组约束字段
fn extract_array_constraints(obj: &ObjectElement, dto: &mut SchemaDto) {
    extract_number_field!(obj, dto, min_items, "minItems", as usize);
    extract_number_field!(obj, dto, max_items, "maxItems", as usize);
    extract_bool_field!(obj, dto, unique_items, "uniqueItems");
    
    if let Some(items_element) = obj.get_element("items") {
        dto.items = Some(Box::new(element_to_schema_dto(items_element)));
    }
}

/// 提取对象约束字段
fn extract_object_constraints(obj: &ObjectElement, dto: &mut SchemaDto) {
    extract_number_field!(obj, dto, min_properties, "minProperties", as usize);
    extract_number_field!(obj, dto, max_properties, "maxProperties", as usize);
    
    // 处理 required 数组 - 使用通用函数
    dto.required = extract_string_array(obj, "required");
    
    // 处理 properties
    if let Some(props_obj) = obj.get_object("properties") {
        let mut properties = HashMap::new();
        for member in &props_obj.content {
            if let Element::String(prop_name) = &*member.key {
                let prop_schema = element_to_schema_dto(&member.value);
                properties.insert(prop_name.content.clone(), prop_schema);
            }
        }
        if !properties.is_empty() {
            dto.properties = Some(properties);
        }
    }
    
    // 处理 additionalProperties
    if let Some(additional_props) = obj.get_element("additionalProperties") {
        dto.additional_properties = Some(Box::new(element_to_schema_dto(additional_props)));
    }
}

/// 提取枚举和组合字段
fn extract_enum_and_composition(obj: &ObjectElement, dto: &mut SchemaDto) {
    // 处理枚举值
    if let Some(enum_arr) = obj.get_array("enum") {
        let mut enum_values = Vec::new();
        for item in &enum_arr.content {
            enum_values.push(json_value_to_extension_string(&element_to_json_value(item)));
        }
        if !enum_values.is_empty() {
            dto.enum_values = Some(enum_values);
        }
    }
    
    // 处理组合 schema
    extract_schema_array(obj, "allOf", &mut dto.all_of);
    extract_schema_array(obj, "anyOf", &mut dto.any_of);
    extract_schema_array(obj, "oneOf", &mut dto.one_of);
    
    if let Some(not_element) = obj.get_element("not") {
        dto.not = Some(Box::new(element_to_schema_dto(not_element)));
    }
}

/// 提取 OpenAPI 特有字段
fn extract_openapi_specific(obj: &ObjectElement, dto: &mut SchemaDto) {
    extract_string_field!(obj, dto, format);
    extract_bool_field!(obj, dto, nullable);
    extract_bool_field!(obj, dto, read_only, "readOnly");
    extract_bool_field!(obj, dto, write_only, "writeOnly");
    extract_bool_field!(obj, dto, deprecated);
    
    // 处理 externalDocs
    if let Some(external_docs_element) = obj.get_element("externalDocs") {
        dto.external_docs = Some(element_to_external_docs_dto(external_docs_element));
    }
}

/// 辅助函数：提取 schema 数组
fn extract_schema_array(obj: &ObjectElement, key: &str, target: &mut Option<Vec<SchemaDto>>) {
    if let Some(arr) = obj.get_array(key) {
        let mut schemas = Vec::new();
        for item in &arr.content {
            schemas.push(element_to_schema_dto(item));
        }
        if !schemas.is_empty() {
            *target = Some(schemas);
        }
    }
}

// ==================== IntoDto 实现 ====================

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

// ==================== 核心转换函数 ====================

/// 核心转换函数：从 AST Element 转换为 SchemaDto（重构后）
fn element_to_schema_dto(element: &Element) -> SchemaDto {
    let obj = match element {
        Element::Object(obj) => obj,
        _ => return SchemaDto::default(),
    };
    
    let mut dto = SchemaDto::default();
    
    // 按类别提取字段
    extract_basic_fields(obj, &mut dto);
    extract_numeric_constraints(obj, &mut dto);
    extract_string_constraints(obj, &mut dto);
    extract_array_constraints(obj, &mut dto);
    extract_object_constraints(obj, &mut dto);
    extract_enum_and_composition(obj, &mut dto);
    extract_openapi_specific(obj, &mut dto);
    
    // 处理扩展字段
    dto.extensions = extract_extensions(obj);
    
    dto
}

/// 从 Element 转换为 ExternalDocsDto
fn element_to_external_docs_dto(element: &Element) -> ExternalDocsDto {
    let obj = match element {
        Element::Object(obj) => obj,
        _ => return ExternalDocsDto::new(""),
    };
    
    let mut dto = ExternalDocsDto::new("");
    
    if let Some(url) = obj.get_string("url") {
        dto.url = url;
    }
    
    if let Some(description) = obj.get_string("description") {
        dto.description = Some(description);
    }
    
    dto.extensions = extract_extensions(obj);
    
    dto
}

/// 从 ObjectElement 提取扩展字段 - 使用通用提取器
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