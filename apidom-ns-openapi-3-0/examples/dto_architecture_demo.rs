//! # DTO 架构演示
//! 
//! 本示例展示完整的**职责分离**架构：
//! 1. AST 层：复杂的内部处理（解析、验证、增强）
//! 2. DTO 层：纯净的数据传输对象
//! 3. 转换层：AST → DTO 映射
//! 4. 序列化：JSON 输出，供前端使用
//! 
//! ## 架构优势
//! 
//! ```
//! Backend (Rust)                Frontend (Flutter/Dart)
//! ┌─────────────────────┐      ┌────────────────────┐
//! │ AST Layer           │  →   │ DTO Layer          │
//! │ ┌─────────────────┐ │      │ ┌────────────────┐ │
//! │ │ ExampleElement  │ │      │ │ ExampleDto     │ │
//! │ │ - metadata      │ │      │ │ - summary      │ │
//! │ │ - classes       │ │      │ │ - value        │ │
//! │ │ - fold_state    │ │      │ │ - extensions   │ │
//! │ │ - validation    │ │      │ │                │ │
//! │ └─────────────────┘ │      │ └────────────────┘ │
//! └─────────────────────┘      └────────────────────┘
//!       复杂但功能强大              简洁但完整
//! ```

use std::collections::HashMap;
use apidom_ns_openapi_3_0::dto::*;
use apidom_ns_openapi_3_0::dto::example::ExampleDto;
use apidom_ns_openapi_3_0::dto::info::{InfoDto, ContactDto, LicenseDto};
use apidom_ns_openapi_3_0::dto::schema::{SchemaDto, SchemaType, ExternalDocsDto};
use apidom_ns_openapi_3_0::dto::openapi::*;
use apidom_ns_openapi_3_0::elements::example::ExampleElement;
use apidom_ast::minim_model::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  DTO 架构演示");
    println!("================\n");
    
    // 1. 演示 AST 层的复杂性
    demo_ast_complexity()?;
    
    // 2. 演示 DTO 层的简洁性
    demo_dto_simplicity()?;
    
    // 3. 演示 AST → DTO 转换
    demo_ast_to_dto_conversion()?;
    
    // 4. 演示完整的 OpenAPI DTO
    demo_complete_openapi_dto()?;
    
    // 5. 演示前端使用场景
    demo_frontend_usage()?;
    
    // 6. 演示 Schema DTO 的功能
    schema_dto_demo();
    
    Ok(())
}

/// 演示 AST 层的复杂性
fn demo_ast_complexity() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 1. AST 层复杂性演示");
    println!("   后端专用，包含丰富的内部处理信息\n");
    
    // 创建复杂的 AST 元素
    let mut example = ExampleElement::new();
    example.set_summary(StringElement::new("User Example"));
    example.set_description(StringElement::new("示例用户数据"));
    
    // 添加复杂的嵌套值
    let mut user_obj = ObjectElement::new();
    user_obj.set("id", Element::Number(NumberElement {
        element: "number".to_string(),
        meta: MetaElement::default(),
        attributes: AttributesElement::default(),
        content: 123.0,
    }));
    user_obj.set("name", Element::String(StringElement::new("张三")));
    user_obj.set("active", Element::Boolean(BooleanElement::new(true)));
    
    example.set_value(Element::Object(user_obj));
    
    // 添加扩展字段和元数据
    example.object.set("x-internal-id", Element::String(StringElement::new("EX001")));
    example.object.meta.properties.insert("processed".to_string(), serde_json::Value::Bool(true));
    example.object.classes.content.push(Element::String(StringElement::new("enhanced")));
    
    println!("   AST 结构 (ExampleElement):");
    println!("   ├── summary: {:?}", example.summary().map(|s| &s.content));
    println!("   ├── description: {:?}", example.description().map(|s| &s.content));
    println!("   ├── value: [复杂对象结构]");
    println!("   ├── metadata: {} 个属性", example.object.meta.properties.len());
    println!("   ├── classes: {} 个语义类", example.object.classes.content.len());
    println!("   └── extensions: 包含 x-* 字段");
    println!("   💡 总计: 丰富的内部状态，用于后端处理\n");
    
    Ok(())
}

