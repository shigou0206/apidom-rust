//! Example DTO implementation
//! 
//! This module provides the DTO (Data Transfer Object) for OpenAPI Example objects,
//! converting complex AST structures into simple, serializable data structures
//! suitable for front-end consumption.

use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::dto::{
    Extensions, IntoDto,
    ObjectElementExt, ExtensionExtractor, extract_reference
};
use crate::extract_field;
use crate::elements::example::ExampleElement;

/// Example DTO - 纯数据传输对象
/// 
/// 包含前端需要的所有 Example 相关数据，去除了 AST 的复杂性：
/// - 没有元数据、类标签、折叠状态等内部信息
/// - 所有字段都是基本类型或 JSON 值
/// - 支持序列化为 JSON 直接传输给前端
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDto {
    /// 示例的简短描述
    pub summary: Option<String>,
    
    /// 示例的详细描述，支持 CommonMark 语法
    pub description: Option<String>,
    
    /// 示例的具体值，可以是任意 JSON 数据（序列化为字符串以兼容 FRB）
    pub value: Option<String>,
    
    /// 外部示例的 URL
    pub external_value: Option<String>,
    
    /// 引用信息（如果这是一个引用）
    pub reference: Option<String>,
    
    /// 扩展字段（x-*）和其他动态内容
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for ExampleDto {
    fn default() -> Self {
        Self {
            summary: None,
            description: None,
            value: None,
            external_value: None,
            reference: None,
            extensions: Extensions::new(),
        }
    }
}

impl ExampleDto {
    /// 创建新的 ExampleDto
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 创建带有基本信息的 ExampleDto
    pub fn with_summary_and_value(summary: impl Into<String>, value: Value) -> Self {
        Self {
            summary: Some(summary.into()),
            value: Some(value.to_string()),
            ..Default::default()
        }
    }
    
    /// 检查是否为引用类型
    pub fn is_reference(&self) -> bool {
        self.reference.is_some()
    }
    
    /// 检查是否为外部引用
    pub fn is_external(&self) -> bool {
        self.external_value.is_some()
    }
}

/// AST → DTO 转换实现
impl IntoDto<ExampleDto> for ExampleElement {
    fn into_dto(self) -> ExampleDto {
        let mut dto = ExampleDto::new();
        
        // 提取基本字段
        extract_field!(self.object => dto.summary: string);
        extract_field!(self.object => dto.description: string);
        extract_field!(self.object => dto.external_value: string, "externalValue");
        
        // 处理 value 字段 - 转换为 JSON 字符串
        extract_field!(self.object => dto.value: json);
        
        // 提取引用信息 - 使用通用函数
        dto.reference = extract_reference(&self.object);
        
        // 提取扩展字段
        dto.extensions = ExtensionExtractor::new()
            .with_known_fields(&["summary", "description", "value", "externalValue", "$ref"])
            .extract(&self.object);
        
        dto
    }
}

/// AST → DTO 转换实现（借用版本）
impl IntoDto<ExampleDto> for &ExampleElement {
    fn into_dto(self) -> ExampleDto {
        let mut dto = ExampleDto::new();
        
        // 提取基本字段
        extract_field!(self.object => dto.summary: string);
        extract_field!(self.object => dto.description: string);
        extract_field!(self.object => dto.external_value: string, "externalValue");
        
        // 处理 value 字段 - 转换为 JSON 字符串
        extract_field!(self.object => dto.value: json);
        
        // 提取引用信息 - 使用通用函数
        dto.reference = extract_reference(&self.object);
        
        // 提取扩展字段
        dto.extensions = ExtensionExtractor::new()
            .with_known_fields(&["summary", "description", "value", "externalValue", "$ref"])
            .extract(&self.object);
        
        dto
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::minim_model::*;
    
    #[test]
    fn test_example_dto_basic_conversion() {
        // 创建测试用的 ExampleElement
        let mut example = ExampleElement::new();
        example.set_summary(StringElement::new("Test example"));
        example.set_description(StringElement::new("A test example for unit testing"));
        example.set_external_value(StringElement::new("https://example.com/test.json"));
        
        // 转换为 DTO
        let dto: ExampleDto = example.into_dto();
        
        // 验证转换结果
        assert_eq!(dto.summary, Some("Test example".to_string()));
        assert_eq!(dto.description, Some("A test example for unit testing".to_string()));
        assert_eq!(dto.external_value, Some("https://example.com/test.json".to_string()));
        assert!(dto.is_external());
        assert!(!dto.is_reference());
    }
    
    #[test]
    fn test_example_dto_with_value() {
        let mut dto = ExampleDto::new();
        dto.value = Some("John".to_string());

        assert_eq!(dto.value, Some("John".to_string()));
    }
    
    #[test]
    fn test_example_dto_with_extensions() {
        let mut obj = ObjectElement::new();
        obj.set("x-example-extension", Element::String(StringElement::new("custom-value")));

        let extracted_extensions = ExtensionExtractor::new()
            .with_known_fields(&["summary", "description"])
            .extract(&obj);

        assert_eq!(extracted_extensions.get("x-example-extension"), Some(&"custom-value".to_string()));
    }
    
    #[test]
    fn test_example_dto_reference() {
        let mut example = ExampleElement::new();
        example.object.set("$ref", Element::String(StringElement::new("#/components/examples/UserExample")));
        
        let dto: ExampleDto = example.into_dto();
        
        assert_eq!(dto.reference, Some("#/components/examples/UserExample".to_string()));
        assert!(dto.is_reference());
    }
} 