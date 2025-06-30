//! # Data Transfer Object (DTO) Architecture
//!
//! ```text
//! Backend (Rust)          Frontend (Flutter/Dart)
//! +-------------+        +--------------+
//! | AST Layer   |   ->   | DTO Layer    |
//! | (Internal)  |        | (Interface)  |
//! +-------------+        +--------------+
//! ```
//!
//! This module provides a clean interface layer between the internal AST representation
//! and the frontend applications. All DTOs are designed to be Flutter Rust Bridge (FRB) compatible.
//!
//! ## Key Design Principles
//! 
//! - **FRB Compatibility**: All types are supported by Flutter Rust Bridge
//! - **Serde Serialization**: Full JSON serialization support
//! - **Type Safety**: Strong typing with clear conversion paths
//! - **Extension Support**: Proper handling of OpenAPI extensions
//!
//! ## Usage Example
//!
//! ```rust
//! use apidom_ns_openapi_3_0::elements::example::ExampleElement;
//! use apidom_ns_openapi_3_0::dto::example::ExampleDto;
//! 
//! // Convert AST to DTO (example placeholder - actual implementation varies)
//! // let dto: ExampleDto = ast_element.into_dto();
//!
//! // Serialize to JSON for frontend
//! // let json = serde_json::to_string(&dto)?;
//! ```

pub mod example;
pub mod info;
pub mod schema;
pub mod openapi;
pub mod conversion;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;

/// 通用扩展字段类型，用于存储 x-* 字段和其他动态内容
/// 为了兼容 flutter_rust_bridge，使用 String 存储 JSON 序列化后的值
pub type Extensions = HashMap<String, String>;

/// 扩展字段辅助函数
pub fn json_value_to_extension_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

/// 从扩展字符串解析 JSON 值
pub fn extension_string_to_json_value(s: &str) -> Value {
    serde_json::from_str(s).unwrap_or_else(|_| Value::String(s.to_string()))
}

/// DTO 转换 trait，定义 AST → DTO 的映射接口
pub trait IntoDto<T> {
    fn into_dto(self) -> T;
}

/// 引用信息 DTO，处理 $ref 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceDto {
    #[serde(rename = "$ref")]
    pub reference: String,
    /// 扩展字段（x-*）
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// 通用错误信息 DTO，用于传输验证错误等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDto {
    pub message: String,
    pub field: Option<String>,
    pub error_type: String,
}

// 重新导出通用转换工具
pub use conversion::{
    ObjectElementExt, 
    ExtensionExtractor, 
    extract_string_array,
    extract_string_map,
    extract_reference,
};

// 重新导出可用的宏 
pub use crate::extract_field; 

pub use self::example::ExampleDto; 