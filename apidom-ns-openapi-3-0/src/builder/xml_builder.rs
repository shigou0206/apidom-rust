use apidom_ast::minim_model::*;
use crate::elements::xml::XmlElement;

pub fn build_xml(element: &Element) -> Option<XmlElement> {
    let obj = element.as_object()?.clone();
    Some(XmlElement::with_content(obj))
}