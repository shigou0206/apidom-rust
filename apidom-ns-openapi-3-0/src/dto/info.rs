//! Info DTO implementation
//! 
//! Provides DTO for OpenAPI Info objects with nested Contact and License information.

use serde::{Serialize, Deserialize};
use crate::dto::{
    Extensions, IntoDto,
    ObjectElementExt, ExtensionExtractor
};
use crate::{extract_string_field};
use crate::elements::info::InfoElement;
use apidom_ast::minim_model::Element;

/// Contact DTO - 联系信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactDto {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// License DTO - 许可证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseDto {
    pub name: String,
    pub url: Option<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

/// Info DTO - API 基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoDto {
    /// API 标题（必填）
    pub title: String,
    
    /// API 版本（必填）
    pub version: String,
    
    /// API 描述
    pub description: Option<String>,
    
    /// 服务条款 URL
    pub terms_of_service: Option<String>,
    
    /// 联系信息
    pub contact: Option<ContactDto>,
    
    /// 许可证信息
    pub license: Option<LicenseDto>,
    
    /// 扩展字段（x-*）
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl InfoDto {
    /// 创建新的 InfoDto
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            version: version.into(),
            description: None,
            terms_of_service: None,
            contact: None,
            license: None,
            extensions: Extensions::new(),
        }
    }
    
    /// 设置描述
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// 设置联系信息
    pub fn with_contact(mut self, contact: ContactDto) -> Self {
        self.contact = Some(contact);
        self
    }
    
    /// 设置许可证
    pub fn with_license(mut self, license: LicenseDto) -> Self {
        self.license = Some(license);
        self
    }
}

