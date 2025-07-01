use apidom_ast::*;
use crate::elements::openapi3_0::OpenApi3_0Element;
use crate::builder::*;

pub fn build_openapi_3_0(element: &Element) -> Option<OpenApi3_0Element> {
    let obj = element.as_object()?;
    let mut openapi = OpenApi3_0Element::new();

    for member in &obj.content {
        if let Element::String(key) = &*member.key {
            let val = &*member.value;

            match key.content.as_str() {
                "openapi" => {
                    if let Some(str_elem) = val.as_string() {
                        openapi.set_openapi(str_elem.clone());
                    }
                }
                "info" => {
                    if let Some(info) = info_builder::build_info(val) {
                        openapi.set_info(info.object);
                    }
                }
                "servers" => {
                    if let Some(array) = val.as_array() {
                        openapi.set_servers(array.clone());
                    }
                }
                "paths" => {
                    if let Some(paths) = paths_builder::build_paths(val) {
                        openapi.set_paths(paths.object);
                    }
                }
                "components" => {
                    if let Some(components) = components_builder::build_components(val.clone()) {
                        openapi.set_components(components.object);
                    }
                }
                "security" => {
                    if let Some(array) = val.as_array() {
                        openapi.set_security(array.clone());
                    }
                }
                "tags" => {
                    if let Some(array) = val.as_array() {
                        openapi.set_tags(array.clone());
                    }
                }
                "externalDocs" => {
                    if let Some(docs) = external_documentation_builder::build_external_docs(val) {
                        openapi.set_external_docs(docs.object);
                    }
                }
                _ => {
                    // unknown key - skip or log
                }
            }
        }
    }

    Some(openapi)
}