use apidom_ast::minim_model::*;
use crate::elements::parameter::ParameterElement;

/// 构建 OpenAPI `ParameterElement`
///
/// 接收 Minim 风格的 Element，如果是合法的 Object，则包装为 `ParameterElement`
pub fn build_parameter(element: &Element) -> Option<ParameterElement> {
    let object = element.as_object()?.clone();
    Some(ParameterElement::with_content(object))
}