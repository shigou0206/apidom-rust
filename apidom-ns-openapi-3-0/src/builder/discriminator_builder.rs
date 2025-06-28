use apidom_ast::minim_model::*;
use crate::elements::discriminator::DiscriminatorElement;

pub fn build_discriminator(element: &Element) -> Option<DiscriminatorElement> {
    let object = element.as_object()?;
    Some(DiscriminatorElement::with_content(object.clone()))
}