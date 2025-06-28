use apidom_ast::minim_model::*;
use crate::elements::server_variable::ServerVariableElement;

/// 从 Minim Element 构建 OpenAPI `ServerVariableElement`
/// 
/// 预期输入为一个 ObjectElement，其中包含字段：
/// - enum (ArrayElement)
/// - default (StringElement)
/// - description (StringElement)
pub fn build_server_variable(element: &Element) -> Option<ServerVariableElement> {
    let object = element.as_object()?.clone();

    let server_var = ServerVariableElement::with_content(object.clone());

    // 校验 default 字段存在是合理的（OpenAPI 要求必须存在）
    if server_var.default_value().is_none() {
        return None; // 无效的 ServerVariable（OpenAPI 规范中 default 是必需的）
    }

    Some(server_var)
}