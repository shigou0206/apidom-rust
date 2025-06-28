use apidom_ast::minim_model::*;
use crate::elements::oauth_flows::OAuthFlowsElement;
use crate::builder::oauth_flow_builder::build_oauth_flow;

/// 构建 OpenAPI OAuthFlowsElement（适配 Minim ObjectElement → OAuthFlowsElement）
///
/// 示例输入:
/// {
///   "implicit": { "authorizationUrl": "...", "scopes": {...} },
///   "password": { "tokenUrl": "...", "scopes": {...} }
/// }
pub fn build_oauth_flows(element: &Element) -> Option<OAuthFlowsElement> {
    let obj = element.as_object()?;
    let mut flows = OAuthFlowsElement::new();

    for member in &obj.content {
        if let Element::String(key) = &*member.key {
            match key.content.as_str() {
                "implicit" | "password" | "clientCredentials" | "authorizationCode" => {
                    if let Some(flow) = build_oauth_flow(&*member.value) {
                        match key.content.as_str() {
                            "implicit" => flows.set_implicit(flow),
                            "password" => flows.set_password(flow),
                            "clientCredentials" => flows.set_client_credentials(flow),
                            "authorizationCode" => flows.set_authorization_code(flow),
                            _ => {}
                        }
                    }
                }
                _ => {
                    // 可选：保留额外字段
                    flows.object.set(&key.content, (*member.value).clone());
                }
            }
        }
    }

    Some(flows)
}