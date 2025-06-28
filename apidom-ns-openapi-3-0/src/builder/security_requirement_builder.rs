use apidom_ast::minim_model::*;
use crate::elements::security_requirement::SecurityRequirementElement;

/// 构建 OpenAPI SecurityRequirementElement（从 Minim ObjectElement 转换）
/// 示例输入：
/// {
///   "petstore_auth": ["write:pets", "read:pets"],
///   "api_key": []
/// }
pub fn build_security_requirement(element: &Element) -> Option<SecurityRequirementElement> {
    let object = element.as_object()?.clone();
    let mut sec_req = SecurityRequirementElement::new();

    for member in &object.content {
        if let Element::String(key) = &*member.key {
            if let Element::Array(arr) = &*member.value {
                sec_req.set_scopes(&key.content, arr.clone());
            }
        }
    }

    Some(sec_req)
}