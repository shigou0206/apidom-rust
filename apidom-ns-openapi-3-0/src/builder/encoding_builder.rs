use apidom_ast::minim_model::*;
use crate::elements::encoding::EncodingElement;

pub fn build_encoding(element: &Element) -> Option<EncodingElement> {
    let object = element.as_object()?;
    Some(EncodingElement::with_content(object.clone()))
}