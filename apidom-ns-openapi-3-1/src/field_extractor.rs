use std::collections::HashMap;
use apidom_ast::{Element, ObjectElement, ArrayElement};

/// 字段提取器，用于从 Element 中安全地提取各种类型的字段
pub struct FieldExtractor;

impl FieldExtractor {
    /// 提取字符串字段
    pub fn extract_string(element: &Element, field_name: &str) -> Option<String> {
        if let Element::Object(obj) = element {
            obj.get(field_name)
                .and_then(Element::as_string)
                .map(|s| s.content.clone())
        } else {
            None
        }
    }

    /// 提取可选字符串字段
    pub fn extract_optional_string(element: &Element, field_name: &str) -> Option<Option<String>> {
        if let Element::Object(obj) = element {
            if obj.has_key(field_name) {
                Some(obj.get(field_name)
                    .and_then(Element::as_string)
                    .map(|s| s.content.clone()))
            } else {
                Some(None)
            }
        } else {
            None
        }
    }

    /// 提取数字字段
    pub fn extract_number(element: &Element, field_name: &str) -> Option<f64> {
        if let Element::Object(obj) = element {
            obj.get(field_name)
                .and_then(Element::as_number)
                .map(|n| n.content)
        } else {
            None
        }
    }

    /// 提取整数字段
    pub fn extract_integer(element: &Element, field_name: &str) -> Option<i64> {
        Self::extract_number(element, field_name)
            .and_then(|n| if n.fract() == 0.0 { Some(n as i64) } else { None })
    }

    /// 提取布尔字段
    pub fn extract_boolean(element: &Element, field_name: &str) -> Option<bool> {
        if let Element::Object(obj) = element {
            obj.get(field_name)
                .and_then(Element::as_boolean)
                .map(|b| b.content)
        } else {
            None
        }
    }

