use crate::{elements::InfoElement, field_registry::FieldHandlerMap, error::{Result, FieldError}, context::BuildContext};
use apidom_ast::SimpleValue;
use apidom_ast::{Element, Fold};

/// 通用构建器 trait，将通用 Element 转换为具体 AST 节点
pub trait Builder<T> {
    /// 从 Element 构建目标类型
    fn build(&self, element: &Element, ctx: &BuildContext) -> Result<T>;
}

/// InfoElement 的构建器
pub struct InfoBuilder {
    handlers: FieldHandlerMap<InfoElement>,
}

impl Default for InfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoBuilder {
    pub fn new() -> Self {
        use crate::register_fixed_fields;

        let handlers = register_fixed_fields!(InfoElement, {
            "title" => |value: &Element, target: &mut InfoElement, _folder: Option<&mut dyn Fold>| {
                if let Some(string_elem) = value.as_string() {
                    target.set_title(string_elem.clone());
                    Some(())
                } else {
                    None
                }
            },
            "version" => |value: &Element, target: &mut InfoElement, _folder: Option<&mut dyn Fold>| {
                if let Some(string_elem) = value.as_string() {
                    target.set_version(string_elem.clone());
                    Some(())
                } else {
                    None
                }
            },
            "description" => |value: &Element, target: &mut InfoElement, _folder: Option<&mut dyn Fold>| {
                if let Some(string_elem) = value.as_string() {
                    target.set_description(string_elem.clone());
                    Some(())
                } else {
                    None
                }
            },
        });

        Self { handlers }
    }
}

impl Builder<InfoElement> for InfoBuilder {
    fn build(&self, element: &Element, ctx: &BuildContext) -> Result<InfoElement> {
        if let Element::Object(obj) = element {
            let mut info = InfoElement::new();
            for member in &obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    let field_name = &key_str.content;
                    self.handlers.dispatch(field_name, member.value.as_ref(), &mut info, None);
                }
            }

            // 收集 x-* 扩展字段并写入 meta
            let mut ext_map = std::collections::HashMap::new();
            for member in &obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    let field_name = &key_str.content;
                    if field_name.starts_with("x-") {
                        let value = member.value.to_value();
                        ext_map.insert(field_name.clone(), value);
                    }
                }
            }
            if !ext_map.is_empty() {
                info.inner.meta.properties.insert(
                    "extensions".to_string(),
                    SimpleValue::object(ext_map),
                );
            }

            // 使用上下文注入路径
            ctx.inject_path_meta(&mut info.inner.meta.properties);
            Ok(info)
        } else {
            Err(FieldError::new("InfoElement expects an object"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::{ObjectElement, StringElement, MemberElement};

    #[test]
    fn test_info_builder() {
        let builder = InfoBuilder::new();

        let mut obj = ObjectElement::new();
        obj.content.push(MemberElement::new(
            Element::String(StringElement::new("title")),
            Element::String(StringElement::new("API")),
        ));
        obj.content.push(MemberElement::new(
            Element::String(StringElement::new("version")),
            Element::String(StringElement::new("1.0")),
        ));

        let element = Element::Object(obj);
        let ctx = BuildContext::default();
        let info = builder.build(&element, &ctx).expect("build failed");
        assert_eq!(info.title().unwrap().content, "API");
        assert_eq!(info.version().unwrap().content, "1.0");
    }
} 