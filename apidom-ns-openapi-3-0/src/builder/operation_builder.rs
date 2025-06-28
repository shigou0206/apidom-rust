use apidom_ast::minim_model::*;
use crate::elements::operation::OperationElement;

/// 从 Minim Object 构建 OperationElement
pub fn build_operation(element: &Element) -> Option<OperationElement> {
    let obj = element.as_object()?.clone();
    Some(OperationElement::with_content(obj))
}