/// 演示 DTO 层的简洁性
fn demo_dto_simplicity() -> Result<(), Box<dyn std::error::Error>> {
    println!("✨ 2. DTO 层简洁性演示");
    println!("   前端友好，只包含必要的数据字段\n");
    
    // 创建简洁的 DTO
    let user_example = ExampleDto {
        summary: Some("User Example".to_string()),
        description: Some("示例用户数据".to_string()),
        value: Some(serde_json::to_string(&json!({
            "id": 123,
            "name": "张三",
            "active": true
        })).unwrap()),
        external_value: None,
        reference: None,
        extensions: {
            let mut ext = HashMap::new();
            ext.insert("x-internal-id".to_string(), "EX001".to_string());
            ext
        },
    };
    
    println!("   DTO 结构 (ExampleDto):");
    println!("   ├── summary: {:?}", user_example.summary);
    println!("   ├── description: {:?}", user_example.description);
    println!("   ├── value: [JSON 值]");
    println!("   ├── external_value: {:?}", user_example.external_value);
    println!("   ├── reference: {:?}", user_example.reference);
    println!("   └── extensions: {} 个扩展字段", user_example.extensions.len());
    println!("   💡 特点: 纯数据，易于序列化和传输\n");
    
    // 演示 JSON 序列化
    let json = serde_json::to_string_pretty(&user_example)?;
    println!("   📤 JSON 序列化结果:");
    println!("{}", json);
    println!();
    
    Ok(())
}

/// 演示 AST → DTO 转换
fn demo_ast_to_dto_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 3. AST → DTO 转换演示");
    println!("   展示如何从复杂 AST 提取纯净数据\n");
    
    // 创建 AST 元素
    let mut ast_example = ExampleElement::new();
    ast_example.set_summary(StringElement::new("API Response Example"));
    ast_example.set_description(StringElement::new("典型的 API 响应示例"));
    
    // 添加复杂值
    let response_value = json!({
        "status": "success",
        "data": {
            "users": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ]
        },
        "meta": {
            "total": 2,
            "page": 1
        }
    });
    
    // 将 JSON 转换为 AST Element（模拟复杂的解析过程）
    ast_example.set_value(json_to_element(response_value));
    
    // 添加 AST 特有的元数据
    ast_example.object.meta.properties.insert("validation_status".to_string(), json!("passed"));
    ast_example.object.meta.properties.insert("enhancement_level".to_string(), json!("full"));
    ast_example.object.set("x-api-version", Element::String(StringElement::new("v1.2.0")));
    
    println!("   🔧 AST 处理完成，包含:");
    println!("      • 解析状态、验证结果");
    println!("      • 语义增强信息");
    println!("      • 内部元数据");
    println!();
    
    // 转换为 DTO
    let dto: ExampleDto = (&ast_example).into_dto();
    
    println!("   ✅ DTO 转换结果:");
    println!("      • summary: {:?}", dto.summary);
    println!("      • description: {:?}", dto.description);
    println!("      • value: [已转换为 JSON]");
    println!("      • extensions: {} 个字段", dto.extensions.len());
    println!();
    
    // 演示数据对比
    println!("   📊 数据量对比:");
    println!("      • AST metadata: {} 个属性", ast_example.object.meta.properties.len());
    println!("      • DTO extensions: {} 个扩展", dto.extensions.len());
    println!("      • 数据精简率: ~{:.1}%", 
        (1.0 - dto.extensions.len() as f64 / ast_example.object.meta.properties.len() as f64) * 100.0);
    println!();
    
    Ok(())
}

/// 演示完整的 OpenAPI DTO
fn demo_complete_openapi_dto() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 4. 完整 OpenAPI DTO 演示");
    println!("   构建完整的 API 文档 DTO 结构\n");
    
    // 构建信息对象
    let contact = ContactDto::new()
        .with_name("API 团队")
        .with_email("api@example.com")
        .with_url("https://example.com/team");
    
    let license = LicenseDto::new("MIT")
        .with_url("https://opensource.org/licenses/MIT");
    
    let info = InfoDto::new("宠物商店 API", "1.0.0")
        .with_description("一个完整的宠物商店 API 示例")
        .with_contact(contact)
        .with_license(license);
    
    // 构建操作
    let list_pets_response = ResponseDto::new("宠物列表")
        .with_content("application/json", MediaTypeDto {
            schema: Some(SchemaDto::array(
                SchemaDto::object()
                    .with_property("id", SchemaDto::integer())
                    .with_property("name", SchemaDto::string())
                    .with_property("tag", SchemaDto::string())
                    .with_required(vec!["id", "name"])
            )),
            example: Some(serde_json::to_string(&json!([
                {"id": 1, "name": "Fluffy", "tag": "cat"},
                {"id": 2, "name": "Buddy", "tag": "dog"}
            ])).unwrap()),
            examples: None,
            encoding: None,
            extensions: Extensions::new(),
        });
    
    let list_pets_operation = OperationDto::new()
        .with_summary("列出所有宠物")
        .with_response("200", list_pets_response);
    
    let pets_path = PathItemDto::new()
        .with_get(list_pets_operation);
    
    // 构建完整的 OpenAPI 文档
    let openapi = OpenApiDto::new("3.0.3", info)
        .with_server(ServerDto::new("https://api.petstore.com/v1")
            .with_description("生产环境服务器"))
        .with_path("/pets", pets_path);
    
    println!("   📋 OpenAPI 文档结构:");
    println!("   ├── openapi: {}", openapi.openapi);
    println!("   ├── info:");
    println!("   │   ├── title: {}", openapi.info.title);
    println!("   │   ├── version: {}", openapi.info.version);
    println!("   │   ├── contact: {:?}", openapi.info.contact.as_ref().map(|c| &c.name));
    println!("   │   └── license: {:?}", openapi.info.license.as_ref().map(|l| &l.name));
    println!("   ├── servers: {} 个", openapi.servers.as_ref().map_or(0, |s| s.len()));
    println!("   └── paths: {} 个路径", openapi.paths.len());
    println!();
    
    // 生成 JSON
    let json = openapi.to_json()?;
    println!("   📦 生成的 JSON 文档大小: {} 字符", json.len());
    println!("   📤 部分 JSON 内容:");
    let lines: Vec<&str> = json.lines().take(15).collect();
    for line in lines {
        println!("      {}", line);
    }
    println!("      ...");
    println!();
    
    Ok(())
}

