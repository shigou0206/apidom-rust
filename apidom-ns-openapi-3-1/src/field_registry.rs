use std::collections::HashMap;
use apidom_ast::{Element, Fold};

/// 字段规格定义
#[derive(Debug, Clone)]
pub struct FieldSpec {
    /// 字段名
    pub name: String,
    /// 类型提示（如 "string", "object", "array" 等）
    pub type_hint: Option<String>,
    /// 是否必填
    pub required: bool,
    /// 是否来自 pattern（如 x-* 扩展字段）
    pub from_pattern: bool,
    /// 原始 schema 定义
    pub schema: Option<Element>,
}

impl PartialEq for FieldSpec {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.type_hint == other.type_hint
            && self.required == other.required
            && self.from_pattern == other.from_pattern
            // 跳过 schema 字段的比较，因为 Element 没有实现 PartialEq
    }
}

impl FieldSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_hint: None,
            required: false,
            from_pattern: false,
            schema: None,
        }
    }

    pub fn with_type(mut self, type_hint: impl Into<String>) -> Self {
        self.type_hint = Some(type_hint.into());
        self
    }

    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn with_pattern(mut self, from_pattern: bool) -> Self {
        self.from_pattern = from_pattern;
        self
    }

    pub fn with_schema(mut self, schema: Element) -> Self {
        self.schema = Some(schema);
        self
    }
}

/// 字段处理器类型
pub type FieldHandler<T> = fn(&Element, &mut T, Option<&mut dyn Fold>) -> Option<()>;

/// 字段处理器映射表
pub struct FieldHandlerMap<T> {
    /// 固定字段处理器
    fixed_handlers: HashMap<String, FieldHandler<T>>,
    /// 模式字段处理器（如 x-* 扩展）
    pattern_handlers: Vec<(regex::Regex, FieldHandler<T>)>,
    /// 默认处理器（fallback）
    default_handler: Option<FieldHandler<T>>,
}

impl<T> Default for FieldHandlerMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FieldHandlerMap<T> {
    pub fn new() -> Self {
        Self {
            fixed_handlers: HashMap::new(),
            pattern_handlers: Vec::new(),
            default_handler: None,
        }
    }

    /// 注册固定字段处理器
    pub fn register_fixed(&mut self, field_name: impl Into<String>, handler: FieldHandler<T>) {
        self.fixed_handlers.insert(field_name.into(), handler);
    }

    /// 注册模式字段处理器
    pub fn register_pattern(&mut self, pattern: &str, handler: FieldHandler<T>) -> Result<(), regex::Error> {
        let regex = regex::Regex::new(pattern)?;
        self.pattern_handlers.push((regex, handler));
        Ok(())
    }

    /// 设置默认处理器
    pub fn set_default(&mut self, handler: FieldHandler<T>) {
        self.default_handler = Some(handler);
    }

    /// 查找字段对应的处理器
    pub fn find_handler(&self, field_name: &str) -> Option<FieldHandler<T>> {
        // 优先查找固定字段
        if let Some(&handler) = self.fixed_handlers.get(field_name) {
            return Some(handler);
        }

        // 查找模式字段
        for (pattern, handler) in &self.pattern_handlers {
            if pattern.is_match(field_name) {
                return Some(*handler);
            }
        }

        // 返回默认处理器
        self.default_handler
    }

    /// 分发字段到对应处理器
    pub fn dispatch(&self, field_name: &str, value: &Element, target: &mut T, folder: Option<&mut dyn Fold>) -> bool {
        if let Some(handler) = self.find_handler(field_name) {
            handler(value, target, folder).is_some()
        } else {
            false
        }
    }
}

/// 字段注册宏 - 注册固定字段
#[macro_export]
macro_rules! register_fixed_fields {
    ($target_type:ty, { $( $field:literal => $handler:expr ),* $(,)? }) => {
        {
            let mut map = $crate::field_registry::FieldHandlerMap::<$target_type>::new();
            $(
                map.register_fixed($field, $handler);
            )*
            map
        }
    };
}

/// 字段注册宏 - 注册模式字段
#[macro_export]
macro_rules! register_pattern_fields {
    ($target_type:ty, { $( $pattern:literal => $handler:expr ),* $(,)? }) => {
        {
            let mut map = $crate::field_registry::FieldHandlerMap::<$target_type>::new();
            $(
                map.register_pattern($pattern, $handler).expect("Invalid regex pattern");
            )*
            map
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::{Element, StringElement};

    struct TestElement {
        fields: HashMap<String, Element>,
    }

    impl TestElement {
        fn new() -> Self {
            Self {
                fields: HashMap::new(),
            }
        }

        fn set_field(&mut self, name: &str, value: Element) {
            self.fields.insert(name.to_string(), value);
        }
    }

    #[test]
    fn test_field_spec_creation() {
        let spec = FieldSpec::new("title")
            .with_type("string")
            .with_required(true);

        assert_eq!(spec.name, "title");
        assert_eq!(spec.type_hint, Some("string".to_string()));
        assert!(spec.required);
        assert!(!spec.from_pattern);
    }

    #[test]
    fn test_field_handler_registration() {
        let handler = |_value: &Element, target: &mut TestElement, _folder: Option<&mut dyn Fold>| {
            target.set_field("title", Element::String(StringElement::new("test")));
            Some(())
        };

        let map = register_fixed_fields!(TestElement, {
            "title" => handler,
        });

        assert!(map.find_handler("title").is_some());
        assert!(map.find_handler("unknown").is_none());
    }

    #[test]
    fn test_field_dispatch() {
        let handler = |_value: &Element, target: &mut TestElement, _folder: Option<&mut dyn Fold>| {
            target.set_field("title", Element::String(StringElement::new("processed")));
            Some(())
        };

        let map = register_fixed_fields!(TestElement, {
            "title" => handler,
        });

        let mut element = TestElement::new();
        let value = Element::String(StringElement::new("test"));

        let success = map.dispatch("title", &value, &mut element, None);
        assert!(success);
        assert!(element.fields.contains_key("title"));
    }
} 