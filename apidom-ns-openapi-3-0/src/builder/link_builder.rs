use apidom_ast::minim_model::*;
use crate::elements::link::LinkElement;

/// 从通用 Minim Element 构造 OpenAPI LinkElement
pub fn build_link(element: &Element) -> Option<LinkElement> {
    let obj = element.as_object()?;
    Some(LinkElement::with_content(obj.clone()))
}