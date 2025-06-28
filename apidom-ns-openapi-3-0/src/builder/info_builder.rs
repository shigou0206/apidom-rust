use apidom_ast::minim_model::*;
use crate::elements::info::InfoElement;

pub fn build_info(element: &Element) -> Option<InfoElement> {
    let object = element.as_object()?;
    Some(InfoElement::with_content(object.clone()))
}