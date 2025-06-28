use apidom_ast::minim_model::*;
use crate::elements::request_body::RequestBodyElement;

pub fn build_request_body(elem: &Element) -> Option<RequestBodyElement> {
    let obj = elem.as_object()?.clone();
    Some(RequestBodyElement::with_content(obj))
}