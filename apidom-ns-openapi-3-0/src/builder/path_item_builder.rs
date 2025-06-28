use apidom_ast::minim_model::*;
use crate::elements::path_item::PathItemElement;

/// 构建 OpenAPI PathItemElement
pub fn build_path_item(element: &Element) -> Option<PathItemElement> {
    let object = element.as_object()?.clone();
    Some(PathItemElement::with_content(object))
}