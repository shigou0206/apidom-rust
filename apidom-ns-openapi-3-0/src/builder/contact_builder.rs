use apidom_ast::minim_model::*;
use crate::elements::contact::ContactElement;

pub fn build_contact(element: &Element) -> Option<ContactElement> {
    let object = element.as_object()?;
    Some(ContactElement::with_content(object.clone()))
}