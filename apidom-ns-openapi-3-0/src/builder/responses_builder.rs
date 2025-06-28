use apidom_ast::minim_model::*;
use crate::elements::responses::ResponsesElement;

/// 构建 OpenAPI ResponsesElement（从 Minim ObjectElement 转换）
/// 例如：
/// {
///   "default": { ... },
///   "200": { ... },
///   "404": { "$ref": "#/components/responses/NotFound" }
/// }
pub fn build_responses(element: &Element) -> Option<ResponsesElement> {
    let object = element.as_object()?;
    let mut responses = ResponsesElement::new();

    for member in &object.content {
        if let Element::String(key_str) = &*member.key {
            responses.set_status_response(&key_str.content, (*member.value).clone());
        }
    }

    Some(responses)
}