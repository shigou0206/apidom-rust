//! OpenAPI DTO implementation
//! 
//! The root DTO for complete OpenAPI 3.0 documents, demonstrating the full
//! AST → DTO conversion architecture.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::dto::Extensions;
use crate::dto::info::InfoDto;
use crate::dto::schema::SchemaDto;

/// Server DTO - 服务器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDto {
    pub url: String,
    pub description: Option<String>,
    pub variables: Option<HashMap<String, ServerVariableDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Server Variable DTO - 服务器变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariableDto {
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    pub default: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Path Item DTO - 路径项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItemDto {
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub get: Option<OperationDto>,
    pub put: Option<OperationDto>,
    pub post: Option<OperationDto>,
    pub delete: Option<OperationDto>,
    pub options: Option<OperationDto>,
    pub head: Option<OperationDto>,
    pub patch: Option<OperationDto>,
    pub trace: Option<OperationDto>,
    pub servers: Option<Vec<ServerDto>>,
    pub parameters: Option<Vec<ParameterDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Operation DTO - 操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDto {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<ParameterDto>>,
    pub request_body: Option<RequestBodyDto>,
    pub responses: HashMap<String, ResponseDto>,
    pub callbacks: Option<HashMap<String, CallbackDto>>,
    pub deprecated: Option<bool>,
    pub security: Option<Vec<SecurityRequirementDto>>,
    pub servers: Option<Vec<ServerDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Parameter DTO - 参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDto {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub schema: Option<SchemaDto>,
    pub example: Option<String>,
    pub examples: Option<HashMap<String, super::example::ExampleDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Request Body DTO - 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBodyDto {
    pub description: Option<String>,
    pub content: HashMap<String, MediaTypeDto>,
    pub required: Option<bool>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Media Type DTO - 媒体类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTypeDto {
    pub schema: Option<SchemaDto>,
    pub example: Option<String>,
    pub examples: Option<HashMap<String, super::example::ExampleDto>>,
    pub encoding: Option<HashMap<String, EncodingDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Encoding DTO - 编码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodingDto {
    pub content_type: Option<String>,
    pub headers: Option<HashMap<String, HeaderDto>>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Response DTO - 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDto {
    pub description: String,
    pub headers: Option<HashMap<String, HeaderDto>>,
    pub content: Option<HashMap<String, MediaTypeDto>>,
    pub links: Option<HashMap<String, LinkDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Header DTO - 头部
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderDto {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub schema: Option<SchemaDto>,
    pub example: Option<String>,
    pub examples: Option<HashMap<String, super::example::ExampleDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Link DTO - 链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDto {
    pub operation_ref: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Option<HashMap<String, String>>,
    pub request_body: Option<String>,
    pub description: Option<String>,
    pub server: Option<ServerDto>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Callback DTO - 回调
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackDto {
    #[serde(flatten)]
    pub paths: HashMap<String, PathItemDto>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Security Requirement DTO - 安全要求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirementDto {
    #[serde(flatten)]
    pub requirements: HashMap<String, Vec<String>>,
}

/// Components DTO - 组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentsDto {
    pub schemas: Option<HashMap<String, SchemaDto>>,
    pub responses: Option<HashMap<String, ResponseDto>>,
    pub parameters: Option<HashMap<String, ParameterDto>>,
    pub examples: Option<HashMap<String, super::example::ExampleDto>>,
    pub request_bodies: Option<HashMap<String, RequestBodyDto>>,
    pub headers: Option<HashMap<String, HeaderDto>>,
    pub security_schemes: Option<HashMap<String, SecuritySchemeDto>>,
    pub links: Option<HashMap<String, LinkDto>>,
    pub callbacks: Option<HashMap<String, CallbackDto>>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Security Scheme DTO - 安全方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySchemeDto {
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<String>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub flows: Option<OAuthFlowsDto>,
    pub open_id_connect_url: Option<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// OAuth Flows DTO - OAuth 流程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlowsDto {
    pub implicit: Option<OAuthFlowDto>,
    pub password: Option<OAuthFlowDto>,
    pub client_credentials: Option<OAuthFlowDto>,
    pub authorization_code: Option<OAuthFlowDto>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// OAuth Flow DTO - OAuth 流程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlowDto {
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Tag DTO - 标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDto {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<super::schema::ExternalDocsDto>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// 根 OpenAPI DTO - 完整的 OpenAPI 3.0 文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiDto {
    pub openapi: String,
    pub info: InfoDto,
    pub servers: Option<Vec<ServerDto>>,
    pub paths: HashMap<String, PathItemDto>,
    pub components: Option<ComponentsDto>,
    pub security: Option<Vec<SecurityRequirementDto>>,
    pub tags: Option<Vec<TagDto>>,
    pub external_docs: Option<super::schema::ExternalDocsDto>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl OpenApiDto {
    /// 创建新的 OpenAPI DTO
    pub fn new(version: impl Into<String>, info: InfoDto) -> Self {
        Self {
            openapi: version.into(),
            info,
            servers: None,
            paths: HashMap::new(),
            components: None,
            security: None,
            tags: None,
            external_docs: None,
            extensions: Extensions::new(),
        }
    }
    
    /// 添加路径
    pub fn with_path(mut self, path: impl Into<String>, path_item: PathItemDto) -> Self {
        self.paths.insert(path.into(), path_item);
        self
    }
    
    /// 添加服务器
    pub fn with_server(mut self, server: ServerDto) -> Self {
        if self.servers.is_none() {
            self.servers = Some(Vec::new());
        }
        self.servers.as_mut().unwrap().push(server);
        self
    }
    
    /// 设置组件
    pub fn with_components(mut self, components: ComponentsDto) -> Self {
        self.components = Some(components);
        self
    }
    
    /// 获取 JSON 字符串表示
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// 从 JSON 字符串解析
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl ServerDto {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            description: None,
            variables: None,
            extensions: Extensions::new(),
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl OperationDto {
    pub fn new() -> Self {
        Self {
            tags: None,
            summary: None,
            description: None,
            operation_id: None,
            parameters: None,
            request_body: None,
            responses: HashMap::new(),
            callbacks: None,
            deprecated: None,
            security: None,
            servers: None,
            extensions: Extensions::new(),
        }
    }
    
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }
    
    pub fn with_response(mut self, status: impl Into<String>, response: ResponseDto) -> Self {
        self.responses.insert(status.into(), response);
        self
    }
}

impl ResponseDto {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            headers: None,
            content: None,
            links: None,
            extensions: Extensions::new(),
        }
    }
    
    pub fn with_content(mut self, media_type: impl Into<String>, content: MediaTypeDto) -> Self {
        if self.content.is_none() {
            self.content = Some(HashMap::new());
        }
        self.content.as_mut().unwrap().insert(media_type.into(), content);
        self
    }
}

impl PathItemDto {
    pub fn new() -> Self {
        Self {
            reference: None,
            summary: None,
            description: None,
            get: None,
            put: None,
            post: None,
            delete: None,
            options: None,
            head: None,
            patch: None,
            trace: None,
            servers: None,
            parameters: None,
            extensions: Extensions::new(),
        }
    }
    
    pub fn with_get(mut self, operation: OperationDto) -> Self {
        self.get = Some(operation);
        self
    }
    
    pub fn with_post(mut self, operation: OperationDto) -> Self {
        self.post = Some(operation);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::info::InfoDto;
    
    #[test]
    fn test_openapi_dto_creation() {
        let info = InfoDto::new("Test API", "1.0.0")
            .with_description("A test API");
        
        let openapi = OpenApiDto::new("3.0.3", info)
            .with_server(ServerDto::new("https://api.example.com"));
        
        assert_eq!(openapi.openapi, "3.0.3");
        assert_eq!(openapi.info.title, "Test API");
        assert!(openapi.servers.is_some());
    }
    
    #[test]
    fn test_openapi_dto_with_paths() {
        let info = InfoDto::new("Test API", "1.0.0");
        
        let operation = OperationDto::new()
            .with_summary("List users")
            .with_response("200", ResponseDto::new("Success"));
        
        let path_item = PathItemDto::new()
            .with_get(operation);
        
        let openapi = OpenApiDto::new("3.0.3", info)
            .with_path("/users", path_item);
        
        assert!(openapi.paths.contains_key("/users"));
        
        let users_path = &openapi.paths["/users"];
        assert!(users_path.get.is_some());
        
        let get_op = users_path.get.as_ref().unwrap();
        assert_eq!(get_op.summary, Some("List users".to_string()));
    }
    
    #[test]
    fn test_openapi_dto_json_serialization() {
        let info = InfoDto::new("Test API", "1.0.0");
        let openapi = OpenApiDto::new("3.0.3", info);
        
        let json = openapi.to_json().expect("Should serialize to JSON");
        assert!(json.contains("\"openapi\": \"3.0.3\""));
        assert!(json.contains("\"title\": \"Test API\""));
        
        let parsed: OpenApiDto = OpenApiDto::from_json(&json).expect("Should parse from JSON");
        assert_eq!(parsed.openapi, "3.0.3");
        assert_eq!(parsed.info.title, "Test API");
    }
} 