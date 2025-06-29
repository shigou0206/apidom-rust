use apidom_ast::minim_model::*;
use crate::builder::*;
use apidom_ast::fold::{Fold, DefaultFolder};
use crate::builder::paths_builder::build_and_decorate_paths;
use crate::builder::schema_builder::{build_openapi_schema, build_and_decorate_schema};
use crate::builder::components_builder::{build_and_decorate_components};

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
                if let Some(built) = build_and_decorate_paths(&Element::Object(element.clone()), Some(self)) {
                    return Element::Object(built.object);
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "pathItem" => {
                if let Some(built) = build_and_decorate_path_item(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
                if let Some(built) = build_and_decorate_parameter(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "requestBody" => {
                if let Some(built) = build_and_decorate_request_body(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
            "schema" => {
                if let Some(built) = build_and_decorate_schema(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.base.object)
                } else if let Some(built) = build_openapi_schema(&Element::Object(element.clone())) {
                    DefaultFolder.fold_object_element(built.base.object)
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
                if let Some(built) = build_and_decorate_components(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
                if let Some(built) = build_and_decorate_server_variable(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "callback" => {
                if let Some(built) = build_and_decorate_callback(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
                if let Some(built) = build_and_decorate_xml(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
                if let Some(built) = build_and_decorate_tag(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
                if let Some(built) = build_and_decorate_license(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
                if let Some(built) = build_and_decorate_oauth_flow(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
                } else {
                    DefaultFolder.fold_object_element(element)
                }
            }
            "oAuthFlows" => {
                if let Some(built) = build_and_decorate_oauth_flows(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
            "externalDocumentation" => {
                if let Some(built) = build_and_decorate_external_docs(&Element::Object(element.clone()), Some(self)) {
                    Element::Object(built.object)
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
    
    use serde_json::Value;

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

    #[test]
    fn test_fold_callback_element_with_runtime_expressions() {
        let mut folder = OpenApiBuilderFolder::new();
        
        // 创建包含运行时表达式的 callback
        let mut callback_obj = ObjectElement::new();
        callback_obj.set_element_type("callback");
        
        // 添加运行时表达式 + PathItem
        let mut path_item1 = ObjectElement::new();
        path_item1.set_element_type("pathItem");
        path_item1.set("post", Element::Object(ObjectElement::new()));
        callback_obj.set("{$request.body#/callbackUrl}", Element::Object(path_item1));
        
        // 添加普通表达式 + PathItem  
        let mut path_item2 = ObjectElement::new();
        path_item2.set_element_type("pathItem");
        path_item2.set("get", Element::Object(ObjectElement::new()));
        callback_obj.set("{$response.header.location}", Element::Object(path_item2));
        
        // 添加 $ref 引用
        let mut ref_obj = ObjectElement::new();
        ref_obj.set("$ref", Element::String(StringElement::new("#/components/pathItems/webhook")));
        callback_obj.set("webhookRef", Element::Object(ref_obj));
        
        let result = folder.fold_object_element(callback_obj);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(folded_obj) = result {
            assert_eq!(folded_obj.element, "callback");
            
            // 验证运行时表达式元数据
            if let Some(Element::Object(path_item_obj)) = folded_obj.get("{$request.body#/callbackUrl}") {
                assert!(path_item_obj.meta.properties.contains_key("runtime-expression"));
                if let Some(Value::String(expr)) = path_item_obj.meta.properties.get("runtime-expression") {
                    assert_eq!(expr, "{$request.body#/callbackUrl}");
                }
            }
            
            if let Some(Element::Object(path_item_obj)) = folded_obj.get("{$response.header.location}") {
                assert!(path_item_obj.meta.properties.contains_key("runtime-expression"));
                if let Some(Value::String(expr)) = path_item_obj.meta.properties.get("runtime-expression") {
                    assert_eq!(expr, "{$response.header.location}");
                }
            }
            
            // 验证 $ref 元数据
            if let Some(Element::Object(ref_obj)) = folded_obj.get("webhookRef") {
                assert!(ref_obj.meta.properties.contains_key("referenced-element"));
                if let Some(Value::String(ref_type)) = ref_obj.meta.properties.get("referenced-element") {
                    assert_eq!(ref_type, "callback");
                }
            }
        }
    }

    #[test]
    fn test_fold_callback_with_nested_operations() {
        let mut folder = OpenApiBuilderFolder::new();
        
        let mut callback_obj = ObjectElement::new();
        callback_obj.set_element_type("callback");
        
        // 创建包含多个操作的 PathItem
        let mut path_item = ObjectElement::new();
        path_item.set_element_type("pathItem");
        
        // 添加 POST 操作
        let mut post_op = ObjectElement::new();
        post_op.set_element_type("operation");
        post_op.set("operationId", Element::String(StringElement::new("handleCallback")));
        post_op.set("summary", Element::String(StringElement::new("Handle callback")));
        path_item.set("post", Element::Object(post_op));
        
        // 添加 GET 操作
        let mut get_op = ObjectElement::new();
        get_op.set_element_type("operation");
        get_op.set("operationId", Element::String(StringElement::new("getCallback")));
        get_op.set("summary", Element::String(StringElement::new("Get callback info")));
        path_item.set("get", Element::Object(get_op));
        
        callback_obj.set("{$request.body#/webhookUrl}", Element::Object(path_item));
        
        let result = folder.fold_object_element(callback_obj);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(folded_obj) = result {
            assert_eq!(folded_obj.element, "callback");
            
            if let Some(Element::Object(path_item_obj)) = folded_obj.get("{$request.body#/webhookUrl}") {
                // 验证 PathItem 有运行时表达式元数据
                assert!(path_item_obj.meta.properties.contains_key("runtime-expression"));
                
                // 验证嵌套的操作也被正确折叠
                if let Some(Element::Object(post_obj)) = path_item_obj.get("post") {
                    assert_eq!(post_obj.element, "operation");
                    if let Some(Element::String(op_id)) = post_obj.get("operationId") {
                        assert_eq!(op_id.content, "handleCallback");
                    }
                }
                
                if let Some(Element::Object(get_obj)) = path_item_obj.get("get") {
                    assert_eq!(get_obj.element, "operation");
                    if let Some(Element::String(op_id)) = get_obj.get("operationId") {
                        assert_eq!(op_id.content, "getCallback");
                    }
                }
            }
        }
    }

    #[test]
    fn test_fold_callback_comprehensive_scenario() {
        let mut folder = OpenApiBuilderFolder::new();
        
        // 创建一个完整的 callback 场景
        let mut callback_obj = ObjectElement::new();
        callback_obj.set_element_type("callback");
        
        // 场景1: 简单的 webhook 回调
        let mut webhook_path = ObjectElement::new();
        webhook_path.set_element_type("pathItem");
        let mut webhook_post = ObjectElement::new();
        webhook_post.set_element_type("operation");
        webhook_post.set("description", Element::String(StringElement::new("Webhook endpoint")));
        webhook_path.set("post", Element::Object(webhook_post));
        callback_obj.set("{$request.body#/webhookUrl}", Element::Object(webhook_path));
        
        // 场景2: 带参数的回调URL
        let mut param_path = ObjectElement::new();
        param_path.set_element_type("pathItem");
        let mut param_get = ObjectElement::new();
        param_get.set_element_type("operation");
        param_get.set("description", Element::String(StringElement::new("Parameterized callback")));
        param_path.set("get", Element::Object(param_get));
        callback_obj.set("{$request.query.callbackUrl}", Element::Object(param_path));
        
        // 场景3: 响应头中的回调
        let mut response_path = ObjectElement::new();
        response_path.set_element_type("pathItem");
        let mut response_put = ObjectElement::new();
        response_put.set_element_type("operation");
        response_put.set("description", Element::String(StringElement::new("Response header callback")));
        response_path.set("put", Element::Object(response_put));
        callback_obj.set("{$response.header.Location}", Element::Object(response_path));
        
        // 场景4: 引用其他的 callback 定义
        let mut ref_obj = ObjectElement::new();
        ref_obj.set("$ref", Element::String(StringElement::new("#/components/callbacks/standardWebhook")));
        callback_obj.set("standardWebhookRef", Element::Object(ref_obj));
        
        // 场景5: 普通的静态回调路径
        let mut static_path = ObjectElement::new();
        static_path.set("post", Element::Object(ObjectElement::new()));
        callback_obj.set("staticCallback", Element::Object(static_path));
        
        let result = folder.fold_object_element(callback_obj);
        
        assert!(matches!(result, Element::Object(_)));
        if let Element::Object(folded_obj) = result {
            assert_eq!(folded_obj.element, "callback");
            
            // 验证所有运行时表达式都有正确的元数据
            let runtime_expressions = [
                "{$request.body#/webhookUrl}",
                "{$request.query.callbackUrl}",
                "{$response.header.Location}"
            ];
            
            for expr in &runtime_expressions {
                if let Some(Element::Object(path_obj)) = folded_obj.get(expr) {
                    assert!(path_obj.meta.properties.contains_key("runtime-expression"));
                    if let Some(Value::String(meta_expr)) = path_obj.meta.properties.get("runtime-expression") {
                        assert_eq!(meta_expr, expr);
                    }
                }
            }
            
            // 验证引用有正确的元数据
            if let Some(Element::Object(ref_obj)) = folded_obj.get("standardWebhookRef") {
                assert!(ref_obj.meta.properties.contains_key("referenced-element"));
                assert!(ref_obj.meta.properties.contains_key("reference-path"));
            }
            
            // 验证静态回调没有运行时表达式元数据
            if let Some(Element::Object(static_obj)) = folded_obj.get("staticCallback") {
                assert!(!static_obj.meta.properties.contains_key("runtime-expression"));
            }
            
            // 验证所有的 PathItem 和 Operation 元素都被正确识别和处理
            assert!(folded_obj.has_key("{$request.body#/webhookUrl}"));
            assert!(folded_obj.has_key("{$request.query.callbackUrl}"));
            assert!(folded_obj.has_key("{$response.header.Location}"));
            assert!(folded_obj.has_key("standardWebhookRef"));
            assert!(folded_obj.has_key("staticCallback"));
        }
    }

    #[test]
    fn test_build_callback_typescript_equivalence() {
        // 这个测试验证我们的 Rust 实现与 TypeScript 版本的等价性
        // TypeScript 版本的关键特性：
        // 1. 运行时表达式检测和元数据注入
        // 2. $ref 支持
        // 3. 递归折叠
        // 4. 类型验证
        
        let mut folder = OpenApiBuilderFolder::new();
        
        // 模拟 TypeScript 中的 callback 对象
        let mut callback_obj = ObjectElement::new();
        callback_obj.set_element_type("callback");
        
        // 测试运行时表达式支持
        let mut runtime_path = ObjectElement::new();
        runtime_path.set_element_type("pathItem");
        runtime_path.set("post", Element::Object(ObjectElement::new()));
        callback_obj.set("{$request.body#/url}", Element::Object(runtime_path));
        
        // 测试 $ref 支持
        let mut ref_obj = ObjectElement::new();
        ref_obj.set("$ref", Element::String(StringElement::new("#/components/callbacks/webhook")));
        callback_obj.set("webhook", Element::Object(ref_obj));
        
        // 测试普通字符串键
        let mut normal_path = ObjectElement::new();
        normal_path.set_element_type("pathItem");
        normal_path.set("get", Element::Object(ObjectElement::new()));
        callback_obj.set("normalPath", Element::Object(normal_path));
        
        let result = folder.fold_object_element(callback_obj);
        
        // 验证结果符合 TypeScript 版本的行为
        if let Element::Object(folded) = result {
            assert_eq!(folded.element, "callback");
            
            // 验证运行时表达式有元数据
            if let Some(Element::Object(rt_path)) = folded.get("{$request.body#/url}") {
                assert!(rt_path.meta.properties.contains_key("runtime-expression"));
                if let Some(Value::String(expr)) = rt_path.meta.properties.get("runtime-expression") {
                    assert_eq!(expr, "{$request.body#/url}");
                }
            }
            
            // 验证 $ref 有元数据  
            if let Some(Element::Object(ref_element)) = folded.get("webhook") {
                assert!(ref_element.meta.properties.contains_key("referenced-element"));
                assert!(ref_element.meta.properties.contains_key("reference-path"));
                if let Some(Value::String(ref_path)) = ref_element.meta.properties.get("reference-path") {
                    assert_eq!(ref_path, "#/components/callbacks/webhook");
                }
            }
            
            // 验证普通路径没有特殊元数据
            if let Some(Element::Object(normal)) = folded.get("normalPath") {
                assert!(!normal.meta.properties.contains_key("runtime-expression"));
                assert!(!normal.meta.properties.contains_key("referenced-element"));
            }
        } else {
            panic!("Expected callback to be folded to ObjectElement");
        }
    }
}