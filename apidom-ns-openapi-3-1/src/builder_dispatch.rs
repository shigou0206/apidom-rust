use crate::field_registry::FieldHandlerMap;
use crate::elements::*;
use crate::error::{FieldError, Result};
use crate::context::BuildContext;
use apidom_ast::{Element, Fold};

/// OpenAPI 3.1 元素构建器分发器
pub struct BuilderDispatch {
    info_handlers: FieldHandlerMap<InfoElement>,
    server_handlers: FieldHandlerMap<ServerElement>,
    path_item_handlers: FieldHandlerMap<PathItemElement>,
}

impl Default for BuilderDispatch {
    fn default() -> Self {
        Self::new()
    }
}

impl BuilderDispatch {
    pub fn new() -> Self {
        let mut dispatch = Self {
            info_handlers: FieldHandlerMap::new(),
            server_handlers: FieldHandlerMap::new(),
            path_item_handlers: FieldHandlerMap::new(),
        };
        
        dispatch.register_handlers();
        dispatch
    }

    /// 注册所有字段处理器
    fn register_handlers(&mut self) {
        self.register_info_handlers();
        self.register_server_handlers();
        self.register_path_item_handlers();
    }

    /// 注册 Info 元素的字段处理器
    fn register_info_handlers(&mut self) {
        use crate::register_fixed_fields;

        self.info_handlers = register_fixed_fields!(InfoElement, {
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
    }

    /// 注册 Server 元素的字段处理器
    fn register_server_handlers(&mut self) {
        use crate::register_fixed_fields;

        self.server_handlers = register_fixed_fields!(ServerElement, {
            "url" => |value: &Element, target: &mut ServerElement, _folder: Option<&mut dyn Fold>| {
                if let Some(string_elem) = value.as_string() {
                    target.set_url(string_elem.clone());
                    Some(())
                } else {
                    None
                }
            },
            "description" => |value: &Element, target: &mut ServerElement, _folder: Option<&mut dyn Fold>| {
                if let Some(string_elem) = value.as_string() {
                    target.set_description(string_elem.clone());
                    Some(())
                } else {
                    None
                }
            },
        });
    }

    /// 注册 PathItem 元素的字段处理器
    fn register_path_item_handlers(&mut self) {
        use crate::register_fixed_fields;

        self.path_item_handlers = register_fixed_fields!(PathItemElement, {
            "summary" => |value: &Element, target: &mut PathItemElement, _folder: Option<&mut dyn Fold>| {
                if let Some(string_elem) = value.as_string() {
                    target.set_summary(string_elem.clone());
                    Some(())
                } else {
                    None
                }
            },
            "description" => |value: &Element, target: &mut PathItemElement, _folder: Option<&mut dyn Fold>| {
                if let Some(string_elem) = value.as_string() {
                    target.set_description(string_elem.clone());
                    Some(())
                } else {
                    None
                }
            },
        });
    }

    /// 从通用 Element 构建 InfoElement
    pub fn build_info(&self, source: &Element, ctx: &BuildContext) -> Result<InfoElement> {
        if let Element::Object(obj) = source {
            let mut info = InfoElement::new();
            
            for member in &obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    let field_name = &key_str.content;
                    self.info_handlers.dispatch(field_name, member.value.as_ref(), &mut info, None);
                }
            }
            
            // 写入路径元数据
            ctx.inject_path_meta(&mut info.inner.meta.properties);
            
            Ok(info)
        } else {
            Err(FieldError::new("InfoElement expects an object"))
        }
    }

    /// 从通用 Element 构建 ServerElement
    pub fn build_server(&self, source: &Element, ctx: &BuildContext) -> Result<ServerElement> {
        if let Element::Object(obj) = source {
            let mut server = ServerElement::new();
            
            for member in &obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    let field_name = &key_str.content;
                    self.server_handlers.dispatch(field_name, member.value.as_ref(), &mut server, None);
                }
            }
            
            ctx.inject_path_meta(&mut server.inner.meta.properties);
            
            Ok(server)
        } else {
            Err(FieldError::new("ServerElement expects an object"))
        }
    }

    /// 从通用 Element 构建 PathItemElement
    pub fn build_path_item(&self, source: &Element, ctx: &BuildContext) -> Result<PathItemElement> {
        if let Element::Object(obj) = source {
            let mut path_item = PathItemElement::new();
            
            for member in &obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    let field_name = &key_str.content;
                    self.path_item_handlers.dispatch(field_name, member.value.as_ref(), &mut path_item, None);
                }
            }
            
            ctx.inject_path_meta(&mut path_item.inner.meta.properties);
            
            Ok(path_item)
        } else {
            Err(FieldError::new("PathItemElement expects an object"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apidom_ast::{ObjectElement, StringElement, MemberElement};

    #[test]
    fn test_build_info() {
        let dispatch = BuilderDispatch::new();
        
        // 构建测试 info 对象
        let mut info_obj = ObjectElement::new();
        info_obj.content.push(MemberElement::new(
            Element::String(StringElement::new("title")),
            Element::String(StringElement::new("Test API"))
        ));
        info_obj.content.push(MemberElement::new(
            Element::String(StringElement::new("version")),
            Element::String(StringElement::new("1.0.0"))
        ));
        
        let info_element = Element::Object(info_obj);
        
        // 构建 InfoElement
        let ctx = BuildContext::default();
        let info = dispatch.build_info(&info_element, &ctx).unwrap();
        
        // 验证字段
        assert!(info.title().is_some());
        assert_eq!(info.title().unwrap().content, "Test API");
        assert!(info.version().is_some());
        assert_eq!(info.version().unwrap().content, "1.0.0");
    }
} 