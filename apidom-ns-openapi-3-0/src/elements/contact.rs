use apidom_ast::minim_model::*;

/// OpenAPI Contact Element
#[derive(Debug, Clone)]
pub struct ContactElement {
    pub object: ObjectElement,
}

impl ContactElement {
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("contact");
        Self { object: obj }
    }

    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("contact");
        Self { object: content }
    }

    pub fn name(&self) -> Option<&StringElement> {
        self.object.get("name").and_then(Element::as_string)
    }

    pub fn set_name(&mut self, value: StringElement) {
        self.object.set("name", Element::String(value));
    }

    pub fn url(&self) -> Option<&StringElement> {
        self.object.get("url").and_then(Element::as_string)
    }

    pub fn set_url(&mut self, value: StringElement) {
        self.object.set("url", Element::String(value));
    }

    pub fn email(&self) -> Option<&StringElement> {
        self.object.get("email").and_then(Element::as_string)
    }

    pub fn set_email(&mut self, value: StringElement) {
        self.object.set("email", Element::String(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_element_new() {
        let contact = ContactElement::new();
        assert_eq!(contact.object.element, "contact");
        assert!(contact.object.content.is_empty());
    }

    #[test]
    fn test_contact_element_with_content() {
        let mut obj = ObjectElement::new();
        obj.set("name", Element::String(StringElement::new("John Doe")));
        
        let contact = ContactElement::with_content(obj);
        assert_eq!(contact.object.element, "contact");
        assert!(contact.name().is_some());
    }

    #[test]
    fn test_name_get_set() {
        let mut contact = ContactElement::new();
        
        // 初始状态应该为空
        assert!(contact.name().is_none());
        
        // 设置 name
        let name = StringElement::new("API Support Team");
        contact.set_name(name);
        
        // 验证设置成功
        let retrieved_name = contact.name();
        assert!(retrieved_name.is_some());
        assert_eq!(retrieved_name.unwrap().content, "API Support Team");
    }

    #[test]
    fn test_url_get_set() {
        let mut contact = ContactElement::new();
        
        // 初始状态应该为空
        assert!(contact.url().is_none());
        
        // 设置 URL
        let url = StringElement::new("https://example.com/support");
        contact.set_url(url);
        
        // 验证设置成功
        let retrieved_url = contact.url();
        assert!(retrieved_url.is_some());
        assert_eq!(retrieved_url.unwrap().content, "https://example.com/support");
    }

    #[test]
    fn test_email_get_set() {
        let mut contact = ContactElement::new();
        
        // 初始状态应该为空
        assert!(contact.email().is_none());
        
        // 设置 email
        let email = StringElement::new("support@example.com");
        contact.set_email(email);
        
        // 验证设置成功
        let retrieved_email = contact.email();
        assert!(retrieved_email.is_some());
        assert_eq!(retrieved_email.unwrap().content, "support@example.com");
    }

    #[test]
    fn test_all_fields_together() {
        let mut contact = ContactElement::new();
        
        // 设置所有字段
        contact.set_name(StringElement::new("API Support"));
        contact.set_url(StringElement::new("https://api.example.com/support"));
        contact.set_email(StringElement::new("api-support@example.com"));
        
        // 验证所有字段都设置成功
        assert!(contact.name().is_some());
        assert!(contact.url().is_some());
        assert!(contact.email().is_some());
        
        assert_eq!(contact.name().unwrap().content, "API Support");
        assert_eq!(contact.url().unwrap().content, "https://api.example.com/support");
        assert_eq!(contact.email().unwrap().content, "api-support@example.com");
    }

    #[test]
    fn test_contact_element_update_existing_fields() {
        let mut contact = ContactElement::new();
        
        // 设置初始值
        contact.set_name(StringElement::new("Old Name"));
        contact.set_email(StringElement::new("old@example.com"));
        
        // 更新值
        contact.set_name(StringElement::new("New Name"));
        contact.set_email(StringElement::new("new@example.com"));
        
        // 验证更新成功
        assert_eq!(contact.name().unwrap().content, "New Name");
        assert_eq!(contact.email().unwrap().content, "new@example.com");
    }

    #[test]
    fn test_openapi_contact_realistic_scenario() {
        let mut contact = ContactElement::new();
        
        // 模拟真实的 OpenAPI Contact 对象
        contact.set_name(StringElement::new("API Support Team"));
        contact.set_url(StringElement::new("https://example.com/contact"));
        contact.set_email(StringElement::new("support@example.com"));
        
        // 验证符合 OpenAPI 规范的 Contact 对象
        assert!(contact.name().is_some());
        assert!(contact.url().is_some());
        assert!(contact.email().is_some());
        
        // 验证内容格式
        let name = contact.name().unwrap();
        let url = contact.url().unwrap();
        let email = contact.email().unwrap();
        
        assert!(!name.content.is_empty());
        assert!(url.content.starts_with("http"));
        assert!(email.content.contains("@"));
    }

    #[test]
    fn test_partial_contact_info() {
        let mut contact = ContactElement::new();
        
        // 只设置部分信息（这在 OpenAPI 中是允许的）
        contact.set_email(StringElement::new("contact@example.com"));
        
        // 验证只有设置的字段存在
        assert!(contact.name().is_none());
        assert!(contact.url().is_none());
        assert!(contact.email().is_some());
        
        assert_eq!(contact.email().unwrap().content, "contact@example.com");
    }
}