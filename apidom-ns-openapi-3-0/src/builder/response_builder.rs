use apidom_ast::minim_model::*;
use crate::elements::response::ResponseElement;

/// 构建 OpenAPI ResponseElement（从 Minim Object 转换）
///
/// 例如：
/// {
///   "description": "Success",
///   "content": {
///     "application/json": {
///       "schema": { ... }
///     }
///   }
/// }
pub fn build_response(element: &Element) -> Option<ResponseElement> {
    let object = element.as_object()?;
    Some(ResponseElement::with_content(object.clone()))
}