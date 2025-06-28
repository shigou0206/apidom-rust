use apidom_ast::minim_model::*;
use crate::elements::example::ExampleElement;

/// 从 Minim Element 构建 OpenAPI ExampleElement
pub fn build_example(element: &Element) -> Option<ExampleElement> {
    let object = element.as_object()?;
    Some(ExampleElement::with_content(object.clone()))
}