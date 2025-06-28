use apidom_ast::minim_model::*;
use crate::builder::*;
use apidom_ast::fold::{Fold, DefaultFolder};

/// Fold that transforms a generic Element AST into an OpenAPI 3.0 Element AST.
#[derive(Debug, Default)]
pub struct OpenApiBuilderFolder;

impl OpenApiBuilderFolder {
    pub fn new() -> Self {
        Self
    }
}

impl Fold for OpenApiBuilderFolder {
    fn fold_object_element(&mut self, element: ObjectElement) -> Element {
        let element_type = element.element.as_str();

        match element_type {
            "openApi3_0" => {
                if let Some(built) = build_openapi3_0(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "info" => {
                if let Some(built) = build_info(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "paths" => {
                if let Some(built) = build_paths(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "pathItem" => {
                if let Some(built) = build_path_item(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "operation" => {
                if let Some(built) = build_operation(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "parameter" => {
                if let Some(built) = build_parameter(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "requestBody" => {
                if let Some(built) = build_request_body(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "responses" => {
                if let Some(built) = build_responses(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "response" => {
                if let Some(built) = build_response(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "mediaType" => {
                if let Some(built) = build_media_type(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "example" => {
                if let Some(built) = build_example(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "header" => {
                if let Some(built) = build_header(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "components" => {
                if let Some(built) = build_components(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "securityRequirement" => {
                if let Some(built) = build_security_requirement(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "securityScheme" => {
                if let Some(built) = build_security_scheme(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "server" => {
                if let Some(built) = build_server(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "serverVariable" => {
                if let Some(built) = build_server_variable(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "callback" => {
                if let Some(built) = build_callback(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "link" => {
                if let Some(built) = build_link(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "xml" => {
                if let Some(built) = build_xml(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "encoding" => {
                if let Some(built) = build_encoding(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "tag" => {
                if let Some(built) = build_tag(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "discriminator" => {
                if let Some(built) = build_discriminator(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "license" => {
                if let Some(built) = build_license(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "contact" => {
                if let Some(built) = build_contact(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "oAuthFlow" => {
                if let Some(built) = build_oauth_flow(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "oAuthFlows" => {
                if let Some(built) = build_oauth_flows(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "reference" => {
                if let Some(built) = build_reference(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            _ => DefaultFolder.fold_object_element(element),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_builder_folder_new() {
        let folder = OpenApiBuilderFolder::new();
        assert!(format!("{:?}", folder).contains("OpenApiBuilderFolder"));
    }

    #[test]
    fn test_openapi_builder_folder_default() {
        let folder = OpenApiBuilderFolder::default();
        assert!(format!("{:?}", folder).contains("OpenApiBuilderFolder"));
    }

    #[test]
    fn test_fold_unknown_element_type() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("unknown");
        
        let result = folder.fold_object_element(element.clone());
        
        // 应该使用 DefaultFolder 处理未知类型
        assert!(matches!(result, Element::Object(_)));
    }

    #[test]
    fn test_fold_contact_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("contact");
        element.set("name", Element::String(StringElement::new("API Team")));
        
        let result = folder.fold_object_element(element);
        
        // 验证结果是 Object 类型
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            // 验证元素类型被正确设置
            assert_eq!(obj.element, "contact");
        }
    }

    #[test]
    fn test_fold_info_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("info");
        element.set("title", Element::String(StringElement::new("Test API")));
        element.set("version", Element::String(StringElement::new("1.0.0")));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "info");
            // 验证字段被保留
            assert!(obj.get("title").is_some());
            assert!(obj.get("version").is_some());
        }
    }

    #[test]
    fn test_fold_components_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("components");
        
        // 添加 schemas
        let mut schemas = ObjectElement::new();
        schemas.set("User", Element::Object(ObjectElement::new()));
        element.set("schemas", Element::Object(schemas));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "components");
            assert!(obj.get("schemas").is_some());
        }
    }

    #[test]
    fn test_fold_callback_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("callback");
        
        // 添加回调表达式
        element.set("{$request.body#/id}", Element::Object(ObjectElement::new()));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "callback");
        }
    }

    #[test]
    fn test_fold_server_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("server");
        element.set("url", Element::String(StringElement::new("https://api.example.com")));
        element.set("description", Element::String(StringElement::new("Production server")));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "server");
            assert!(obj.get("url").is_some());
            assert!(obj.get("description").is_some());
        }
    }

    #[test]
    fn test_fold_parameter_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("parameter");
        element.set("name", Element::String(StringElement::new("id")));
        element.set("in", Element::String(StringElement::new("path")));
        element.set("required", Element::Boolean(BooleanElement::new(true)));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "parameter");
            assert!(obj.get("name").is_some());
            assert!(obj.get("in").is_some());
            assert!(obj.get("required").is_some());
        }
    }

    #[test]
    fn test_fold_response_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("response");
        element.set("description", Element::String(StringElement::new("Successful response")));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "response");
            assert!(obj.get("description").is_some());
        }
    }

    #[test]
    fn test_fold_security_scheme_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("securityScheme");
        element.set("type", Element::String(StringElement::new("http")));
        element.set("scheme", Element::String(StringElement::new("bearer")));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "securityScheme");
            assert!(obj.get("type").is_some());
            assert!(obj.get("scheme").is_some());
        }
    }

    #[test]
    fn test_fold_license_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("license");
        element.set("name", Element::String(StringElement::new("MIT")));
        element.set("url", Element::String(StringElement::new("https://opensource.org/licenses/MIT")));
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "license");
            assert!(obj.get("name").is_some());
            assert!(obj.get("url").is_some());
        }
    }

    #[test]
    fn test_fold_multiple_element_types() {
        let mut folder = OpenApiBuilderFolder::new();
        
        // 测试多种元素类型
        let element_types = vec![
            "openApi3_0", "info", "paths", "pathItem", "operation", 
            "parameter", "requestBody", "responses", "response", "mediaType",
            "example", "header", "components", "securityRequirement", 
            "securityScheme", "server", "serverVariable", "callback",
            "link", "xml", "encoding", "tag", "discriminator", 
            "license", "contact", "oAuthFlow", "oAuthFlows", "reference"
        ];
        
        for element_type in element_types {
            let mut element = ObjectElement::new();
            element.set_element_type(element_type);
            
            let result = folder.fold_object_element(element);
            
            assert!(matches!(result, Element::Object(_)), "Failed for element type: {}", element_type);
            if let Element::Object(obj) = result {
                assert_eq!(obj.element, element_type, "Element type mismatch for: {}", element_type);
            }
        }
    }

    #[test]
    fn test_fold_nested_elements() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut root = ObjectElement::new();
        root.set_element_type("info");
        root.set("title", Element::String(StringElement::new("Test API")));
        root.set("version", Element::String(StringElement::new("1.0.0")));
        
        // 添加嵌套的 contact 元素
        let mut contact = ObjectElement::new();
        contact.set_element_type("contact");
        contact.set("name", Element::String(StringElement::new("Support")));
        root.set("contact", Element::Object(contact));
        
        let result = folder.fold_object_element(root);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "info");
            assert!(obj.get("contact").is_some());
            
            // 验证嵌套元素也被正确处理
            if let Some(Element::Object(contact_obj)) = obj.get("contact") {
                assert_eq!(contact_obj.element, "contact");
            }
        }
    }

    #[test]
    fn test_fold_empty_element() {
        let mut folder = OpenApiBuilderFolder::new();
        let mut element = ObjectElement::new();
        element.set_element_type("info");
        
        let result = folder.fold_object_element(element);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "info");
            // 空元素应该仍然被正确处理
        }
    }

    #[test]
    fn test_fold_realistic_openapi_scenario() {
        let mut folder = OpenApiBuilderFolder::new();
        
        // 创建一个真实的 OpenAPI 结构
        let mut openapi = ObjectElement::new();
        openapi.set_element_type("openApi3_0");
        openapi.set("openapi", Element::String(StringElement::new("3.0.3")));
        
        // 添加 info
        let mut info = ObjectElement::new();
        info.set_element_type("info");
        info.set("title", Element::String(StringElement::new("Pet Store API")));
        info.set("version", Element::String(StringElement::new("1.0.0")));
        
        // 添加 contact 到 info
        let mut contact = ObjectElement::new();
        contact.set_element_type("contact");
        contact.set("email", Element::String(StringElement::new("support@petstore.com")));
        info.set("contact", Element::Object(contact));
        
        openapi.set("info", Element::Object(info));
        
        // 添加 components
        let mut components = ObjectElement::new();
        components.set_element_type("components");
        let mut schemas = ObjectElement::new();
        schemas.set("Pet", Element::Object(ObjectElement::new()));
        components.set("schemas", Element::Object(schemas));
        openapi.set("components", Element::Object(components));
        
        let result = folder.fold_object_element(openapi);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(obj) = result {
            assert_eq!(obj.element, "openApi3_0");
            assert!(obj.get("info").is_some());
            assert!(obj.get("components").is_some());
            
            // 验证嵌套结构
            if let Some(Element::Object(info_obj)) = obj.get("info") {
                assert_eq!(info_obj.element, "info");
                assert!(info_obj.get("contact").is_some());
            }
            
            if let Some(Element::Object(components_obj)) = obj.get("components") {
                assert_eq!(components_obj.element, "components");
                assert!(components_obj.get("schemas").is_some());
            }
        }
    }

    // JSON to OpenAPI AST conversion tests
    #[test]
    fn test_json_to_openapi_ast_simple() {
        use apidom_ast::fold::{JsonFolder, FoldFromCst};
        
        let json_str = r#"{"openapi": "3.0.3", "info": {"title": "Test API", "version": "1.0.0"}}"#;
        
        // 1. JSON -> CST
        let cst = apidom_cst::parse_json_to_cst(json_str);
        
        // 2. CST -> Generic AST
        let mut json_folder = JsonFolder::new();
        let generic_ast = json_folder.fold_from_cst(&cst);
        
        // 3. 设置元素类型
        if let Element::Object(mut obj) = generic_ast {
            obj.set_element_type("openApi3_0");
            
            // 设置嵌套对象类型
            for member in &mut obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    if key_str.content == "info" {
                        if let Element::Object(info_obj) = member.value.as_mut() {
                            info_obj.set_element_type("info");
                        }
                    }
                }
            }
            
            // 4. Generic AST -> OpenAPI AST
            let mut openapi_folder = OpenApiBuilderFolder::new();
            let openapi_ast = openapi_folder.fold_object_element(obj);
            
            // 5. 验证结果
            assert!(matches!(openapi_ast, Element::Object(_)));
            if let Element::Object(openapi_obj) = openapi_ast {
                assert_eq!(openapi_obj.element, "openApi3_0");
                assert!(openapi_obj.get("openapi").is_some());
                assert!(openapi_obj.get("info").is_some());
                
                if let Some(Element::Object(info_obj)) = openapi_obj.get("info") {
                    assert_eq!(info_obj.element, "info");
                }
            }
        }
    }

    #[test]
    fn test_json_to_openapi_ast_with_contact() {
        use apidom_ast::fold::{JsonFolder, FoldFromCst};
        
        let json_str = r#"{
            "openapi": "3.0.3",
            "info": {
                "title": "Test API",
                "version": "1.0.0",
                "contact": {
                    "name": "API Team",
                    "email": "api@example.com"
                }
            }
        }"#;
        
        // 1. JSON -> CST
        let cst = apidom_cst::parse_json_to_cst(json_str);
        
        // 2. CST -> Generic AST
        let mut json_folder = JsonFolder::new();
        let generic_ast = json_folder.fold_from_cst(&cst);
        
        // 3. 设置元素类型
        if let Element::Object(mut obj) = generic_ast {
            obj.set_element_type("openApi3_0");
            
            // 设置嵌套对象类型
            for member in &mut obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    if key_str.content == "info" {
                        if let Element::Object(info_obj) = member.value.as_mut() {
                            info_obj.set_element_type("info");
                            // 设置 contact
                            for info_member in &mut info_obj.content {
                                if let Element::String(info_key) = info_member.key.as_ref() {
                                    if info_key.content == "contact" {
                                        if let Element::Object(contact_obj) = info_member.value.as_mut() {
                                            contact_obj.set_element_type("contact");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 4. Generic AST -> OpenAPI AST
            let mut openapi_folder = OpenApiBuilderFolder::new();
            let openapi_ast = openapi_folder.fold_object_element(obj);
            
            // 5. 验证结果
            assert!(matches!(openapi_ast, Element::Object(_)));
            if let Element::Object(openapi_obj) = openapi_ast {
                assert_eq!(openapi_obj.element, "openApi3_0");
                
                if let Some(Element::Object(info_obj)) = openapi_obj.get("info") {
                    assert_eq!(info_obj.element, "info");
                    
                    if let Some(Element::Object(contact_obj)) = info_obj.get("contact") {
                        assert_eq!(contact_obj.element, "contact");
                    }
                }
            }
        }
    }

    #[test]
    fn test_json_to_openapi_ast_with_servers() {
        use apidom_ast::fold::{JsonFolder, FoldFromCst};
        
        let json_str = r#"{
            "openapi": "3.0.3",
            "info": {"title": "Test API", "version": "1.0.0"},
            "servers": [
                {"url": "https://api.example.com", "description": "Production"},
                {"url": "https://staging-api.example.com", "description": "Staging"}
            ]
        }"#;
        
        // 1. JSON -> CST
        let cst = apidom_cst::parse_json_to_cst(json_str);
        
        // 2. CST -> Generic AST
        let mut json_folder = JsonFolder::new();
        let generic_ast = json_folder.fold_from_cst(&cst);
        
        // 3. 设置元素类型
        if let Element::Object(mut obj) = generic_ast {
            obj.set_element_type("openApi3_0");
            
            for member in &mut obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    match key_str.content.as_str() {
                        "info" => {
                            if let Element::Object(info_obj) = member.value.as_mut() {
                                info_obj.set_element_type("info");
                            }
                        }
                        "servers" => {
                            if let Element::Array(servers_arr) = member.value.as_mut() {
                                for server in &mut servers_arr.content {
                                    if let Element::Object(server_obj) = server {
                                        server_obj.set_element_type("server");
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            
            // 4. Generic AST -> OpenAPI AST
            let mut openapi_folder = OpenApiBuilderFolder::new();
            let openapi_ast = openapi_folder.fold_object_element(obj);
            
            // 5. 验证结果
            assert!(matches!(openapi_ast, Element::Object(_)));
            if let Element::Object(openapi_obj) = openapi_ast {
                assert_eq!(openapi_obj.element, "openApi3_0");
                
                if let Some(Element::Array(servers_arr)) = openapi_obj.get("servers") {
                    assert_eq!(servers_arr.content.len(), 2);
                    for server in &servers_arr.content {
                        if let Element::Object(server_obj) = server {
                            assert_eq!(server_obj.element, "server");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_json_to_openapi_ast_complete_document() {
        use apidom_ast::fold::{JsonFolder, FoldFromCst};
        
        let json_str = r#"{
            "openapi": "3.0.3",
            "info": {
                "title": "Pet Store API",
                "version": "1.0.0",
                "contact": {
                    "name": "API Support",
                    "email": "support@petstore.com"
                }
            },
            "servers": [
                {
                    "url": "https://petstore.swagger.io/v2",
                    "description": "Production server"
                }
            ],
            "components": {
                "responses": {
                    "NotFound": {
                        "description": "Entity not found."
                    }
                }
            }
        }"#;
        
        // 1. JSON -> CST
        let cst = apidom_cst::parse_json_to_cst(json_str);
        
        // 2. CST -> Generic AST
        let mut json_folder = JsonFolder::new();
        let generic_ast = json_folder.fold_from_cst(&cst);
        
        // 3. 设置元素类型
        if let Element::Object(mut obj) = generic_ast {
            // 手动设置根级对象类型
            obj.set_element_type("openApi3_0");
            
            // 设置嵌套对象类型
            for member in &mut obj.content {
                if let Element::String(key_str) = member.key.as_ref() {
                    match key_str.content.as_str() {
                        "info" => {
                            if let Element::Object(info_obj) = member.value.as_mut() {
                                info_obj.set_element_type("info");
                                // 设置 info 内的 contact
                                for info_member in &mut info_obj.content {
                                    if let Element::String(info_key) = info_member.key.as_ref() {
                                        if info_key.content == "contact" {
                                            if let Element::Object(contact_obj) = info_member.value.as_mut() {
                                                contact_obj.set_element_type("contact");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        "servers" => {
                            if let Element::Array(servers_arr) = member.value.as_mut() {
                                for server in &mut servers_arr.content {
                                    if let Element::Object(server_obj) = server {
                                        server_obj.set_element_type("server");
                                    }
                                }
                            }
                        }
                        "components" => {
                            if let Element::Object(components_obj) = member.value.as_mut() {
                                components_obj.set_element_type("components");
                                // 设置 components 内的 responses
                                for comp_member in &mut components_obj.content {
                                    if let Element::String(comp_key) = comp_member.key.as_ref() {
                                        if comp_key.content == "responses" {
                                            if let Element::Object(responses_obj) = comp_member.value.as_mut() {
                                                for response_member in &mut responses_obj.content {
                                                    if let Element::Object(response_obj) = response_member.value.as_mut() {
                                                        response_obj.set_element_type("response");
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            
            // 4. Generic AST -> OpenAPI AST
            let mut openapi_folder = OpenApiBuilderFolder::new();
            let openapi_ast = openapi_folder.fold_object_element(obj);
            
            // 5. 验证结果
            assert!(matches!(openapi_ast, Element::Object(_)));
            if let Element::Object(openapi_obj) = openapi_ast {
                assert_eq!(openapi_obj.element, "openApi3_0");
                assert!(openapi_obj.get("openapi").is_some());
                assert!(openapi_obj.get("info").is_some());
                assert!(openapi_obj.get("servers").is_some());
                assert!(openapi_obj.get("components").is_some());
                
                // 验证嵌套结构
                if let Some(Element::Object(info_obj)) = openapi_obj.get("info") {
                    assert_eq!(info_obj.element, "info");
                    if let Some(Element::Object(contact_obj)) = info_obj.get("contact") {
                        assert_eq!(contact_obj.element, "contact");
                    }
                }
                
                if let Some(Element::Array(servers_arr)) = openapi_obj.get("servers") {
                    for server in &servers_arr.content {
                        if let Element::Object(server_obj) = server {
                            assert_eq!(server_obj.element, "server");
                        }
                    }
                }
                
                if let Some(Element::Object(components_obj)) = openapi_obj.get("components") {
                    assert_eq!(components_obj.element, "components");
                    if let Some(Element::Object(responses_obj)) = components_obj.get("responses") {
                        for response_member in &responses_obj.content {
                            if let Element::Object(response_obj) = response_member.value.as_ref() {
                                assert_eq!(response_obj.element, "response");
                            }
                        }
                    }
                }
            }
        }
    }
}