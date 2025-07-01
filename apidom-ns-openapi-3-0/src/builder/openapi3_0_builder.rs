use apidom_ast::*;
use crate::elements::openapi3_0::OpenApi3_0Element;

/// 构建 OpenAPI 3.0 主结构 Element
pub fn build_openapi3_0(root: &Element) -> Option<OpenApi3_0Element> {
    let obj = root.as_object()?.clone();
    let mut api = OpenApi3_0Element::with_content(obj.clone());

    // 可选：验证 openapi 字段是否存在且合法
    if let Some(openapi_str) = obj.get("openapi").and_then(Element::as_string) {
        api.set_openapi(openapi_str.clone());
    }

    if let Some(info) = obj.get("info").and_then(Element::as_object) {
        api.set_info(info.clone());
    }

    if let Some(servers) = obj.get("servers").and_then(Element::as_array) {
        api.set_servers(servers.clone());
    }

    if let Some(paths) = obj.get("paths").and_then(Element::as_object) {
        api.set_paths(paths.clone());
    }

    if let Some(components) = obj.get("components").and_then(Element::as_object) {
        api.set_components(components.clone());
    }

    if let Some(security) = obj.get("security").and_then(Element::as_array) {
        api.set_security(security.clone());
    }

    if let Some(tags) = obj.get("tags").and_then(Element::as_array) {
        api.set_tags(tags.clone());
    }

    if let Some(docs) = obj.get("externalDocs").and_then(Element::as_object) {
        api.set_external_docs(docs.clone());
    }

    Some(api)
}