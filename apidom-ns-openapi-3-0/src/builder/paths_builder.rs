use apidom_ast::minim_model::*;
use crate::elements::paths::PathsElement;

/// 从 Minim ObjectElement 构造 PathsElement
pub fn build_paths(element: &Element) -> Option<PathsElement> {
    let object = element.as_object()?.clone();
    Some(PathsElement::with_content(object))
}