use apidom_ast::minim_model::*;
use crate::elements::media_type::MediaTypeElement;

pub fn build_media_type(element: &Element) -> Option<MediaTypeElement> {
    let obj = element.as_object()?;
    Some(MediaTypeElement::with_content(obj.clone()))
}