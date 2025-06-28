use apidom_ast::minim_model::*;
use crate::elements::security_scheme::SecuritySchemeElement;

/// 构建 OpenAPI SecuritySchemeElement（从 Minim Element 转换）
/// 例如：
/// {
///   "type": "http",
///   "scheme": "basic",
///   "description": "Basic auth"
/// }
pub fn build_security_scheme(element: &Element) -> Option<SecuritySchemeElement> {
    let object = element.as_object()?.clone();
    Some(SecuritySchemeElement::with_content(object))
}