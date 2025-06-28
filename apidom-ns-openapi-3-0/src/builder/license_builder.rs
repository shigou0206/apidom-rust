use apidom_ast::minim_model::*;
use crate::elements::license::LicenseElement;

/// 构建 OpenAPI LicenseElement（从 Minim AST Element 转换）
pub fn build_license(element: &Element) -> Option<LicenseElement> {
    let object = element.as_object()?;
    Some(LicenseElement::with_content(object.clone()))
}