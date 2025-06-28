use apidom_ast::minim_model::*;
use crate::elements::oauth_flow::OAuthFlowElement;

/// 构建 OpenAPI OAuthFlowElement（适配 Minim ObjectElement → OAuthFlowElement）
///
/// 示例输入:
/// {
///   "authorizationUrl": "https://example.com/auth",
///   "tokenUrl": "https://example.com/token",
///   "refreshUrl": "https://example.com/refresh",
///   "scopes": { "read:pets": "read your pets" }
/// }
pub fn build_oauth_flow(element: &Element) -> Option<OAuthFlowElement> {
    let obj = element.as_object()?;
    let mut flow = OAuthFlowElement::new();

    for member in &obj.content {
        if let Element::String(key) = &*member.key {
            match key.content.as_str() {
                "authorizationUrl" => {
                    if let Element::String(val) = &*member.value {
                        flow.set_authorization_url(val.clone());
                    }
                }
                "tokenUrl" => {
                    if let Element::String(val) = &*member.value {
                        flow.set_token_url(val.clone());
                    }
                }
                "refreshUrl" => {
                    if let Element::String(val) = &*member.value {
                        flow.set_refresh_url(val.clone());
                    }
                }
                "scopes" => {
                    if let Element::Object(val) = &*member.value {
                        flow.set_scopes(val.clone());
                    }
                }
                _ => {
                    // 保留未知字段（可选）
                    flow.object.set(&key.content, (*member.value).clone());
                }
            }
        }
    }

    Some(flow)
}