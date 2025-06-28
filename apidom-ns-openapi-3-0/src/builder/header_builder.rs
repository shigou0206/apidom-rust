use apidom_ast::minim_model::*;
use crate::elements::header::HeaderElement;

/// 构建 HeaderElement（从 Minim ObjectElement → OpenAPI HeaderElement）
pub fn build_header(element: &Element) -> Option<HeaderElement> {
    let object = element.as_object()?;
    Some(HeaderElement::with_content(object.clone()))
}