/// 演示前端使用场景
fn demo_frontend_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("📱 5. 前端使用场景演示");
    println!("   模拟 Flutter/Dart 前端如何使用 DTO 数据\n");
    
    // 模拟从后端接收的 JSON 数据
    let api_response_json = r#"{
        "openapi": "3.0.3",
        "info": {
            "title": "Mobile App API",
            "version": "2.1.0",
            "description": "移动应用后端 API"
        },
        "paths": {
            "/users/profile": {
                "get": {
                    "summary": "获取用户资料",
                    "responses": {
                        "200": {
                            "description": "用户资料信息",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "id": {"type": "integer"},
                                            "name": {"type": "string"},
                                            "avatar": {"type": "string"}
                                        }
                                    },
                                    "example": "{\"id\": 42, \"name\": \"移动用户\", \"avatar\": \"https://example.com/avatar.jpg\"}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }"#;
    
    println!("   📨 1. 前端接收 JSON 数据 ({} 字符)", api_response_json.len());
    
    // 模拟前端解析 DTO
    let openapi_dto: OpenApiDto = serde_json::from_str(api_response_json)?;
    
    println!("   🔍 2. 解析为 DTO 对象:");
    println!("      • API 标题: {}", openapi_dto.info.title);
    println!("      • API 版本: {}", openapi_dto.info.version);
    println!("      • 路径数量: {}", openapi_dto.paths.len());
    
    // 模拟前端提取关键信息
    if let Some(profile_path) = openapi_dto.paths.get("/users/profile") {
        if let Some(get_op) = &profile_path.get {
            println!("   📋 3. 提取操作信息:");
            println!("      • 操作描述: {:?}", get_op.summary);
            println!("      • 响应数量: {}", get_op.responses.len());
            
            if let Some(success_response) = get_op.responses.get("200") {
                println!("      • 成功响应: {}", success_response.description);
                
                if let Some(content) = &success_response.content {
                    if let Some(json_content) = content.get("application/json") {
                        println!("      • 示例数据: {:?}", json_content.example);
                    }
                }
            }
        }
    }
    
    println!();
    println!("   🎯 前端优势:");
    println!("      ✅ 无需了解 AST 复杂性");
    println!("      ✅ 直接使用类型安全的 DTO");
    println!("      ✅ 支持 JSON 序列化/反序列化");
    println!("      ✅ 扩展字段自动处理");
    println!("      ✅ 可生成 Dart 类（通过 json_serializable）");
    
    println!();
    println!("   📝 Dart 代码生成示例:");
    println!("      ```dart");
    println!("      @JsonSerializable()");
    println!("      class OpenApiDto {{");
    println!("        final String openapi;");
    println!("        final InfoDto info;");
    println!("        final Map<String, PathItemDto> paths;");
    println!("        // ...");
    println!("      }}");
    println!("      ```");
    println!();
    
    Ok(())
}

/// 演示 Schema DTO 的功能
fn schema_dto_demo() {
    println!("\n🏗️  Schema DTO 架构演示");
    
    // 创建一个复杂的 Schema DTO
    let mut properties = HashMap::new();
    properties.insert("id".to_string(), SchemaDto {
        schema_type: Some(SchemaType::Integer),
        format: Some("int64".to_string()),
        description: Some("用户 ID".to_string()),
        ..Default::default()
    });
    
    properties.insert("name".to_string(), SchemaDto {
        schema_type: Some(SchemaType::String),
        min_length: Some(1),
        max_length: Some(100),
        description: Some("用户名".to_string()),
        ..Default::default()
    });
    
    properties.insert("email".to_string(), SchemaDto {
        schema_type: Some(SchemaType::String),
        format: Some("email".to_string()),
        description: Some("邮箱地址".to_string()),
        ..Default::default()
    });
    
    // 创建嵌套的地址 Schema
    let mut address_properties = HashMap::new();
    address_properties.insert("street".to_string(), SchemaDto {
        schema_type: Some(SchemaType::String),
        description: Some("街道地址".to_string()),
        ..Default::default()
    });
    
    address_properties.insert("city".to_string(), SchemaDto {
        schema_type: Some(SchemaType::String),
        description: Some("城市".to_string()),
        ..Default::default()
    });
    
    properties.insert("address".to_string(), SchemaDto {
        schema_type: Some(SchemaType::Object),
        properties: Some(address_properties),
        required: Some(vec!["street".to_string(), "city".to_string()]),
        description: Some("用户地址".to_string()),
        ..Default::default()
    });
    
    // 创建用户 Schema
    let user_schema = SchemaDto {
        schema_type: Some(SchemaType::Object),
        title: Some("User".to_string()),
        description: Some("用户信息".to_string()),
        properties: Some(properties),
        required: Some(vec!["id".to_string(), "name".to_string(), "email".to_string()]),
        external_docs: Some(ExternalDocsDto {
            url: "https://api.example.com/docs/user".to_string(),
            description: Some("用户文档".to_string()),
            extensions: Default::default(),
        }),
        example: Some("{\n  \"id\": 12345,\n  \"name\": \"张三\",\n  \"email\": \"zhangsan@example.com\",\n  \"address\": {\n    \"street\": \"中关村大街1号\",\n    \"city\": \"北京\"\n  }\n}".to_string()),
        ..Default::default()
    };
    
    // 序列化为 JSON
    match serde_json::to_string_pretty(&user_schema) {
        Ok(json_str) => {
            println!("✅ Schema DTO 序列化成功:");
            println!("{}", json_str);
        },
        Err(e) => {
            println!("❌ Schema DTO 序列化失败: {}", e);
        }
    }
    
    // 演示从 JSON 反序列化
    let json_input = r#"
    {
        "type": "object",
        "description": "用户信息",
        "properties": {
            "id": {
                "type": "integer",
                "format": "int64"
            },
            "name": {
                "type": "string",
                "minLength": 1,
                "maxLength": 100
            }
        },
        "required": ["id", "name"]
    }
    "#;
    
    match serde_json::from_str::<SchemaDto>(json_input) {
        Ok(schema) => {
            println!("\n✅ 从 JSON 反序列化 Schema DTO 成功:");
            println!("  类型: {:?}", schema.schema_type);
            println!("  描述: {:?}", schema.description);
            if let Some(properties) = &schema.properties {
                println!("  属性数量: {}", properties.len());
                for (name, prop_schema) in properties {
                    println!("    {}: {:?}", name, prop_schema.schema_type);
                }
            }
            if let Some(required) = &schema.required {
                println!("  必需字段: {:?}", required);
            }
        },
        Err(e) => {
            println!("❌ 从 JSON 反序列化失败: {}", e);
        }
    }
    
    println!("🔧 Schema DTO 转换功能演示完成");
}

/// 辅助函数：将 JSON 值转换为 AST Element（简化版）
fn json_to_element(value: serde_json::Value) -> Element {
    match value {
        serde_json::Value::Null => Element::Null(NullElement::default()),
        serde_json::Value::Bool(b) => Element::Boolean(BooleanElement::new(b)),
        serde_json::Value::Number(n) => {
            Element::Number(NumberElement {
                element: "number".to_string(),
                meta: MetaElement::default(),
                attributes: AttributesElement::default(),
                content: n.as_f64().unwrap_or(0.0),
            })
        },
        serde_json::Value::String(s) => Element::String(StringElement::new(&s)),
        serde_json::Value::Array(arr) => {
            Element::Array(ArrayElement {
                element: "array".to_string(),
                meta: MetaElement::default(),
                attributes: AttributesElement::default(),
                content: arr.into_iter().map(json_to_element).collect(),
            })
        },
        serde_json::Value::Object(obj) => {
            let mut object = ObjectElement::new();
            for (key, value) in obj {
                object.set(&key, json_to_element(value));
            }
            Element::Object(object)
        },
    }
} 