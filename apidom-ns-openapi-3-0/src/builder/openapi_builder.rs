use apidom_ast::*;
use crate::elements::openapi::OpenapiElement;

/// 构建 `OpenapiElement`，从 Minim StringElement → OpenapiElement
///
/// 示例输入: StringElement("3.0.3")
pub fn build_openapi(element: &Element) -> Option<OpenapiElement> {
    element.as_string().map(|s| OpenapiElement::from_element(s.clone()))
}