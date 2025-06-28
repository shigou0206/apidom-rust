use apidom_ast::minim_model::*;
use crate::elements::tag::TagElement;

pub fn build_tag(element: &Element) -> Option<TagElement> {
    let obj = element.as_object()?.clone();
    let tag = TagElement::with_content(obj);
    if tag.name().is_none() {
        return None;
    }
    Some(tag)
}