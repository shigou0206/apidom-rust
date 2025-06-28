use apidom_ast::minim_model::*;
use crate::elements::components::ComponentsElement;

pub fn build_components(element: &Element) -> Option<ComponentsElement> {
    let object = element.as_object()?;
    let mut components = ComponentsElement::new();

    for member in &object.content {
        if let Element::String(key) = &*member.key {
            let value = &*member.value;
            match key.content.as_str() {
                "schemas" => {
                    if let Some(obj) = value.as_object() {
                        components.set_schemas(obj.clone());
                    }
                }
                "responses" => {
                    if let Some(obj) = value.as_object() {
                        components.set_responses(obj.clone());
                    }
                }
                "parameters" => {
                    if let Some(obj) = value.as_object() {
                        components.set_parameters(obj.clone());
                    }
                }
                "examples" => {
                    if let Some(obj) = value.as_object() {
                        components.set_examples(obj.clone());
                    }
                }
                "requestBodies" => {
                    if let Some(obj) = value.as_object() {
                        components.set_request_bodies(obj.clone());
                    }
                }
                "headers" => {
                    if let Some(obj) = value.as_object() {
                        components.set_headers(obj.clone());
                    }
                }
                "securitySchemes" => {
                    if let Some(obj) = value.as_object() {
                        components.set_security_schemes(obj.clone());
                    }
                }
                "links" => {
                    if let Some(obj) = value.as_object() {
                        components.set_links(obj.clone());
                    }
                }
                "callbacks" => {
                    if let Some(obj) = value.as_object() {
                        components.set_callbacks(obj.clone());
                    }
                }
                _ => {
                    // 其他未知字段跳过或记录日志
                }
            }
        }
    }

    Some(components)
}