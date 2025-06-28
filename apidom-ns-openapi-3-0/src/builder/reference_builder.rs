use apidom_ast::minim_model::*;
use crate::elements::reference::ReferenceElement;

pub fn build_reference(element: &Element) -> Option<ReferenceElement> {
    let obj = element.as_object()?.clone();
    Some(ReferenceElement::with_content(obj))
}