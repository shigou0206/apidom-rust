use apidom_ast::minim_model::*;
use crate::elements::callback::CallbackElement;

/// 构建 OpenAPI CallbackElement（适配 Minim ObjectElement → CallbackElement）
/// 例如：
/// {
///   "{$request.body#/callbackUrl}": {
///     "post": { ... }
///   }
/// }
pub fn build_callback(element: &Element) -> Option<CallbackElement> {
    let object = element.as_object()?;
    let mut callback = CallbackElement::new();

    for member in &object.content {
        if let Element::String(key_str) = &*member.key {
            callback.set(&key_str.content, (*member.value).clone());
        }
    }

    Some(callback)
}