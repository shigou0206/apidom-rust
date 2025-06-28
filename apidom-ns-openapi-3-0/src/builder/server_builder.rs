use apidom_ast::minim_model::*;
use crate::elements::server::ServerElement;

/// 构建 OpenAPI ServerElement（从 Minim Element → ServerElement）
/// 结构示例：
/// {
///   "url": "https://api.example.com",
///   "description": "Main server",
///   "variables": {
///     "port": {
///       "default": "443",
///       "enum": ["80", "443"],
///       "description": "Port to connect to"
///     }
///   }
/// }
pub fn build_server(element: &Element) -> Option<ServerElement> {
    let object = element.as_object()?.clone();

    let mut server = ServerElement::with_content(object.clone());

    // 校验必要字段
    if server.url().is_none() {
        return None;
    }

    // 变量解析（可选字段）
    if let Some(vars_obj) = server.variables() {
        let mut processed_vars = ObjectElement::new();

        for member in &vars_obj.content {
            if let Element::String(key) = &*member.key {
                let value = &*member.value;

                if let Some(var_elem) = crate::builder::server_variable_builder::build_server_variable(value) {
                    processed_vars.set(&key.content, Element::Object(var_elem.object));
                }
            }
        }

        server.set_variables(processed_vars);
    }

    Some(server)
}