    /// 提取数组字段
    pub fn extract_array<'a>(element: &'a Element, field_name: &str) -> Option<&'a ArrayElement> {
        if let Element::Object(obj) = element {
            obj.get(field_name).and_then(Element::as_array)
        } else {
            None
        }
    }

    /// 提取对象字段
    pub fn extract_object<'a>(element: &'a Element, field_name: &str) -> Option<&'a ObjectElement> {
        if let Element::Object(obj) = element {
            obj.get(field_name).and_then(Element::as_object)
        } else {
            None
        }
    }

    /// 提取字符串数组
    pub fn extract_string_array(element: &Element, field_name: &str) -> Option<Vec<String>> {
        Self::extract_array(element, field_name).map(|arr| {
            arr.content
                .iter()
                .filter_map(|elem| elem.as_string().map(|s| s.content.clone()))
                .collect()
        })
    }

    /// 提取嵌套对象中的字段
    pub fn extract_nested_string(element: &Element, path: &[&str]) -> Option<String> {
        let mut current = element;
        
        // 导航到嵌套对象
        for &segment in &path[..path.len()-1] {
            if let Element::Object(obj) = current {
                current = obj.get(segment)?;
            } else {
                return None;
            }
        }
        
        // 提取最终字段
        if let Some(&field_name) = path.last() {
            Self::extract_string(current, field_name)
        } else {
            None
        }
    }

    /// 验证必填字段是否存在
    pub fn validate_required_fields(element: &Element, required_fields: &[&str]) -> Result<(), Vec<String>> {
        if let Element::Object(obj) = element {
            let missing_fields: Vec<String> = required_fields
                .iter()
                .filter(|&&field| !obj.has_key(field))
                .map(|&field| field.to_string())
                .collect();
            
            if missing_fields.is_empty() {
                Ok(())
            } else {
                Err(missing_fields)
            }
        } else {
            Err(vec!["Element is not an object".to_string()])
        }
    }

    /// 提取所有字段名
    pub fn extract_field_names(element: &Element) -> Vec<String> {
        if let Element::Object(obj) = element {
            obj.content
                .iter()
                .filter_map(|member| {
                    member.key.as_string().map(|s| s.content.clone())
                })
                .collect()
        } else {
            vec![]
        }
    }

    /// 提取扩展字段（以 x- 开头的字段）
    pub fn extract_extension_fields(element: &Element) -> HashMap<String, Element> {
        let mut extensions = HashMap::new();
        
        if let Element::Object(obj) = element {
            for member in &obj.content {
                if let Element::String(key) = member.key.as_ref() {
                    if key.content.starts_with("x-") {
                        extensions.insert(key.content.clone(), (*member.value).clone());
                    }
                }
            }
        }
        
        extensions
    }

    /// 检查字段是否为特定类型
    pub fn is_field_of_type(element: &Element, field_name: &str, expected_type: &str) -> bool {
        if let Element::Object(obj) = element {
            if let Some(field_value) = obj.get(field_name) {
                match (field_value, expected_type) {
                    (Element::String(_), "string") => true,
                    (Element::Number(_), "number") => true,
                    (Element::Boolean(_), "boolean") => true,
                    (Element::Array(_), "array") => true,
                    (Element::Object(_), "object") => true,
                    (Element::Null(_), "null") => true,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// 提取字段并应用默认值
    pub fn extract_with_default<T, F>(element: &Element, field_name: &str, extractor: F, default: T) -> T
    where
        F: FnOnce(&Element, &str) -> Option<T>,
    {
        extractor(element, field_name).unwrap_or(default)
    }

    /// 批量提取字符串字段
    pub fn extract_string_fields(element: &Element, field_names: &[&str]) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for &field_name in field_names {
            if let Some(value) = Self::extract_string(element, field_name) {
                result.insert(field_name.to_string(), value);
            }
        }
        
        result
    }

    /// 安全提取枚举值
    pub fn extract_enum_value<T>(
        element: &Element,
        field_name: &str,
        valid_values: &[(&str, T)]
    ) -> Option<T>
    where
        T: Clone,
    {
        Self::extract_string(element, field_name).and_then(|value| {
            valid_values
                .iter()
                .find(|(v, _)| *v == value)
                .map(|(_, t)| t.clone())
        })
    }

    /// 提取并验证 URL 字段
    pub fn extract_url(element: &Element, field_name: &str) -> Option<String> {
        Self::extract_string(element, field_name).filter(|url| {
            // 简单的 URL 验证
            url.starts_with("http://") || url.starts_with("https://") || url.starts_with("/")
        })
    }

    /// 提取版本号字段
    pub fn extract_version(element: &Element, field_name: &str) -> Option<String> {
        Self::extract_string(element, field_name).filter(|version| {
            // 简单的版本号验证 (x.y.z 格式)
            version.chars().any(|c| c.is_ascii_digit()) && 
            version.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '+')
        })
    }
}

/// 字段提取宏，简化常见提取操作
#[macro_export]
macro_rules! extract_fields {
    ($element:expr, { $( $field:literal => $var:ident : $type:ty ),* $(,)? }) => {
        {
            use $crate::field_extractor::FieldExtractor;
            
            $(
                let $var: Option<$type> = match stringify!($type) {
                    "String" => FieldExtractor::extract_string($element, $field),
                    "f64" => FieldExtractor::extract_number($element, $field),
                    "i64" => FieldExtractor::extract_integer($element, $field),
                    "bool" => FieldExtractor::extract_boolean($element, $field),
                    _ => None, // 对于其他类型，需要手动实现
                };
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::{ObjectElement, StringElement, NumberElement, BooleanElement, ArrayElement};

    fn create_test_object() -> Element {
        let mut obj = ObjectElement::new();
        
        // 添加各种类型的字段
        obj.set("title", Element::String(StringElement::new("Test API")));
        obj.set("version", Element::String(StringElement::new("1.0.0")));
        obj.set("port", Element::Number(NumberElement::new(8080.0)));
        obj.set("enabled", Element::Boolean(BooleanElement::new(true)));
        
        // 添加数组
        let tags = ArrayElement::from_strings(&["api", "rest", "v1"]);
        obj.set("tags", Element::Array(tags));
        
        // 添加扩展字段
        obj.set("x-custom", Element::String(StringElement::new("custom-value")));
        obj.set("x-version", Element::Number(NumberElement::new(2.0)));
        
        Element::Object(obj)
    }

    #[test]
    fn test_extract_string() {
        let element = create_test_object();
        
        assert_eq!(
            FieldExtractor::extract_string(&element, "title"),
            Some("Test API".to_string())
        );
        
        assert_eq!(
            FieldExtractor::extract_string(&element, "nonexistent"),
            None
        );
    }

    #[test]
    fn test_extract_number() {
        let element = create_test_object();
        
        assert_eq!(
            FieldExtractor::extract_number(&element, "port"),
            Some(8080.0)
        );
        
        assert_eq!(
            FieldExtractor::extract_integer(&element, "port"),
            Some(8080)
        );
    }

    #[test]
    fn test_extract_boolean() {
        let element = create_test_object();
        
        assert_eq!(
            FieldExtractor::extract_boolean(&element, "enabled"),
            Some(true)
        );
    }

    #[test]
    fn test_extract_string_array() {
        let element = create_test_object();
        
        let tags = FieldExtractor::extract_string_array(&element, "tags");
        assert_eq!(tags, Some(vec!["api".to_string(), "rest".to_string(), "v1".to_string()]));
    }

    #[test]
    fn test_validate_required_fields() {
        let element = create_test_object();
        
        // 所有必填字段都存在
        let result = FieldExtractor::validate_required_fields(&element, &["title", "version"]);
        assert!(result.is_ok());
        
        // 缺少必填字段
        let result = FieldExtractor::validate_required_fields(&element, &["title", "missing"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), vec!["missing".to_string()]);
    }

    #[test]
    fn test_extract_extension_fields() {
        let element = create_test_object();
        
        let extensions = FieldExtractor::extract_extension_fields(&element);
        assert_eq!(extensions.len(), 2);
        assert!(extensions.contains_key("x-custom"));
        assert!(extensions.contains_key("x-version"));
    }

    #[test]
    fn test_is_field_of_type() {
        let element = create_test_object();
        
        assert!(FieldExtractor::is_field_of_type(&element, "title", "string"));
        assert!(FieldExtractor::is_field_of_type(&element, "port", "number"));
        assert!(FieldExtractor::is_field_of_type(&element, "enabled", "boolean"));
        assert!(FieldExtractor::is_field_of_type(&element, "tags", "array"));
        
        assert!(!FieldExtractor::is_field_of_type(&element, "title", "number"));
    }

    #[test]
    fn test_extract_version() {
        let element = create_test_object();
        
        assert_eq!(
            FieldExtractor::extract_version(&element, "version"),
            Some("1.0.0".to_string())
        );
    }

    #[test]
    fn test_extract_with_default() {
        let element = create_test_object();
        
        let title = FieldExtractor::extract_with_default(
            &element,
            "title",
            FieldExtractor::extract_string,
            "Default Title".to_string()
        );
        assert_eq!(title, "Test API");
        
        let missing = FieldExtractor::extract_with_default(
            &element,
            "missing",
            FieldExtractor::extract_string,
            "Default Title".to_string()
        );
        assert_eq!(missing, "Default Title");
    }

    #[test]
    fn test_extract_enum_value() {
        let mut obj = ObjectElement::new();
        obj.set("type", Element::String(StringElement::new("object")));
        let element = Element::Object(obj);
        
        #[derive(Debug, Clone, PartialEq)]
        enum SchemaType {
            Object,
            Array,
            String,
        }
        
        let valid_values = [
            ("object", SchemaType::Object),
            ("array", SchemaType::Array),
            ("string", SchemaType::String),
        ];
        
        let result = FieldExtractor::extract_enum_value(&element, "type", &valid_values);
        assert_eq!(result, Some(SchemaType::Object));
        
        let invalid = FieldExtractor::extract_enum_value(&element, "nonexistent", &valid_values);
        assert_eq!(invalid, None);
    }
} 