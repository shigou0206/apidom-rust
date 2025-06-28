use apidom_ast::minim_model::*;
use crate::elements::external_documentation::ExternalDocumentationElement;

/// 构建 ExternalDocumentationElement
pub fn build_external_docs(element: &Element) -> Option<ExternalDocumentationElement> {
    let object = element.as_object()?;
    Some(ExternalDocumentationElement::with_content(object.clone()))
}