impl ContactDto {
    pub fn new() -> Self {
        Self {
            name: None,
            url: None,
            email: None,
            extensions: Extensions::new(),
        }
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
    
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

impl LicenseDto {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
            extensions: Extensions::new(),
        }
    }
    
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// AST → DTO 转换实现
impl IntoDto<InfoDto> for InfoElement {
    fn into_dto(self) -> InfoDto {
        let mut dto = InfoDto::new(
            self.object.get_string("title").unwrap_or_default(),
            self.object.get_string("version").unwrap_or_default(),
        );
        
        // 提取可选字段
        extract_string_field!(self.object, dto, description);
        extract_string_field!(self.object, dto, terms_of_service, "termsOfService");
        
        // 提取嵌套对象
        if let Some(contact_elem) = self.object.get_element("contact") {
            dto.contact = extract_contact_dto(contact_elem);
        }
        
        if let Some(license_elem) = self.object.get_element("license") {
            dto.license = extract_license_dto(license_elem);
        }
        
        // 提取扩展字段 - 使用通用提取器
        dto.extensions = ExtensionExtractor::new()
            .with_known_fields(&["title", "version", "description", "termsOfService", "contact", "license"])
            .extract(&self.object);
        
        dto
    }
}

impl IntoDto<InfoDto> for &InfoElement {
    fn into_dto(self) -> InfoDto {
        let mut dto = InfoDto::new(
            self.object.get_string("title").unwrap_or_default(),
            self.object.get_string("version").unwrap_or_default(),
        );
        
        extract_string_field!(self.object, dto, description);
        extract_string_field!(self.object, dto, terms_of_service, "termsOfService");
        
        if let Some(contact_elem) = self.object.get_element("contact") {
            dto.contact = extract_contact_dto(contact_elem);
        }
        
        if let Some(license_elem) = self.object.get_element("license") {
            dto.license = extract_license_dto(license_elem);
        }
        
        dto.extensions = ExtensionExtractor::new()
            .with_known_fields(&["title", "version", "description", "termsOfService", "contact", "license"])
            .extract(&self.object);
        
        dto
    }
}

/// 提取 Contact DTO
fn extract_contact_dto(element: &Element) -> Option<ContactDto> {
    if let Element::Object(obj) = element {
        let mut contact = ContactDto::new();
        
        extract_string_field!(obj, contact, name);
        extract_string_field!(obj, contact, email);
        extract_string_field!(obj, contact, url);
        
        // 提取扩展字段
        contact.extensions = ExtensionExtractor::new()
            .with_known_fields(&["name", "email", "url"])
            .extract(obj);
        
        Some(contact)
    } else {
        None
    }
}

/// 提取 License DTO
fn extract_license_dto(element: &Element) -> Option<LicenseDto> {
    if let Element::Object(obj) = element {
        let mut license = LicenseDto::new(
            obj.get_string("name").unwrap_or_default()
        );
        
        extract_string_field!(obj, license, url);
        
        // 提取扩展字段
        license.extensions = ExtensionExtractor::new()
            .with_known_fields(&["name", "url"])
            .extract(obj);
        
        Some(license)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::minim_model::*;
    
    #[test]
    fn test_info_dto_basic_conversion() {
        let mut info = InfoElement::new();
        info.object.set("title", Element::String(StringElement::new("Test API")));
        info.object.set("version", Element::String(StringElement::new("1.0.0")));
        info.object.set("description", Element::String(StringElement::new("A test API")));
        
        let dto: InfoDto = info.into_dto();
        
        assert_eq!(dto.title, "Test API");
        assert_eq!(dto.version, "1.0.0");
        assert_eq!(dto.description, Some("A test API".to_string()));
    }
    
    #[test]
    fn test_info_dto_with_contact() {
        let mut info = InfoElement::new();
        info.object.set("title", Element::String(StringElement::new("Test API")));
        info.object.set("version", Element::String(StringElement::new("1.0.0")));
        
        // 添加联系信息
        let mut contact_obj = ObjectElement::new();
        contact_obj.set("name", Element::String(StringElement::new("API Team")));
        contact_obj.set("email", Element::String(StringElement::new("api@example.com")));
        contact_obj.set("url", Element::String(StringElement::new("https://example.com")));
        info.object.set("contact", Element::Object(contact_obj));
        
        let dto: InfoDto = info.into_dto();
        
        assert!(dto.contact.is_some());
        let contact = dto.contact.unwrap();
        assert_eq!(contact.name, Some("API Team".to_string()));
        assert_eq!(contact.email, Some("api@example.com".to_string()));
        assert_eq!(contact.url, Some("https://example.com".to_string()));
    }
    
    #[test]
    fn test_info_dto_with_license() {
        let mut info = InfoElement::new();
        info.object.set("title", Element::String(StringElement::new("Test API")));
        info.object.set("version", Element::String(StringElement::new("1.0.0")));
        
        // 添加许可证信息
        let mut license_obj = ObjectElement::new();
        license_obj.set("name", Element::String(StringElement::new("MIT")));
        license_obj.set("url", Element::String(StringElement::new("https://opensource.org/licenses/MIT")));
        info.object.set("license", Element::Object(license_obj));
        
        let dto: InfoDto = info.into_dto();
        
        assert!(dto.license.is_some());
        let license = dto.license.unwrap();
        assert_eq!(license.name, "MIT");
        assert_eq!(license.url, Some("https://opensource.org/licenses/MIT".to_string()));
    }
    
    #[test]
    fn test_info_dto_with_extensions() {
        let mut info = InfoElement::new();
        info.object.set("title", Element::String(StringElement::new("Test API")));
        info.object.set("version", Element::String(StringElement::new("1.0.0")));
        info.object.set("x-api-id", Element::String(StringElement::new("test-api-123")));
        info.object.set("x-internal", Element::Boolean(BooleanElement::new(true)));
        
        let dto: InfoDto = info.into_dto();
        
        assert_eq!(dto.extensions.get("x-api-id"), Some(&"test-api-123".to_string()));
        assert_eq!(dto.extensions.get("x-internal"), Some(&"true".to_string()));
    }
} 