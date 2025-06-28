use apidom_ast::minim_model::*;

/// OpenAPI 3.x CallbackElement
#[derive(Debug, Clone)]
pub struct CallbackElement {
    pub object: ObjectElement,
}

impl CallbackElement {
    /// 创建一个空的 CallbackElement，element type 为 "callback"
    pub fn new() -> Self {
        let mut obj = ObjectElement::new();
        obj.set_element_type("callback");
        Self { object: obj }
    }

    /// 从已有 ObjectElement 创建，并设置 element type
    pub fn with_content(content: ObjectElement) -> Self {
        let mut content = content;
        content.set_element_type("callback");
        Self { object: content }
    }

    /// 获取指定 callback key 的内容（如：post、get 等 operation path）
    pub fn get(&self, key: &str) -> Option<&Element> {
        self.object.get(key)
    }

    pub fn set(&mut self, key: impl Into<String>, value: Element) {
        let key_str = key.into();
        self.object.set(&key_str, value);
    }

    /// 获取整个内容对象
    pub fn content(&self) -> &ObjectElement {
        &self.object
    }

    /// 设置整个内容对象
    pub fn set_content(&mut self, obj: ObjectElement) {
        self.object = obj;
        self.object.set_element_type("callback");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback_element_new() {
        let callback = CallbackElement::new();
        assert_eq!(callback.object.element, "callback");
        assert!(callback.object.content.is_empty());
    }

    #[test]
    fn test_callback_element_with_content() {
        let mut obj = ObjectElement::new();
        obj.set("test", Element::String(StringElement::new("value")));
        
        let callback = CallbackElement::with_content(obj);
        assert_eq!(callback.object.element, "callback");
        assert!(callback.get("test").is_some());
    }

    #[test]
    fn test_callback_element_get_set() {
        let mut callback = CallbackElement::new();
        
        // 测试设置和获取
        let test_value = Element::String(StringElement::new("test_value"));
        callback.set("test_key", test_value);
        
        let retrieved = callback.get("test_key");
        assert!(retrieved.is_some());
        
        if let Some(Element::String(s)) = retrieved {
            assert_eq!(s.content, "test_value");
        } else {
            panic!("Expected string element");
        }
    }

    #[test]
    fn test_callback_element_set_with_different_key_types() {
        let mut callback = CallbackElement::new();
        
        // 测试不同类型的 key
        callback.set("string_key", Element::String(StringElement::new("value1")));
        callback.set(String::from("owned_string_key"), Element::String(StringElement::new("value2")));
        
        assert!(callback.get("string_key").is_some());
        assert!(callback.get("owned_string_key").is_some());
    }

    #[test]
    fn test_callback_element_content_access() {
        let mut callback = CallbackElement::new();
        callback.set("key1", Element::String(StringElement::new("value1")));
        
        let content = callback.content();
        assert_eq!(content.element, "callback");
        assert!(!content.content.is_empty());
    }

    #[test]
    fn test_callback_element_set_content() {
        let mut callback = CallbackElement::new();
        
        let mut new_obj = ObjectElement::new();
        new_obj.set("new_key", Element::String(StringElement::new("new_value")));
        
        callback.set_content(new_obj);
        
        assert_eq!(callback.object.element, "callback");
        assert!(callback.get("new_key").is_some());
    }

    #[test]
    fn test_callback_element_openapi_scenario() {
        let mut callback = CallbackElement::new();
        
        // 模拟典型的 OpenAPI callback 场景
        let callback_url = "{$request.body#/callbackUrl}";
        
        // 创建一个 POST operation
        let mut post_operation = ObjectElement::new();
        post_operation.set("summary", Element::String(StringElement::new("Callback endpoint")));
        post_operation.set("operationId", Element::String(StringElement::new("callbackOperation")));
        
        callback.set(callback_url, Element::Object(post_operation));
        
        // 验证设置成功
        let retrieved = callback.get(callback_url);
        assert!(retrieved.is_some());
        
        if let Some(Element::Object(op)) = retrieved {
            if let Some(Element::String(summary)) = op.get("summary") {
                assert_eq!(summary.content, "Callback endpoint");
            } else {
                panic!("Expected summary field");
            }
        } else {
            panic!("Expected object element");
        }
    }
}